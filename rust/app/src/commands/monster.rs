use crate::utils;
use std::process;
use thiserror::Error;
use trpg_json_core::{export, io, query, stats, Monster};

/// 検索コマンドのハンドラ
pub fn handle_find(data_paths: &[String], name: &str, level: Option<i32>, category: Option<&str>) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 検索を実行
    let results = query::find_multi(&monsters, Some(name), level, category);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(Some(name), level, category);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(
                &results.iter().map(|&m| m.clone()).collect::<Vec<_>>(),
            );
        }
        n => {
            // 複数件の場合は件数を出力
            println!("{} 件のモンスターが見つかりました", n);

            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == name) {
                let exact_monster = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_monster]);
            }
        }
    }
}

/// 一覧コマンドのハンドラ
pub fn handle_list(data_paths: &[String], pattern: &str) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // パターンマッチで検索
    let results = query::find_by_name(&monsters, pattern);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(Some(pattern), None, None);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(
                &results.iter().map(|&m| m.clone()).collect::<Vec<_>>(),
            );
        }
        n => {
            // 複数件の場合は名前の一覧を出力
            for (i, monster) in results.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                println!(
                    "Lv.{:2} {} [{}]",
                    monster.level, monster.name, monster.category
                );
            }
            println!("\n計 {} 件のモンスターが見つかりました", n);

            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == pattern) {
                let exact_monster = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_monster]);
            }
        }
    }
}

/// 選択・エクスポートコマンドのハンドラ
pub fn handle_select(
    data_paths: &[String],
    name: Option<&str>,
    level: Option<i32>,
    category: Option<&str>,
    export_format: Option<&str>,
    output_dest: Option<&str>,
    part: Option<usize>,
) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 複合検索を実行
    let results = query::find_multi(&monsters, name, level, category);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(name, level, category);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        _ => {
            let json_results = select_exact_match_if_available(results, name);

            if part.is_some() && !export_format.is_some_and(is_ccfolia_format) {
                eprintln!("エラー: --part は --export ccfolia と組み合わせて使用してください");
                process::exit(1);
            }

            // エクスポート機能が指定されている場合
            if export_format.is_some_and(is_ccfolia_format) {
                // ココフォリア形式は --output 省略時にクリップボードへコピー
                export_ccfolia(&json_results, part, output_dest);
            } else if let Some(fmt) = export_format {
                if let Some(output) = output_dest {
                    export_results(&json_results, fmt, output);
                } else {
                    eprintln!(
                        "エラー: --export を使用する場合は --output で出力先を指定してください"
                    );
                    process::exit(1);
                }
            } else {
                // 通常の stdout 出力
                utils::save_json_stdout_or_exit(&json_results);
            }
        }
    }
}

fn select_exact_match_if_available(results: Vec<&Monster>, name: Option<&str>) -> Vec<Monster> {
    if let Some(target_name) = name {
        if let Some(exact_match) = results.iter().find(|m| m.name == target_name) {
            return vec![(*exact_match).clone()];
        }
    }

    results.into_iter().cloned().collect()
}

/// ココフォリア形式の指定かどうか
fn is_ccfolia_format(format: &str) -> bool {
    matches!(format.to_lowercase().as_str(), "ccfolia" | "cocofolia")
}

/// ココフォリア変換のエラー
///
/// 表示文言は CLI のエラーメッセージ（「エラー: 」以降）と同一。
#[derive(Debug, Error)]
pub enum CcfoliaError {
    #[error("\"{name}\" には部位データがありません")]
    NoParts { name: String },

    #[error("--part は 1 から指定してください")]
    PartZero,

    #[error("--part {part} は範囲外です（部位数: {}）", .part_names.len())]
    PartOutOfRange {
        part: usize,
        /// 選択可能な部位名（1始まりの番号順）
        part_names: Vec<String>,
    },

