//! 選択リスト（右端のペイン。空のときは出さない）

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{HighlightSpacing, List, ListState};

use super::super::app::{App, Focus, Tab};
use super::pane_block;

pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let view = app.view();
    let names: Vec<&str> = view
        .picks
        .iter()
        .map(|&i| match app.tab {
            Tab::Monster => app.monsters[i].name.as_str(),
            Tab::Spell => app.spells[i].name.as_str(),
        })
        .collect();
    let focused = app.focus == Focus::Picks;
    let title = format!(" 選択リスト ({}) ", names.len());
    let list = List::new(names)
        .block(pane_block(&title, focused))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ")
        .highlight_spacing(HighlightSpacing::Always);

    // カーソルはフォーカス中だけ見せる
    let mut state = ListState::default()
        .with_offset(view.pick_offset.get())
        .with_selected(focused.then_some(view.pick_cursor));
    frame.render_stateful_widget(list, area, &mut state);
    view.pick_offset.set(state.offset());
}
