//! 副作用（クリップボードなど）の実行
//!
//! `update` は純粋に保ち、I/O が必要な操作は `Effect` として返す。
//! イベントループがここで実行し、結果をステータス行のメッセージにする。

use std::io;

use super::app::StatusMessage;
use crate::utils;

/// `update` が要求する副作用
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// クリップボードへ文字列をコピーする
    CopyToClipboard {
        /// コピーする本文
        text: String,
        /// 成功時にステータス行へ出す文言（「成功: 」以降）
        success: String,
    },
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::StatusKind;
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
}
