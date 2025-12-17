use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::ZipWriter;
use zip::write::FileOptions;

/// ZIP圧縮機能を提供
pub struct ZipFileWriter;

impl ZipFileWriter {
    /// 複数のXML文字列をZIPファイルに圧縮して保存
    ///
    /// # 引数
    /// * `zip_path` - 出力ZIPファイルのパス
    /// * `xml_files` - (ファイル名, XML内容) のタプルのベクトル
    ///   例: vec![("monster.xml", "<character>...</character>")]
    ///
    /// # 戻り値
    /// * `Ok(())` - 成功
    /// * `Err(String)` - エラーメッセージ
    pub fn create_zip<P: AsRef<Path>>(
        zip_path: P,
        xml_files: Vec<(&str, &str)>,
    ) -> Result<(), String> {
        // ZIPファイルを作成
        let file = File::create(&zip_path).map_err(|e| {
            format!(
                "Failed to create ZIP file {}: {}",
                zip_path.as_ref().display(),
                e
            )
        })?;

        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default();

        // 各XMLファイルをZIPに追加
        for (filename, content) in xml_files {
            zip.start_file(filename, options).map_err(|e| {
                format!("Failed to add file '{}' to ZIP: {}", filename, e)
            })?;

            zip.write_all(content.as_bytes()).map_err(|e| {
                format!("Failed to write content for '{}': {}", filename, e)
            })?;
        }

        // ZIPファイルを閉じる
        zip.finish().map_err(|e| {
            format!("Failed to finalize ZIP file: {}", e)
        })?;

        Ok(())
    }

    /// 単一のXML文字列をZIPファイルに圧縮して保存
    ///
    /// # 引数
    /// * `zip_path` - 出力ZIPファイルのパス
    /// * `filename` - ZIPファイル内のXMLファイル名
    /// * `content` - XML内容
    ///
    /// # 戻り値
    /// * `Ok(())` - 成功
    /// * `Err(String)` - エラーメッセージ
    pub fn create_zip_single<P: AsRef<Path>>(
        zip_path: P,
        filename: &str,
        content: &str,
    ) -> Result<(), String> {
        Self::create_zip(zip_path, vec![(filename, content)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;
    use zip::ZipArchive;

    #[test]
    fn test_create_single_zip() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let zip_path = temp_file.path().to_string_lossy().to_string();

        let xml_content = r#"<?xml version="1.0" encoding="utf-8"?>
<character>
  <data name="test">Test Content</data>
</character>"#;

        let result = ZipFileWriter::create_zip_single(&zip_path, "test.xml", xml_content);
        assert!(result.is_ok());

        // ZIPファイルが作成されたことを確認
        assert!(Path::new(&zip_path).exists());

        // ZIPの内容を検証
        let file = File::open(&zip_path).expect("Failed to open ZIP");
        let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
        assert_eq!(archive.len(), 1);

        let mut file = archive
            .by_index(0)
            .expect("Failed to get file from ZIP");
        assert_eq!(file.name(), "test.xml");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read file content");
        assert!(content.contains("Test Content"));
    }

    #[test]
    fn test_create_multiple_xml_zip() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let zip_path = temp_file.path().to_string_lossy().to_string();

        let xml_files = vec![
            ("part1.xml", "<character><data name=\"part1\"/></character>"),
            ("part2.xml", "<character><data name=\"part2\"/></character>"),
            ("part3.xml", "<character><data name=\"part3\"/></character>"),
        ];

        let result = ZipFileWriter::create_zip(&zip_path, xml_files);
        assert!(result.is_ok());

        // ZIPファイルが作成されたことを確認
        assert!(Path::new(&zip_path).exists());

        // ZIPの内容を検証
        let file = File::open(&zip_path).expect("Failed to open ZIP");
        let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
        assert_eq!(archive.len(), 3);

        // ファイル名を確認
        let filenames: Vec<String> = (0..archive.len())
            .map(|i| {
                archive
                    .by_index(i)
                    .expect("Failed to get file")
                    .name()
                    .to_string()
            })
            .collect();
        assert!(filenames.contains(&"part1.xml".to_string()));
        assert!(filenames.contains(&"part2.xml".to_string()));
        assert!(filenames.contains(&"part3.xml".to_string()));
    }

    #[test]
    fn test_invalid_zip_path() {
        let result = ZipFileWriter::create_zip_single(
            "/invalid/nonexistent/path/file.zip",
            "test.xml",
            "<test/>",
        );
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .contains("Failed to create ZIP file"));
    }

    #[test]
    fn test_zip_with_special_characters_in_filename() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let zip_path = temp_file.path().to_string_lossy().to_string();

        let result = ZipFileWriter::create_zip_single(
            &zip_path,
            "テスト_モンスター.xml",
            "<character/>",
        );
        assert!(result.is_ok());

        // ファイル名が日本語で保存されていることを確認
        let file = File::open(&zip_path).expect("Failed to open ZIP");
        let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
        let file = archive.by_index(0).expect("Failed to get file");
        assert_eq!(file.name(), "テスト_モンスター.xml");
    }

    #[test]
    fn test_empty_xml_content() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let zip_path = temp_file.path().to_string_lossy().to_string();

        let result = ZipFileWriter::create_zip_single(&zip_path, "empty.xml", "");
        assert!(result.is_ok());

        let file = File::open(&zip_path).expect("Failed to open ZIP");
        let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
        let mut file = archive.by_index(0).expect("Failed to get file");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read content");
        assert!(content.is_empty());
    }
}
