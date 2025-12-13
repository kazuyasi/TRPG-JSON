use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tiny_http::{Server, Response};
use url::Url;

/// OAuth 2.0 認証エラー型
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to read credentials file: {0}")]
    CredentialsReadError(#[from] std::io::Error),

    #[error("Failed to parse credentials: {0}")]
    CredentialsParsError(#[from] serde_json::Error),

    #[error("Invalid credentials format")]
    InvalidCredentialsFormat,

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Token refresh failed: {0}")]
    TokenRefreshFailed(String),

    #[error("Missing required OAuth credentials")]
    MissingCredentials,

    #[error("OAuth server error: {0}")]
    ServerError(String),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("HTTP request error: {0}")]
    HttpError(String),

    #[error("User cancelled authentication")]
    AuthenticationCancelled,
}

/// OAuth 2.0 認証情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
    /// アクセストークン
    pub access_token: String,

    /// トークンタイプ（通常は "Bearer"）
    pub token_type: String,

    /// トークンの有効期限（UNIX timestamp）
    pub expires_at: i64,

    /// リフレッシュトークン（トークン更新用）
    pub refresh_token: Option<String>,
}

/// OAuth 2.0 トークンレスポンス
#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    /// アクセストークン
    access_token: String,

    /// トークンタイプ（通常は "Bearer"）
    token_type: String,

    /// トークンの有効期限（秒）
    expires_in: u64,

    /// リフレッシュトークン（存在しない場合がある）
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,

    /// スコープ
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

impl OAuthCredentials {
    /// トークンが有効期限切れかチェック
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        self.expires_at <= now + 60 // 60秒前に期限切れと判定
    }
}

/// OAuth 2.0 設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// Google Cloud Console から取得したクライアント ID
    pub client_id: String,

    /// Google Cloud Console から取得したクライアントシークレット
    pub client_secret: String,

    /// リダイレクトURI (通常は http://localhost:8080/callback)
    pub redirect_uri: String,

    /// Google Sheets API のスコープ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl OAuthConfig {
    /// 新しい OAuth 設定を作成
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        OAuthConfig {
            client_id,
            client_secret,
            redirect_uri,
            scopes: Some(vec![
                "https://www.googleapis.com/auth/spreadsheets".to_string(),
            ]),
        }
    }

    /// 環境変数から OAuth 設定を読み込む
    pub fn from_env() -> Result<Self, AuthError> {
        let client_id = std::env::var("GOOGLE_CLIENT_ID")
            .map_err(|_| AuthError::MissingCredentials)?;
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .map_err(|_| AuthError::MissingCredentials)?;
        let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
            .unwrap_or_else(|_| "http://localhost:8080/callback".to_string());

        Ok(OAuthConfig::new(client_id, client_secret, redirect_uri))
    }

    /// 設定ファイルから OAuth 設定を読み込む
    pub fn from_file(path: &PathBuf) -> Result<Self, AuthError> {
        let content = std::fs::read_to_string(path)?;
        let config: OAuthConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// OAuth 認可エンドポイントのURL を生成
    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode("https://www.googleapis.com/auth/spreadsheets"),
            state
        )
    }
}

/// Google Sheets API 認証マネージャー
pub struct GoogleSheetsAuth {
    credentials_path: PathBuf,
    config_path: PathBuf,
}

impl GoogleSheetsAuth {
    /// 新しい認証マネージャーを作成
    pub fn new() -> Result<Self, AuthError> {
        let credentials_path = Self::get_credentials_path()?;
        let config_path = Self::get_config_path()?;
        Ok(GoogleSheetsAuth { credentials_path, config_path })
    }

    /// 認証情報ファイルのパスを取得（~/.config/trpg-json/credentials.json）
    fn get_credentials_path() -> Result<PathBuf, AuthError> {
        let config_dir = if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            return Err(AuthError::MissingCredentials);
        };

