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
    /// 入力欄（検索バー・エクスポートの入力欄）のカーソル位置へ1文字入力
    InsertChar(char),
    /// 入力欄のカーソルの前の1文字を削除（`Backspace`）
    DeleteChar,
    /// 入力欄のカーソル位置の1文字を削除（`Delete`）
    DeleteCharForward,
    /// 入力欄の入力をクリア
    ClearQuery,
    /// 入力欄のカーソルを1文字左へ
    CursorLeft,
    /// 入力欄のカーソルを1文字右へ
    CursorRight,
    /// 入力欄のカーソルを先頭へ
    CursorHome,
    /// 入力欄のカーソルを末尾へ
    CursorEnd,
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
    /// 統計モーダルを開く
    OpenStats,
    /// ヒット中のモンスターのエクスポートモーダルを開く
    OpenExport,
    /// エクスポート形式を切り替える
    CycleExportFormat,
    /// 入力した内容でエクスポートする
    SubmitExport,
    /// 設定のデータファイルを読み直す
    Reload,
    /// カーソル行の項目を選択リストへ追加する／外す
    TogglePick,
    /// 選択リストのカーソル行を外す
    RemovePick,
    /// 選択リストのペインと結果リストの間でフォーカスを移す
    TogglePicksFocus,
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
            KeyCode::Enter | KeyCode::Esc => Some(Action::FocusList),
            _ => from_line_input_key(key.code, ctrl),
        };
    }

    if ctrl {
        return None;
    }
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('1') => Some(Action::SwitchTab(Tab::Monster)),
        KeyCode::Char('2') => Some(Action::SwitchTab(Tab::Spell)),
        // タブは2つなので ← が左（モンスター）、→ が右（スペル）
        KeyCode::Left => Some(Action::SwitchTab(Tab::Monster)),
        KeyCode::Right => Some(Action::SwitchTab(Tab::Spell)),
        KeyCode::Char('/') => Some(Action::FocusSearch),
        KeyCode::Char('c') => Some(Action::CopyCcfolia),
        KeyCode::Char('y') => Some(Action::CopyPalette),
        KeyCode::Char('Y') => Some(Action::CopyAllPalettes),
        KeyCode::Char('?') => Some(Action::OpenHelp),
        KeyCode::Char('J') => Some(Action::OpenJson),
        KeyCode::Char('e') => Some(Action::OpenExport),
        KeyCode::Char('S') => Some(Action::OpenStats),
        KeyCode::Char('R') => Some(Action::Reload),
        KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::Top),
        KeyCode::Char('G') | KeyCode::End => Some(Action::Bottom),
        KeyCode::Char(' ') if focus == Focus::List => Some(Action::TogglePick),
        KeyCode::Char('x') if focus == Focus::Picks => Some(Action::RemovePick),
        KeyCode::Char('v') => Some(Action::TogglePicksFocus),
        KeyCode::Esc if focus == Focus::Picks => Some(Action::FocusList),
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
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q' | '?' | 'J' | 'S') => Some(Action::CloseModal),
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

/// 1行入力（検索バー・エクスポートの入力欄）で共通のキー: 文字入力・削除・カーソル移動
///
/// `Ctrl-c`（終了）は呼び出し側で先に処理する。
fn from_line_input_key(code: KeyCode, ctrl: bool) -> Option<Action> {
    if ctrl {
        return match code {
            KeyCode::Char('u') => Some(Action::ClearQuery),
            KeyCode::Char('a') => Some(Action::CursorHome),
            KeyCode::Char('e') => Some(Action::CursorEnd),
            _ => None,
        };
    }
    match code {
        KeyCode::Char(c) => Some(Action::InsertChar(c)),
        KeyCode::Backspace => Some(Action::DeleteChar),
        KeyCode::Delete => Some(Action::DeleteCharForward),
        KeyCode::Left => Some(Action::CursorLeft),
        KeyCode::Right => Some(Action::CursorRight),
        KeyCode::Home => Some(Action::CursorHome),
        KeyCode::End => Some(Action::CursorEnd),
        _ => None,
    }
}

