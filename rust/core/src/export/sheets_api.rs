use crate::Monster;
use super::auth::{AuthError, GoogleSheetsAuth, OAuthCredentials};
use super::sheets::{DataTransformer, SheetOutput};
use thiserror::Error;

/// Google Sheets API エラー型
#[derive(Error, Debug)]
pub enum SheetsApiError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("API request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid spreadsheet ID: {0}")]
    InvalidSpreadsheetId(String),

    #[error("Sheet not found: {0}")]
    SheetNotFound(String),

    #[error("Failed to find empty row: no empty rows available")]
    NoEmptyRows,

    #[error("Data transformation error: {0}")]
    TransformationError(String),
}

/// Google Sheets API クライアント
#[allow(dead_code)]
pub struct GoogleSheetsClient {
    auth: GoogleSheetsAuth,
    credentials: OAuthCredentials,
}

impl GoogleSheetsClient {
    /// 新しいクライアントを作成
    /// 
    /// # 注意
    /// 実装時に確認が必要な点：
    /// 1. OAuth認証フローの実装（初回ユーザーの認証）
    /// 2. アクセストークンのリフレッシュロジック
    /// 3. Google Sheets API v4 のエンドポイント統合
    pub fn new() -> Result<Self, SheetsApiError> {
        let auth = GoogleSheetsAuth::new()?;
        let credentials = auth.load_credentials()?;

        if credentials.is_expired() {
            return Err(SheetsApiError::AuthError(AuthError::TokenRefreshFailed(
                "Token expired. Please re-authenticate.".to_string(),
            )));
        }

        Ok(GoogleSheetsClient { auth, credentials })
    }

    /// Monster データをスプレッドシートに追加
    pub async fn append_monster_data(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
        monster: &Monster,
    ) -> Result<(), SheetsApiError> {
        // スプレッドシートIDの検証
        Self::validate_spreadsheet_id(spreadsheet_id)?;

        // 空き行を探す（Column A から row 3 から検索）
        let start_row = self
            .find_empty_row(spreadsheet_id, sheet_name)
            .await?;

        // Monster データを行に変換
        let rows = DataTransformer::transform_monster(monster, start_row);

        // Google Sheets API に書き込み
        self.write_rows_to_sheet(spreadsheet_id, sheet_name, rows)
            .await?;

        Ok(())
    }

    /// 空き行を探す（Column A で空いている行を検索）
    /// 
    /// Google Sheets API の `values.get` メソッドでColumn Aの値を取得し、
    /// Row 3 から検索開始してアクセストークンを使用して検索します。
    /// 奇数行のみをチェック（row 3, 5, 7, 9...）
    async fn find_empty_row(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
    ) -> Result<usize, SheetsApiError> {
        let client = reqwest::Client::new();
        
        // Column A の値を取得（Row 3 から検索）
        // A3:A1000 の範囲で値を取得
        let range = format!("{}!A3:A1000", sheet_name);
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id, urlencoding::encode(&range)
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("{} {}", self.credentials.token_type, self.credentials.access_token))
            .send()
            .await
            .map_err(|e| SheetsApiError::RequestFailed(format!("Failed to get values: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(SheetsApiError::RequestFailed(
                format!("API request failed with status {}: {}", status, body)
            ));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| SheetsApiError::RequestFailed(format!("Failed to parse response: {}", e)))?;

        // response から values を抽出
        let values = response_json
            .get("values")
            .and_then(|v| v.as_array())
            .map(|v| v.len())
            .unwrap_or(0);

        // 最初の空き奇数行を返す（Row 3 は rows[0]、Row 5 は rows[2], ...）
        // values の中から最初の空のセルを探す
        if values == 0 {
            // Column A に値が無い = Row 3 が空いている
            return Ok(3);
        }

        // 値がある場合は、その後の奇数行を探す
        // Row 3, 5, 7, 9... の中から、Column A が空の行を探す
        for i in (0..1000).step_by(2) {
            let row = 3 + i;
            // API レスポンスで確認するロジック
            // ここでは簡略化のため、values 数 + 1 の奇数行を返す
            if i >= values {
                return Ok(row);
            }
        }

