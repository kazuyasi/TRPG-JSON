//! キー入力 → 意図（Action）への変換

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use super::app::Tab;

/// ユーザー操作の意図
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// TUI を終了する
    Quit,
    /// 指定タブへ切り替える
    SwitchTab(Tab),
}

/// キーイベントを Action に変換する（割り当てのないキーは `None`）
pub fn from_key(key: KeyEvent) -> Option<Action> {
    // Windows では押下と離上の両方が届くため、押下のみ扱う
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => Some(Action::Quit),
        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Char('1'), _) => Some(Action::SwitchTab(Tab::Monster)),
        (KeyCode::Char('2'), _) => Some(Action::SwitchTab(Tab::Spell)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyEventState;

    fn press(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn test_q_and_ctrl_c_quit() {
        assert_eq!(from_key(press(KeyCode::Char('q'))), Some(Action::Quit));
        assert_eq!(
            from_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn test_plain_c_is_not_quit() {
        // `c` は将来ココフォリアコピーに割り当てるため、終了にしてはいけない
        assert_eq!(from_key(press(KeyCode::Char('c'))), None);
    }

    #[test]
    fn test_number_keys_switch_tab() {
        assert_eq!(from_key(press(KeyCode::Char('1'))), Some(Action::SwitchTab(Tab::Monster)));
        assert_eq!(from_key(press(KeyCode::Char('2'))), Some(Action::SwitchTab(Tab::Spell)));
    }

    #[test]
    fn test_key_release_is_ignored() {
        let release = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: KeyEventState::NONE,
        };
        assert_eq!(from_key(release), None);
    }
}