    #[error("JSON 変換に失敗しました: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// ココフォリア変換の結果（1部位ぶんの駒）
#[derive(Debug, Clone)]
pub struct CcfoliaPiece {
    /// クリップボードに貼り付ける JSON 文字列
    pub json: String,
    /// 選択された部位の表示名
    pub part_name: String,
    /// 全部位の表示名（0始まりのインデックス順）
    pub part_names: Vec<String>,
}

/// モンスター1体をココフォリアの駒 JSON に変換する
///
/// `part` は 1 始まりの部位番号。`None` の場合は最初のコア部位（なければ先頭）を選ぶ。
/// I/O や `process::exit` を行わないため、CLI と TUI の両方から使える。
pub fn ccfolia_json(monster: &Monster, part: Option<usize>) -> Result<CcfoliaPiece, CcfoliaError> {
    let characters = export::ccfolia::monster_to_characters(monster);
    if characters.is_empty() {
        return Err(CcfoliaError::NoParts {
            name: monster.name.clone(),
        });
    }

    let part_names: Vec<String> = characters.iter().map(|c| c.data.name.clone()).collect();

    let index = match part {
        Some(0) => return Err(CcfoliaError::PartZero),
        Some(n) if n > characters.len() => {
            return Err(CcfoliaError::PartOutOfRange { part: n, part_names });
        }
        Some(n) => n - 1,
        None => export::ccfolia::default_part_index(monster),
    };

    let json = export::ccfolia::to_clipboard_json(&characters[index])?;

    Ok(CcfoliaPiece {
        json,
        part_name: part_names[index].clone(),
        part_names,
    })
}

/// ココフォリア形式に変換し、クリップボード（または --output のファイル）へ出力する
///
/// ココフォリアは1回の貼り付けで1キャラクターしか取り込めないため、
/// 対象は1体のモンスターに限定し、複数部位の場合は --part で部位を選ぶ。
fn export_ccfolia(monsters: &[Monster], part: Option<usize>, output: Option<&str>) {
    let monster = match monsters {
        [m] => m,
        _ => {
            eprintln!(
                "エラー: ココフォリア形式で出力できるのは1体のみです（{} 件がマッチしました）。条件を絞り込んでください:",
                monsters.len()
            );
            for m in monsters {
                eprintln!("  Lv.{:2} {} [{}]", m.level, m.name, m.category);
            }
            process::exit(1);
        }
    };

    let piece = match ccfolia_json(monster, part) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("エラー: {}", e);
            if let CcfoliaError::PartOutOfRange { part_names, .. } = &e {
                print_ccfolia_parts(part_names);
            }
            process::exit(1);
        }
    };

    match output {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &piece.json) {
                eprintln!("エラー: ファイルに保存できません: {}", e);
                process::exit(1);
            }
            println!(
                "成功: \"{}\" をココフォリア形式で {} に保存しました",
                piece.part_name, path
            );
        }
        None => {
            if let Err(e) = utils::copy_to_clipboard(&piece.json) {
                eprintln!("エラー: クリップボードへのコピーに失敗しました: {}", e);
                process::exit(1);
            }
            println!(
                "成功: \"{}\" をココフォリア形式でクリップボードにコピーしました（ココフォリアのルームで貼り付けてください）",
                piece.part_name
            );
        }
    }

    if piece.part_names.len() > 1 {
        eprintln!(
            "情報: このモンスターは {} 部位あります。他の部位は --part <番号> で指定してください:",
            piece.part_names.len()
        );
        print_ccfolia_parts(&piece.part_names);
    }
}

fn print_ccfolia_parts(part_names: &[String]) {
    for (i, name) in part_names.iter().enumerate() {
        eprintln!("  {}: {}", i + 1, name);
    }
}

/// 汎用エクスポート（JSON / Udonarium / Google Sheets）のエラー
///
/// 表示文言は CLI のエラーメッセージ（「エラー: 」以降）と同一。
#[derive(Debug, Error)]
pub enum ExportError {
    #[error("{0}")]
    InvalidFormat(String),

    #[error("{0}")]
    CreateExporter(export::ExportError),

    #[error("エクスポートに失敗しました: {0}")]
    Export(export::ExportError),
}

/// モンスター群を指定形式で出力先へエクスポートする
///
/// I/O（ファイル書き込み・Sheets API）は行うが、標準出力への表示や
/// `process::exit` は行わないため、CLI と TUI の両方から使える。
pub fn export_monsters(monsters: &[Monster], format: &str, output: &str) -> Result<(), ExportError> {
    let export_format = format
        .parse::<export::ExportFormat>()
        .map_err(ExportError::InvalidFormat)?;

    let exporter = export::ExporterFactory::create_exporter(export_format)
        .map_err(ExportError::CreateExporter)?;

    let config = export::ExportConfig {
        destination: output.to_string(),
        format: export_format,
    };

    exporter
        .export(monsters, &config)
        .map_err(ExportError::Export)
}

