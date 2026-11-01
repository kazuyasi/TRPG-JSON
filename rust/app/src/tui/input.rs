//! 1行入力の部品（入力文字列＋カーソル位置）
//!
//! 検索バーとエクスポートモーダルの入力欄で共用する。カーソル位置は文字（`char`）単位で持ち、
//! 全角文字も1文字として移動・削除する。枠に収まらない入力は、カーソルが見える位置まで
//! 横にずらして表示する（ずらし量は描画時に書き戻し、次の描画でなるべく動かさない）。

use std::cell::Cell;
use std::ops::Deref;

use ratatui::text::Span;

/// 1行入力の状態
#[derive(Debug, Clone, Default)]
pub struct LineInput {
    text: String,
    /// カーソル位置（先頭からの文字数。`0..=文字数`）
    cursor: usize,
    /// 直近の描画で先頭に表示した文字の位置（描画時に書き戻す）
    scroll: Cell<usize>,
}

/// 入力欄の表示内容（`LineInput::view` の結果）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputView {
    /// 表示する部分の文字列
    pub text: String,
    /// 左側を省いたか（`<` を出す）
    pub clipped_left: bool,
    /// 右側を省いたか（`>` を出す）
    pub clipped_right: bool,
    /// 入力欄の左端から見たカーソルの桁（`<` の分を含む）
    pub cursor_col: u16,
}

impl LineInput {
    /// 入力文字列（カーソルは末尾）
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.chars().count();
        Self { text, cursor, scroll: Cell::new(0) }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// カーソル位置（先頭からの文字数）
    #[cfg(test)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    fn len_chars(&self) -> usize {
        self.text.chars().count()
    }

    /// カーソル位置の byte offset
    fn byte_at(&self, char_index: usize) -> usize {
        self.text.char_indices().nth(char_index).map_or(self.text.len(), |(i, _)| i)
    }

    /// カーソル位置に1文字挿入し、カーソルをその後ろへ
    pub fn insert(&mut self, c: char) {
        let at = self.byte_at(self.cursor);
        self.text.insert(at, c);
        self.cursor += 1;
    }

    /// カーソルの前の1文字を削除する（`Backspace`）。消したら `true`
    pub fn backspace(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        self.cursor -= 1;
        let at = self.byte_at(self.cursor);
        self.text.remove(at);
        true
    }

    /// カーソル位置の1文字を削除する（`Delete`）。消したら `true`
    pub fn delete(&mut self) -> bool {
        if self.cursor >= self.len_chars() {
            return false;
        }
        let at = self.byte_at(self.cursor);
        self.text.remove(at);
        true
    }

    /// すべて消す。消したら `true`
    pub fn clear(&mut self) -> bool {
        self.cursor = 0;
        self.scroll.set(0);
        if self.text.is_empty() {
            return false;
        }
        self.text.clear();
        true
    }

    /// カーソルを1文字左へ
    pub fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// カーソルを1文字右へ
    pub fn move_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.len_chars());
    }

    /// カーソルを先頭へ
    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    /// カーソルを末尾へ
    pub fn move_end(&mut self) {
        self.cursor = self.len_chars();
    }

    /// 表示幅 `width` の入力欄に出す内容を求め、ずらし量を書き戻す
    ///
    /// カーソル位置の1桁（末尾ならその後ろの空き）も幅に含める。入力が収まれば先頭から表示し、
    /// 収まらなければ、直前の表示からなるべく動かさずにカーソルが見えるまで横にずらす。
    /// 省いた側には `<` / `>` の1桁を使う（幅 4 未満では使わない）。
    pub fn view(&self, width: usize) -> InputView {
        let widths: Vec<usize> = self.text.chars().map(char_width).collect();
        let n = widths.len();
        let cursor = self.cursor.min(n);
        let span = |from: usize, to: usize| -> usize { widths[from..to].iter().sum() };
        // カーソルの桁（文字の上ならその文字の幅、末尾なら 1）
        let cursor_cell = widths.get(cursor).copied().unwrap_or(1).max(1);

        if width == 0 {
            self.scroll.set(0);
            return InputView { text: String::new(), clipped_left: false, clipped_right: false, cursor_col: 0 };
        }

        // 省いた側の印（`<` / `>`）の幅。狭すぎる欄では印を出さない
        let marker = usize::from(width >= 4);
        let left_marker = |start: usize| marker * usize::from(start > 0);
        // `start` から表示したとき、末尾（とその後ろのカーソル桁）まで収まるか
        let fits_to_end = |start: usize| span(start, n) + 1 + left_marker(start) <= width;
        // `start` から表示したとき、カーソルが見えるか
        let cursor_visible = |start: usize| {
            let right = marker * usize::from(!fits_to_end(start));
            span(start, cursor) + cursor_cell + left_marker(start) + right <= width
        };

        let mut start = if fits_to_end(0) { 0 } else { self.scroll.get().min(cursor) };
        while start < cursor && !cursor_visible(start) {
            start += 1;
        }
        // 右側に空きがあるときは、収まる限り左へ戻す（削除で短くなったときなど）
        while start > 0 && fits_to_end(start - 1) {
            start -= 1;
        }
        self.scroll.set(start);

        let clipped_left = marker > 0 && start > 0;
        let clipped_right = marker > 0 && !fits_to_end(start);
        let room = width.saturating_sub(usize::from(clipped_left) + usize::from(clipped_right));
        let mut used = 0;
        let mut text = String::new();
        for (c, w) in self.text.chars().zip(widths.iter()).skip(start) {
            if used + w > room {
                break;
            }
            used += w;
            text.push(c);
        }
        let cursor_col = usize::from(clipped_left) + span(start, cursor);
        InputView {
            text,
            clipped_left,
            clipped_right,
            cursor_col: cursor_col.min(width.saturating_sub(1)).min(u16::MAX as usize) as u16,
        }
    }
}

