//! 詳細（右ペイン）
//!
//! モンスター: 上から「見出し・基本値」「部位表」「特殊能力などの本文（スクロール可）」の3段で描画する。
//! スペル: 「見出し」の下に基本値と効果をまとめて本文（スクロール可）として描画する。
//! チャットパレットはプレビュー表示しない（`y` でコピーのみ。TUI_DESIGN.md Q6）。

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Paragraph, Row, Table, TableState, Wrap};
use trpg_json_core::{Monster, Spell};

use super::super::app::{App, Focus, Tab};
use super::super::format;
use super::{heading, pane_block, wrapped_height};

/// 部位表に一度に出す最大行数（ヘッダー除く）。超えた分は表内でスクロールする
const MAX_PART_ROWS: u16 = 6;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Detail;
    let block = pane_block(" 詳細 ", focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    match app.tab {
        Tab::Monster => match app.selected_monster() {
            Some(monster) => draw_monster(frame, inner, app, monster, focused),
            None => app.detail_max_scroll.set(0),
        },
        Tab::Spell => match app.selected_spell() {
            Some(spell) => draw_spell(frame, inner, app, spell),
            None => app.detail_max_scroll.set(0),
        },
    }
}

fn draw_monster(frame: &mut Frame, inner: Rect, app: &App, monster: &Monster, focused: bool) {

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

    let (name, right) = format::monster_title(monster);
    draw_title(frame, title, name, right);
    let summary_lines: Vec<Line> = summary.iter().map(|row| summary_line(row)).collect();
    frame.render_widget(Paragraph::new(summary_lines), summary_area);

    frame.render_widget(
        Paragraph::new(heading(&format!("部位 ({})", monster.part.len()))),
        parts_heading,
    );
    draw_parts(frame, parts_area, app, monster, focused);
    let lines = section_lines(format::monster_sections(monster, app.selected_part));
    draw_body(frame, body, app, lines);
}

fn draw_spell(frame: &mut Frame, inner: Rect, app: &App, spell: &Spell) {
    let [title, body] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner);
    let (name, right) = format::spell_title(spell);
    draw_title(frame, title, name, right);

    // 基本値は狭い端末で折り返せるよう、効果と同じスクロール領域に入れる
    let summary = format::spell_summary(spell);
    let mut lines: Vec<Line> = summary.iter().map(|row| summary_line(row)).collect();
    let sections = section_lines(format::spell_sections(spell));
    if !sections.is_empty() {
        lines.push(Line::from(""));
        lines.extend(sections);
    }
    draw_body(frame, body, app, lines);
}

fn draw_title(frame: &mut Frame, area: Rect, name: String, right: String) {
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

/// 見出し付きのテキスト節を行にする（節の間は空行）
fn section_lines(sections: Vec<(String, String)>) -> Vec<Line<'static>> {
    let mut lines: Vec<Line> = Vec::new();
    for (title, text) in sections {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(heading(&title));
        lines.extend(text.lines().map(|l| Line::from(format!(" {}", l))));
    }
    lines
}

/// 折り返し・スクロール可能な本文を描画する
fn draw_body(frame: &mut Frame, area: Rect, app: &App, lines: Vec<Line>) {
    // 折り返し後の行数を見積もり、スクロール上限を update に伝える
    let wrapped = wrapped_height(&lines, area.width);
    let max_scroll = wrapped.saturating_sub(area.height as usize).min(u16::MAX as usize) as u16;
    app.detail_max_scroll.set(max_scroll);
    app.detail_page.set(area.height.max(1));

    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((app.view().detail_scroll.min(max_scroll), 0));
    frame.render_widget(para, area);
}