/// エクスポート処理を実行（CLI 用: 結果を表示し、失敗時は終了する）
fn export_results(monsters: &[Monster], format: &str, output: &str) {
    match export_monsters(monsters, format, output) {
        Ok(()) => {
            println!(
                "成功: {} 件のモンスターを {} にエクスポートしました",
                monsters.len(),
                output
            );
        }
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    }
}

/// 追加コマンドのハンドラ
pub fn handle_add(data_paths: &[String], file: &str) {
    // JSON ファイルから新規モンスターを読み込む
    let new_monster_json = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("エラー: ファイルを読み込めません: {}", e);
            process::exit(1);
        }
    };

    let new_monster: Monster = match serde_json::from_str(&new_monster_json) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: JSON を解析できません: {}", e);
            process::exit(1);
        }
    };

    let result = add_monster_to_files(data_paths, &new_monster, |paths| {
        utils::confirm_action(&format!(
            "\"{}\" という名前のモンスターは既に存在します（{}）。上書きしますか？",
            new_monster.name,
            paths.join(", ")
        ))
    });

    match result {
        Ok(AddOutcome::Added { path }) => {
            println!("成功: \"{}\" を {} に追加しました", new_monster.name, path);
        }
        Ok(AddOutcome::Replaced { paths }) => {
            println!(
                "成功: \"{}\" を上書きしました（{}）",
                new_monster.name,
                paths.join(", ")
            );
        }
        Err(MonsterFileError::Cancelled) => {
            eprintln!("キャンセルされました");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    }
}