        let trpg_config_dir = config_dir.join("trpg-json");
        Ok(trpg_config_dir.join("credentials.json"))
    }

    /// OAuth 設定ファイルのパスを取得（~/.config/trpg-json/oauth_config.json）
    fn get_config_path() -> Result<PathBuf, AuthError> {
        let config_dir = if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(config_home)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            return Err(AuthError::MissingCredentials);
        };

        let trpg_config_dir = config_dir.join("trpg-json");
        Ok(trpg_config_dir.join("oauth_config.json"))
    }

    /// 保存された認証情報を読み込む
    pub fn load_credentials(&self) -> Result<OAuthCredentials, AuthError> {
        if !self.credentials_path.exists() {
            return Err(AuthError::MissingCredentials);
        }

        let content = fs::read_to_string(&self.credentials_path)?;
        let credentials: OAuthCredentials = serde_json::from_str(&content)?;

        if credentials.is_expired() {
            return Err(AuthError::TokenRefreshFailed(
                "Token has expired. Please re-authenticate.".to_string(),
            ));
        }

        Ok(credentials)
    }

    /// 認証情報を保存
    pub fn save_credentials(&self, credentials: &OAuthCredentials) -> Result<(), AuthError> {
        let config_dir = self.credentials_path.parent().ok_or(AuthError::InvalidCredentialsFormat)?;

        // ディレクトリが存在しない場合は作成
        fs::create_dir_all(config_dir)?;

        let json = serde_json::to_string_pretty(&credentials)?;
        fs::write(&self.credentials_path, json)?;

        Ok(())
    }

    /// OAuth 2.0 フロー全体を実行して認証情報を取得
    ///
    /// # 処理フロー
    /// 1. OAuth 設定を読み込む（環境変数またはファイル）
    /// 2. ブラウザで Google 認可画面を開く
    /// 3. ユーザー認可後、ローカルHTTPサーバーで認可コードを受け取る
    /// 4. トークンエンドポイントでアクセストークンを取得
    /// 5. 認証情報を保存して返す
    pub fn authenticate(&self) -> Result<OAuthCredentials, AuthError> {
        // OAuth 設定を読み込む
        let config = OAuthConfig::from_env()
            .or_else(|_| OAuthConfig::from_file(&self.config_path))
            .map_err(|_| AuthError::AuthenticationFailed(
                "Failed to load OAuth config. Please set GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET environment variables, or create ~/.config/trpg-json/oauth_config.json".to_string()
            ))?;

        // 認可コードを取得
        let auth_code = self.get_authorization_code(&config)?;

        // トークンを取得
        let credentials = self.exchange_code_for_token(&config, &auth_code)?;

        // 認証情報を保存
        self.save_credentials(&credentials)?;

        Ok(credentials)
    }

    /// ユーザー認可画面を開き、認可コードを取得
    fn get_authorization_code(&self, config: &OAuthConfig) -> Result<String, AuthError> {
        // ランダムな state を生成
        let state = format!("{}", uuid::Uuid::new_v4());

        // 認可エンドポイントの URL を生成
        let auth_url = config.authorization_url(&state);

        // ブラウザで URL を開く
        webbrowser::open(&auth_url)
            .map_err(|e| AuthError::AuthenticationFailed(
                format!("Failed to open browser: {}. Please visit: {}", e, auth_url)
            ))?;

        eprintln!("Opening browser for authentication...");
        eprintln!("If it doesn't open automatically, please visit: {}", auth_url);

        // ローカルHTTPサーバーで認可コードを受け取る
        let code = self.start_callback_server(&state)?;

        Ok(code)
    }

    /// ローカルHTTPサーバーを起動して認可コールバックを受け取る
    fn start_callback_server(&self, expected_state: &str) -> Result<String, AuthError> {
        let server = Server::http("127.0.0.1:8080")
            .map_err(|e| AuthError::ServerError(format!("Failed to start server: {}", e)))?;

        eprintln!("Waiting for authorization callback on http://localhost:8080/callback");

        for request in server.incoming_requests().take(1) {
            let uri = request.url().to_string();

            // コールバックURLをパース
            let full_url = format!("http://127.0.0.1:8080{}", uri);
            let parsed_url = Url::parse(&full_url)
                .map_err(|e| AuthError::UrlParseError(e))?;

            let query_pairs: std::collections::HashMap<_, _> = parsed_url
                .query_pairs()
                .into_owned()
                .collect();

            // エラーチェック
            if let Some(error) = query_pairs.get("error") {
                let error_desc = query_pairs.get("error_description")
                    .cloned()
                    .unwrap_or_else(|| "Unknown error".to_string());
                let _ = request.respond(Response::from_string("Authentication failed. You can close this window."));
                return Err(AuthError::AuthenticationFailed(
                    format!("{}: {}", error, error_desc)
                ));
            }

            // State チェック
            if let Some(state) = query_pairs.get("state") {
                if state != expected_state {
                    let _ = request.respond(Response::from_string("State mismatch. You can close this window."));
                    return Err(AuthError::AuthenticationFailed("State mismatch".to_string()));
                }
            } else {
                let _ = request.respond(Response::from_string("Missing state parameter. You can close this window."));
                return Err(AuthError::AuthenticationFailed("Missing state parameter".to_string()));
            }

            // 認可コードを取得
            if let Some(code) = query_pairs.get("code") {
                let _ = request.respond(Response::from_string("Authentication successful! You can close this window and return to the terminal."));
                return Ok(code.clone());
            } else {
                let _ = request.respond(Response::from_string("Missing authorization code. You can close this window."));
                return Err(AuthError::AuthenticationFailed("Missing authorization code".to_string()));
            }
        }

        Err(AuthError::AuthenticationFailed("Server did not receive a request".to_string()))
    }

    /// 認可コードをアクセストークンに交換
    #[tokio::main]
    async fn exchange_code_for_token(&self, config: &OAuthConfig, code: &str) -> Result<OAuthCredentials, AuthError> {
        let client = reqwest::Client::new();

        let params = [
            ("code", code),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("redirect_uri", &config.redirect_uri),
            ("grant_type", "authorization_code"),
        ];

        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| AuthError::HttpError(format!("Token exchange failed: {}", e)))?;

        let body = response
            .text()
            .await
            .map_err(|e| AuthError::HttpError(format!("Failed to read response: {}", e)))?;

        let token_response: TokenResponse = serde_json::from_str(&body)
            .map_err(|e| AuthError::AuthenticationFailed(
                format!("Invalid token response: {}. Response: {}", e, body)
            ))?;

        // アクセストークンを OAuthCredentials に変換
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + token_response.expires_in as i64;

        Ok(OAuthCredentials {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_at,
            refresh_token: token_response.refresh_token,
        })
    }

    /// 認証情報をクリア（ログアウト）
    pub fn clear_credentials(&self) -> Result<(), AuthError> {
        if self.credentials_path.exists() {
            fs::remove_file(&self.credentials_path)?;
        }
        Ok(())
    }
}