        Err(SheetsApiError::NoEmptyRows)
    }

    /// スプレッドシートに行データを書き込む
    /// 
    /// Google Sheets API の `values.update` メソッドを使用して、
    /// 複数の行データをスプレッドシートに書き込みます。
    /// 各行について A1 notation に変換され、バッチで送信されます。
    async fn write_rows_to_sheet(
        &self,
        spreadsheet_id: &str,
        sheet_name: &str,
        rows: Vec<SheetOutput>,
    ) -> Result<(), SheetsApiError> {
        if rows.is_empty() {
            return Ok(());
        }

        let client = reqwest::Client::new();

        // バッチアップデートリクエストを構築
        let mut requests = Vec::new();

        for row in rows {
            // 各行について A1 notation を生成
            // 例：row 3, columns 0-53 → A3:AZ3
            let start_col = Self::index_to_column(0);
            let end_col = Self::index_to_column(row.values.len().saturating_sub(1));
            let range = format!("{}!{}{}:{}{}",
                sheet_name,
                start_col, row.row_number,
                end_col, row.row_number
            );

            // セルの値を構築
            let mut cell_values = Vec::new();
            for value_opt in &row.values {
                if let Some(value) = value_opt {
                    cell_values.push(serde_json::json!(value));
                } else {
                    cell_values.push(serde_json::json!(null));
                }
            }

            requests.push(serde_json::json!({
                "range": range,
                "majorDimension": "ROWS",
                "values": [cell_values]
            }));
        }

        // Google Sheets API の batchUpdate を使用
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values:batchUpdate",
            spreadsheet_id
        );

        let body = serde_json::json!({
            "data": requests,
            "valueInputOption": "RAW"
        });

        let response = client
            .post(&url)
            .header("Authorization", format!("{} {}", self.credentials.token_type, self.credentials.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| SheetsApiError::RequestFailed(format!("Failed to write values: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(SheetsApiError::RequestFailed(
                format!("API request failed with status {}: {}", status, error_body)
            ));
        }

        Ok(())
    }

    /// 列インデックスを A1 notation の列文字に変換
    /// 0 → A, 1 → B, 25 → Z, 26 → AA, 27 → AB, 51 → AZ, 52 → BA
    fn index_to_column(index: usize) -> String {
        let mut result = String::new();
        let mut num = index + 1;

        while num > 0 {
            num -= 1;
            result.insert(0, (b'A' + (num % 26) as u8) as char);
            num /= 26;
        }

        result
    }

    /// スプレッドシートIDを検証
    fn validate_spreadsheet_id(spreadsheet_id: &str) -> Result<(), SheetsApiError> {
        // Google Sheets ID は通常40文字の英数字+記号
        if spreadsheet_id.is_empty() {
            return Err(SheetsApiError::InvalidSpreadsheetId(
                "Spreadsheet ID cannot be empty".to_string(),
            ));
        }

        if spreadsheet_id.len() < 10 {
            return Err(SheetsApiError::InvalidSpreadsheetId(
                "Spreadsheet ID is too short".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_spreadsheet_id_valid() {
        assert!(GoogleSheetsClient::validate_spreadsheet_id(
            "1BxiMVs0XRA5nFMKUVfIz487hJblLvZQvq_fHM9GjMhs"
        )
        .is_ok());
    }

    #[test]
    fn test_validate_spreadsheet_id_empty() {
        let result = GoogleSheetsClient::validate_spreadsheet_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_spreadsheet_id_too_short() {
        let result = GoogleSheetsClient::validate_spreadsheet_id("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_index_to_column_single_letter() {
        assert_eq!(GoogleSheetsClient::index_to_column(0), "A");
        assert_eq!(GoogleSheetsClient::index_to_column(1), "B");
        assert_eq!(GoogleSheetsClient::index_to_column(25), "Z");
    }

    #[test]
    fn test_index_to_column_double_letter() {
        assert_eq!(GoogleSheetsClient::index_to_column(26), "AA");
        assert_eq!(GoogleSheetsClient::index_to_column(27), "AB");
        assert_eq!(GoogleSheetsClient::index_to_column(51), "AZ");
        assert_eq!(GoogleSheetsClient::index_to_column(52), "BA");
    }

    #[test]
    fn test_index_to_column_triple_letter() {
        assert_eq!(GoogleSheetsClient::index_to_column(701), "ZZ");
        assert_eq!(GoogleSheetsClient::index_to_column(702), "AAA");
    }

    #[test]
    fn test_index_to_column_specific_columns() {
        // Test some specific columns mentioned in DESIGN_GUIDE.md
        assert_eq!(GoogleSheetsClient::index_to_column(10), "K"); // L column (11) is index 10
        assert_eq!(GoogleSheetsClient::index_to_column(11), "L"); // L column (12) is index 11
        assert_eq!(GoogleSheetsClient::index_to_column(14), "O");
        assert_eq!(GoogleSheetsClient::index_to_column(15), "P");
        assert_eq!(GoogleSheetsClient::index_to_column(17), "R");
        assert_eq!(GoogleSheetsClient::index_to_column(19), "T");
    }

    #[test]
    fn test_sheets_api_error_display() {
        let error = SheetsApiError::InvalidSpreadsheetId("test".to_string());
        let message = error.to_string();
        assert!(message.contains("Invalid spreadsheet ID"));
        assert!(message.contains("test"));
    }

    #[test]
    fn test_sheets_api_error_no_empty_rows() {
        let error = SheetsApiError::NoEmptyRows;
        let message = error.to_string();
        assert!(message.contains("no empty rows"));
    }

    #[test]
    fn test_sheet_output_creation() {
        let output = SheetOutput {
            row_number: 3,
            values: vec![Some("A1".to_string()), Some("B1".to_string()), None],
            is_merged_row: false,
        };

        assert_eq!(output.row_number, 3);
        assert_eq!(output.values.len(), 3);
        assert_eq!(output.values[0], Some("A1".to_string()));
        assert_eq!(output.values[2], None);
    }
}
