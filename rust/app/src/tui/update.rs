//! 状態遷移（純粋関数。I/O は行わない）

use trpg_json_core::export::palette::generate_spell_palette;

use super::action::Action;
use super::app::{App, Focus, StatusMessage, Tab};
use super::effect::Effect;
use crate::commands::monster::ccfolia_json;

/// Action を App に適用する
///
/// クリップボード書き込みなどの副作用が必要な場合は `Effect` を返す（実行はイベントループ側）。
pub fn update(app: &mut App, action: Action) -> Option<Effect> {
    match action {
        Action::CopyCcfolia if app.tab == Tab::Monster => return copy_ccfolia(app),
        Action::CopyPalette if app.tab == Tab::Spell => return copy_palette(app),
        Action::CopyAllPalettes if app.tab == Tab::Spell => return copy_all_palettes(app),
        // 他のタブのコピーキーは無視する
        Action::CopyCcfolia | Action::CopyPalette | Action::CopyAllPalettes => {}
        Action::Quit => app.should_quit = true,
        Action::SwitchTab(tab) => app.tab = tab,
        Action::FocusSearch => app.focus = Focus::Search,
        Action::FocusList => app.focus = Focus::List,
        Action::FocusNext => app.focus = app.focus.next(),
        Action::FocusPrev => app.focus = app.focus.prev(),
        Action::InsertChar(c) => {
            app.view_mut().query.push(c);
            app.refresh_hits();
        }
        Action::DeleteChar => {
            if app.view_mut().query.pop().is_some() {
                app.refresh_hits();
            }
        }
        Action::ClearQuery => {
            if !app.view().query.is_empty() {
                app.view_mut().query.clear();
                app.refresh_hits();
            }
        }
        Action::Up | Action::Down | Action::PageUp | Action::PageDown | Action::Top | Action::Bottom => {
            match (app.focus, app.tab) {
                (Focus::Detail, Tab::Monster) => move_in_monster_detail(app, action),
                // スペル詳細は部位表がないので ↑↓ も本文の1行スクロールにする
                (Focus::Detail, Tab::Spell) => scroll_detail(app, action),
                _ => move_in_list(app, action),
            }
        }
    }
    None
}

/// 選択中モンスターの選択部位をココフォリアの駒 JSON にし、コピーを要求する
///
/// 変換に失敗した場合は Effect を返さず、ステータス行にエラーを出す。
fn copy_ccfolia(app: &mut App) -> Option<Effect> {
    let Some(monster) = app.selected_monster() else {
        app.status = Some(StatusMessage::info("コピーするモンスターが選択されていません"));
        return None;
    };
    // selected_part は part[] の 0 始まり index、ccfolia_json は 1 始まり
    match ccfolia_json(monster, Some(app.selected_part + 1)) {
        Ok(piece) => Some(Effect::CopyToClipboard {
            text: piece.json,
            success: format!(
                "\"{}\" をココフォリア形式でクリップボードにコピーしました",
                piece.part_name
            ),
        }),
        Err(e) => {
            app.status = Some(StatusMessage::error(e.to_string()));
            None
        }
    }
}

/// 選択中スペルのチャットパレットのコピーを要求する
///
/// 成功時のステータス行にはパレットの先頭部分も出す（詳細ペインにはプレビューを出さないため）。
fn copy_palette(app: &mut App) -> Option<Effect> {
    let Some(spell) = app.selected_spell() else {
        app.status = Some(StatusMessage::info("コピーするスペルが選択されていません"));
        return None;
    };
    match generate_spell_palette(spell) {
        Ok(palette) => Some(Effect::CopyToClipboard {
            success: format!(
                "\"{}\" のチャットパレットをクリップボードにコピーしました: {}",
                spell.name, palette
            ),
            text: palette,
        }),
        Err(e) => {
            app.status = Some(StatusMessage::error(format!(
                "スペル '{}' のパレット生成に失敗しました: {}",
                spell.name, e
            )));
            None
        }
    }
}

