//! 検索バー

use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::super::app::{App, Focus, Tab};
use super::input_spans;

const PROMPT: &str = "検索> ";

/// 未入力時に出す入力例（タブごと）
fn placeholder(tab: Tab) -> &'static str {
    match tab {
        Tab::Monster => "（/ で検索。例: テスト lv:6 cat:蛮族）",
        Tab::Spell => "（/ で検索。例: テスト lv:3 s:MagicCat_1）",
    }
}

/// 検索バーを描画する。フォーカス中は端末のカーソルを入力のカーソル位置に置く（IME の変換窓もそこに出る）
///
/// 検索構文を解釈できないときはプロンプトと入力を警告色にし、理由を入力の後ろに出す。
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Search;
    let view = app.view();
    let query = &view.query;
    let error = view.query_error.as_deref();

    let warn = Style::default().fg(Color::Yellow);
    let prompt_style = match (error, focused) {
        (Some(_), _) => warn.add_modifier(Modifier::BOLD),
        (None, true) => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        (None, false) => Style::default().fg(Color::DarkGray),
    };
    let query_style = if error.is_some() { warn } else { Style::default() };
    let prompt_width = Line::from(PROMPT).width() as u16;
    // 入力が長いときはカーソルが見える位置まで横にずらす（エラーの理由は入力の後ろに続ける）
    let room = area.width.saturating_sub(prompt_width) as usize;
    let (input, cursor_col) = input_spans(query, room, query_style);
    let mut spans = vec![Span::styled(PROMPT, prompt_style)];
    spans.extend(input);
    if let Some(e) = error {
        spans.push(Span::styled(format!("  [{}]", e), warn));
    } else if query.is_empty() && !focused {
        spans.push(Span::styled(placeholder(app.tab), Style::default().fg(Color::DarkGray)));
    }
    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);

    if focused {
        let x = area.x.saturating_add(prompt_width).saturating_add(cursor_col);
        let x = x.min(area.right().saturating_sub(1));
        frame.set_cursor_position(Position::new(x, area.y));
    }
}
