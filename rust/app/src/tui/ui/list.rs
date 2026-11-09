//! 結果リスト（左ペイン）

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table, TableState, Wrap};

use super::super::app::{App, Focus, Tab};
use super::super::format;
use super::pane_block;
use crate::utils::{format_monster_filter_conditions, format_spell_filter_conditions};

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let view = app.view();
    let block = pane_block(" 結果 ", app.focus == Focus::List);

    if view.hits.is_empty() {
        draw_empty(frame, area, app, block);
        return;
    }

    // 各行は（Lv 列, 名前, 右端の分類列）。分類はモンスターがカテゴリ、スペルが系統
    let (third, rows): (&str, Vec<(String, &str, &str)>) = match app.tab {
        Tab::Monster => (
            "カテゴリ",
            view.hits
                .iter()
                .map(|&i| {
                    let m = &app.monsters[i];
                    (m.level.to_string(), m.name.as_str(), m.category.as_str())
                })
                .collect(),
        ),
        Tab::Spell => (
            "系統",
            view.hits
                .iter()
                .map(|&i| {
                    let s = &app.spells[i];
                    (format::spell_level(s).short(), s.name.as_str(), s.school.as_str())
                })
                .collect(),
        ),
    };
    let rows = rows.into_iter().zip(&view.hits).map(|((lv, name, class), &index)| {
        // 選択リストにある行は名前の前に印を付ける（印のない行の名前列の幅は変えない）
        let name = if view.is_picked(index) { format!("✓ {}", name) } else { name.to_string() };
        Row::new(vec![
            Cell::from(Line::from(lv).alignment(Alignment::Right)),
            Cell::from(name),
            Cell::from(class),
        ])
    });
    let header = Row::new(vec![
        Cell::from(Line::from("Lv").alignment(Alignment::Right)),
        Cell::from("名前"),
        Cell::from(third),
    ])
    .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD));

    let table = Table::new(
        rows,
        // 分類列はモンスターの最長カテゴリ「アンデッド」（全角5文字 = 10桁）が収まる幅にし、名前列を詰める。
        // スペルの系統も同じ幅（「MagicCat_1」のような 10 桁まで）
        [Constraint::Length(3), Constraint::Fill(1), Constraint::Length(10)],
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
    let text = match app.tab {
        Tab::Monster if app.monsters.is_empty() => Text::from("モンスターデータがありません"),
        Tab::Monster => {
            let q = &app.monster_filter;
            Text::from(format_monster_filter_conditions(
                q.name_label().as_deref(),
                q.level,
                q.category.as_deref(),
            ))
        }
        Tab::Spell if app.spells.is_empty() => Text::from("スペルデータがありません"),
        Tab::Spell => {
            let q = &app.spell_filter;
            Text::from(format_spell_filter_conditions(
                q.name_label().as_deref(),
                q.school.as_deref(),
                q.level,
                None,
                None,
                None,
            ))
        }
    };
    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}
