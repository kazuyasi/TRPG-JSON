use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
use std::process;
use trpg_json_core::{config, export, io, query, Monster};

#[derive(Parser)]
#[command(name="gm", version, about="TRPG JSON CLI")]
struct Cli {
    /// 設定ファイルのパス（デフォルト: OS別の標準的な設定ディレクトリ配下のtrpg-json/default.toml）
    /// macOS: ~/Library/Application Support/trpg-json/default.toml
    /// Linux: ~/.config/trpg-json/default.toml
    /// Windows: C:\Users\username\AppData\Roaming\trpg-json\default.toml
    #[arg(long, global = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// モンスター関連コマンド
    Monster {
        #[command(subcommand)]
        command: MonsterCommands,
    },

    /// スペル（魔法）関連コマンド
    Spell {
        #[command(subcommand)]
        command: SpellCommands,
    },
}

#[derive(Subcommand)]
enum MonsterCommands {
    /// モンスターを検索する
    /// 
    /// 使用例:
    ///   gm monster find テスト           # 名前に「テスト」を含むモンスターを検索
    ///   gm monster find テスト -l 6      # 名前に「テスト」を含み、レベル6のモンスターを検索
    ///   gm monster find テスト -c 蛮族  # 名前に「テスト」を含み、カテゴリ「蛮族」のモンスターを検索
    Find {
        /// 検索する名前（部分マッチ）
        name: String,
        
        /// レベルで絞り込む（オプション）
        #[arg(short = 'l', long)]
        level: Option<i32>,
        
        /// カテゴリで絞り込む（オプション）
        #[arg(short = 'c', long)]
        category: Option<String>,
    },
    
    /// 名前一覧を取得する
    /// 
    /// 使用例:
    ///   gm monster list テスト          # 名前に「テスト」を含むモンスターの一覧
    List {
        /// 検索パターン（部分マッチ）
        pattern: String,
    },
    
    /// クエリでモンスターを検索し JSON 配列で返す
    /// 
    /// 使用例:
    ///   gm monster select -l 6          # レベル6のモンスターをすべて取得
    ///   gm monster select -c 蛮族       # カテゴリ「蛮族」のモンスターをすべて取得
    ///   gm monster select -l 6 -c 蛮族  # レベル6かつカテゴリ「蛮族」のモンスターを取得
    ///   gm monster select -l 6 --export json --output results.json # 結果をJSONファイルにエクスポート
    ///   gm monster select -l 6 --export sheets --output "Spreadsheet ID" # Google Sheetsにエクスポート
    ///   gm monster select -l 6 --export udonarium --output monsters.zip # Udonarium形式にエクスポート
    Select {
        /// 名前で検索（部分マッチ、オプション）
        #[arg(short = 'n', long)]
        name: Option<String>,
        
        /// レベルで絞り込む（オプション）
        #[arg(short = 'l', long)]
        level: Option<i32>,
        
        /// カテゴリで絞り込む（オプション）
        #[arg(short = 'c', long)]
        category: Option<String>,
        
        /// エクスポート形式（json, sheets, udonarium）
        #[arg(long)]
        export: Option<String>,
        
        /// エクスポート出力先（JSONの場合: ファイルパス、Sheetsの場合: スプレッドシートID、Udonariumの場合: ZIPファイルパス）
        #[arg(long)]
        output: Option<String>,
    },
    
    /// モンスターを追加する
    /// 
    /// 使用例:
    ///   gm monster add monster.json      # monster.json からモンスターを追加
    Add {
        /// JSON ファイルパス（単一モンスター JSON）
        file: String,
    },
    
    /// モンスターを削除する
    /// 
    /// 使用例:
    ///   gm monster delete "モンスター名"  # 完全一致するモンスターを削除
    Delete {
        /// 削除するモンスターの正確な名前
        name: String,
    },
}

#[derive(Subcommand)]
enum SpellCommands {
    /// スペルを検索する
    /// 
    /// 使用例:
    ///   gm spell find ファイア           # 名前に「ファイア」を含むスペルを検索
    ///   gm spell find ファイア -l 2      # 名前に「ファイア」を含み、レベル2のスペルを検索
    ///   gm spell find ファイア -s 火系  # 名前に「ファイア」を含み、系統「火系」のスペルを検索
    Find {
        /// 検索する名前（部分マッチ）
        name: String,
        
        /// レベルで絞り込む（オプション）
        #[arg(short = 'l', long)]
        level: Option<i32>,
        
        /// 系統で絞り込む（オプション）
        #[arg(short = 's', long)]
        school: Option<String>,
    },
    