/// 1文字の表示幅
fn char_width(c: char) -> usize {
    let mut buf = [0u8; 4];
    Span::raw(&*c.encode_utf8(&mut buf)).width()
}

impl Deref for LineInput {
    type Target = str;

    fn deref(&self) -> &str {
        &self.text
    }
}

impl PartialEq for LineInput {
    /// 表示のずらし量は比べない
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.cursor == other.cursor
    }
}

impl Eq for LineInput {}

impl PartialEq<str> for LineInput {
    fn eq(&self, other: &str) -> bool {
        self.text == other
    }
}

impl PartialEq<&str> for LineInput {
    fn eq(&self, other: &&str) -> bool {
        self.text == *other
    }
}

impl From<&str> for LineInput {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for LineInput {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_cursor_and_move() {
        let mut input = LineInput::new("ac");
        assert_eq!(input.cursor(), 2);
        input.move_left();
        input.insert('b');
        assert_eq!(input, "abc");
        assert_eq!(input.cursor(), 2);
        input.move_home();
        input.insert('テ');
        assert_eq!(input, "テabc");
        input.move_end();
        input.insert('ト');
        assert_eq!(input, "テabcト");
        assert_eq!(input.cursor(), 5);
    }

    #[test]
    fn test_moves_stop_at_ends() {
        let mut input = LineInput::new("テス");
        input.move_right();
        assert_eq!(input.cursor(), 2);
        input.move_left();
        input.move_left();
        input.move_left();
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_backspace_and_delete_around_cursor() {
        let mut input = LineInput::new("テスト");
        input.move_left();
        // カーソルの前（ス）を消す
        assert!(input.backspace());
        assert_eq!(input, "テト");
        assert_eq!(input.cursor(), 1);
        // カーソル位置（ト）を消す
        assert!(input.delete());
        assert_eq!(input, "テ");
        assert!(!input.delete());
        input.move_home();
        assert!(!input.backspace());
        assert_eq!(input, "テ");
        assert!(input.clear());
        assert!(!input.clear());
        assert_eq!(input.cursor(), 0);
    }

    #[test]
    fn test_view_fits() {
        let input = LineInput::new("abc");
        let v = input.view(10);
        assert_eq!(v, InputView { text: "abc".into(), clipped_left: false, clipped_right: false, cursor_col: 3 });
        // 全角は2桁
        let input = LineInput::new("テスト");
        assert_eq!(input.view(10).cursor_col, 6);
    }

    #[test]
    fn test_view_scrolls_to_cursor() {
        // 幅 6 に 10 文字。末尾のカーソルは右端の手前
        let mut input = LineInput::new("0123456789");
        let v = input.view(6);
        assert!(v.clipped_left && !v.clipped_right, "{v:?}");
        assert_eq!(v.text, "6789");
        assert_eq!(v.cursor_col, 5);

        // 左へ動かしても、見えている間はずらさない
        input.move_left();
        input.move_left();
        let v = input.view(6);
        assert_eq!(v.text, "6789");
        assert_eq!(v.cursor_col, 3);

        // 見えている範囲の左へ出たらずらす（右側も省く）
        for _ in 0..3 {
            input.move_left();
        }
        let v = input.view(6);
        assert!(v.clipped_left && v.clipped_right, "{v:?}");
        assert_eq!(v.text, "5678");
        assert_eq!(v.cursor_col, 1);

        // 先頭へ戻ると左は省かない
        input.move_home();
        let v = input.view(6);
        assert!(!v.clipped_left && v.clipped_right, "{v:?}");
        assert_eq!(v.text, "01234");
        assert_eq!(v.cursor_col, 0);
    }

    #[test]
    fn test_view_returns_left_after_shrinking() {
        let mut input = LineInput::new("0123456789");
        input.view(6);
        for _ in 0..6 {
            input.backspace();
        }
        let v = input.view(6);
        assert_eq!(v, InputView { text: "0123".into(), clipped_left: false, clipped_right: false, cursor_col: 4 });
    }

    #[test]
    fn test_view_wide_chars_do_not_overflow() {
        let mut input = LineInput::new("テストテストテスト");
        for width in 1..20 {
            for _ in 0..12 {
                let v = input.view(width);
                let shown = char_width_sum(&v.text) + usize::from(v.clipped_left) + usize::from(v.clipped_right);
                assert!(shown <= width, "{width}: {v:?}");
                assert!((v.cursor_col as usize) < width, "{width}: {v:?}");
                input.move_left();
            }
            input.move_end();
        }
    }

    fn char_width_sum(s: &str) -> usize {
        s.chars().map(char_width).sum()
    }
}