/// ヒット中の全スペルのチャットパレットを改行区切りでコピーするよう要求する
///
/// CLI の `spell palette` と同じく、生成に失敗したスペルは飛ばして残りをコピーする。
fn copy_all_palettes(app: &mut App) -> Option<Effect> {
    let view = &app.spell_view;
    if view.hits.is_empty() {
        app.status = Some(StatusMessage::info("コピーするスペルがありません"));
        return None;
    }

    let mut palettes = Vec::new();
    let mut failed = Vec::new();
    for spell in view.hits.iter().map(|&i| &app.spells[i]) {
        match generate_spell_palette(spell) {
            Ok(p) => palettes.push(p),
            Err(_) => failed.push(spell.name.as_str()),
        }
    }

    if palettes.is_empty() {
        app.status = Some(StatusMessage::error(format!(
            "ヒット中の {} 件すべてでパレット生成に失敗しました",
            failed.len()
        )));
        return None;
    }

    let mut success = format!(
        "{} 件のチャットパレットをクリップボードにコピーしました",
        palettes.len()
    );
    if !failed.is_empty() {
        success.push_str(&format!(
            "（生成に失敗した {} 件を除外: {}）",
            failed.len(),
            failed.join(", ")
        ));
    }
    Some(Effect::CopyToClipboard { text: palettes.join("\n"), success })
}

