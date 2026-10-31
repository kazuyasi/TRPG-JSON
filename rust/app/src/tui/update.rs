//! 状態遷移（純粋関数。I/O は行わない）

use super::action::Action;
use super::app::{App, Focus, Tab};

/// Action を App に適用する
///
/// 副作用（クリップボード等）が必要になった段階で `Option<Effect>` を返すよう拡張する。
pub fn update(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,
        Action::SwitchTab(tab) => app.tab = tab,
        // スペルタブの検索・一覧は T049e で実装する
        _ if app.tab == Tab::Spell => {}
        Action::FocusSearch => app.focus = Focus::Search,
        Action::FocusList => app.focus = Focus::List,
        Action::FocusNext => app.focus = app.focus.next(),
        Action::FocusPrev => app.focus = app.focus.prev(),
        Action::InsertChar(c) => {
            app.monster_view.query.push(c);
            app.refresh_monster_hits();
        }
        Action::DeleteChar => {
            if app.monster_view.query.pop().is_some() {
                app.refresh_monster_hits();
            }
        }
        Action::ClearQuery => {
            if !app.monster_view.query.is_empty() {
                app.monster_view.query.clear();
                app.refresh_monster_hits();
            }
        }
        Action::Up | Action::Down | Action::PageUp | Action::PageDown | Action::Top | Action::Bottom => {
            if app.focus == Focus::Detail {
                move_in_detail(app, action);
            } else {
                move_in_list(app, action);
            }
        }
    }
}

fn move_in_list(app: &mut App, action: Action) {
    let view = &mut app.monster_view;
    let before = view.selected_index();
    let page = view.page() as isize;
    match action {
        Action::Up => view.move_by(-1),
        Action::Down => view.move_by(1),
        Action::PageUp => view.move_by(-page),
        Action::PageDown => view.move_by(page),
        Action::Top => view.selected = 0,
        Action::Bottom => view.selected = view.hits.len().saturating_sub(1),
        _ => {}
    }
    if view.selected_index() != before {
        app.reset_detail();
    }
}

