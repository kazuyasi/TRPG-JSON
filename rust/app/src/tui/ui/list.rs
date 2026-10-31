//! 結果リスト（左ペイン）

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState, Wrap};

use super::super::app::{App, Focus};
use super::pane_block;
use crate::utils::format_monster_filter_conditions;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let view = &app.monster_view;
    let block = pane_block(" 結果 ", app.focus == Focus::List);

    if view.hits.is_empty() {
        draw_empty(frame, area, app, block);
        return;
    }

    let rows = view.hits.iter().map(|&i| {
        let m = &app.monsters[i];
        Row::new(vec![
            Cell::from(Line::from(m.level.to_string()).alignment(Alignment::Right)),
            Cell::from(m.name.as_str()),
            Cell::from(m.category.as_str()),
        ])
    });
    let header = Row::new(vec![
        Cell::from(Line::from("Lv").alignment(Alignment::Right)),
        Cell::from("名前"),
        Cell::from("カテゴリ"),
    ])
    .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD));

    let table = Table::new(
        rows,
        [Constraint::Length(3), Constraint::Fill(1), Constraint::Length(8)],
    )
    .header(header)
    .block(block)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol("> ");

    let mut state = TableState::default()
        .with_offset(view.offset.get())
        .with_selected(Some(view.selected));
    frame.render_stateful_widget(table, area, &mut state);

    // ratatui が選択行を表示範囲に収めるよう調整したオフセットを次回の描画に引き継ぐ
    view.offset.set(state.offset());
    // 枠線（上下 2 行）とヘッダー（1 行）を除いた行数
    view.page_size.set(area.height.saturating_sub(3).max(1) as usize);
}

fn draw_empty(frame: &mut Frame, area: Rect, app: &App, block: Block) {
    let text = if app.monsters.is_empty() {
        Text::from("モンスターデータがありません")
    } else {
        Text::from(format_monster_filter_conditions(app.monster_name_query(), None, None))
    };
    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}
