//! モーダル（ヘルプ・JSON ビュー）
//!
//! 本文の領域の上に重ねて描画する。ヘッダー・ステータス行・キーヘルプ行は隠さない
//! （JSON のコピー結果をステータス行で確認できるように）。

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use super::super::app::{Modal, ModalKind};
use super::super::help::SECTIONS;
use super::{heading, wrapped_height};

/// モーダルを描画する
pub fn draw(frame: &mut Frame, area: Rect, modal: &Modal) {
    let rect = modal_rect(area);
    let (title, hint, lines) = match &modal.kind {
        ModalKind::Help => (" ヘルプ ".to_string(), " Esc/q:閉じる ", help_lines()),
        ModalKind::Json { name, text } => (
            format!(" JSON: {} ", name),
            " y:コピー  Esc/q:閉じる ",
            text.lines().map(|l| Line::from(l.to_string())).collect(),
        ),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(title, Style::default().add_modifier(Modifier::BOLD)))
        .title_bottom(Line::from(hint).right_aligned());
    let inner = block.inner(rect);

    // 折り返し後の行数からスクロール上限を求め、update に伝える
    let max_scroll = wrapped_height(&lines, inner.width).saturating_sub(inner.height as usize);
    let max_scroll = max_scroll.min(u16::MAX as usize) as u16;
    modal.max_scroll.set(max_scroll);
    modal.page.set(inner.height.max(1));

    frame.render_widget(Clear, rect);
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((modal.scroll.min(max_scroll), 0));
    frame.render_widget(para, rect);
}

/// 本文領域の中央に置く。狭い端末では本文領域いっぱいに広げる
fn modal_rect(area: Rect) -> Rect {
    let width = if area.width >= 100 { area.width * 4 / 5 } else { area.width };
    let height = if area.height >= 20 { area.height * 9 / 10 } else { area.height };
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}

/// ヘルプの本文。キー列は全節で桁をそろえる
pub fn help_lines() -> Vec<Line<'static>> {
    let key_width = SECTIONS
        .iter()
        .flat_map(|s| s.entries.iter())
        .map(|(key, _)| Line::from(*key).width())
        .max()
        .unwrap_or(0);

    let mut lines = Vec::new();
    for section in SECTIONS {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(heading(section.title));
        for (key, desc) in section.entries {
            let pad = key_width - Line::from(*key).width();
            lines.push(Line::from(vec![
                Span::raw(" "),
                Span::styled(*key, Style::default().fg(Color::Yellow)),
                Span::raw(" ".repeat(pad + 2)),
                Span::raw(*desc),
            ]));
        }
    }
    lines
}
