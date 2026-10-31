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

    // 現在のデータを読み込む
    let mut monsters = utils::load_monsters_or_exit(data_paths);

    // 重複チェック
    if let Some(_duplicate) = query::find_by_exact_name(&monsters, &new_monster.name) {
        // 確認ダイアログを表示
        if !utils::confirm_action(&format!(
            "\"{}\" という名前のモンスターは既に存在します。上書きしますか？",
            new_monster.name
        )) {
            eprintln!("キャンセルされました");
            process::exit(1);
        }

        // 既存のモンスターを削除
        monsters.retain(|m| m.name != new_monster.name);
    }

    // モンスターを追加
    monsters.push(new_monster.clone());

    // ファイルに保存（最初のファイルに保存）
    if let Err(e) = io::save_json_array_file(&data_paths[0], &monsters) {
        eprintln!("エラー: ファイルに保存できません: {}", e);
        process::exit(1);
    }

    println!("成功: \"{}\" を追加しました", new_monster.name);
}

/// 削除コマンドのハンドラ
pub fn handle_delete(data_paths: &[String], name: &str) {
    // 現在のデータを読み込む
    let mut monsters = utils::load_monsters_or_exit(data_paths);

    // 完全一致で検索
    if query::find_by_exact_name(&monsters, name).is_none() {
        eprintln!(
            "エラー: \"{}\" という名前のモンスターが見つかりません",
            name
        );
        process::exit(1);
    }

    // 確認ダイアログを表示
    if !utils::confirm_action(&format!("\"{}\" を削除しますか？", name)) {
        eprintln!("キャンセルされました");
        process::exit(1);
    }

    // モンスターを削除
    monsters.retain(|m| m.name != name);

    // ファイルに保存（最初のファイルに保存）
    if let Err(e) = io::save_json_array_file(&data_paths[0], &monsters) {
        eprintln!("エラー: ファイルに保存できません: {}", e);
        process::exit(1);
    }

    println!("成功: \"{}\" を削除しました", name);
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
        ccfolia_json, export_monsters, select_exact_match_if_available, CcfoliaError, ExportError,
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