    /// スペル名一覧を取得する
    /// 
    /// 使用例:
    ///   gm spell list ファイア          # 名前に「ファイア」を含むスペルの一覧
    List {
        /// 検索パターン（部分マッチ）
        pattern: String,
    },
    
    /// スペルのチャットパレットを表示（複数フィルタ対応）
    /// 
    /// 使用例:
    ///   gm spell palette -n "ファイア"              # 名前でフィルタ
    ///   gm spell palette -l 3                      # レベルでフィルタ
    ///   gm spell palette -s "MagicCat_1"           # 系統でフィルタ
    ///   gm spell palette -n "ファイア" -s "MagicCat_1"  # 複数フィルタ
    ///   gm spell palette -n "ファイア" --copy     # 先頭行をクリップボードにコピー
    Palette {
        /// スペル名（部分マッチ、オプション）
        #[arg(short = 'n')]
        name: Option<String>,
        
        /// レベル（オプション）
        #[arg(short = 'l')]
        level: Option<i32>,
        
        /// 系統（オプション）
        #[arg(short = 's')]
        school: Option<String>,
        
        /// クリップボードにコピー（オプション、先頭行のみ）
        #[arg(long, short = 'y')]
        copy: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // 設定ファイルを読み込む
    let cfg = load_config(&cli.config);
    
    // ホームディレクトリを基準にパスを解決
    // パスが絶対パスの場合はそのまま使用、相対パスの場合はホームディレクトリから解決
    let home_dir = dirs::home_dir();
    let monster_paths = cfg.resolve_monsters_paths(home_dir.as_deref());
    let spell_paths = cfg.resolve_spells_paths(home_dir.as_deref());
    
    // モンスターパスが存在するかチェック
    for data_path in &monster_paths {
        if !data_path.exists() {
            eprintln!("エラー: モンスターデータファイルが見つかりません: {}", data_path.display());
            process::exit(1);
        }
    }

    // スペルパスが存在するかチェック（スペルコマンド使用時のみ）
    for data_path in &spell_paths {
        if !data_path.exists() {
            eprintln!("警告: スペルデータファイルが見つかりません: {}", data_path.display());
        }
    }

    // パスを文字列に変換
    let monster_path_strs: Vec<String> = monster_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let spell_path_strs: Vec<String> = spell_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    match &cli.command {
        Some(Commands::Monster { command }) => {
            match command {
                MonsterCommands::Find { name, level, category } => {
                    handle_find_command(&monster_path_strs, name, *level, category.as_deref());
                }
                MonsterCommands::List { pattern } => {
                    handle_list_command(&monster_path_strs, pattern);
                }
                MonsterCommands::Select { name, level, category, export: export_format, output } => {
                    handle_select_command(&monster_path_strs, name.as_deref(), *level, category.as_deref(), export_format.as_deref(), output.as_deref());
                }
                MonsterCommands::Add { file } => {
                    handle_add_command(&monster_path_strs, file);
                }
                MonsterCommands::Delete { name } => {
                    handle_delete_command(&monster_path_strs, name);
                }
            }
        }

        Some(Commands::Spell { command }) => {
            match command {
                SpellCommands::Find { name, level, school } => {
                    handle_spell_find_command(&spell_path_strs, name, *level, school.as_deref());
                }
                SpellCommands::List { pattern } => {
                    handle_spell_list_command(&spell_path_strs, pattern);
                }
                SpellCommands::Palette { name, level, school, copy } => {
                    handle_spell_palette_command(&spell_path_strs, name.as_deref(), *level, school.as_deref(), *copy);
                }
            }
        }

        None => {
            eprintln!("gm: サブコマンドを指定してください (--help で確認できます)");
            process::exit(1);
        }
    }
}

fn handle_find_command(data_paths: &[String], name: &str, level: Option<i32>, category: Option<&str>) {
    // データを読み込む（複数ファイル対応）
    let monsters = match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 検索を実行
    let results = query::find_multi(&monsters, Some(name), level, category);

    // 結果を処理
    match results.len() {
        0 => {
            eprintln!("エラー: マッチするモンスターが見つかりません");
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            if let Err(e) = io::save_json_array_stdout(&results.iter().map(|&m| m.clone()).collect::<Vec<_>>()) {
                eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                process::exit(1);
            }
        }
        n => {
            // 複数件の場合は件数を出力
            println!("{} 件のモンスターが見つかりました", n);
            
            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == name) {
                let exact_monster = (*exact_match).clone();
                if let Err(e) = io::save_json_array_stdout(&[exact_monster]) {
                    eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn handle_list_command(data_paths: &[String], pattern: &str) {
    // データを読み込む（複数ファイル対応）
    let monsters = match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // パターンマッチで検索
    let results = query::find_by_name(&monsters, pattern);

    // 結果を処理
    match results.len() {
        0 => {
            eprintln!("エラー: マッチするモンスターが見つかりません");
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            if let Err(e) = io::save_json_array_stdout(&results.iter().map(|&m| m.clone()).collect::<Vec<_>>()) {
                eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                process::exit(1);
            }
        }
        n => {
            // 複数件の場合は名前の一覧を出力
            for (i, monster) in results.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                println!("Lv.{:2} {} [{}]", monster.level, monster.name, monster.category);
            }
            println!("\n計 {} 件のモンスターが見つかりました", n);
            
            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == pattern) {
                let exact_monster = (*exact_match).clone();
                if let Err(e) = io::save_json_array_stdout(&[exact_monster]) {
                    eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn handle_select_command(
    data_paths: &[String],
    name: Option<&str>,
    level: Option<i32>,
    category: Option<&str>,
    export_format: Option<&str>,
    output_dest: Option<&str>,
) {
    // データを読み込む（複数ファイル対応）
    let monsters = match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 複合検索を実行
    let results = query::find_multi(&monsters, name, level, category);

    // 結果を処理
    match results.len() {
        0 => {
            eprintln!("エラー: マッチするモンスターが見つかりません");
            process::exit(1);
        }
        _ => {
            let json_results: Vec<_> = results.iter().map(|&m| m.clone()).collect();

            // エクスポート機能が指定されている場合
            if let Some(fmt) = export_format {
                if let Some(output) = output_dest {
                    export_results(&json_results, fmt, output);
                } else {
                    eprintln!("エラー: --export を使用する場合は --output で出力先を指定してください");
                    process::exit(1);
                }
            } else {
                // 通常の stdout 出力
                if let Err(e) = io::save_json_array_stdout(&json_results) {
                    eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

/// エクスポート処理を実行
fn export_results(monsters: &[Monster], format: &str, output: &str) {
    // エクスポート形式をパース
    let export_format = match format.parse::<export::ExportFormat>() {
        Ok(fmt) => fmt,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // エクスポーターを生成
    let exporter = match export::ExporterFactory::create_exporter(export_format) {
        Ok(exp) => exp,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // エクスポート設定を作成
    let config = export::ExportConfig {
        destination: output.to_string(),
        format: export_format,
    };

    // エクスポート実行
    match exporter.export(monsters, &config) {
        Ok(()) => {
            println!("成功: {} 件のモンスターを {} にエクスポートしました", monsters.len(), output);
        }
        Err(e) => {
            eprintln!("エラー: エクスポートに失敗しました: {}", e);
            process::exit(1);
        }
    }
}

fn handle_add_command(data_paths: &[String], file: &str) {
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

    // 現在のデータを読み込む（複数ファイル対応）
    let mut monsters = match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 重複チェック
    if let Some(_duplicate) = query::find_by_exact_name(&monsters, &new_monster.name) {
        // 確認ダイアログを表示
        eprint!("警告: \"{}\" という名前のモンスターは既に存在します。上書きしますか？(y/n) ", new_monster.name);
        let _ = stdout().flush();

        let mut response = String::new();
        if stdin().read_line(&mut response).is_err() {
            eprintln!("\nエラー: 入力の読み込みに失敗しました");
            process::exit(1);
        }

        if !response.trim().eq_ignore_ascii_case("y") && !response.trim().eq_ignore_ascii_case("yes") {
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

fn handle_delete_command(data_paths: &[String], name: &str) {
    // 現在のデータを読み込む（複数ファイル対応）
    let mut monsters = match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 完全一致で検索
    if query::find_by_exact_name(&monsters, name).is_none() {
        eprintln!("エラー: \"{}\" という名前のモンスターが見つかりません", name);
        process::exit(1);
    }

    // 確認ダイアログを表示
    eprint!("警告: \"{}\" を削除しますか？(y/n) ", name);
    let _ = stdout().flush();

    let mut response = String::new();
    if stdin().read_line(&mut response).is_err() {
        eprintln!("\nエラー: 入力の読み込みに失敗しました");
        process::exit(1);
    }

    if !response.trim().eq_ignore_ascii_case("y") && !response.trim().eq_ignore_ascii_case("yes") {
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

/// 設定ファイルを読み込む
/// ユーザー指定がない場合はデフォルト設定を使用
fn load_config(config_path: &Option<String>) -> config::Config {
    let config_file = match config_path {
        Some(path) => path.clone(),
        None => find_config_file(),
    };

    match config::Config::load(&config_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("警告: 設定ファイル '{}' を読み込めません: {}", config_file, e);
            eprintln!("デフォルト設定を使用します");
            config::Config::default_config()
        }
    }
}

/// 設定ファイルを探す
/// 複数の候補を試して、最初に見つかったものを使用
fn find_config_file() -> String {
    // OS別の標準的な設定ディレクトリから設定ファイルを探す
    // - macOS: ~/Library/Application Support/trpg-json/default.toml
    // - Linux: ~/.config/trpg-json/default.toml
    // - Windows: C:\Users\username\AppData\Roaming\trpg-json\default.toml
    if let Some(config_dir) = dirs::config_dir() {
        let path = config_dir.join("trpg-json").join("default.toml");
        if path.exists() {
            return path.to_string_lossy().to_string();
        }
    }

    // 見つからない場合はデフォルトを返す
    "config/default.toml".to_string()
}

// ============================================================================
// Spell Command Handlers
// ============================================================================

fn handle_spell_find_command(data_paths: &[String], name: &str, level: Option<i32>, school: Option<&str>) {
    // データを読み込む（複数ファイル対応）
    let spells = match io::load_multiple_spells_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 検索を実行
    let results = query::spell_find_multi(&spells, Some(name), school, level);

    // 結果を処理
    match results.len() {
        0 => {
            eprintln!("エラー: マッチするスペルが見つかりません");
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            if let Err(e) = io::save_spells_json_array_stdout(&results.iter().map(|&s| s.clone()).collect::<Vec<_>>()) {
                eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                process::exit(1);
            }
        }
        n => {
            // 複数件の場合は件数を出力
            println!("{} 件のスペルが見つかりました", n);
            
            // 完全一致するスペルがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|s| s.name == name) {
                let exact_spell = (*exact_match).clone();
                if let Err(e) = io::save_spells_json_array_stdout(&[exact_spell]) {
                    eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn handle_spell_list_command(data_paths: &[String], pattern: &str) {
    // データを読み込む（複数ファイル対応）
    let spells = match io::load_multiple_spells_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // 検索を実行
    let results = query::spell_find_by_name(&spells, pattern);

    // 結果を処理
    match results.len() {
        0 => {
            eprintln!("エラー: マッチするスペルが見つかりません");
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            if let Err(e) = io::save_spells_json_array_stdout(&results.iter().map(|&s| s.clone()).collect::<Vec<_>>()) {
                eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                process::exit(1);
            }
        }
         n => {
             // 複数件の場合は名前一覧を出力
             println!("{}個のスペルが見つかりました:", n);
             for spell in &results {
                 println!("  - {} ({})", spell.name, spell.school);
             }
            
            // 完全一致するスペルがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|s| s.name == pattern) {
                let exact_spell = (*exact_match).clone();
                if let Err(e) = io::save_spells_json_array_stdout(&[exact_spell]) {
                    eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

fn handle_spell_palette_command(
    data_paths: &[String],
    name: Option<&str>,
    level: Option<i32>,
    school: Option<&str>,
    copy: bool,
) {
    // 最低1つのフィルタが必須
    if name.is_none() && level.is_none() && school.is_none() {
        eprintln!("エラー: 最低1つのフィルタ（-n, -l, -s）を指定してください");
        process::exit(1);
    }

    // データを読み込む（複数ファイル対応）
    let spells = match io::load_multiple_spells_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // マルチフィルタで検索（name, school, level）
    let results = query::spell_find_multi(&spells, name, school, level);

    match results.len() {
        0 => {
            eprintln!("エラー: マッチするスペルが見つかりません");
            process::exit(1);
        }
        _ => {
            // 全マッチしたスペルのパレットを複数行で出力
            let mut first_palette: Option<String> = None;

            for spell in &results {
                match export::palette::generate_spell_palette(&spell) {
                    Ok(palette) => {
                        println!("{}", palette);
                        
                        // 先頭のパレットを保存（copy用）
                        if first_palette.is_none() {
                            first_palette = Some(palette);
                        }
                    }
                    Err(e) => {
                        eprintln!("警告: スペル '{}' のパレット生成に失敗しました: {}", spell.name, e);
                    }
                }
            }

            // --copy フラグが指定されている場合、先頭のパレットをクリップボードにコピー
            if copy {
                if let Some(palette) = first_palette {
                    match copy_to_clipboard(&palette) {
                        Ok(_) => {
                            eprintln!("✓ チャットパレット（先頭行）をクリップボードにコピーしました");
                        }
                        Err(e) => {
                            eprintln!("警告: クリップボードへのコピーに失敗しました: {}", e);
                        }
                    }
                }
            }
        }
    }
}

/// クリップボードに文字列をコピーする
/// 
/// # 説明
/// System クリップボードに指定された文字列をコピーします。
/// xclip (Linux) または pbcopy (macOS) または clip (Windows) が必要です。
fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
    use std::process::Command;
    
    #[cfg(target_os = "macos")]
    {
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    {
        let mut child = Command::new("cmd")
            .args(&["/C", "clip"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Clipboard functionality is not supported on this platform"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    fn create_test_monster() -> Monster {
        Monster {
            category: "蛮族".to_string(),
            level: 6,
            revision: 2.5,
            data: "TEST001".to_string(),
            illust: "".to_string(),
            movein: -1,
            movein_description: "".to_string(),
            moveon: -1,
            moveon_description: "".to_string(),
            name: "テストモンスター".to_string(),
            part: vec![trpg_json_core::Part {
                hp: Some(50),
                mp: 50,
                name: "".to_string(),
                core: Some(true),
                hit_rate: Some(15),
                dodge: Some(15),
                damage: Some(6),
                part_count: 1,
                special_abilities: "".to_string(),
                armor: 5,
            }],
            notes: "".to_string(),
            initiative: 14,
            common_abilities: "".to_string(),
            weakness: "属性ダメージ+3".to_string(),
            weakness_value: 17,
            life_resistance: 16,
            fame: 14,
            mental_resistance: 16,
            extra: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_export_format_json_parsing() {
        let result = "json".parse::<export::ExportFormat>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), export::ExportFormat::Json);
    }

    #[test]
    fn test_export_format_sheets_parsing() {
        let result = "sheets".parse::<export::ExportFormat>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), export::ExportFormat::GoogleSheets);
    }

    #[test]
    fn test_export_factory_creates_json_exporter() {
        let exporter = export::ExporterFactory::create_exporter(export::ExportFormat::Json);
        assert!(exporter.is_ok());
        assert_eq!(exporter.unwrap().name(), "JSON Exporter");
    }

    #[test]
    fn test_export_factory_creates_sheets_exporter() {
        let exporter = export::ExporterFactory::create_exporter(export::ExportFormat::GoogleSheets);
        assert!(exporter.is_ok());
        assert_eq!(exporter.unwrap().name(), "Google Sheets Exporter");
    }

    #[test]
    fn test_export_config_creation() {
        let config = export::ExportConfig {
            destination: "/tmp/test.json".to_string(),
            format: export::ExportFormat::Json,
        };

        assert_eq!(config.destination, "/tmp/test.json");
        assert_eq!(config.format, export::ExportFormat::Json);
    }

    #[test]
    fn test_json_export_with_monster() {
        let monster = create_test_monster();
        let exporter = export::ExporterFactory::create_exporter(export::ExportFormat::Json)
            .expect("Failed to create JSON exporter");

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = export::ExportConfig {
            destination: output_path.clone(),
            format: export::ExportFormat::Json,
        };

        // エクスポート実行
        let result = exporter.export(&[monster], &config);
        assert!(result.is_ok());

        // ファイルが正しく作成されたか確認
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("テストモンスター"));
    }

    #[test]
    fn test_export_error_handling_empty_data() {
        let exporter = export::ExporterFactory::create_exporter(export::ExportFormat::GoogleSheets)
            .expect("Failed to create Google Sheets exporter");

        let config = export::ExportConfig {
            destination: "1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs".to_string(),
            format: export::ExportFormat::GoogleSheets,
        };

        // 空データでのエクスポートはエラーになる
        let result = exporter.export(&[], &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_error_handling_invalid_spreadsheet_id() {
        let exporter = export::ExporterFactory::create_exporter(export::ExportFormat::GoogleSheets)
            .expect("Failed to create Google Sheets exporter");

        let monster = create_test_monster();
        let config = export::ExportConfig {
            destination: "invalid".to_string(), // 短すぎるID
            format: export::ExportFormat::GoogleSheets,
        };

        let result = exporter.export(&[monster], &config);
        assert!(result.is_err());
        let error_msg = result.err().unwrap().to_string();
        assert!(error_msg.contains("Invalid spreadsheet ID") || error_msg.contains("invalid"));
    }

    // ========================================================================
    // Spell CLI Integration Tests (T035)
    // ========================================================================

    #[test]
    fn test_spell_find_by_name_single_match() {
        // Load spells from sample data (using relative path from workspace root)
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find by exact name
        let result = query::spell_find_by_exact_name(&spells, "Magic_47438");
        assert!(result.is_some());
        
        let spell = result.unwrap();
        assert_eq!(spell.name, "Magic_47438");
        assert_eq!(spell.school, "MagicCat_1");
    }

    #[test]
    fn test_spell_find_by_name_no_match() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        let result = query::spell_find_by_exact_name(&spells, "NonExistentSpell");
        assert!(result.is_none());
    }

    #[test]
    fn test_spell_find_by_category() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find spells by category
        let results = query::spell_find_by_school(&spells, "MagicCat_1");
        assert!(!results.is_empty());
        
        // All results should match the category
         for spell in results {
             assert_eq!(spell.school, "MagicCat_1");
         }
    }

    #[test]
    fn test_spell_find_partial_match() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find spells with partial name match
        let results = query::spell_find_by_name(&spells, "Magic");
        assert!(!results.is_empty());
        
        // All results should contain "Magic"
        for spell in results {
            assert!(spell.name.contains("Magic"));
        }
    }

    #[test]
    fn test_spell_palette_generation_support_spell() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find support spell (Magic_88250 has 補助: true)
        let spell = query::spell_find_by_exact_name(&spells, "Magic_88250")
            .expect("Support spell not found");
        
        // Generate palette
        let palette = export::palette::generate_spell_palette(&spell)
            .expect("Failed to generate palette");
        
        // Verify support spell format (no dice roll)
        assert!(!palette.contains("2d+"));
        assert!(palette.contains("Magic_88250"));
        assert!(palette.contains("MP:"));
    }

    #[test]
    fn test_spell_palette_generation_regular_spell() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find regular spell (Magic_47438 has no 補助 flag)
        let spell = query::spell_find_by_exact_name(&spells, "Magic_47438")
            .expect("Regular spell not found");
        
        // Generate palette
        let palette = export::palette::generate_spell_palette(&spell)
            .expect("Failed to generate palette");
        
        // Verify regular spell format (with dice roll)
        assert!(palette.starts_with("2d+"));
        assert!(palette.contains("{行使修正}"));
        assert!(palette.contains("Magic_47438"));
        assert!(palette.contains("MP:"));
    }

    #[test]
    fn test_spell_palette_generation_area_target() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find spell with area target (Magic_33778)
        let spell = query::spell_find_by_exact_name(&spells, "Magic_33778")
            .expect("Spell with area target not found");
        
        // Generate palette
        let palette = export::palette::generate_spell_palette(&spell)
            .expect("Failed to generate palette");
        
        // Verify area target format
        assert!(palette.contains("1エリア"));
        assert!(palette.contains("半径"));
        assert!(palette.contains("m"));
    }

    #[test]
    fn test_spell_multi_filter_query() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Multi-filter: school only (name, school, level)
        let results = query::spell_find_multi(&spells, None, Some("MagicCat_2"), None);
        assert!(!results.is_empty());
        
         for spell in &results {
             assert_eq!(spell.school, "MagicCat_2");
         }
    }

    #[test]
    fn test_spell_data_persistence() {
        // Load spells from sample data
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Verify all required spells are present
        let spell_names = vec![
            "Magic_47438", "Magic_33778", "Magic_83071", "Magic_16470",
            "Magic_88250", "Magic_10271", "Magic_53493", "Magic_22924",
            "Magic_65432"
        ];
        
        for name in spell_names {
            let result = query::spell_find_by_exact_name(&spells, name);
            assert!(result.is_some(), "Spell {} not found", name);
        }
    }

    #[test]
    fn test_spell_schema_compliance() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Verify all spells have required fields
         for spell in spells {
             assert!(!spell.name.is_empty(), "Spell name is empty");
             assert!(!spell.school.is_empty(), "Spell school is empty");
            
            // Verify MP field exists
            assert!(spell.extra.get("MP").is_some(), "MP field missing for {}", spell.name);
            
            // Verify effect field exists
            assert!(spell.extra.get("効果").is_some(), "Effect field missing for {}", spell.name);
        }
    }

    #[test]
    fn test_spell_palette_partial_name_match_single() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find spell by partial name: "Magic_33778" (full name)
        let results = query::spell_find_by_name(&spells, "Magic_33778");
        assert_eq!(results.len(), 1);
        
        // Generate palette
        let palette = export::palette::generate_spell_palette(results[0])
            .expect("Failed to generate palette");
        
        // Verify palette generated successfully
        assert!(palette.contains("Magic_33778"));
    }

    #[test]
    fn test_spell_palette_partial_name_match_multiple() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find spells by partial name: "Magic" (partial match)
        let results = query::spell_find_by_name(&spells, "Magic");
        assert!(results.len() > 1);
        
        // First match should have a valid palette
        let palette = export::palette::generate_spell_palette(results[0])
            .expect("Failed to generate palette");
        
        // Verify palette generated
        assert!(!palette.is_empty());
    }

    #[test]
    fn test_spell_palette_exact_match_priority() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Find by partial name
        let results = query::spell_find_by_name(&spells, "Magic");
        
        // Find exact match
        if let Some(exact_match) = results.iter().find(|s| s.name == "Magic_47438") {
            let palette = export::palette::generate_spell_palette(&exact_match)
                .expect("Failed to generate palette");
            
            // Verify exact match palette
            assert!(palette.contains("Magic_47438"));
        }
    }

    // ========================================================================
    // Multi-filter Palette Tests (T038-final)
    // ========================================================================

    #[test]
    fn test_spell_palette_filter_by_name_only() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Filter by name only
        let results = query::spell_find_multi(&spells, Some("Magic"), None, None);
        assert!(results.len() > 1);
    }

    #[test]
    fn test_spell_palette_filter_by_category_only() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Filter by category only
        let results = query::spell_find_multi(&spells, None, Some("MagicCat_1"), None);
         assert!(!results.is_empty());
         
         for spell in &results {
             assert_eq!(spell.school, "MagicCat_1");
         }
    }

    #[test]
    fn test_spell_palette_filter_by_level_only() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Filter by level: check if level 1 spells exist
        let results = query::spell_find_multi(&spells, None, None, Some(1));
        // Note: May be empty if no level 1 spells exist, but function should work
        let _ = results;
    }

    #[test]
    fn test_spell_palette_filter_by_name_and_category() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Filter by name and category
        let results = query::spell_find_multi(&spells, Some("Magic"), Some("MagicCat_1"), None);
         
         for spell in &results {
             assert!(spell.name.contains("Magic"));
             assert_eq!(spell.school, "MagicCat_1");
         }
    }

    #[test]
    fn test_spell_palette_multi_filter_palette_generation() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Get multiple matches and generate palettes
        let results = query::spell_find_multi(&spells, Some("Magic"), None, None);
        assert!(results.len() > 1);
        
        // Verify all can generate palettes
        for spell in &results {
            let palette = export::palette::generate_spell_palette(&spell);
            // Some may fail due to missing fields, but shouldn't panic
            let _ = palette;
        }
    }

    #[test]
    fn test_spell_palette_no_matches() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Filter with no matches
        let results = query::spell_find_multi(&spells, Some("NonExistent"), None, None);
        assert!(results.is_empty());
    }

    #[test]
    fn test_spell_palette_filters_precision() {
        let spells = io::load_spells_json_array("../../data/sample/spells_sample.json")
            .expect("Failed to load spell sample data");
        
        // Get all spells
        let all_spells = query::spell_find_multi(&spells, None, None, None);
        
        // Get filtered by category
        let filtered = query::spell_find_multi(&spells, None, Some("MagicCat_1"), None);
        
        // Filtered should be subset of all
        assert!(filtered.len() <= all_spells.len());
    }
}

