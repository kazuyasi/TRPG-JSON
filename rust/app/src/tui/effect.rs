//! 副作用（クリップボード・ファイル出力など）の実行
//!
//! `update` は純粋に保ち、I/O が必要な操作は `Effect` として返す。
//! イベントループがここで実行し、結果をステータス行のメッセージにする。

use std::io;
use std::path::Path;

use trpg_json_core::Monster;

use super::app::{ExportChoice, StatusMessage};
use crate::commands::monster::export_monsters;
use crate::utils;

/// `update` が要求する副作用
#[derive(Debug, Clone)]
pub enum Effect {
    /// クリップボードへ文字列をコピーする
    CopyToClipboard {
        /// コピーする本文
        text: String,
        /// 成功時にステータス行へ出す文言（「成功: 」以降）
        success: String,
    },
    /// モンスター群をファイルへエクスポートする
    Export {
        /// 書き出すモンスター（結果リストの並び順）
        monsters: Vec<Monster>,
        format: ExportChoice,
        /// 入力された出力先（ファイル形式の `~/` は実行時にホームディレクトリへ展開する）。Sheets はスプレッドシート ID
        output: String,
    },
}

impl Effect {
    /// 実行後に画面全体を描き直す必要があるか
    ///
    /// core のエクスポート処理は進捗を stderr に書くため、alternate screen 上に文字が残る。
    pub fn needs_full_redraw(&self) -> bool {
        matches!(self, Effect::Export { .. })
    }

    /// 実行中は TUI を一時停止して通常の画面に戻す必要があるか
    ///
    /// Google Sheets は OAuth でブラウザを開き、認証の案内や進捗を stderr に出しながら
    /// 完了まで処理をブロックするため、利用者がその出力を読めるようにする。
    pub fn needs_suspend(&self) -> bool {
        matches!(self, Effect::Export { format: ExportChoice::Sheets, .. })
    }
}

/// 副作用を実行し、結果をステータス行のメッセージで返す
pub fn execute(effect: Effect) -> StatusMessage {
    execute_with(effect, utils::copy_to_clipboard)
}

/// クリップボード書き込みを差し替えて実行する（テスト用に分離）
fn execute_with(effect: Effect, copy: impl FnOnce(&str) -> io::Result<()>) -> StatusMessage {
    match effect {
        Effect::CopyToClipboard { text, success } => match copy(&text) {
            Ok(()) => StatusMessage::success(success),
            Err(e) => StatusMessage::error(format!("クリップボードへのコピーに失敗しました: {}", e)),
        },
        Effect::Export { monsters, format, output } => {
            let path = if format.is_file() {
                expand_home(&output, dirs::home_dir().as_deref())
            } else {
                output
            };
            match export_monsters(&monsters, format.format_name(), &path) {
                // 文言は CLI の `monster select --export` と同じ
                Ok(()) => StatusMessage::success(format!(
                    "{} 件のモンスターを {} にエクスポートしました",
                    monsters.len(),
                    path
                )),
                Err(e) => StatusMessage::error(e.to_string()),
            }
        }
    }
}

