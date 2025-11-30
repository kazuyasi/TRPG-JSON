use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process;
use trpg_json_core::{config, io, query, Monster};

#[derive(Parser)]
#[command(name="gm", version, about="TRPG JSON CLI")]
struct Cli {
    /// 設定ファイルのパス（デフォルト: ~/.config/trpg-json/default.toml）
    #[arg(long, global = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// モンスターを検索する
    /// 
    /// 使用例:
    ///   gm find テスト           # 名前に「テスト」を含むモンスターを検索
    ///   gm find テスト -l 6      # 名前に「テスト」を含み、レベル6のモンスターを検索
    ///   gm find テスト -c 蛮族  # 名前に「テスト」を含み、カテゴリ「蛮族」のモンスターを検索
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
    ///   gm list テスト          # 名前に「テスト」を含むモンスターの一覧
    List {
        /// 検索パターン（部分マッチ）
        pattern: String,
    },
    
    /// クエリでモンスターを検索し JSON 配列で返す
    /// 
    /// 使用例:
    ///   gm select -l 6          # レベル6のモンスターをすべて取得
    ///   gm select -c 蛮族       # カテゴリ「蛮族」のモンスターをすべて取得
    ///   gm select -l 6 -c 蛮族  # レベル6かつカテゴリ「蛮族」のモンスターを取得
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
    },
    
    /// モンスターを追加する
    /// 
    /// 使用例:
    ///   gm add monster.json      # monster.json からモンスターを追加
    Add {
        /// JSON ファイルパス（単一モンスター JSON）
        file: String,
    },
    
    /// モンスターを削除する
    /// 
    /// 使用例:
    ///   gm delete "モンスター名"  # 完全一致するモンスターを削除
    Delete {
        /// 削除するモンスターの正確な名前
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // 設定ファイルを読み込む
    let cfg = load_config(&cli.config);
    
    // ホームディレクトリを基準にパスを解決
    // パスが絶対パスの場合はそのまま使用、相対パスの場合はホームディレクトリから解決
    let home_dir = std::env::var("HOME").ok().map(PathBuf::from);
    let data_paths = cfg.resolve_monsters_paths(home_dir.as_deref());
    
    // パスが存在するかチェック
    for data_path in &data_paths {
        if !data_path.exists() {
            eprintln!("エラー: データファイルが見つかりません: {}", data_path.display());
            process::exit(1);
        }
    }

    // パスを文字列に変換
    let data_path_strs: Vec<String> = data_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    match &cli.command {
        Some(Commands::Find { name, level, category }) => {
            handle_find_command(&data_path_strs, name, *level, category.as_deref());
        }
        Some(Commands::List { pattern }) => {
            handle_list_command(&data_path_strs, pattern);
        }
        Some(Commands::Select { name, level, category }) => {
            handle_select_command(&data_path_strs, name.as_deref(), *level, category.as_deref());
        }
        Some(Commands::Add { file }) => {
            handle_add_command(&data_path_strs, file);
        }
        Some(Commands::Delete { name }) => {
            handle_delete_command(&data_path_strs, name);
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

fn handle_select_command(data_paths: &[String], name: Option<&str>, level: Option<i32>, category: Option<&str>) {
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
            // すべての結果を JSON 配列で出力
            let json_results: Vec<_> = results.iter().map(|&m| m.clone()).collect();
            if let Err(e) = io::save_json_array_stdout(&json_results) {
                eprintln!("エラー: JSON 出力に失敗しました: {}", e);
                process::exit(1);
            }
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
    // ~/.config/trpg-json/default.toml を確認
    if let Ok(home) = std::env::var("HOME") {
        let path = format!("{}/.config/trpg-json/default.toml", home);
        if PathBuf::from(&path).exists() {
            return path;
        }
    }

    // 見つからない場合はデフォルトを返す
    "config/default.toml".to_string()
}