/// 削除コマンドのハンドラ
pub fn handle_delete(data_paths: &[String], name: &str) {
    let result = delete_monster_from_files(data_paths, name, |paths| {
        if paths.len() > 1 {
            utils::confirm_action(&format!(
                "\"{}\" は {} 個のファイルにあります（{}）。すべてから削除しますか？",
                name,
                paths.len(),
                paths.join(", ")
            ))
        } else {
            utils::confirm_action(&format!("\"{}\" を {} から削除しますか？", name, paths[0]))
        }
    });

    match result {
        Ok(paths) => {
            println!("成功: \"{}\" を削除しました（{}）", name, paths.join(", "));
        }
        Err(MonsterFileError::Cancelled) => {
            eprintln!("キャンセルされました");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    }
}

/// モンスターデータファイルの追加・削除のエラー
///
/// 表示文言は CLI のエラーメッセージ（「エラー: 」以降）と同一。
#[derive(Debug, Error)]
pub enum MonsterFileError {
    #[error("モンスターのデータファイルが設定されていません")]
    NoDataFile,

    #[error("{path} を読み込めません: {source}")]
    Load { path: String, source: io::IoError },

    #[error("{path} に保存できません: {source}")]
    Save { path: String, source: io::IoError },

    #[error("\"{0}\" という名前のモンスターが見つかりません")]
    NotFound(String),

    #[error("キャンセルされました")]
    Cancelled,
}

/// 追加の結果
#[derive(Debug, PartialEq, Eq)]
pub enum AddOutcome {
    /// 新規として最初のファイルの末尾に追加した
    Added { path: String },
    /// 既存のモンスターを、それを含む各ファイル内で置き換えた
    Replaced { paths: Vec<String> },
}

/// 各データファイルを個別に読み込む（保存時にファイルの境界を保つため）
fn load_each_file(data_paths: &[String]) -> Result<Vec<Vec<Monster>>, MonsterFileError> {
    if data_paths.is_empty() {
        return Err(MonsterFileError::NoDataFile);
    }
    data_paths
        .iter()
        .map(|path| {
            io::load_json_array(path).map_err(|source| MonsterFileError::Load {
                path: path.clone(),
                source,
            })
        })
        .collect()
}

fn save_file(path: &str, monsters: &[Monster]) -> Result<(), MonsterFileError> {
    io::save_json_array_file(path, monsters).map_err(|source| MonsterFileError::Save {
        path: path.to_string(),
        source,
    })
}

/// 名前が完全一致するモンスターを含むファイルのインデックス
fn files_containing(files: &[Vec<Monster>], name: &str) -> Vec<usize> {
    files
        .iter()
        .enumerate()
        .filter(|(_, monsters)| query::find_by_exact_name(monsters, name).is_some())
        .map(|(i, _)| i)
        .collect()
}

/// 同名の最初の項目をその位置で置き換え、以降の同名の項目は取り除く
fn replace_by_name(monsters: &mut Vec<Monster>, new_monster: &Monster) {
    let mut replaced = false;
    monsters.retain_mut(|m| {
        if m.name != new_monster.name {
            return true;
        }
        if replaced {
            return false;
        }
        *m = new_monster.clone();
        replaced = true;
        true
    });
}

/// モンスターをデータファイルへ追加する
///
/// 同名のモンスターがなければ最初のファイルの末尾に追加する。あれば `confirm` に
/// 該当ファイルのパスを渡し、承認されたら該当ファイルそれぞれで置き換える。
/// 書き換えるのは該当するファイルだけで、他のファイルには触れない。
pub fn add_monster_to_files(
    data_paths: &[String],
    new_monster: &Monster,
    confirm: impl FnOnce(&[&str]) -> bool,
) -> Result<AddOutcome, MonsterFileError> {
    let mut files = load_each_file(data_paths)?;
    let holders = files_containing(&files, &new_monster.name);

    if holders.is_empty() {
        files[0].push(new_monster.clone());
        save_file(&data_paths[0], &files[0])?;
        return Ok(AddOutcome::Added {
            path: data_paths[0].clone(),
        });
    }

    let paths: Vec<&str> = holders.iter().map(|&i| data_paths[i].as_str()).collect();
    if !confirm(&paths) {
        return Err(MonsterFileError::Cancelled);
    }

    for &i in &holders {
        replace_by_name(&mut files[i], new_monster);
        save_file(&data_paths[i], &files[i])?;
    }
    Ok(AddOutcome::Replaced {
        paths: paths.into_iter().map(String::from).collect(),
    })
}

/// 名前が完全一致するモンスターをデータファイルから削除する
///
/// `confirm` に該当ファイルのパスを渡し、承認されたら該当ファイルすべてから削除する。
/// 書き換えるのは該当するファイルだけで、他のファイルには触れない。
/// 戻り値は書き換えたファイルのパス。
pub fn delete_monster_from_files(
    data_paths: &[String],
    name: &str,
    confirm: impl FnOnce(&[&str]) -> bool,
) -> Result<Vec<String>, MonsterFileError> {
    let mut files = load_each_file(data_paths)?;
    let holders = files_containing(&files, name);

    if holders.is_empty() {
        return Err(MonsterFileError::NotFound(name.to_string()));
    }

    let paths: Vec<&str> = holders.iter().map(|&i| data_paths[i].as_str()).collect();
    if !confirm(&paths) {
        return Err(MonsterFileError::Cancelled);
    }

    for &i in &holders {
        files[i].retain(|m| m.name != name);
        save_file(&data_paths[i], &files[i])?;
    }
    Ok(paths.into_iter().map(String::from).collect())
}

/// 統計コマンドのハンドラ
pub fn handle_stats(data_paths: &[String]) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 統計情報を計算
    let stats = stats::MonsterStats::calculate(&monsters);

    // 整形して出力
    print!("{}", stats.format());
}

#[cfg(test)]
mod tests {
    use super::{
        add_monster_to_files, ccfolia_json, delete_monster_from_files, export_monsters,
        replace_by_name, select_exact_match_if_available, AddOutcome, CcfoliaError, ExportError,
        MonsterFileError,
    };
    use std::collections::HashMap;
    use trpg_json_core::{Monster, Part};

