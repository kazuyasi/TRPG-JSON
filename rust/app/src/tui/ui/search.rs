//! 検索バー

use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::super::app::{App, Focus};

const PROMPT: &str = "検索> ";

/// 検索バーを描画する。フォーカス中は端末のカーソルを入力位置に置く（IME の変換窓もそこに出る）
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Search;
    let query = app.view().query.as_str();

    let prompt_style = if focused {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let mut spans = vec![Span::styled(PROMPT, prompt_style), Span::raw(query)];
    if query.is_empty() && !focused {
        spans.push(Span::styled(
            "（/ で名前を入力して絞り込み）",
            Style::default().fg(Color::DarkGray),
        ));
    }
    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);

    if focused {
        let offset = Line::from(vec![Span::raw(PROMPT), Span::raw(query)]).width() as u16;
        let x = area.x.saturating_add(offset).min(area.right().saturating_sub(1));
        frame.set_cursor_position(Position::new(x, area.y));
    }
}
