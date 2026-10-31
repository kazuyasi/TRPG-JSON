//! キー入力 → 意図（Action）への変換

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use super::app::{Focus, Tab};

/// ユーザー操作の意図
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// TUI を終了する
    Quit,
    /// 指定タブへ切り替える
    SwitchTab(Tab),
    /// 検索バーへフォーカスする
    FocusSearch,
    /// 結果リストへフォーカスを戻す（検索の入力は保持）
    FocusList,
    /// 次のペインへ
    FocusNext,
    /// 前のペインへ
    FocusPrev,
    /// 検索バーへ1文字入力
    InsertChar(char),
    /// 検索バーの末尾1文字を削除
    DeleteChar,
    /// 検索バーの入力をクリア
    ClearQuery,
    /// 上へ1行
    Up,
    /// 下へ1行
    Down,
    /// 1ページ上へ
    PageUp,
    /// 1ページ下へ
    PageDown,
    /// 先頭へ
    Top,
    /// 末尾へ
    Bottom,
    /// 選択中モンスターの選択部位をココフォリア形式でクリップボードへコピー
    CopyCcfolia,
    /// 選択中スペルのチャットパレットをクリップボードへコピー
    CopyPalette,
    /// ヒット中の全スペルのチャットパレットを改行区切りでクリップボードへコピー
    CopyAllPalettes,
}

/// キーイベントを Action に変換する（割り当てのないキーは `None`）
///
/// 検索バーにフォーカスがある間は英字・数字キーを文字入力として扱い、コマンドキーは無効にする。
pub fn from_key(key: KeyEvent, focus: Focus) -> Option<Action> {
    // Windows では押下と離上の両方が届くため、押下のみ扱う
    if key.kind != KeyEventKind::Press {
        return None;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    // どのフォーカスでも共通のキー
    match key.code {
        KeyCode::Char('c') if ctrl => return Some(Action::Quit),
        KeyCode::Tab => return Some(Action::FocusNext),
        KeyCode::BackTab => return Some(Action::FocusPrev),
        KeyCode::Up => return Some(Action::Up),
        KeyCode::Down => return Some(Action::Down),
        KeyCode::PageUp => return Some(Action::PageUp),
        KeyCode::PageDown => return Some(Action::PageDown),
        _ => {}
    }

    if focus == Focus::Search {
        return match key.code {
            KeyCode::Char('u') if ctrl => Some(Action::ClearQuery),
            KeyCode::Char(_) if ctrl => None,
            KeyCode::Char(c) => Some(Action::InsertChar(c)),
            KeyCode::Backspace => Some(Action::DeleteChar),
            KeyCode::Enter | KeyCode::Esc => Some(Action::FocusList),
            _ => None,
        };
    }

    if ctrl {
        return None;
    }
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('1') => Some(Action::SwitchTab(Tab::Monster)),
        KeyCode::Char('2') => Some(Action::SwitchTab(Tab::Spell)),
        KeyCode::Char('/') => Some(Action::FocusSearch),
        KeyCode::Char('c') => Some(Action::CopyCcfolia),
        KeyCode::Char('y') => Some(Action::CopyPalette),
        KeyCode::Char('Y') => Some(Action::CopyAllPalettes),
        KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::Top),
        KeyCode::Char('G') | KeyCode::End => Some(Action::Bottom),
        KeyCode::Esc if focus == Focus::Detail => Some(Action::FocusList),
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

    fn ctrl(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    #[test]
    fn test_q_and_ctrl_c_quit() {
        assert_eq!(from_key(press(KeyCode::Char('q')), Focus::List), Some(Action::Quit));
        assert_eq!(from_key(ctrl('c'), Focus::List), Some(Action::Quit));
        assert_eq!(from_key(ctrl('c'), Focus::Search), Some(Action::Quit));
    }

    #[test]
    fn test_plain_c_copies_ccfolia() {
        // `c` はココフォリアコピー（Ctrl-c の終了と区別する）
        assert_eq!(from_key(press(KeyCode::Char('c')), Focus::List), Some(Action::CopyCcfolia));
        assert_eq!(from_key(press(KeyCode::Char('c')), Focus::Detail), Some(Action::CopyCcfolia));
        // 検索中は文字入力
        assert_eq!(
            from_key(press(KeyCode::Char('c')), Focus::Search),
            Some(Action::InsertChar('c'))
        );
    }

    #[test]
    fn test_y_copies_palette() {
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::List), Some(Action::CopyPalette));
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::Detail), Some(Action::CopyPalette));
        // 端末によって Shift 付きで届くこともある
        let shift_y = KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::SHIFT);
        assert_eq!(from_key(shift_y, Focus::List), Some(Action::CopyAllPalettes));
        assert_eq!(from_key(press(KeyCode::Char('Y')), Focus::Detail), Some(Action::CopyAllPalettes));
        // 検索中は文字入力
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::Search), Some(Action::InsertChar('y')));
        assert_eq!(from_key(shift_y, Focus::Search), Some(Action::InsertChar('Y')));
    }

    #[test]
    fn test_number_keys_switch_tab() {
        assert_eq!(
            from_key(press(KeyCode::Char('1')), Focus::List),
            Some(Action::SwitchTab(Tab::Monster))
        );
        assert_eq!(
            from_key(press(KeyCode::Char('2')), Focus::Detail),
            Some(Action::SwitchTab(Tab::Spell))
        );
    }

    #[test]
    fn test_search_focus_treats_letters_as_input() {
        for c in ['q', 'j', '1', '/', 'テ'] {
            assert_eq!(from_key(press(KeyCode::Char(c)), Focus::Search), Some(Action::InsertChar(c)));
        }
        assert_eq!(from_key(press(KeyCode::Backspace), Focus::Search), Some(Action::DeleteChar));
        assert_eq!(from_key(ctrl('u'), Focus::Search), Some(Action::ClearQuery));
        assert_eq!(from_key(press(KeyCode::Enter), Focus::Search), Some(Action::FocusList));
        assert_eq!(from_key(press(KeyCode::Esc), Focus::Search), Some(Action::FocusList));
        // 矢印キーは検索中でもリストを動かせる
        assert_eq!(from_key(press(KeyCode::Down), Focus::Search), Some(Action::Down));
    }

    #[test]
    fn test_navigation_keys() {
        assert_eq!(from_key(press(KeyCode::Char('/')), Focus::List), Some(Action::FocusSearch));
        assert_eq!(from_key(press(KeyCode::Char('j')), Focus::List), Some(Action::Down));
        assert_eq!(from_key(press(KeyCode::Char('k')), Focus::Detail), Some(Action::Up));
        assert_eq!(from_key(press(KeyCode::Char('g')), Focus::List), Some(Action::Top));
        assert_eq!(from_key(press(KeyCode::Char('G')), Focus::List), Some(Action::Bottom));
        assert_eq!(from_key(press(KeyCode::Tab), Focus::List), Some(Action::FocusNext));
        assert_eq!(from_key(press(KeyCode::BackTab), Focus::List), Some(Action::FocusPrev));
        assert_eq!(from_key(press(KeyCode::Esc), Focus::Detail), Some(Action::FocusList));
        assert_eq!(from_key(ctrl('u'), Focus::List), None);
    }

    #[test]
    fn test_key_release_is_ignored() {
        let release = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: KeyEventState::NONE,
        };
        assert_eq!(from_key(release, Focus::List), None);
    }
}