fn move_in_list(app: &mut App, action: Action) {
    let view = app.view_mut();
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

/// モンスター詳細: ↑↓ は部位表の選択、PgUp/PgDn・g/G は下部テキストのスクロール
fn move_in_monster_detail(app: &mut App, action: Action) {
    let part_count = app.selected_monster().map(|m| m.part.len()).unwrap_or(0);
    match action {
        Action::Up if app.selected_part > 0 => {
            app.selected_part -= 1;
            app.monster_view.detail_scroll = 0;
        }
        Action::Down if app.selected_part + 1 < part_count => {
            app.selected_part += 1;
            app.monster_view.detail_scroll = 0;
        }
        Action::PageUp | Action::PageDown | Action::Top | Action::Bottom => scroll_detail(app, action),
        _ => {}
    }
}

/// 詳細ペイン本文のスクロール（上限は直近の描画で計算した値）
fn scroll_detail(app: &mut App, action: Action) {
    let max = app.detail_max_scroll.get();
    let page = app.detail_page.get().max(1);
    let scroll = &mut app.view_mut().detail_scroll;
    *scroll = match action {
        Action::Up => scroll.saturating_sub(1),
        Action::Down => scroll.saturating_add(1).min(max),
        Action::PageUp => scroll.saturating_sub(page),
        Action::PageDown => scroll.saturating_add(page).min(max),
        Action::Top => 0,
        Action::Bottom => max,
        _ => *scroll,
    };
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
        assert_eq!(app.monster_view.detail_scroll, 2);
        update(&mut app, Action::PageDown);
        assert_eq!(app.monster_view.detail_scroll, 3);
        update(&mut app, Action::PageUp);
        assert_eq!(app.monster_view.detail_scroll, 1);
        update(&mut app, Action::Bottom);
        assert_eq!(app.monster_view.detail_scroll, 3);
        update(&mut app, Action::Top);
        assert_eq!(app.monster_view.detail_scroll, 0);
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

    /// コピー要求の本文（JSON）と成功文言を取り出す
    fn copy_request(effect: Option<Effect>) -> (serde_json::Value, String) {
        match effect {
            Some(Effect::CopyToClipboard { text, success }) => {
                (serde_json::from_str(&text).expect("JSON として読めること"), success)
            }
            None => panic!("コピーが要求されていない"),
        }
    }

    #[test]
    fn test_copy_ccfolia_uses_default_core_part() {
        let mut app = sample_app();
        update(&mut app, Action::Bottom);
        update(&mut app, Action::Up);
        assert_eq!(app.selected_monster().unwrap().name, "テストモンスター");

        let (json, success) = copy_request(update(&mut app, Action::CopyCcfolia));
        assert_eq!(json["kind"], "character");
        let name = json["data"]["name"].as_str().unwrap();
        assert!(name.contains("胴体"), "{name}");
        assert_eq!(success, format!("\"{name}\" をココフォリア形式でクリップボードにコピーしました"));
        // ステータスは Effect の実行結果で設定するので、ここでは変えない
        assert!(app.status.is_none());
    }

    #[test]
    fn test_copy_ccfolia_follows_selected_part() {
        let mut app = sample_app();
        update(&mut app, Action::Bottom);
        update(&mut app, Action::Up);
        app.focus = Focus::Detail;
        update(&mut app, Action::Up);
        assert_eq!(app.selected_part, 0);

        let (json, _) = copy_request(update(&mut app, Action::CopyCcfolia));
        let name = json["data"]["name"].as_str().unwrap();
        assert!(name.contains("頭部"), "{name}");
    }

    #[test]
    fn test_copy_ccfolia_single_part_monster() {
        let mut app = sample_app();
        assert_eq!(app.selected_monster().unwrap().name, "サンプル動物");
        let (json, success) = copy_request(update(&mut app, Action::CopyCcfolia));
        assert_eq!(json["data"]["name"], "サンプル動物");
        assert!(success.starts_with("\"サンプル動物\" を"));
    }

    #[test]
    fn test_copy_ccfolia_without_selection_shows_info() {
        let mut app = sample_app();
        type_str(&mut app, "該当なし");
        assert!(update(&mut app, Action::CopyCcfolia).is_none());
        let status = app.status.as_ref().unwrap();
        assert_eq!(status.kind, crate::tui::app::StatusKind::Info);
        assert_eq!(status.text, "コピーするモンスターが選択されていません");
    }

    #[test]
    fn test_copy_ccfolia_without_parts_shows_error() {
        let monsters = vec![monster("テスト部位なし", 1, "動物", vec![])];
        let mut app = App::new(monsters, Vec::new(), Tab::Monster);
        assert!(update(&mut app, Action::CopyCcfolia).is_none());
        let status = app.status.as_ref().unwrap();
        assert_eq!(status.kind, crate::tui::app::StatusKind::Error);
        assert_eq!(status.text, "\"テスト部位なし\" には部位データがありません");
    }

    #[test]
    fn test_copy_ccfolia_ignored_on_spell_tab() {
        let mut app = sample_app();
        update(&mut app, Action::SwitchTab(Tab::Spell));
        assert!(update(&mut app, Action::CopyCcfolia).is_none());
        assert!(app.status.is_none());
    }

    // ---- スペルタブ ----

    use crate::tui::app::StatusKind;
    use crate::tui::format::tests::{full_spell, spell};
    use serde_json::json;

    fn lv(n: i32) -> serde_json::Value {
        json!({"kind": "value", "value": n})
    }

    fn spell_app() -> App {
        let spells = vec![
            full_spell("テストスペルB", lv(3)),
            full_spell("サンプルスペル", json!({"kind": "rank", "rank": 1})),
            full_spell("テストスペル", lv(3)),
            full_spell("テストの呪文", lv(1)),
            // 効果がないのでパレットを生成できない
            spell("テスト不完全", "MagicCat_1", json!({"Lv": lv(5)})),
        ];
        let monsters = vec![monster("テストモンスター", 1, "蛮族", vec![part("", true)])];
        App::new(monsters, spells, Tab::Spell)
    }

    fn spell_names(app: &App) -> Vec<&str> {
        app.spell_view.hits.iter().map(|&i| app.spells[i].name.as_str()).collect()
    }

    fn copy_text(effect: Option<Effect>) -> (String, String) {
        match effect {
            Some(Effect::CopyToClipboard { text, success }) => (text, success),
            None => panic!("コピーが要求されていない"),
        }
    }

    #[test]
    fn test_spell_hits_sorted_by_level_then_rank_then_name() {
        let app = spell_app();
        assert_eq!(
            spell_names(&app),
            vec!["テストの呪文", "テストスペル", "テストスペルB", "テスト不完全", "サンプルスペル"]
        );
        assert_eq!(app.hit_count(), 5);
        assert_eq!(app.total_count(), 5);
    }

    #[test]
    fn test_spell_search_filters_by_partial_name() {
        let mut app = spell_app();
        update(&mut app, Action::FocusSearch);
        type_str(&mut app, "スペル");
        assert_eq!(spell_names(&app), vec!["テストスペル", "テストスペルB", "サンプルスペル"]);
        // モンスター側の検索語には影響しない
        assert!(app.monster_view.query.is_empty());
        assert_eq!(app.monster_view.hits.len(), 1);

        update(&mut app, Action::ClearQuery);
        assert_eq!(app.spell_view.hits.len(), 5);
    }

    #[test]
    fn test_tabs_keep_separate_queries() {
        let mut app = spell_app();
        type_str(&mut app, "呪文");
        update(&mut app, Action::SwitchTab(Tab::Monster));
        type_str(&mut app, "モンスター");
        update(&mut app, Action::SwitchTab(Tab::Spell));
        assert_eq!(app.spell_view.query, "呪文");
        assert_eq!(app.monster_view.query, "モンスター");
        assert_eq!(spell_names(&app), vec!["テストの呪文"]);
    }

    #[test]
    fn test_spell_list_movement_and_detail_scroll() {
        let mut app = spell_app();
        update(&mut app, Action::Down);
        assert_eq!(app.selected_spell().unwrap().name, "テストスペル");
        // モンスターの選択は動かない
        assert_eq!(app.monster_view.selected, 0);

        app.focus = Focus::Detail;
        app.detail_max_scroll.set(5);
        app.detail_page.set(3);
        update(&mut app, Action::Down);
        assert_eq!(app.spell_view.detail_scroll, 1);
        update(&mut app, Action::PageDown);
        assert_eq!(app.spell_view.detail_scroll, 4);
        update(&mut app, Action::PageDown);
        assert_eq!(app.spell_view.detail_scroll, 5);
        update(&mut app, Action::Up);
        assert_eq!(app.spell_view.detail_scroll, 4);
        update(&mut app, Action::Top);
        assert_eq!(app.spell_view.detail_scroll, 0);
        update(&mut app, Action::Bottom);
        assert_eq!(app.spell_view.detail_scroll, 5);
        // 詳細でのスクロールはリストの選択を変えない
        assert_eq!(app.selected_spell().unwrap().name, "テストスペル");

        // 別のスペルを選ぶとスクロールが戻る
        app.focus = Focus::List;
        update(&mut app, Action::Down);
        assert_eq!(app.spell_view.detail_scroll, 0);
    }

    #[test]
    fn test_copy_palette_of_selected_spell() {
        let mut app = spell_app();
        update(&mut app, Action::Down);
        let (text, success) = copy_text(update(&mut app, Action::CopyPalette));
        assert_eq!(
            text,
            "2d+{MagicCat_1}+{行使修正}  テストスペル / MP:5 / 対象:1体 / 射程:術者 / 時間:一瞬 / テスト効果"
        );
        assert_eq!(
            success,
            format!("\"テストスペル\" のチャットパレットをクリップボードにコピーしました: {text}")
        );
        assert!(app.status.is_none());
    }

    #[test]
    fn test_copy_palette_failure_shows_error() {
        let mut app = spell_app();
        type_str(&mut app, "不完全");
        assert!(update(&mut app, Action::CopyPalette).is_none());
        let status = app.status.as_ref().unwrap();
        assert_eq!(status.kind, StatusKind::Error);
        assert!(
            status.text.starts_with("スペル 'テスト不完全' のパレット生成に失敗しました: "),
            "{}",
            status.text
        );
    }

    #[test]
    fn test_copy_palette_without_selection_shows_info() {
        let mut app = spell_app();
        type_str(&mut app, "該当なし");
        assert!(update(&mut app, Action::CopyPalette).is_none());
        assert_eq!(app.status.as_ref().unwrap().text, "コピーするスペルが選択されていません");
        assert!(update(&mut app, Action::CopyAllPalettes).is_none());
        let status = app.status.as_ref().unwrap();
        assert_eq!(status.kind, StatusKind::Info);
        assert_eq!(status.text, "コピーするスペルがありません");
    }

    #[test]
    fn test_copy_all_palettes_joins_hits_in_list_order() {
        let mut app = spell_app();
        type_str(&mut app, "テストスペル");
        let (text, success) = copy_text(update(&mut app, Action::CopyAllPalettes));
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains(" テストスペル / "), "{}", lines[0]);
        assert!(lines[1].contains(" テストスペルB / "), "{}", lines[1]);
        assert_eq!(success, "2 件のチャットパレットをクリップボードにコピーしました");
    }

    #[test]
    fn test_copy_all_palettes_skips_failures() {
        let mut app = spell_app();
        let (text, success) = copy_text(update(&mut app, Action::CopyAllPalettes));
        assert_eq!(text.lines().count(), 4);
        assert!(!text.contains("テスト不完全"));
        assert_eq!(
            success,
            "4 件のチャットパレットをクリップボードにコピーしました（生成に失敗した 1 件を除外: テスト不完全）"
        );

        // すべて失敗したらエラー
        type_str(&mut app, "不完全");
        assert!(update(&mut app, Action::CopyAllPalettes).is_none());
        let status = app.status.as_ref().unwrap();
        assert_eq!(status.kind, StatusKind::Error);
        assert_eq!(status.text, "ヒット中の 1 件すべてでパレット生成に失敗しました");
    }

    #[test]
    fn test_copy_keys_are_tab_specific() {
        let mut app = spell_app();
        assert!(update(&mut app, Action::CopyCcfolia).is_none());
        update(&mut app, Action::SwitchTab(Tab::Monster));
        assert!(update(&mut app, Action::CopyPalette).is_none());
        assert!(update(&mut app, Action::CopyAllPalettes).is_none());
        assert!(app.status.is_none());
    }

    #[test]
    fn test_spell_tab_without_data() {
        let mut app = App::new(Vec::new(), Vec::new(), Tab::Spell);
        type_str(&mut app, "テスト");
        for a in [Action::Down, Action::Bottom, Action::PageUp] {
            update(&mut app, a);
        }
        assert!(app.selected_spell().is_none());
        assert!(update(&mut app, Action::CopyPalette).is_none());
    }
}
