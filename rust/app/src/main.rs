use clap::{Parser, Subcommand};
use std::process;
use trpg_json_core::config;

mod commands;
mod utils;

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
    
    /// データセット統計情報を表示する
    /// 
    /// 使用例:
    ///   gm monster stats  # モンスターデータの統計を表示
    Stats,
}

#[derive(Subcommand)]
enum SpellCommands {
    /// スペルを検索する
    /// 
    /// 使用例:
    ///   gm spell find ファイア           # 名前に「ファイア」を含むスペルを検索
    ///   gm spell find ファイア -l 2      # 名前に「ファイア」を含み、レベル2のスペルを検索
    ///   gm spell find ファイア -s 火系  # 名前に「ファイア」を含み、系統「火系」のスペルを検索
    ///   gm spell find 妖精 -r 3          # 名前に「妖精」を含み、ランク3のスペルを検索
    ///   gm spell find 神聖 -v 特殊       # 名前に「神聖」を含み、schoolVariant「特殊」のスペルを検索
    ///   gm spell find 神聖 -v 特殊 -g 神名  # schoolVariant「特殊」かつgod「神名」のスペルを検索
    Find {
        /// 検索する名前（部分マッチ）
        name: String,
        
        /// レベルで絞り込む（オプション、-rと同時指定不可）
        #[arg(short = 'l', long)]
        level: Option<i32>,
        
        /// ランクで絞り込む（オプション、-lと同時指定不可）
        #[arg(short = 'r', long)]
        rank: Option<i32>,
        
        /// 系統で絞り込む（オプション）
        #[arg(short = 's', long)]
        school: Option<String>,
        
        /// schoolVariantで絞り込む（オプション）
        #[arg(short = 'v', long)]
        school_variant: Option<String>,
        
        /// godで絞り込む（オプション）
        #[arg(short = 'g', long)]
        god: Option<String>,
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
    ///   gm spell palette -r 2                      # ランクでフィルタ
    ///   gm spell palette -s "MagicCat_1"           # 系統でフィルタ
    ///   gm spell palette -v "特殊"                 # schoolVariantでフィルタ
    ///   gm spell palette -g "神名"                 # godでフィルタ
    ///   gm spell palette -n "ファイア" -s "MagicCat_1"  # 複数フィルタ
    ///   gm spell palette -n "ファイア" --copy     # 先頭行をクリップボードにコピー
    Palette {
        /// スペル名（部分マッチ、オプション）
        #[arg(short = 'n')]
        name: Option<String>,
        
        /// レベル（オプション、-rと同時指定不可）
        #[arg(short = 'l')]
        level: Option<i32>,
        
        /// ランク（オプション、-lと同時指定不可）
        #[arg(short = 'r')]
        rank: Option<i32>,
        
        /// 系統（オプション）
        #[arg(short = 's')]
        school: Option<String>,
        
        /// schoolVariantで絞り込む（オプション）
        #[arg(short = 'v', long)]
        school_variant: Option<String>,
        
        /// godで絞り込む（オプション）
        #[arg(short = 'g', long)]
        god: Option<String>,
        
        /// クリップボードにコピー（オプション、先頭行のみ）
        #[arg(long, short = 'y')]
        copy: bool,
    },
    
    /// データセット統計情報を表示する
    /// 
    /// 使用例:
    ///   gm spell stats  # スペルデータの統計を表示
    Stats,
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
                    commands::monster::handle_find(&monster_path_strs, name, *level, category.as_deref());
                }
                MonsterCommands::List { pattern } => {
                    commands::monster::handle_list(&monster_path_strs, pattern);
                }
                MonsterCommands::Select { name, level, category, export: export_format, output } => {
                    commands::monster::handle_select(&monster_path_strs, name.as_deref(), *level, category.as_deref(), export_format.as_deref(), output.as_deref());
                }
                MonsterCommands::Add { file } => {
                    commands::monster::handle_add(&monster_path_strs, file);
                }
                MonsterCommands::Delete { name } => {
                    commands::monster::handle_delete(&monster_path_strs, name);
                }
                MonsterCommands::Stats => {
                    commands::monster::handle_stats(&monster_path_strs);
                }
            }
        }

        Some(Commands::Spell { command }) => {
            match command {
                SpellCommands::Find { name, level, rank, school, school_variant, god } => {
                    commands::spell::handle_find(&spell_path_strs, name, *level, *rank, school.as_deref(), school_variant.as_deref(), god.as_deref());
                }
                SpellCommands::List { pattern } => {
                    commands::spell::handle_list(&spell_path_strs, pattern);
                }
                SpellCommands::Palette { name, level, rank, school, school_variant, god, copy } => {
                    commands::spell::handle_palette(&spell_path_strs, name.as_deref(), *level, *rank, school.as_deref(), school_variant.as_deref(), god.as_deref(), *copy);
                }
                SpellCommands::Stats => {
                    commands::spell::handle_stats(&spell_path_strs);
                }
            }
        }

        None => {
            eprintln!("gm: サブコマンドを指定してください (--help で確認できます)");
            process::exit(1);
        }
    }
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
