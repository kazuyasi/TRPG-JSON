//! モンスター詳細（右ペイン）
//!
//! 上から「見出し・基本値」「部位表」「特殊能力などの本文（スクロール可）」の3段で描画する。

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table, TableState, Wrap};
use trpg_json_core::Monster;

use super::super::app::{App, Focus};
use super::super::format;
use super::pane_block;

/// 部位表に一度に出す最大行数（ヘッダー除く）。超えた分は表内でスクロールする
const MAX_PART_ROWS: u16 = 6;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Detail;
    let block = pane_block(" 詳細 ", focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(monster) = app.selected_monster() else {
        app.detail_max_scroll.set(0);
        return;
    };

    let summary = format::monster_summary(monster);
    let part_rows = (monster.part.len() as u16).clamp(1, MAX_PART_ROWS);
    let [title, summary_area, parts_heading, parts_area, body] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(summary.len() as u16),
        Constraint::Length(1),
        Constraint::Length(part_rows + 1),
        Constraint::Min(0),
    ])
    .areas(inner);

    draw_title(frame, title, monster);
    let summary_lines: Vec<Line> = summary.iter().map(|row| summary_line(row)).collect();
    frame.render_widget(Paragraph::new(summary_lines), summary_area);

    frame.render_widget(
        Paragraph::new(heading(&format!("部位 ({})", monster.part.len()))),
        parts_heading,
    );
    draw_parts(frame, parts_area, app, monster, focused);
    draw_body(frame, body, app, monster);
}

fn draw_title(frame: &mut Frame, area: Rect, monster: &Monster) {
    let (name, right) = format::monster_title(monster);
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(Line::from(right.as_str()).width() as u16)])
            .areas(area);
    frame.render_widget(
        Paragraph::new(Span::styled(name, Style::default().add_modifier(Modifier::BOLD))),
        left_area,
    );
    frame.render_widget(Paragraph::new(right).alignment(Alignment::Right), right_area);
}

fn summary_line<'a>(row: &'a [(&'static str, String)]) -> Line<'a> {
    let mut spans = Vec::new();
    for (i, (label, value)) in row.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("    "));
        }
        spans.push(Span::styled(*label, Style::default().fg(Color::DarkGray)));
        spans.push(Span::raw("  "));
        spans.push(Span::raw(value.as_str()));
    }
    Line::from(spans)
}

/// 節の見出し（曖昧幅の罫線を避け、全角括弧で囲む）
fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("【{}】", text),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))
}

fn draw_parts(frame: &mut Frame, area: Rect, app: &App, monster: &Monster, focused: bool) {
    let right = |s: String| Cell::from(Line::from(s).alignment(Alignment::Right));
    let rows = format::part_rows(monster).into_iter().map(|r| {
        Row::new(vec![
            right(r.number.to_string()),
            Cell::from(r.name),
            right(r.count),
            right(r.hp),
            right(r.mp),
            right(r.hit),
            right(r.dodge),
            right(r.damage),
            right(r.armor),
            Cell::from(if r.core { "*" } else { "" }),
        ])
    });
    let header = Row::new(
        ["#", "部位", "数", "HP", "MP", "命中", "回避", "打撃", "防護", "コア"]
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                if i == 1 || i == 9 {
                    Cell::from(h)
                } else {
                    Cell::from(Line::from(h).alignment(Alignment::Right))
                }
            }),
    )
    .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD));

    // 80 桁でも部位名が読めるよう、数値列は実データの桁数に合わせて詰める
    let widths = [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ];
    // 詳細ペインにフォーカスがあるときだけ選択行を強調する（どの部位が選ばれているかは常に記号で示す）
    let highlight = if focused {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };
    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(highlight)
        .highlight_symbol("> ");

    let mut state = TableState::default().with_selected(Some(app.selected_part));
    frame.render_stateful_widget(table, area, &mut state);
}

fn draw_body(frame: &mut Frame, area: Rect, app: &App, monster: &Monster) {
    let mut lines: Vec<Line> = Vec::new();
    for (title, text) in format::monster_sections(monster, app.selected_part) {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(heading(&title));
        lines.extend(text.lines().map(|l| Line::from(format!(" {}", l))));
    }

    // 折り返し後の行数を見積もり、スクロール上限を update に伝える
    let width = area.width.max(1) as usize;
    let wrapped: usize = lines.iter().map(|l| l.width().max(1).div_ceil(width)).sum();
    let max_scroll = wrapped.saturating_sub(area.height as usize).min(u16::MAX as usize) as u16;
    app.detail_max_scroll.set(max_scroll);
    app.detail_page.set(area.height.max(1));

    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll.min(max_scroll), 0));
    frame.render_widget(para, area);
}
