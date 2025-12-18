use crate::Monster;
use super::{DataExporter, ExportConfig, ExportError};
use super::auth::{GoogleSheetsAuth, AuthError};
use super::sheets_api::GoogleSheetsClient;

/// Google Sheets エクスポーター
pub struct GoogleSheetsExporter;

impl DataExporter for GoogleSheetsExporter {
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), ExportError> {
        // スプレッドシートID を検証
        validate_spreadsheet_id(&config.destination)?;

        // エラーハンドリング：データが空の場合
        if data.is_empty() {
            return Err(ExportError::GoogleSheetsError(
                "Cannot export empty monster list. Please filter data first.".to_string(),
            ));
        }

        // 初回認証フロー
        eprintln!("Initializing Google Sheets export...");
        
        let auth = GoogleSheetsAuth::new()
            .map_err(|e| ExportError::GoogleSheetsError(format!("Authentication error: {}", e)))?;

        // 認証情報を読み込む
         let _credentials = match auth.load_credentials() {
             Ok(_creds) => {
                 eprintln!("✓ Loaded existing credentials");
             }
             Err(AuthError::MissingCredentials) => {
                 eprintln!("! No credentials found. Starting OAuth flow...");
                 eprintln!();
                 
                 auth.authenticate()
                     .map_err(|e| ExportError::GoogleSheetsError(
                         format!("OAuth authentication failed: {}\n\nSetup required:\n1. Ensure GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET are set\n2. Check that http://localhost:8080/callback is registered as a redirect URI", e)
                     ))?;
             }
             Err(e) => {
                 eprintln!("⚠ Authentication error: {}. Removing invalid credentials...", e);
                 let _ = auth.clear_credentials();
                 return Err(ExportError::GoogleSheetsError(
                     format!("Failed to load credentials: {}", e)
                 ));
             }
         };

        eprintln!("✓ Authenticated");

        // Google Sheets API クライアントを作成
        let client = GoogleSheetsClient::new()
            .map_err(|e| ExportError::GoogleSheetsError(
                format!("Failed to create Sheets client: {}", e)
            ))?;

        eprintln!("✓ Created Google Sheets client");

        // 各モンスターをスプレッドシートに追加
        eprintln!("Writing {} monsters to spreadsheet...", data.len());
        
        for (i, monster) in data.iter().enumerate() {
            eprintln!("  [{}/{}] Exporting: {}", i + 1, data.len(), monster.name);
            
            // 非同期処理を実行
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| ExportError::GoogleSheetsError(
                    format!("Failed to create async runtime: {}", e)
                ))?;

            rt.block_on(async {
                client.append_monster_data(
                    &config.destination,
                    "search",
                    monster,
                )
                .await
                .map_err(|e| ExportError::GoogleSheetsError(format!("Export failed: {}", e)))
            })?;
        }

        eprintln!("✓ Successfully exported {} monsters to Google Sheets", data.len());
        eprintln!("  Spreadsheet: https://docs.google.com/spreadsheets/d/{}/edit", config.destination);

        Ok(())
    }

    fn name(&self) -> &str {
        "Google Sheets Exporter"
    }
}

