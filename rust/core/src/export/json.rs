use super::{DataExporter, ExportConfig, ExportError};
use crate::Monster;
use std::fs;
use std::path::Path;

/// JSON形式でのエクスポーターの実装
pub struct JsonExporter;

impl DataExporter for JsonExporter {
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), ExportError> {
        // 出力先パスの検証
        let path = Path::new(&config.destination);

        // ディレクトリが存在するか確認
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                return Err(ExportError::InvalidDestination(format!(
                    "Output directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        // JSONにシリアライズ
        let json_content = serde_json::to_string_pretty(&data)?;

        // ファイルに書き込み
        fs::write(path, json_content)?;

        Ok(())
    }

    fn name(&self) -> &str {
        "JSON Exporter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Part;
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    fn create_test_monster(name: &str, level: i32) -> Monster {
        Monster {
            category: "蛮族".to_string(),
            level,
            revision: 2.5,
            data: "TEST001".to_string(),
            illust: "".to_string(),
            movein: 22,
            movein_description: "飛行".to_string(),
            moveon: 22,
            moveon_description: "".to_string(),
            name: name.to_string(),
            part: vec![Part {
                hp: Some(48),
                mp: 75,
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
            common_abilities: "飛行".to_string(),
            weakness: "純エネルギー属性ダメージ+3".to_string(),
            weakness_value: 17,
            life_resistance: 16,
            fame: 14,
            mental_resistance: 16,
            extra: HashMap::new(),
        }
    }

    #[test]
    fn test_json_exporter_name() {
        let exporter = JsonExporter;
        assert_eq!(exporter.name(), "JSON Exporter");
    }

    #[test]
    fn test_export_single_monster() {
        let exporter = JsonExporter;
        let monsters = vec![create_test_monster("テストモンスター", 6)];

        // 一時ファイルを作成
        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = ExportConfig {
            destination: output_path.clone(),
            format: super::super::ExportFormat::Json,
        };

        // エクスポート
        let result = exporter.export(&monsters, &config);
        assert!(result.is_ok());

        // ファイルが作成されたことを確認
        assert!(Path::new(&output_path).exists());

        // ファイル内容を確認
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("テストモンスター"));
        assert!(content.contains("\"Lv\": 6"));
    }

    #[test]
    fn test_export_multiple_monsters() {
        let exporter = JsonExporter;
        let monsters = vec![
            create_test_monster("モンスター1", 6),
            create_test_monster("モンスター2", 7),
            create_test_monster("モンスター3", 8),
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = ExportConfig {
            destination: output_path.clone(),
            format: super::super::ExportFormat::Json,
        };

        let result = exporter.export(&monsters, &config);
        assert!(result.is_ok());

        // ファイル内容を確認
        let content = fs::read_to_string(&output_path).unwrap();
        let parsed: Vec<Monster> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].name, "モンスター1");
        assert_eq!(parsed[1].name, "モンスター2");
        assert_eq!(parsed[2].name, "モンスター3");
    }

    #[test]
    fn test_export_empty_data() {
        let exporter = JsonExporter;
        let monsters: Vec<Monster> = vec![];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = ExportConfig {
            destination: output_path.clone(),
            format: super::super::ExportFormat::Json,
        };

        let result = exporter.export(&monsters, &config);
        assert!(result.is_ok());

        // ファイル内容を確認
        let content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(content.trim(), "[]");
    }

    #[test]
    fn test_export_preserves_formatting() {
        let exporter = JsonExporter;
        let monsters = vec![create_test_monster("テストモンスター", 6)];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = ExportConfig {
            destination: output_path.clone(),
            format: super::super::ExportFormat::Json,
        };

        exporter.export(&monsters, &config).unwrap();

        // 出力はpretty-printedされていることを確認
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("  ")); // インデント確認
        assert!(content.contains("\n")); // 改行確認
    }

    #[test]
    fn test_export_invalid_directory() {
        let exporter = JsonExporter;
        let monsters = vec![create_test_monster("テストモンスター", 6)];

        let config = ExportConfig {
            destination: "/nonexistent/directory/output.json".to_string(),
            format: super::super::ExportFormat::Json,
        };

        let result = exporter.export(&monsters, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_roundtrip() {
        let exporter = JsonExporter;
        let original_monsters = vec![
            create_test_monster("モンスター1", 6),
            create_test_monster("モンスター2", 7),
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_string_lossy().to_string();

        let config = ExportConfig {
            destination: output_path.clone(),
            format: super::super::ExportFormat::Json,
        };

        // エクスポート
        exporter.export(&original_monsters, &config).unwrap();

        // ファイルから読み込み
        let content = fs::read_to_string(&output_path).unwrap();
        let loaded_monsters: Vec<Monster> = serde_json::from_str(&content).unwrap();

        // データが一致することを確認
        assert_eq!(loaded_monsters.len(), original_monsters.len());
        for (original, loaded) in original_monsters.iter().zip(loaded_monsters.iter()) {
            assert_eq!(loaded.name, original.name);
            assert_eq!(loaded.level, original.level);
            assert_eq!(loaded.category, original.category);
        }
    }
}