impl Default for GoogleSheetsAuth {
    fn default() -> Self {
        Self::new().expect("Failed to initialize GoogleSheetsAuth")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_credentials_not_expired() {
        let future_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600; // 1時間後

        let creds = OAuthCredentials {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: future_timestamp,
            refresh_token: Some("refresh_token".to_string()),
        };

        assert!(!creds.is_expired());
    }

    #[test]
    fn test_oauth_credentials_expired() {
        let past_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - 3600; // 1時間前

        let creds = OAuthCredentials {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: past_timestamp,
            refresh_token: Some("refresh_token".to_string()),
        };

        assert!(creds.is_expired());
    }

    #[test]
    fn test_oauth_credentials_serialization() {
        let creds = OAuthCredentials {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: 1700000000,
            refresh_token: Some("refresh_token".to_string()),
        };

        let json = serde_json::to_string(&creds).unwrap();
        let deserialized: OAuthCredentials = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.access_token, "test_token");
        assert_eq!(deserialized.token_type, "Bearer");
    }

    #[test]
    fn test_oauth_credentials_without_refresh_token() {
        let future_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 7200; // 2時間後

        let creds = OAuthCredentials {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: future_timestamp,
            refresh_token: None,
        };

        assert_eq!(creds.refresh_token, None);
        assert!(!creds.is_expired());
    }

    #[test]
    fn test_oauth_credentials_expires_at_boundary() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // 60秒後に期限切れと判定される時刻
        let boundary_time = now + 60;

        let creds = OAuthCredentials {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_at: boundary_time,
            refresh_token: Some("refresh_token".to_string()),
        };

        // 60秒以内の場合は期限切れと判定
        assert!(creds.is_expired());
    }

    #[test]
    fn test_oauth_config_new() {
        let config = OAuthConfig::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        assert_eq!(config.client_id, "test_client_id");
        assert_eq!(config.client_secret, "test_client_secret");
        assert_eq!(config.redirect_uri, "http://localhost:8080/callback");
        assert!(config.scopes.is_some());
        assert_eq!(
            config.scopes.as_ref().unwrap()[0],
            "https://www.googleapis.com/auth/spreadsheets"
        );
    }

    #[test]
    fn test_oauth_config_authorization_url() {
        let config = OAuthConfig::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        let auth_url = config.authorization_url("test_state");
        
        assert!(auth_url.contains("https://accounts.google.com/o/oauth2/v2/auth"));
        assert!(auth_url.contains("client_id=test_client_id"));
        assert!(auth_url.contains("response_type=code"));
        assert!(auth_url.contains("state=test_state"));
        assert!(auth_url.contains("scope="));
    }

    #[test]
    fn test_oauth_config_serialization() {
        let config = OAuthConfig::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        );

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OAuthConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.client_id, config.client_id);
        assert_eq!(deserialized.client_secret, config.client_secret);
        assert_eq!(deserialized.redirect_uri, config.redirect_uri);
    }

    #[test]
    fn test_token_response_serialization() {
        let token_response = TokenResponse {
            access_token: "test_access_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some("test_refresh_token".to_string()),
            scope: Some("https://www.googleapis.com/auth/spreadsheets".to_string()),
        };

        let json = serde_json::to_string(&token_response).unwrap();
        let deserialized: TokenResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.access_token, "test_access_token");
        assert_eq!(deserialized.token_type, "Bearer");
        assert_eq!(deserialized.expires_in, 3600);
        assert_eq!(deserialized.refresh_token, Some("test_refresh_token".to_string()));
    }

    #[test]
    fn test_token_response_without_optional_fields() {
        let json = r#"{
            "access_token": "test_token",
            "token_type": "Bearer",
            "expires_in": 3600
        }"#;

        let token_response: TokenResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(token_response.access_token, "test_token");
        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 3600);
        assert_eq!(token_response.refresh_token, None);
        assert_eq!(token_response.scope, None);
    }

    #[test]
    fn test_google_sheets_auth_new() {
        let auth = GoogleSheetsAuth::new();
        assert!(auth.is_ok());
    }

    #[test]
    fn test_credentials_path_creation() {
        // This test verifies that the path is created correctly with HOME env var
        let credentials_path = GoogleSheetsAuth::get_credentials_path();
        assert!(credentials_path.is_ok());
        
        if let Ok(path) = credentials_path {
            assert!(path.to_string_lossy().contains(".config/trpg-json/credentials.json"));
        }
    }
}
