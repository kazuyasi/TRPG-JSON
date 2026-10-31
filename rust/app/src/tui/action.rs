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
    /// 詳細ペインへフォーカスする（狭い端末では詳細の1ペイン表示に切り替わる）
    FocusDetail,
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
    /// ヘルプモーダルを開く
    OpenHelp,
    /// 選択中項目の JSON ビューモーダルを開く
    OpenJson,
    /// モーダルを閉じる
    CloseModal,
    /// JSON ビューの内容をクリップボードへコピー
    CopyJson,
}

/// キーイベントを Action に変換する（割り当てのないキーは `None`）
///
/// 検索バーにフォーカスがある間は英字・数字キーを文字入力として扱い、コマンドキーは無効にする。
/// モーダル表示中（`modal_open`）はモーダル用のキーだけを受け付ける。
pub fn from_key(key: KeyEvent, focus: Focus, modal_open: bool) -> Option<Action> {
    // Windows では押下と離上の両方が届くため、押下のみ扱う
    if key.kind != KeyEventKind::Press {
        return None;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if modal_open {
        return from_modal_key(key.code, ctrl);
    }

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
        KeyCode::Char('?') => Some(Action::OpenHelp),
        KeyCode::Char('J') => Some(Action::OpenJson),
        KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::Top),
        KeyCode::Char('G') | KeyCode::End => Some(Action::Bottom),
        KeyCode::Enter if focus == Focus::List => Some(Action::FocusDetail),
        KeyCode::Esc if focus == Focus::Detail => Some(Action::FocusList),
        _ => None,
    }
}

