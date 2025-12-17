use crate::Monster;
use thiserror::Error;

pub mod auth;
pub mod google_sheets;
pub mod json;
pub mod sheets;
pub mod sheets_api;
pub mod udonarium;

/// エクスポート形式の定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    GoogleSheets,
    Udonarium,
}

impl std::str::FromStr for ExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "sheets" | "google-sheets" | "googlesheets" => Ok(ExportFormat::GoogleSheets),
            "udonarium" => Ok(ExportFormat::Udonarium),
            _ => Err(format!(
                "Unknown export format: '{}'. Supported: json, sheets, udonarium",
                s
            )),
        }
    }
}

/// エクスポート設定
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// 出力先（JSONの場合: ファイルパス、Google Sheetsの場合: スプレッドシートID）
    pub destination: String,
    /// エクスポート形式
    pub format: ExportFormat,
}

/// エクスポート操作のエラー型
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Authentication error: {0}")]
    AuthError(#[from] auth::AuthError),

    #[error("Google Sheets API error: {0}")]
    SheetsApiError(#[from] sheets_api::SheetsApiError),

    #[error("Google Sheets error: {0}")]
    GoogleSheetsError(String),

    #[error("Invalid export destination: {0}")]
    InvalidDestination(String),

    #[error("Export format not supported: {0}")]
    UnsupportedFormat(String),
}

/// データエクスポーターのトレイト
pub trait DataExporter {
    /// モンスター配列をエクスポートする
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), ExportError>;

    /// エクスポーター名を返す
    fn name(&self) -> &str;
}

/// エクスポーターファクトリ
pub struct ExporterFactory;

impl ExporterFactory {
    /// 指定されたフォーマットに対応するエクスポーターを取得
    pub fn create_exporter(format: ExportFormat) -> Result<Box<dyn DataExporter>, ExportError> {
        match format {
            ExportFormat::Json => Ok(Box::new(json::JsonExporter)),
            ExportFormat::GoogleSheets => Ok(Box::new(google_sheets::GoogleSheetsExporter)),
            ExportFormat::Udonarium => Ok(Box::new(udonarium::UdonariumExporter)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_str() {
        assert_eq!("json".parse::<ExportFormat>().unwrap(), ExportFormat::Json);
        assert_eq!(
            "sheets".parse::<ExportFormat>().unwrap(),
            ExportFormat::GoogleSheets
        );
        assert_eq!(
            "google-sheets".parse::<ExportFormat>().unwrap(),
            ExportFormat::GoogleSheets
        );
        assert_eq!(
            "googlesheets".parse::<ExportFormat>().unwrap(),
            ExportFormat::GoogleSheets
        );
        assert_eq!(
            "udonarium".parse::<ExportFormat>().unwrap(),
            ExportFormat::Udonarium
        );
    }

    #[test]
    fn test_export_format_case_insensitive() {
        assert_eq!("JSON".parse::<ExportFormat>().unwrap(), ExportFormat::Json);
        assert_eq!("SHEETS".parse::<ExportFormat>().unwrap(), ExportFormat::GoogleSheets);
        assert_eq!(
            "UDONARIUM".parse::<ExportFormat>().unwrap(),
            ExportFormat::Udonarium
        );
    }

    #[test]
    fn test_export_format_invalid() {
        let result = "csv".parse::<ExportFormat>();
        assert!(result.is_err());
    }

    #[test]
    fn test_export_config_creation() {
        let config = ExportConfig {
            destination: "/path/to/output.json".to_string(),
            format: ExportFormat::Json,
        };

        assert_eq!(config.destination, "/path/to/output.json");
        assert_eq!(config.format, ExportFormat::Json);
    }

    #[test]
    fn test_exporter_factory_json() {
        let exporter = ExporterFactory::create_exporter(ExportFormat::Json);
        assert!(exporter.is_ok());
        assert_eq!(exporter.unwrap().name(), "JSON Exporter");
    }

    #[test]
    fn test_exporter_factory_google_sheets() {
        let exporter = ExporterFactory::create_exporter(ExportFormat::GoogleSheets);
        assert!(exporter.is_ok());
        assert_eq!(exporter.unwrap().name(), "Google Sheets Exporter");
    }

    #[test]
    fn test_exporter_factory_udonarium() {
        let exporter = ExporterFactory::create_exporter(ExportFormat::Udonarium);
        assert!(exporter.is_ok());
        assert_eq!(exporter.unwrap().name(), "Udonarium Exporter");
    }
}