/// 詳細ペイン: ↑↓ は部位表の選択、PgUp/PgDn・g/G は下部テキストのスクロール
fn move_in_detail(app: &mut App, action: Action) {
    let part_count = app.selected_monster().map(|m| m.part.len()).unwrap_or(0);
    let max = app.detail_max_scroll.get();
    let page = app.detail_page.get().max(1);
    match action {
        Action::Up if app.selected_part > 0 => {
            app.selected_part -= 1;
            app.detail_scroll = 0;
        }
        Action::Down if app.selected_part + 1 < part_count => {
            app.selected_part += 1;
            app.detail_scroll = 0;
        }
        Action::PageUp => app.detail_scroll = app.detail_scroll.saturating_sub(page),
        Action::PageDown => app.detail_scroll = app.detail_scroll.saturating_add(page).min(max),
        Action::Top => app.detail_scroll = 0,
        Action::Bottom => app.detail_scroll = max,
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::format::tests::{monster, part};

    fn sample_app() -> App {
        let monsters = vec![
            monster("テストモンスターB", 6, "蛮族", vec![part("", true)]),
            monster("テストの戦士", 3, "蛮族", vec![part("", true)]),
            monster("テストモンスター", 6, "蛮族", vec![part("頭部", false), part("胴体", true)]),
            monster("サンプル動物", 1, "動物", vec![part("", true)]),
        ];
        App::new(monsters, Vec::new(), Tab::Monster)
    }

    fn names(app: &App) -> Vec<&str> {
        app.monster_view.hits.iter().map(|&i| app.monsters[i].name.as_str()).collect()
    }

    fn type_str(app: &mut App, s: &str) {
        for c in s.chars() {
            update(app, Action::InsertChar(c));
        }
    }

    #[test]
    fn test_quit_sets_flag() {
        let mut app = App::default();
        update(&mut app, Action::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn test_switch_tab_keeps_monster_state() {
        let mut app = sample_app();
        update(&mut app, Action::FocusSearch);
        type_str(&mut app, "テスト");
        update(&mut app, Action::FocusList);
        update(&mut app, Action::Down);
        update(&mut app, Action::SwitchTab(Tab::Spell));
        assert_eq!(app.tab, Tab::Spell);
        update(&mut app, Action::SwitchTab(Tab::Monster));
        assert_eq!(app.monster_view.query, "テスト");
        assert_eq!(app.monster_view.selected, 1);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_initial_hits_sorted_by_level_then_name() {
        let app = sample_app();
        assert_eq!(
            names(&app),
            vec!["サンプル動物", "テストの戦士", "テストモンスター", "テストモンスターB"]
        );
        assert_eq!(app.selected_monster().unwrap().name, "サンプル動物");
    }

    #[test]
    fn test_search_filters_by_partial_name() {
        let mut app = sample_app();
        update(&mut app, Action::FocusSearch);
        type_str(&mut app, "モンスター");
        assert_eq!(names(&app), vec!["テストモンスター", "テストモンスターB"]);

        type_str(&mut app, "Z");
        assert!(app.monster_view.hits.is_empty());
        assert!(app.selected_monster().is_none());

        update(&mut app, Action::DeleteChar);
        assert_eq!(app.monster_view.hits.len(), 2);

        update(&mut app, Action::ClearQuery);
        assert_eq!(app.monster_view.hits.len(), 4);
    }

    #[test]
    fn test_whitespace_only_query_shows_all() {
        let mut app = sample_app();
        type_str(&mut app, "  ");
        assert_eq!(app.monster_view.hits.len(), 4);
    }

    #[test]
    fn test_search_keeps_selection_when_still_hit() {
        let mut app = sample_app();
        update(&mut app, Action::Bottom);
        assert_eq!(app.selected_monster().unwrap().name, "テストモンスターB");
        type_str(&mut app, "テスト");
        assert_eq!(app.selected_monster().unwrap().name, "テストモンスターB");
        // 外れたら先頭に戻る
        type_str(&mut app, "の");
        assert_eq!(app.selected_monster().unwrap().name, "テストの戦士");
    }

    #[test]
    fn test_list_movement_is_clamped() {
        let mut app = sample_app();
        update(&mut app, Action::Up);
        assert_eq!(app.monster_view.selected, 0);
        update(&mut app, Action::PageDown);
        assert_eq!(app.monster_view.selected, 3);
        update(&mut app, Action::Down);
        assert_eq!(app.monster_view.selected, 3);
        update(&mut app, Action::Top);
        assert_eq!(app.monster_view.selected, 0);
    }

    #[test]
    fn test_movement_on_empty_hits_does_not_panic() {
        let mut app = sample_app();
        type_str(&mut app, "該当なし");
        for a in [Action::Down, Action::Up, Action::PageDown, Action::Bottom, Action::Top] {
            update(&mut app, a);
        }
        app.focus = Focus::Detail;
        for a in [Action::Down, Action::Up, Action::PageDown, Action::Bottom] {
            update(&mut app, a);
        }
        assert_eq!(app.monster_view.selected, 0);
    }

    #[test]
    fn test_selected_part_defaults_to_core_and_moves_in_detail() {
        let mut app = sample_app();
        update(&mut app, Action::Down);
        update(&mut app, Action::Down);
        assert_eq!(app.selected_monster().unwrap().name, "テストモンスター");
        // コア部位（胴体 = index 1）が既定
        assert_eq!(app.selected_part, 1);

        update(&mut app, Action::FocusNext);
        assert_eq!(app.focus, Focus::Detail);
        update(&mut app, Action::Down);
        assert_eq!(app.selected_part, 1);
        update(&mut app, Action::Up);
        assert_eq!(app.selected_part, 0);
        update(&mut app, Action::Up);
        assert_eq!(app.selected_part, 0);
        // 詳細ペインでの移動はリストの選択を変えない
        assert_eq!(app.selected_monster().unwrap().name, "テストモンスター");

        // リストに戻って別のモンスターを選ぶと既定部位に戻る
        update(&mut app, Action::FocusList);
        update(&mut app, Action::Down);
        assert_eq!(app.selected_part, 0);
    }

    #[test]
    fn test_detail_scroll_is_clamped_by_last_render() {
        let mut app = sample_app();
        app.focus = Focus::Detail;
        app.detail_max_scroll.set(3);
        app.detail_page.set(2);
        update(&mut app, Action::PageDown);
        assert_eq!(app.detail_scroll, 2);
        update(&mut app, Action::PageDown);
        assert_eq!(app.detail_scroll, 3);
        update(&mut app, Action::PageUp);
        assert_eq!(app.detail_scroll, 1);
        update(&mut app, Action::Bottom);
        assert_eq!(app.detail_scroll, 3);
        update(&mut app, Action::Top);
        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn test_focus_cycle() {
        let mut app = sample_app();
        assert_eq!(app.focus, Focus::List);
        update(&mut app, Action::FocusNext);
        update(&mut app, Action::FocusNext);
        assert_eq!(app.focus, Focus::Search);
        update(&mut app, Action::FocusPrev);
        assert_eq!(app.focus, Focus::Detail);
    }

    #[test]
    fn test_spell_tab_ignores_monster_actions() {
        let mut app = sample_app();
        update(&mut app, Action::SwitchTab(Tab::Spell));
        update(&mut app, Action::FocusSearch);
        update(&mut app, Action::InsertChar('x'));
        assert_eq!(app.focus, Focus::List);
        assert!(app.monster_view.query.is_empty());
    }
}
