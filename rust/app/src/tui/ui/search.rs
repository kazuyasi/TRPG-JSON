//! 検索バー

use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::super::app::{App, Focus, Tab};

const PROMPT: &str = "検索> ";

/// 未入力時に出す入力例（タブごと）
fn placeholder(tab: Tab) -> &'static str {
    match tab {
        Tab::Monster => "（/ で検索。例: テスト lv:6 cat:蛮族）",
        Tab::Spell => "（/ で検索。例: テスト lv:3 s:MagicCat_1）",
    }
}

/// 検索バーを描画する。フォーカス中は端末のカーソルを入力位置に置く（IME の変換窓もそこに出る）
///
/// 検索構文を解釈できないときはプロンプトと入力を警告色にし、理由を入力の後ろに出す。
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Search;
    let view = app.view();
    let query = view.query.as_str();
    let error = view.query_error.as_deref();

    let warn = Style::default().fg(Color::Yellow);
    let prompt_style = match (error, focused) {
        (Some(_), _) => warn.add_modifier(Modifier::BOLD),
        (None, true) => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        (None, false) => Style::default().fg(Color::DarkGray),
    };
    let query_style = if error.is_some() { warn } else { Style::default() };
    let mut spans = vec![Span::styled(PROMPT, prompt_style), Span::styled(query, query_style)];
    if let Some(e) = error {
        spans.push(Span::styled(format!("  [{}]", e), warn));
    } else if query.is_empty() && !focused {
        spans.push(Span::styled(placeholder(app.tab), Style::default().fg(Color::DarkGray)));
    }
    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);

    if focused {
        let offset = Line::from(vec![Span::raw(PROMPT), Span::raw(query)]).width() as u16;
        let x = area.x.saturating_add(offset).min(area.right().saturating_sub(1));
        frame.set_cursor_position(Position::new(x, area.y));
    }
}
