use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 設定ファイルの構造
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub data: DataConfig,
    pub system: Option<SystemConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MonstersConfig {
    /// 単一ファイル（後方互換性用）
    Single(String),
    /// 複数ファイル
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SpellsConfig {
    /// 単一ファイル
    Single(String),
    /// 複数ファイル
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataConfig {
    #[serde(alias = "monsters")]
    pub monsters: MonstersConfig,

    /// スペル（魔法）設定（オプション）
    #[serde(default)]
    pub spells: Option<SpellsConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfig {
    pub name: Option<String>,
}

impl Config {
    /// 設定ファイルを読み込む
    ///
    /// # 引数
    /// * `path` - 設定ファイルのパス
    ///
    /// # 戻り値
    /// * `Ok(Config)` - 読み込まれた設定
    /// * `Err(String)` - エラーメッセージ
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("設定ファイルを読み込めません: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("設定ファイルを解析できません: {}", e))
    }

    /// デフォルト設定を返す
    pub fn default_config() -> Self {
        Config {
            data: DataConfig {
                monsters: MonstersConfig::Single("data/SW2.5/monsters.json".to_string()),
                spells: None,
            },
            system: Some(SystemConfig {
                name: Some("sw25".to_string()),
            }),
        }
    }

    /// モンスターファイルの絶対パスを取得（単一ファイル用）
    ///
    /// # 引数
    /// * `base_path` - 基準パス（通常はホームディレクトリ）
    ///   絶対パスの場合はそのまま使用、相対パスの場合は base_path から解決
    ///
    /// # 戻り値
    /// * `PathBuf` - 解決されたパス
    /// 
    /// # パニック
    /// 複数ファイルが設定されている場合は resolve_monsters_paths() を使用すること
    pub fn resolve_monsters_path(&self, base_path: Option<&Path>) -> PathBuf {
        match &self.data.monsters {
            MonstersConfig::Single(path) => self.resolve_single_path(path, base_path),
            MonstersConfig::Multiple(_) => {
                panic!("複数ファイルが設定されています。resolve_monsters_paths() を使用してください。")
            }
        }
    }

    /// モンスターファイルの絶対パスを複数取得
    ///
    /// # 引数
    /// * `base_path` - 基準パス（通常はホームディレクトリ）
    ///   絶対パスの場合はそのまま使用、相対パスの場合は base_path から解決
    ///
    /// # 戻り値
    /// * `Vec<PathBuf>` - 解決されたパスのリスト
    pub fn resolve_monsters_paths(&self, base_path: Option<&Path>) -> Vec<PathBuf> {
        match &self.data.monsters {
            MonstersConfig::Single(path) => {
                vec![self.resolve_single_path(path, base_path)]
            }
            MonstersConfig::Multiple(paths) => {
                paths
                    .iter()
                    .map(|path| self.resolve_single_path(path, base_path))
                    .collect()
            }
        }
    }

    /// スペルファイルの絶対パスを複数取得
    ///
    /// # 引数
    /// * `base_path` - 基準パス（通常はホームディレクトリ）
    ///   絶対パスの場合はそのまま使用、相対パスの場合は base_path から解決
    ///
    /// # 戻り値
    /// * `Vec<PathBuf>` - 解決されたパスのリスト（スペル設定がない場合は空配列）
    pub fn resolve_spells_paths(&self, base_path: Option<&Path>) -> Vec<PathBuf> {
        match &self.data.spells {
            Some(SpellsConfig::Single(path)) => {
                vec![self.resolve_single_path(path, base_path)]
            }
            Some(SpellsConfig::Multiple(paths)) => {
                paths
                    .iter()
                    .map(|path| self.resolve_single_path(path, base_path))
                    .collect()
            }
            None => vec![],
        }
    }

    /// 単一パスを解決する（内部用ヘルパー）
    fn resolve_single_path(&self, path: &str, base_path: Option<&Path>) -> PathBuf {
        let data_path = Path::new(path);

        // 絶対パスの場合はそのまま返す
        if data_path.is_absolute() {
            return data_path.to_path_buf();
        }

        // 相対パスの場合は base_path から解決
        match base_path {
            Some(base) => base.join(data_path),
            None => data_path.to_path_buf(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_single_file() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = r#"
[data]
monsters = "data/SW2.5/monsters.json"

[system]
name = "sw25"
"#;
        writeln!(file, "{}", config_content).expect("Failed to write to temp file");

        let config = Config::load(file.path()).expect("Failed to load config");
        match &config.data.monsters {
            MonstersConfig::Single(path) => assert_eq!(path, "data/SW2.5/monsters.json"),
            MonstersConfig::Multiple(_) => panic!("Expected single file"),
        }
        assert_eq!(config.system.unwrap().name.unwrap(), "sw25");
        assert!(config.data.spells.is_none());
    }

    #[test]
    fn test_load_config_multiple_files() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = r#"
[data]
monsters = ["data/SW2.5/monsters_part1.json", "data/SW2.5/monsters_part2.json"]

[system]
name = "sw25"
"#;
        writeln!(file, "{}", config_content).expect("Failed to write to temp file");

        let config = Config::load(file.path()).expect("Failed to load config");
        match &config.data.monsters {
            MonstersConfig::Single(_) => panic!("Expected multiple files"),
            MonstersConfig::Multiple(paths) => {
                assert_eq!(paths.len(), 2);
                assert_eq!(paths[0], "data/SW2.5/monsters_part1.json");
                assert_eq!(paths[1], "data/SW2.5/monsters_part2.json");
            }
        }
        assert_eq!(config.system.unwrap().name.unwrap(), "sw25");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default_config();
        match &config.data.monsters {
            MonstersConfig::Single(path) => assert_eq!(path, "data/SW2.5/monsters.json"),
            MonstersConfig::Multiple(_) => panic!("Expected single file in default config"),
        }
        assert_eq!(config.system.unwrap().name.unwrap(), "sw25");
    }

    #[test]
    fn test_resolve_absolute_path() {
        let config = Config {
            data: DataConfig {
                monsters: MonstersConfig::Single("/absolute/path/monsters.json".to_string()),
                spells: None,
            },
            system: None,
        };

        let resolved = config.resolve_monsters_path(Some(Path::new("/repo")));
        assert_eq!(resolved, PathBuf::from("/absolute/path/monsters.json"));
    }

    #[test]
    fn test_resolve_relative_path() {
        let config = Config {
            data: DataConfig {
                monsters: MonstersConfig::Single("data/SW2.5/monsters.json".to_string()),
                spells: None,
            },
            system: None,
        };

        let resolved = config.resolve_monsters_path(Some(Path::new("/repo")));
        assert_eq!(resolved, PathBuf::from("/repo/data/SW2.5/monsters.json"));
    }

    #[test]
    fn test_resolve_multiple_paths() {
        let config = Config {
            data: DataConfig {
                monsters: MonstersConfig::Multiple(vec![
                    "data/SW2.5/monsters_part1.json".to_string(),
                    "data/SW2.5/monsters_part2.json".to_string(),
                ]),
                spells: None,
            },
            system: None,
        };

        let resolved = config.resolve_monsters_paths(Some(Path::new("/repo")));
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0], PathBuf::from("/repo/data/SW2.5/monsters_part1.json"));
        assert_eq!(resolved[1], PathBuf::from("/repo/data/SW2.5/monsters_part2.json"));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = Config::load("/nonexistent/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_with_spells() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = r#"
[data]
monsters = "data/SW2.5/monsters.json"
spells = "data/SW2.5/spells.json"

[system]
name = "sw25"
"#;
        writeln!(file, "{}", config_content).expect("Failed to write to temp file");

        let config = Config::load(file.path()).expect("Failed to load config");
        assert!(config.data.spells.is_some());
        match config.data.spells {
            Some(SpellsConfig::Single(path)) => assert_eq!(path, "data/SW2.5/spells.json"),
            _ => panic!("Expected single spell file"),
        }
    }

    #[test]
    fn test_load_config_with_multiple_spells() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = r#"
[data]
monsters = "data/SW2.5/monsters.json"
spells = ["data/SW2.5/spells_part1.json", "data/SW2.5/spells_part2.json"]

[system]
name = "sw25"
"#;
        writeln!(file, "{}", config_content).expect("Failed to write to temp file");

        let config = Config::load(file.path()).expect("Failed to load config");
        assert!(config.data.spells.is_some());
        match config.data.spells {
            Some(SpellsConfig::Multiple(paths)) => {
                assert_eq!(paths.len(), 2);
                assert_eq!(paths[0], "data/SW2.5/spells_part1.json");
                assert_eq!(paths[1], "data/SW2.5/spells_part2.json");
            }
            _ => panic!("Expected multiple spell files"),
        }
    }

    #[test]
    fn test_resolve_spells_paths() {
        let config = Config {
            data: DataConfig {
                monsters: MonstersConfig::Single("data/SW2.5/monsters.json".to_string()),
                spells: Some(SpellsConfig::Multiple(vec![
                    "data/SW2.5/spells_part1.json".to_string(),
                    "data/SW2.5/spells_part2.json".to_string(),
                ])),
            },
            system: None,
        };

        let resolved = config.resolve_spells_paths(Some(Path::new("/repo")));
        assert_eq!(resolved.len(), 2);
        assert_eq!(resolved[0], PathBuf::from("/repo/data/SW2.5/spells_part1.json"));
        assert_eq!(resolved[1], PathBuf::from("/repo/data/SW2.5/spells_part2.json"));
    }

    #[test]
    fn test_resolve_spells_paths_empty_when_none() {
        let config = Config {
            data: DataConfig {
                monsters: MonstersConfig::Single("data/SW2.5/monsters.json".to_string()),
                spells: None,
            },
            system: None,
        };

        let resolved = config.resolve_spells_paths(Some(Path::new("/repo")));
        assert!(resolved.is_empty());
    }
}