    fn sample_monster(name: &str, level: i32, category: &str) -> Monster {
        Monster {
            category: category.to_string(),
            level,
            revision: 2.5,
            data: "TEST".to_string(),
            illust: String::new(),
            movein: -1,
            movein_description: String::new(),
            moveon: -1,
            moveon_description: String::new(),
            name: name.to_string(),
            part: vec![Part {
                hp: Some(10),
                mp: -1,
                name: String::new(),
                core: Some(true),
                hit_rate: Some(5),
                dodge: Some(5),
                damage: Some(5),
                part_count: 1,
                special_abilities: String::new(),
                armor: 0,
            }],
            notes: String::new(),
            initiative: 0,
            common_abilities: String::new(),
            weakness: String::new(),
            weakness_value: 0,
            life_resistance: 0,
            fame: 0,
            mental_resistance: 0,
            extra: HashMap::new(),
        }
    }

    #[test]
    fn select_returns_only_exact_match_when_available() {
        let goblin = sample_monster("ゴブリン", 2, "蛮族");
        let goblin_scout = sample_monster("ゴブリンスカウト", 2, "蛮族");
        let results = vec![&goblin, &goblin_scout];

        let selected = select_exact_match_if_available(results, Some("ゴブリン"));

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].name, "ゴブリン");
    }

    fn multi_part_monster() -> Monster {
        let mut m = sample_monster("テストモンスター", 3, "テスト分類");
        let mut non_core = m.part[0].clone();
        non_core.name = "部位A".to_string();
        non_core.core = Some(false);
        let mut core = m.part[0].clone();
        core.name = "部位B".to_string();
        core.core = Some(true);
        m.part = vec![non_core, core];
        m
    }

    #[test]
    fn ccfolia_json_defaults_to_first_core_part() {
        let m = multi_part_monster();
        let piece = ccfolia_json(&m, None).unwrap();

        assert_eq!(piece.part_names.len(), 2);
        assert_eq!(piece.part_name, piece.part_names[1]);
        let v: serde_json::Value = serde_json::from_str(&piece.json).unwrap();
        assert_eq!(v["kind"], "character");
        assert_eq!(v["data"]["name"], piece.part_name);
    }

    #[test]
    fn ccfolia_json_selects_explicit_part() {
        let m = multi_part_monster();
        let piece = ccfolia_json(&m, Some(1)).unwrap();

        assert_eq!(piece.part_name, piece.part_names[0]);
        assert_ne!(piece.part_name, ccfolia_json(&m, None).unwrap().part_name);
    }

    #[test]
    fn ccfolia_json_rejects_part_zero() {
        let m = multi_part_monster();
        let err = ccfolia_json(&m, Some(0)).unwrap_err();

        assert!(matches!(err, CcfoliaError::PartZero));
        assert_eq!(err.to_string(), "--part は 1 から指定してください");
    }

    #[test]
    fn ccfolia_json_rejects_out_of_range_part() {
        let m = multi_part_monster();
        let err = ccfolia_json(&m, Some(3)).unwrap_err();

        assert_eq!(err.to_string(), "--part 3 は範囲外です（部位数: 2）");
        match err {
            CcfoliaError::PartOutOfRange { part, part_names } => {
                assert_eq!(part, 3);
                assert_eq!(part_names.len(), 2);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn ccfolia_json_rejects_monster_without_parts() {
        let mut m = sample_monster("テストモンスター", 1, "テスト分類");
        m.part.clear();
        let err = ccfolia_json(&m, None).unwrap_err();

        assert_eq!(err.to_string(), "\"テストモンスター\" には部位データがありません");
    }

    #[test]
    fn export_monsters_rejects_unknown_format() {
        let m = sample_monster("テストモンスター", 1, "テスト分類");
        let err = export_monsters(&[m], "unknown", "out.json").unwrap_err();

        assert!(matches!(err, ExportError::InvalidFormat(_)));
        assert!(err.to_string().starts_with("Unknown export format: 'unknown'"));
    }

    #[test]
    fn export_monsters_writes_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json");
        let m = sample_monster("テストモンスター", 1, "テスト分類");

        export_monsters(&[m], "json", path.to_str().unwrap()).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("テストモンスター"));
    }

    // ---- add / delete（ファイルごとの保存）----

    struct TwoFiles {
        _dir: tempfile::TempDir,
        paths: Vec<String>,
    }

    /// 2ファイル構成: 1つ目に A・B、2つ目に C・D
    fn two_files() -> TwoFiles {
        let dir = tempfile::tempdir().unwrap();
        let first = dir.path().join("first.json");
        let second = dir.path().join("second.json");
        trpg_json_core::io::save_json_array_file(
            &first,
            &[sample_monster("モンスターA", 1, "分類1"), sample_monster("モンスターB", 2, "分類1")],
        )
        .unwrap();
        trpg_json_core::io::save_json_array_file(
            &second,
            &[sample_monster("モンスターC", 3, "分類2"), sample_monster("モンスターD", 4, "分類2")],
        )
        .unwrap();
        TwoFiles {
            _dir: dir,
            paths: vec![
                first.to_string_lossy().to_string(),
                second.to_string_lossy().to_string(),
            ],
        }
    }

    fn names_in(path: &str) -> Vec<(String, i32)> {
        trpg_json_core::io::load_json_array(path)
            .unwrap()
            .into_iter()
            .map(|m| (m.name, m.level))
            .collect()
    }

    fn owned(v: &[(&str, i32)]) -> Vec<(String, i32)> {
        v.iter().map(|(n, l)| (n.to_string(), *l)).collect()
    }

    #[test]
    fn add_new_monster_goes_to_first_file_only() {
        let f = two_files();
        let second_before = std::fs::read_to_string(&f.paths[1]).unwrap();

        let outcome = add_monster_to_files(
            &f.paths,
            &sample_monster("モンスターE", 5, "分類3"),
            |_| panic!("新規追加で確認してはいけない"),
        )
        .unwrap();

        assert_eq!(outcome, AddOutcome::Added { path: f.paths[0].clone() });
        assert_eq!(
            names_in(&f.paths[0]),
            owned(&[("モンスターA", 1), ("モンスターB", 2), ("モンスターE", 5)])
        );
        assert_eq!(std::fs::read_to_string(&f.paths[1]).unwrap(), second_before);
    }

    #[test]
    fn add_overwrite_rewrites_only_the_file_holding_the_monster() {
        let f = two_files();
        let first_before = std::fs::read_to_string(&f.paths[0]).unwrap();
        let mut asked = Vec::new();

        let outcome = add_monster_to_files(
            &f.paths,
            &sample_monster("モンスターC", 9, "分類2"),
            |paths| {
                asked = paths.iter().map(|p| p.to_string()).collect();
                true
            },
        )
        .unwrap();

        assert_eq!(asked, vec![f.paths[1].clone()]);
        assert_eq!(outcome, AddOutcome::Replaced { paths: vec![f.paths[1].clone()] });
        // 位置を保ったまま置き換わる
        assert_eq!(
            names_in(&f.paths[1]),
            owned(&[("モンスターC", 9), ("モンスターD", 4)])
        );
        assert_eq!(std::fs::read_to_string(&f.paths[0]).unwrap(), first_before);
    }

    #[test]
    fn add_overwrite_cancelled_changes_nothing() {
        let f = two_files();
        let before: Vec<String> =
            f.paths.iter().map(|p| std::fs::read_to_string(p).unwrap()).collect();

        let err = add_monster_to_files(&f.paths, &sample_monster("モンスターA", 9, "分類1"), |_| {
            false
        })
        .unwrap_err();

        assert!(matches!(err, MonsterFileError::Cancelled));
        for (p, b) in f.paths.iter().zip(before) {
            assert_eq!(std::fs::read_to_string(p).unwrap(), b);
        }
    }

    #[test]
    fn add_overwrite_replaces_in_every_file_holding_the_name() {
        let f = two_files();
        trpg_json_core::io::save_json_array_file(
            &f.paths[1],
            &[sample_monster("モンスターA", 1, "分類1"), sample_monster("モンスターD", 4, "分類2")],
        )
        .unwrap();

        let outcome =
            add_monster_to_files(&f.paths, &sample_monster("モンスターA", 7, "分類1"), |paths| {
                assert_eq!(paths.len(), 2);
                true
            })
            .unwrap();

        assert_eq!(outcome, AddOutcome::Replaced { paths: f.paths.clone() });
        assert_eq!(
            names_in(&f.paths[0]),
            owned(&[("モンスターA", 7), ("モンスターB", 2)])
        );
        assert_eq!(
            names_in(&f.paths[1]),
            owned(&[("モンスターA", 7), ("モンスターD", 4)])
        );
    }

    #[test]
    fn delete_rewrites_only_the_file_holding_the_monster() {
        let f = two_files();
        let first_before = std::fs::read_to_string(&f.paths[0]).unwrap();

        let paths = delete_monster_from_files(&f.paths, "モンスターD", |p| {
            assert_eq!(p, [f.paths[1].as_str()]);
            true
        })
        .unwrap();

        assert_eq!(paths, vec![f.paths[1].clone()]);
        assert_eq!(names_in(&f.paths[1]), owned(&[("モンスターC", 3)]));
        // 1つ目のファイルは書き換えない（2つ目の内容が複製されない）
        assert_eq!(std::fs::read_to_string(&f.paths[0]).unwrap(), first_before);
    }

    #[test]
    fn delete_from_first_file_does_not_copy_second_file() {
        let f = two_files();
        let second_before = std::fs::read_to_string(&f.paths[1]).unwrap();

        delete_monster_from_files(&f.paths, "モンスターA", |_| true).unwrap();

        assert_eq!(names_in(&f.paths[0]), owned(&[("モンスターB", 2)]));
        assert_eq!(std::fs::read_to_string(&f.paths[1]).unwrap(), second_before);
    }

    #[test]
    fn delete_removes_from_all_files_holding_the_name() {
        let f = two_files();
        trpg_json_core::io::save_json_array_file(
            &f.paths[1],
            &[sample_monster("モンスターB", 2, "分類1"), sample_monster("モンスターD", 4, "分類2")],
        )
        .unwrap();

        let paths = delete_monster_from_files(&f.paths, "モンスターB", |p| p.len() == 2).unwrap();

        assert_eq!(paths, f.paths);
        assert_eq!(names_in(&f.paths[0]), owned(&[("モンスターA", 1)]));
        assert_eq!(names_in(&f.paths[1]), owned(&[("モンスターD", 4)]));
    }

    #[test]
    fn delete_unknown_name_and_cancel_change_nothing() {
        let f = two_files();
        let before: Vec<String> =
            f.paths.iter().map(|p| std::fs::read_to_string(p).unwrap()).collect();

        let err = delete_monster_from_files(&f.paths, "存在しない", |_| {
            panic!("見つからないときは確認しない")
        })
        .unwrap_err();
        assert_eq!(err.to_string(), "\"存在しない\" という名前のモンスターが見つかりません");

        let err = delete_monster_from_files(&f.paths, "モンスターA", |_| false).unwrap_err();
        assert!(matches!(err, MonsterFileError::Cancelled));

        for (p, b) in f.paths.iter().zip(before) {
            assert_eq!(std::fs::read_to_string(p).unwrap(), b);
        }
    }

    #[test]
    fn add_and_delete_reject_empty_data_paths() {
        let m = sample_monster("モンスターA", 1, "分類1");
        assert!(matches!(
            add_monster_to_files(&[], &m, |_| true),
            Err(MonsterFileError::NoDataFile)
        ));
        assert!(matches!(
            delete_monster_from_files(&[], "モンスターA", |_| true),
            Err(MonsterFileError::NoDataFile)
        ));
    }

    #[test]
    fn replace_by_name_keeps_position_and_drops_duplicates_in_same_file() {
        let mut v = vec![
            sample_monster("X", 1, "c"),
            sample_monster("Y", 1, "c"),
            sample_monster("X", 2, "c"),
        ];
        replace_by_name(&mut v, &sample_monster("X", 9, "c"));
        let got: Vec<(String, i32)> = v.into_iter().map(|m| (m.name, m.level)).collect();
        assert_eq!(got, owned(&[("X", 9), ("Y", 1)]));
    }

    #[test]
    fn select_keeps_partial_matches_when_no_exact_match_exists() {
        let goblin = sample_monster("ゴブリン", 2, "蛮族");
        let goblin_scout = sample_monster("ゴブリンスカウト", 2, "蛮族");
        let results = vec![&goblin, &goblin_scout];

        let selected = select_exact_match_if_available(results, Some("ゴブ"));

        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].name, "ゴブリン");
        assert_eq!(selected[1].name, "ゴブリンスカウト");
    }
}