/// 先頭の `~`（単独または `~/`）をホームディレクトリに置き換える
///
/// シェルを通さないため、TUI 側で展開する。ホームが分からなければそのまま返す。
pub fn expand_home(path: &str, home: Option<&Path>) -> String {
    let rest = match path.strip_prefix('~') {
        Some(rest) if rest.is_empty() || rest.starts_with('/') || rest.starts_with('\\') => rest,
        _ => return path.to_string(),
    };
    match home {
        Some(h) => format!("{}{}", h.display(), rest),
        None => path.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::StatusKind;
    use crate::tui::format::tests::{monster, part};
    use std::cell::RefCell;

    fn copy_effect() -> Effect {
        Effect::CopyToClipboard {
            text: "{\"kind\":\"character\"}".into(),
            success: "\"テストモンスター\" をコピーしました".into(),
        }
    }

    #[test]
    fn test_copy_success_passes_text_and_reports_success() {
        let copied = RefCell::new(String::new());
        let status = execute_with(copy_effect(), |t| {
            copied.replace(t.to_string());
            Ok(())
        });
        assert_eq!(copied.into_inner(), "{\"kind\":\"character\"}");
        assert_eq!(status.kind, StatusKind::Success);
        assert_eq!(status.text, "\"テストモンスター\" をコピーしました");
    }

    #[test]
    fn test_copy_failure_reports_error() {
        let status = execute_with(copy_effect(), |_| Err(io::Error::other("テスト用の失敗")));
        assert_eq!(status.kind, StatusKind::Error);
        assert_eq!(status.text, "クリップボードへのコピーに失敗しました: テスト用の失敗");
    }

    fn export_effect(format: ExportChoice, output: String) -> Effect {
        Effect::Export {
            monsters: vec![
                monster("テストモンスター", 3, "蛮族", vec![part("", true)]),
                monster("サンプル動物", 1, "動物", vec![part("", true)]),
            ],
            format,
            output,
        }
    }

    #[test]
    fn test_export_json_writes_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.json").to_string_lossy().to_string();
        let effect = export_effect(ExportChoice::Json, path.clone());
        assert!(effect.needs_full_redraw());

        let status = execute(effect);
        assert_eq!(status.kind, StatusKind::Success, "{}", status.text);
        assert_eq!(status.text, format!("2 件のモンスターを {} にエクスポートしました", path));
        let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(value[0]["name"], "テストモンスター");
        assert_eq!(value[1]["name"], "サンプル動物");
    }

    #[test]
    fn test_export_udonarium_writes_zip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sub").join("out.zip").to_string_lossy().to_string();
        let status = execute(export_effect(ExportChoice::Udonarium, path.clone()));
        assert_eq!(status.kind, StatusKind::Success, "{}", status.text);
        // ZIP のローカルファイルヘッダ
        assert!(std::fs::read(&path).unwrap().starts_with(b"PK"));
    }

    #[test]
    fn test_export_failure_reports_error() {
        let dir = tempfile::tempdir().unwrap();
        // ディレクトリには書き込めない
        let path = dir.path().to_string_lossy().to_string();
        let status = execute(export_effect(ExportChoice::Json, path));
        assert_eq!(status.kind, StatusKind::Error);
        assert!(status.text.starts_with("エクスポートに失敗しました: "), "{}", status.text);
    }

    #[test]
    fn test_copy_does_not_need_redraw() {
        assert!(!copy_effect().needs_full_redraw());
        assert!(!copy_effect().needs_suspend());
    }

    #[test]
    fn test_only_sheets_export_suspends() {
        assert!(!export_effect(ExportChoice::Json, "out.json".into()).needs_suspend());
        assert!(!export_effect(ExportChoice::Udonarium, "out.zip".into()).needs_suspend());
        let sheets = export_effect(ExportChoice::Sheets, "id".into());
        assert!(sheets.needs_suspend());
        assert!(sheets.needs_full_redraw());
    }

    #[test]
    fn test_sheets_export_with_invalid_id_reports_error_without_auth() {
        // ID の検証は認証より前に行われるので、ネットワークやブラウザには触れない
        let status = execute(export_effect(ExportChoice::Sheets, "~/not an id".into()));
        assert_eq!(status.kind, StatusKind::Error);
        assert!(status.text.starts_with("エクスポートに失敗しました: "), "{}", status.text);
        // ファイル形式でないので ~ は展開しない
        assert!(status.text.contains("'~/not an id'"), "{}", status.text);
    }

    #[test]
    fn test_expand_home() {
        let home = Path::new("/home/test");
        assert_eq!(expand_home("~/out.json", Some(home)), "/home/test/out.json");
        assert_eq!(expand_home("~", Some(home)), "/home/test");
        // `~user` や途中の `~` は展開しない
        assert_eq!(expand_home("~other/out.json", Some(home)), "~other/out.json");
        assert_eq!(expand_home("dir/~/out.json", Some(home)), "dir/~/out.json");
        assert_eq!(expand_home("out.json", Some(home)), "out.json");
        assert_eq!(expand_home("~/out.json", None), "~/out.json");
    }
}