/// モーダル表示中のキー: スクロール・閉じる・（JSON ビューの）コピーのみ
fn from_modal_key(code: KeyCode, ctrl: bool) -> Option<Action> {
    if ctrl {
        return (code == KeyCode::Char('c')).then_some(Action::Quit);
    }
    match code {
        // 開いたキーをもう一度押しても閉じる
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q' | '?' | 'J') => Some(Action::CloseModal),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),
        KeyCode::PageUp => Some(Action::PageUp),
        KeyCode::PageDown | KeyCode::Char(' ') => Some(Action::PageDown),
        KeyCode::Home | KeyCode::Char('g') => Some(Action::Top),
        KeyCode::End | KeyCode::Char('G') => Some(Action::Bottom),
        KeyCode::Char('y') => Some(Action::CopyJson),
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
        assert_eq!(from_key(press(KeyCode::Char('q')), Focus::List, false), Some(Action::Quit));
        assert_eq!(from_key(ctrl('c'), Focus::List, false), Some(Action::Quit));
        assert_eq!(from_key(ctrl('c'), Focus::Search, false), Some(Action::Quit));
    }

    #[test]
    fn test_plain_c_copies_ccfolia() {
        // `c` はココフォリアコピー（Ctrl-c の終了と区別する）
        assert_eq!(from_key(press(KeyCode::Char('c')), Focus::List, false), Some(Action::CopyCcfolia));
        assert_eq!(from_key(press(KeyCode::Char('c')), Focus::Detail, false), Some(Action::CopyCcfolia));
        // 検索中は文字入力
        assert_eq!(
            from_key(press(KeyCode::Char('c')), Focus::Search, false),
            Some(Action::InsertChar('c'))
        );
    }

    #[test]
    fn test_y_copies_palette() {
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::List, false), Some(Action::CopyPalette));
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::Detail, false), Some(Action::CopyPalette));
        // 端末によって Shift 付きで届くこともある
        let shift_y = KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::SHIFT);
        assert_eq!(from_key(shift_y, Focus::List, false), Some(Action::CopyAllPalettes));
        assert_eq!(from_key(press(KeyCode::Char('Y')), Focus::Detail, false), Some(Action::CopyAllPalettes));
        // 検索中は文字入力
        assert_eq!(from_key(press(KeyCode::Char('y')), Focus::Search, false), Some(Action::InsertChar('y')));
        assert_eq!(from_key(shift_y, Focus::Search, false), Some(Action::InsertChar('Y')));
    }

    #[test]
    fn test_number_keys_switch_tab() {
        assert_eq!(
            from_key(press(KeyCode::Char('1')), Focus::List, false),
            Some(Action::SwitchTab(Tab::Monster))
        );
        assert_eq!(
            from_key(press(KeyCode::Char('2')), Focus::Detail, false),
            Some(Action::SwitchTab(Tab::Spell))
        );
    }

    #[test]
    fn test_search_focus_treats_letters_as_input() {
        for c in ['q', 'j', '1', '/', 'テ'] {
            assert_eq!(from_key(press(KeyCode::Char(c)), Focus::Search, false), Some(Action::InsertChar(c)));
        }
        assert_eq!(from_key(press(KeyCode::Backspace), Focus::Search, false), Some(Action::DeleteChar));
        assert_eq!(from_key(ctrl('u'), Focus::Search, false), Some(Action::ClearQuery));
        assert_eq!(from_key(press(KeyCode::Enter), Focus::Search, false), Some(Action::FocusList));
        assert_eq!(from_key(press(KeyCode::Esc), Focus::Search, false), Some(Action::FocusList));
        // 矢印キーは検索中でもリストを動かせる
        assert_eq!(from_key(press(KeyCode::Down), Focus::Search, false), Some(Action::Down));
    }

    #[test]
    fn test_navigation_keys() {
        assert_eq!(from_key(press(KeyCode::Char('/')), Focus::List, false), Some(Action::FocusSearch));
        assert_eq!(from_key(press(KeyCode::Char('j')), Focus::List, false), Some(Action::Down));
        assert_eq!(from_key(press(KeyCode::Char('k')), Focus::Detail, false), Some(Action::Up));
        assert_eq!(from_key(press(KeyCode::Char('g')), Focus::List, false), Some(Action::Top));
        assert_eq!(from_key(press(KeyCode::Char('G')), Focus::List, false), Some(Action::Bottom));
        assert_eq!(from_key(press(KeyCode::Tab), Focus::List, false), Some(Action::FocusNext));
        assert_eq!(from_key(press(KeyCode::BackTab), Focus::List, false), Some(Action::FocusPrev));
        assert_eq!(from_key(press(KeyCode::Esc), Focus::Detail, false), Some(Action::FocusList));
        assert_eq!(from_key(ctrl('u'), Focus::List, false), None);
    }

    #[test]
    fn test_modal_open_keys() {
        assert_eq!(from_key(press(KeyCode::Char('?')), Focus::List, false), Some(Action::OpenHelp));
        assert_eq!(from_key(press(KeyCode::Char('?')), Focus::Detail, false), Some(Action::OpenHelp));
        let shift_j = KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT);
        assert_eq!(from_key(shift_j, Focus::List, false), Some(Action::OpenJson));
        assert_eq!(from_key(shift_j, Focus::Detail, false), Some(Action::OpenJson));
        // 検索中は文字入力
        assert_eq!(from_key(press(KeyCode::Char('?')), Focus::Search, false), Some(Action::InsertChar('?')));
        assert_eq!(from_key(shift_j, Focus::Search, false), Some(Action::InsertChar('J')));
    }

    #[test]
    fn test_enter_in_list_focuses_detail() {
        assert_eq!(from_key(press(KeyCode::Enter), Focus::List, false), Some(Action::FocusDetail));
        assert_eq!(from_key(press(KeyCode::Enter), Focus::Detail, false), None);
    }

    #[test]
    fn test_keys_while_modal_open() {
        let m = |code| from_key(press(code), Focus::List, true);
        for code in [KeyCode::Esc, KeyCode::Enter, KeyCode::Char('q'), KeyCode::Char('?'), KeyCode::Char('J')] {
            assert_eq!(m(code), Some(Action::CloseModal), "{code:?}");
        }
        assert_eq!(m(KeyCode::Char('j')), Some(Action::Down));
        assert_eq!(m(KeyCode::Up), Some(Action::Up));
        assert_eq!(m(KeyCode::PageDown), Some(Action::PageDown));
        assert_eq!(m(KeyCode::Char(' ')), Some(Action::PageDown));
        assert_eq!(m(KeyCode::Char('g')), Some(Action::Top));
        assert_eq!(m(KeyCode::Char('G')), Some(Action::Bottom));
        assert_eq!(m(KeyCode::Char('y')), Some(Action::CopyJson));
        // 他のコマンドキーは無効（モーダルの裏で状態が変わらないように）
        for c in ['c', 'Y', '1', '/', 'x'] {
            assert_eq!(m(KeyCode::Char(c)), None, "{c}");
        }
        assert_eq!(m(KeyCode::Tab), None);
        assert_eq!(from_key(ctrl('c'), Focus::Search, true), Some(Action::Quit));
        assert_eq!(from_key(ctrl('u'), Focus::Search, true), None);
    }

    #[test]
    fn test_key_release_is_ignored() {
        let release = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: KeyEventState::NONE,
        };
        assert_eq!(from_key(release, Focus::List, false), None);
    }
}