/// スプレッドシートID を検証
fn validate_spreadsheet_id(id: &str) -> Result<(), ExportError> {
    if id.is_empty() {
        return Err(ExportError::InvalidDestination(
            "Spreadsheet ID cannot be empty.\n\
             Usage: gm select ... --export sheets --output <spreadsheet-id>\n\
             Example: gm select -l 6 --export sheets --output 1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs"
                .to_string(),
        ));
    }

    if id.len() < 20 {
        return Err(ExportError::InvalidDestination(
            format!(
                "Invalid spreadsheet ID '{}'. \n\
                 Spreadsheet IDs are typically 40+ characters long.\n\
                 You can find your spreadsheet ID in the URL:\n\
                 https://docs.google.com/spreadsheets/d/<SPREADSHEET_ID>/edit",
                id
            ),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_sheets_exporter_name() {
        let exporter = GoogleSheetsExporter;
        assert_eq!(exporter.name(), "Google Sheets Exporter");
    }

    #[test]
    fn test_export_empty_data() {
        let exporter = GoogleSheetsExporter;
        let config = ExportConfig {
            destination: "1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs".to_string(),
            format: crate::export::ExportFormat::GoogleSheets,
        };

        let result = exporter.export(&[], &config);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("empty monster list"));
    }

    #[test]
    fn test_validate_spreadsheet_id_empty() {
        let result = validate_spreadsheet_id("");
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error.to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_spreadsheet_id_too_short() {
        let result = validate_spreadsheet_id("abc123");
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error.to_string().contains("Invalid spreadsheet ID"));
    }

    #[test]
    fn test_validate_spreadsheet_id_valid() {
        let result = validate_spreadsheet_id("1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_google_sheets_export_with_valid_data() {
        let exporter = GoogleSheetsExporter;
        let monster_json = r#"
        {
            "Category": "蛮族",
            "Lv": 6,
            "Revision": 2.5,
            "data": "TEST001",
            "illust": "",
            "movein": 22,
            "movein_des": "飛行",
            "moveon": 22,
            "moveon_des": "",
            "name": "テストモンスター",
            "part": [
                {
                    "HP": 48,
                    "MP": 75,
                    "name": "",
                    "コア": true,
                    "命中力": 15,
                    "回避力": 15,
                    "打撃点": 6,
                    "部位数": 1,
                    "部位特殊能力": "飛行適性",
                    "防護点": 5
                }
            ],
            "備考": "",
            "先制値": 14,
            "共通特殊能力": "飛行",
            "弱点": "属性ダメージ+3",
            "弱点値": 17,
            "生命抵抗力": 16,
            "知名度": 14,
            "精神抵抗力": 16
        }"#;

        let monster: crate::Monster = serde_json::from_str(monster_json).unwrap();
        let config = ExportConfig {
            destination: "1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs".to_string(),
            format: crate::export::ExportFormat::GoogleSheets,
        };

        // 実装完了のため、credentials がない場合は認証エラーが返る
        // (実際には OAuth フロー が実行される)
        let result = exporter.export(&[monster], &config);
        // 認証情報がない環境では失敗することを確認
        assert!(result.is_err());
    }

    #[test]
    fn test_google_sheets_exporter_with_multiple_monsters() {
        let exporter = GoogleSheetsExporter;
        let monster_json = r#"
        {
            "Category": "蛮族",
            "Lv": 6,
            "Revision": 2.5,
            "data": "TEST",
            "illust": "",
            "movein": -1,
            "movein_des": "",
            "moveon": -1,
            "moveon_des": "",
            "name": "テスト",
            "part": [
                {
                    "HP": 50,
                    "MP": 50,
                    "name": "",
                    "コア": true,
                    "命中力": 15,
                    "回避力": 15,
                    "打撃点": 6,
                    "部位数": 1,
                    "部位特殊能力": "",
                    "防護点": 5
                }
            ],
            "備考": "",
            "先制値": 14,
            "共通特殊能力": "",
            "弱点": "属性ダメージ+3",
            "弱点値": 17,
            "生命抵抗力": 16,
            "知名度": 14,
            "精神抵抗力": 16
        }"#;

        let monster: crate::Monster = serde_json::from_str(monster_json).unwrap();
        let monsters = vec![monster.clone(), monster.clone(), monster];

        let config = ExportConfig {
            destination: "1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs".to_string(),
            format: crate::export::ExportFormat::GoogleSheets,
        };

        // 認証情報がない環境では エラーが返る（詳細なエラーメッセージは実行環境に依存）
        let result = exporter.export(&monsters, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_spreadsheet_id_with_special_chars() {
        // Google Sheets IDに含まれる可能性のある特殊文字
        let result = validate_spreadsheet_id("1-BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs");
        assert!(result.is_ok());
    }
}
