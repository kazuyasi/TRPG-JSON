pub mod data_transformer;
pub mod part_namer;
pub mod xml_generator;
pub mod zip_writer;

use crate::Monster;
use super::{DataExporter, ExportConfig, ExportError};
use data_transformer::DataTransformer;
use part_namer::PartNamer;
use xml_generator::XmlGenerator;
use zip_writer::ZipFileWriter;
use std::fs;
use std::path::Path;

/// Udonarium エクスポーター
pub struct UdonariumExporter;

impl DataExporter for UdonariumExporter {
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), ExportError> {
        if data.is_empty() {
            return Err(ExportError::GoogleSheetsError(
                "Cannot export empty monster list".to_string(),
            ));
        }

        // 出力ZIPファイルのパス
        let zip_path = Path::new(&config.destination);

        // 出力ディレクトリを作成
        if let Some(parent) = zip_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 全モンスターのXMLを収集
        let mut all_xml_files: Vec<(String, String)> = Vec::new();

        for monster in data {
            // Part名を生成
            let mut namer = PartNamer::new(&monster.part);
            let part_names = namer.generate_names(&monster.part, &monster.name);

            // Part表示名をStringに変換
            let display_names: Vec<String> = part_names
                .iter()
                .map(|pn| pn.display_name.clone())
                .collect();

            // データ変換
            let transformed = DataTransformer::transform(monster, display_names);

            // 各Part用XMLを生成
            for (i, part) in transformed.parts.iter().enumerate() {
                match XmlGenerator::generate_xml(&transformed, i) {
                    Ok(xml) => {
                        // XMLファイル名をPartNameから取得
                        let xml_filename = format!("{}.xml", part_names[i].filename);
                        all_xml_files.push((xml_filename, xml));
                        eprintln!("Generated XML for: {} - {}", monster.name, part.display_name);
                    }
                    Err(e) => {
                        return Err(ExportError::GoogleSheetsError(format!(
                            "XML generation failed for {}: {}",
                            monster.name, e
                        )));
                    }
                }
            }
        }

        // 全XMLファイルを1つのZIPに格納
        let zip_xml_files: Vec<(&str, &str)> = all_xml_files
            .iter()
            .map(|(name, content)| (name.as_str(), content.as_str()))
            .collect();

        ZipFileWriter::create_zip(zip_path, zip_xml_files).map_err(|e| {
            ExportError::GoogleSheetsError(format!(
                "Failed to create ZIP file: {}",
                e
            ))
        })?;

        eprintln!(
            "Successfully exported {} monsters to {}",
            data.len(),
            zip_path.display()
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "Udonarium Exporter"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udonarium_exporter_name() {
        let exporter = UdonariumExporter;
        assert_eq!(exporter.name(), "Udonarium Exporter");
    }
}