/// 入力フォームのモーダル（エクスポート）表示中のキー
///
/// 英字・数字はすべて出力先への文字入力として扱う（`q` などで閉じない）。
/// `←→` は入力欄のカーソル移動、形式の切り替えは `Tab` / `Shift-Tab` / `↑↓`。
pub fn from_form_key(key: KeyEvent) -> Option<Action> {
    if key.kind != KeyEventKind::Press {
        return None;
    }
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match key.code {
        KeyCode::Char('c') if ctrl => Some(Action::Quit),
        KeyCode::Esc => Some(Action::CloseModal),
        KeyCode::Enter => Some(Action::SubmitExport),
        KeyCode::Tab | KeyCode::BackTab | KeyCode::Up | KeyCode::Down => Some(Action::CycleExportFormat),
        code => from_line_input_key(code, ctrl),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyEventState;

    #[test]
    fn test_e_opens_export_outside_search() {
        assert_eq!(from_key(press(KeyCode::Char('e')), Focus::List, false), Some(Action::OpenExport));
        assert_eq!(from_key(press(KeyCode::Char('e')), Focus::Detail, false), Some(Action::OpenExport));
        assert_eq!(from_key(press(KeyCode::Char('e')), Focus::Search, false), Some(Action::InsertChar('e')));
        assert_eq!(from_key(press(KeyCode::Char('e')), Focus::List, true), None);
    }

    #[test]
    fn test_shift_s_opens_stats() {
        let shift_s = KeyEvent::new(KeyCode::Char('S'), KeyModifiers::SHIFT);
        assert_eq!(from_key(shift_s, Focus::List, false), Some(Action::OpenStats));
        assert_eq!(from_key(shift_s, Focus::Detail, false), Some(Action::OpenStats));
        assert_eq!(from_key(shift_s, Focus::Search, false), Some(Action::InsertChar('S')));
        // 統計の表示中にもう一度押すと閉じる
        assert_eq!(from_key(shift_s, Focus::List, true), Some(Action::CloseModal));
    }

    #[test]
    fn test_shift_r_reloads_outside_search_and_modal() {
        let shift_r = KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT);
        assert_eq!(from_key(shift_r, Focus::List, false), Some(Action::Reload));
        assert_eq!(from_key(shift_r, Focus::Detail, false), Some(Action::Reload));
        assert_eq!(from_key(press(KeyCode::Char('R')), Focus::List, false), Some(Action::Reload));
        // 検索中は文字入力、モーダル表示中・入力フォームでは読み直さない
        assert_eq!(from_key(shift_r, Focus::Search, false), Some(Action::InsertChar('R')));
        assert_eq!(from_key(shift_r, Focus::List, true), None);
        assert_eq!(from_form_key(shift_r), Some(Action::InsertChar('R')));
        // 小文字の r は割り当てなし
        assert_eq!(from_key(press(KeyCode::Char('r')), Focus::List, false), None);
    }

    #[test]
    fn test_form_keys() {
        for c in ['q', 'e', 'J', '?', '~', '/', 'テ'] {
            assert_eq!(from_form_key(press(KeyCode::Char(c))), Some(Action::InsertChar(c)), "{c}");
        }
        assert_eq!(from_form_key(press(KeyCode::Backspace)), Some(Action::DeleteChar));
        assert_eq!(from_form_key(ctrl('u')), Some(Action::ClearQuery));
        assert_eq!(from_form_key(ctrl('c')), Some(Action::Quit));
        assert_eq!(from_form_key(ctrl('x')), None);
        assert_eq!(from_form_key(press(KeyCode::Esc)), Some(Action::CloseModal));
        assert_eq!(from_form_key(press(KeyCode::Enter)), Some(Action::SubmitExport));
        for code in [KeyCode::Tab, KeyCode::BackTab, KeyCode::Up, KeyCode::Down] {
            assert_eq!(from_form_key(press(code)), Some(Action::CycleExportFormat), "{code:?}");
        }
        assert_eq!(from_form_key(press(KeyCode::Left)), Some(Action::CursorLeft));
        assert_eq!(from_form_key(press(KeyCode::Right)), Some(Action::CursorRight));
        assert_eq!(from_form_key(press(KeyCode::Home)), Some(Action::CursorHome));
        assert_eq!(from_form_key(press(KeyCode::End)), Some(Action::CursorEnd));
        assert_eq!(from_form_key(ctrl('a')), Some(Action::CursorHome));
        assert_eq!(from_form_key(ctrl('e')), Some(Action::CursorEnd));
        assert_eq!(from_form_key(press(KeyCode::Delete)), Some(Action::DeleteCharForward));
    }

    #[test]
    fn test_search_focus_cursor_keys() {
        let s = |key| from_key(key, Focus::Search, false);
        assert_eq!(s(press(KeyCode::Left)), Some(Action::CursorLeft));
        assert_eq!(s(press(KeyCode::Right)), Some(Action::CursorRight));
        assert_eq!(s(press(KeyCode::Home)), Some(Action::CursorHome));
        assert_eq!(s(press(KeyCode::End)), Some(Action::CursorEnd));
        assert_eq!(s(ctrl('a')), Some(Action::CursorHome));
        assert_eq!(s(ctrl('e')), Some(Action::CursorEnd));
        assert_eq!(s(press(KeyCode::Delete)), Some(Action::DeleteCharForward));
        assert_eq!(s(ctrl('x')), None);
        // 検索バー以外では従来どおり（←→ はタブ切り替え、Home/End は先頭・末尾の項目）
        assert_eq!(from_key(press(KeyCode::Home), Focus::List, false), Some(Action::Top));
        assert_eq!(from_key(press(KeyCode::End), Focus::Detail, false), Some(Action::Bottom));
        assert_eq!(from_key(ctrl('a'), Focus::List, false), None);
        assert_eq!(from_key(press(KeyCode::Delete), Focus::List, false), None);
    }

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
    fn test_space_toggles_pick_only_in_list() {
        let space = press(KeyCode::Char(' '));
        assert_eq!(from_key(space, Focus::List, false), Some(Action::TogglePick));
        assert_eq!(from_key(space, Focus::Detail, false), None);
        assert_eq!(from_key(space, Focus::Search, false), Some(Action::InsertChar(' ')));
    }

    #[test]
    fn test_picks_pane_keys() {
        let key = |c, focus| from_key(press(c), focus, false);
        assert_eq!(key(KeyCode::Char('x'), Focus::Picks), Some(Action::RemovePick));
        assert_eq!(key(KeyCode::Char('x'), Focus::List), None);
        assert_eq!(key(KeyCode::Char('v'), Focus::List), Some(Action::TogglePicksFocus));
        assert_eq!(key(KeyCode::Char('v'), Focus::Picks), Some(Action::TogglePicksFocus));
        assert_eq!(key(KeyCode::Char('v'), Focus::Search), Some(Action::InsertChar('v')));
        assert_eq!(key(KeyCode::Esc, Focus::Picks), Some(Action::FocusList));
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
        assert_eq!(from_key(press(KeyCode::Left), Focus::Detail, false), Some(Action::SwitchTab(Tab::Monster)));
        assert_eq!(from_key(press(KeyCode::Right), Focus::List, false), Some(Action::SwitchTab(Tab::Spell)));
        // 検索中（カーソル移動になる）とモーダル表示中は切り替えない
        assert_eq!(from_key(press(KeyCode::Right), Focus::Search, false), Some(Action::CursorRight));
        assert_eq!(from_key(press(KeyCode::Left), Focus::List, true), None);
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
