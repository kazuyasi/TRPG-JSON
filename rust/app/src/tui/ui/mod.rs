//! 描画（ratatui）

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};

use super::app::{App, Focus, ModalKind, StatusKind, Tab};

mod detail;
mod list;
mod modal;
mod search;

/// 描画を行う最小の端末の高さ（これ未満は警告のみ表示）
pub const MIN_HEIGHT: u16 = 15;

/// 結果リストと詳細を左右に並べる最小の端末の幅（これ未満は1ペイン表示）
pub const SPLIT_MIN_WIDTH: u16 = 100;

/// 幅が狭く、結果リストと詳細を切り替えて1ペインずつ表示するか
pub fn is_narrow(width: u16) -> bool {
    width < SPLIT_MIN_WIDTH
}

/// 画面全体を描画する
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if area.height < MIN_HEIGHT {
        draw_too_small(frame, area);
        return;
    }

    let [header, body, status, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    draw_header(frame, header, app);
    draw_body(frame, body, app);
    draw_status(frame, status, app);
    draw_help(frame, help, app);
    if let Some(m) = &app.modal {
        modal::draw(frame, body, m);
    }
}

/// 節の見出し（曖昧幅の罫線を避け、全角括弧で囲む）
fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("【{}】", text),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))
}

/// 幅 `width` で折り返したときの行数の見積もり（スクロール上限の計算に使う）
fn wrapped_height(lines: &[Line], width: u16) -> usize {
    let width = width.max(1) as usize;
    lines.iter().map(|l| l.width().max(1).div_ceil(width)).sum()
}

fn draw_too_small(frame: &mut Frame, area: Rect) {
    let msg = Paragraph::new(format!("端末が小さすぎます（高さ {} 行以上が必要です）", MIN_HEIGHT))
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true });
    frame.render_widget(msg, area);
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let [title, tabs, count] = Layout::horizontal([
        Constraint::Length(4),
        Constraint::Min(0),
        Constraint::Length(20),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new(Span::styled("gm", Style::default().add_modifier(Modifier::BOLD))),
        title,
    );

    let tab_widget = Tabs::new(Tab::ALL.iter().map(|t| t.label()))
        .select(app.tab.index())
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
        )
        .divider(" ");
    frame.render_widget(tab_widget, tabs);

    frame.render_widget(
        Paragraph::new(count_label(app)).alignment(Alignment::Right),
        count,
    );
}

/// ヘッダー右端の件数表示（「総件数中 ヒット件数」）
fn count_label(app: &App) -> String {
    format!("{}件中 {}件", app.total_count(), app.hit_count())
}

/// 枠付きペイン。フォーカス中は枠を強調する
fn pane_block(title: &str, focused: bool) -> Block<'_> {
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    Block::default().borders(Borders::ALL).border_style(style).title(title)
}

/// 本文: 検索バーの下に結果リスト（左）と詳細（右）。モンスター・スペルとも同じ配置
///
/// 幅が狭い端末では左右に分けず、詳細ペインにフォーカスがあるときだけ詳細を、
/// それ以外（検索バー・結果リスト）は結果リストを全幅で表示する。
fn draw_body(frame: &mut Frame, area: Rect, app: &App) {
    let [search_area, panes] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
    search::draw(frame, search_area, app);

    if is_narrow(area.width) {
        if app.focus == Focus::Detail {
            detail::draw(frame, panes, app);
        } else {
            list::draw(frame, panes, app);
        }
        return;
    }

    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(panes);
    list::draw(frame, list_area, app);
    detail::draw(frame, detail_area, app);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let Some(status) = &app.status else {
        return;
    };
    let prefix = status.kind.prefix();
    let color = match status.kind {
        StatusKind::Success => Color::Green,
        StatusKind::Info => Color::Cyan,
        StatusKind::Error => Color::Red,
    };
    let line = Line::from(vec![
        Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::raw(status.text.as_str()),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

/// 現在のフォーカスで使える主要キー
///
/// 幅 80 桁に収まるよう短縮表記にしている。すべてのキーは `?` のヘルプで確認できる。
fn help_text(app: &App) -> &'static str {
    if let Some(m) = &app.modal {
        return match m.kind {
            ModalKind::Help => "↑↓/jk/PgUp/Dn/g/G:スクロール  Esc/q/?:閉じる  Ctrl-c:終了",
            ModalKind::Json { .. } => {
                "↑↓/jk/PgUp/Dn/g/G:スクロール  y:JSONをコピー  Esc/q/J:閉じる  Ctrl-c:終了"
            }
            ModalKind::Stats { .. } => "↑↓/jk/PgUp/Dn/g/G:スクロール  Esc/q/S:閉じる  Ctrl-c:終了",
            ModalKind::Export(_) => {
                "文字入力:出力先  Tab/↑↓:形式  Ctrl-u:クリア  Enter:出力  Esc:キャンセル"
            }
        };
    }
    match (app.tab, app.focus) {
        (_, Focus::Search) => {
            "文字入力:名前で絞り込み  Enter/Esc:リストへ  Ctrl-u:クリア  ↑↓:選択  Ctrl-c:終了"
        }
        (Tab::Monster, Focus::List) => {
            "↑↓/jk:選択  Enter:詳細  c:ココフォリア  e:出力  J:JSON  /:検索  ?:ヘルプ  q:終了"
        }
        (Tab::Monster, Focus::Detail) => {
            "↑↓/jk:部位  c:ココフォリア  e:出力  J:JSON  Esc:リスト  ?:ヘルプ  q:終了"
        }
        (Tab::Spell, Focus::List) => {
            "↑↓/jk:選択  Enter:詳細  y:パレット  Y:全件  J:JSON  /:検索  ?:ヘルプ  q:終了"
        }
        (Tab::Spell, Focus::Detail) => {
            "↑↓/jk/PgUp/Dn:本文  y:パレット  Y:全件  J:JSON  Esc:リスト  ?:ヘルプ  q:終了"
        }
    }
}

fn draw_help(frame: &mut Frame, area: Rect, app: &App) {
    let help = Paragraph::new(help_text(app)).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::StatusMessage;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    /// 描画結果を行ごとの文字列にする（全角文字の後続セル由来の空白は除く）
    fn render(app: &App, width: u16, height: u16) -> Vec<String> {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|f| draw(f, app)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| buffer[(x, y)].symbol().to_string())
                    .collect::<String>()
                    .replace(' ', "")
            })
            .collect()
    }

    fn contains(rows: &[String], needle: &str) -> bool {
        let needle = needle.replace(' ', "");
        rows.iter().any(|r| r.contains(&needle))
    }

    fn sample_app() -> App {
        use crate::tui::format::tests::{monster, part};
        let mut head = part("頭部", false);
        head.mp = -1;
        head.special_abilities = "テスト部位能力".into();
        let body = part("胴体", true);
        let mut m = monster("テストモンスター", 6, "蛮族", vec![head, body]);
        m.common_abilities = "共通テスト能力".into();
        m.weakness = "テスト属性ダメージ+3".into();
        m.moveon_description = "飛行".into();
        let other = monster("サンプル動物", 2, "アンデッド", vec![part("", true)]);
        App::new(vec![m, other], Vec::new(), Tab::Monster)
    }

    #[test]
    fn test_render_monster_tab() {
        let mut app = sample_app();
        // Lv 昇順なので先頭はサンプル動物。テストモンスターを選ぶ
        crate::tui::update::update(&mut app, crate::tui::action::Action::Down);
        // 幅 120 は左右に並べ、幅 80 は結果リスト → 詳細（Enter）を切り替えて表示する
        for (width, steps) in [(120, vec![Focus::List]), (80, vec![Focus::List, Focus::Detail])] {
            let mut list_rows = Vec::new();
            let mut detail_rows = Vec::new();
            for focus in steps {
                app.focus = focus;
                let rows = render(&app, width, 24);
                if focus == Focus::List || width >= SPLIT_MIN_WIDTH {
                    list_rows = rows.clone();
                }
                if focus == Focus::Detail || width >= SPLIT_MIN_WIDTH {
                    detail_rows = rows;
                }
            }
            app.focus = Focus::List;
            let rows = &list_rows;
            assert!(contains(rows, "1:モンスター"));
            assert!(contains(rows, "2:スペル"));
            assert!(contains(rows, "2件中 2件"));
            assert!(contains(rows, "検索>"));
            // 結果リスト
            assert!(contains(rows, "サンプル動物"));
            // 全角5文字のカテゴリが見切れない
            assert!(contains(rows, "アンデッド"), "{width}: {rows:#?}");
            assert!(contains(rows, "> 6"), "{width}: {rows:#?}");
            assert!(contains(rows, "q:終了"));
            // 詳細ペイン
            let rows = detail_rows;
            assert!(contains(&rows, "Lv.6 [蛮族]"));
            assert!(contains(&rows, "知名度/弱点値 9/14"));
            assert!(contains(&rows, "地上 15(飛行) / 水中 -"));
            assert!(contains(&rows, "弱点 テスト属性ダメージ+3"));
            assert!(contains(&rows, "【部位 (2)】"));
            assert!(contains(&rows, "頭部"));
            assert!(contains(&rows, "--"));
            assert!(contains(&rows, "【共通特殊能力】"));
            assert!(contains(&rows, "共通テスト能力"));
            // 既定はコア部位（胴体）で、胴体には部位特殊能力がない
            assert!(!contains(&rows, "テスト部位能力"));
        }
    }

    #[test]
    fn test_narrow_terminal_shows_one_pane() {
        let mut app = sample_app();
        // 結果リスト（と検索バー）にフォーカスがある間はリストだけ
        for focus in [Focus::List, Focus::Search] {
            app.focus = focus;
            let rows = render(&app, SPLIT_MIN_WIDTH - 1, 24);
            assert!(contains(&rows, "┌結果"));
            assert!(contains(&rows, "サンプル動物"));
            assert!(!contains(&rows, "┌詳細"), "{focus:?}: {rows:#?}");
        }
        assert!(contains(&render(&app, SPLIT_MIN_WIDTH - 1, 24), "Ctrl-u:クリア"));
        // 詳細にフォーカスすると詳細だけ
        app.focus = Focus::Detail;
        let rows = render(&app, SPLIT_MIN_WIDTH - 1, 24);
        assert!(contains(&rows, "┌詳細"));
        assert!(contains(&rows, "Lv.2 [アンデッド]"), "{rows:#?}");
        assert!(!contains(&rows, "┌結果"));
        assert!(contains(&rows, "Esc:リスト"));
        // 幅 100 以上は左右に並べる
        let rows = render(&app, SPLIT_MIN_WIDTH, 24);
        assert!(contains(&rows, "┌結果"));
        assert!(contains(&rows, "┌詳細"));
    }

    #[test]
    fn test_render_help_modal() {
        use crate::tui::app::Modal;
        let mut app = sample_app();
        app.modal = Some(Modal::new(ModalKind::Help));
        for width in [60, 80, 120] {
            let rows = render(&app, width, 30);
            assert!(contains(&rows, "ヘルプ"), "{width}: {rows:#?}");
            assert!(contains(&rows, "【全体】"));
            assert!(contains(&rows, "Tab / Shift-Tab"));
            // ヘッダー・キーヘルプ行は隠さない
            assert!(contains(&rows, "1:モンスター"));
            assert!(contains(&rows, "Esc/q/?:閉じる"));
            // スクロール上限が描画時に設定される（全節は 26 行に収まらない）
            assert!(app.modal.as_ref().unwrap().max_scroll.get() > 0, "{width}");
        }

        // 最後までスクロールすると末尾の節が見える
        let modal = app.modal.as_mut().unwrap();
        modal.scroll = u16::MAX;
        let rows = render(&app, 80, 30);
        assert!(contains(&rows, "【画面】"), "{rows:#?}");
        assert!(contains(&rows, "端末が小さすぎます"));
    }

    #[test]
    fn test_help_modal_lists_every_section() {
        let text: Vec<String> = modal::help_lines().iter().map(|l| l.to_string()).collect();
        for s in crate::tui::help::SECTIONS {
            assert!(text.contains(&format!("【{}】", s.title)), "{}", s.title);
            for (key, desc) in s.entries {
                assert!(
                    text.iter().any(|l| l.contains(key) && l.contains(desc)),
                    "{key}: {desc}"
                );
            }
        }
        // キーの列がそろっている（説明の開始桁が全行で同じ）
        let starts: std::collections::BTreeSet<usize> = modal::help_lines()
            .iter()
            .filter(|l| l.spans.len() == 4)
            .map(|l| l.spans[..3].iter().map(|s| s.width()).sum())
            .collect();
        assert_eq!(starts.len(), 1, "{starts:?}");
    }

    #[test]
    fn test_render_json_modal() {
        use crate::tui::app::Modal;
        let mut app = sample_app();
        crate::tui::update::update(&mut app, crate::tui::action::Action::OpenJson);
        assert!(matches!(app.modal, Some(Modal { kind: ModalKind::Json { .. }, .. })));
        for width in [60, 120] {
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "JSON: サンプル動物"), "{width}: {rows:#?}");
            assert!(contains(&rows, "\"name\": \"サンプル動物\""), "{width}: {rows:#?}");
            assert!(contains(&rows, "y:JSONをコピー"));
        }
    }

    #[test]
    fn test_render_stats_modal() {
        use crate::tui::action::Action;
        use crate::tui::update::update;
        let mut app = sample_app();
        app.monster_view.query = "テスト".into();
        app.refresh_monster_hits();
        update(&mut app, Action::OpenStats);
        for width in [60, 120] {
            let rows = render(&app, width, 30);
            assert!(contains(&rows, "モンスター統計"), "{width}: {rows:#?}");
            assert!(contains(&rows, "【ヒット中 1 件（検索: テスト）】"), "{width}: {rows:#?}");
            assert!(contains(&rows, "【全件 2 件】"));
            assert!(contains(&rows, "レベル分布"));
            assert!(contains(&rows, "Esc/q/S:閉じる"));
        }
    }

    #[test]
    fn test_render_export_modal() {
        use crate::tui::action::Action;
        use crate::tui::update::update;
        let mut app = sample_app();
        update(&mut app, Action::OpenExport);
        for width in [60, 80, 120] {
            let mut terminal = Terminal::new(TestBackend::new(width, 24)).unwrap();
            terminal.draw(|f| draw(f, &app)).unwrap();
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "エクスポート"), "{width}: {rows:#?}");
            assert!(contains(&rows, "ヒット中のモンスター 2 件"), "{width}: {rows:#?}");
            assert!(contains(&rows, "JSON"));
            assert!(contains(&rows, "Udonarium (ZIP)"));
            assert!(contains(&rows, "出力先 monsters.json"), "{width}: {rows:#?}");
            assert!(contains(&rows, "Esc:キャンセル"));
            // カーソルは出力先の末尾（行は出力先の行）
            let pos = terminal.get_cursor_position().unwrap();
            let row_text = &rows[pos.y as usize];
            assert!(row_text.contains("monsters.json"), "{width}: {row_text}");
        }

        // Sheets はシートID の入力欄と一時停止の注意書き
        update(&mut app, Action::CycleExportFormat);
        update(&mut app, Action::CycleExportFormat);
        update(&mut app, Action::InsertChar('x'));
        let rows = render(&app, 120, 24);
        assert!(contains(&rows, "Google Sheets"));
        assert!(contains(&rows, "シートID x"), "{rows:#?}");
        assert!(contains(&rows, "TUI を一時停止します"));
        assert!(!contains(&rows, "monsters"));
        update(&mut app, Action::CycleExportFormat);

        // 長い出力先は末尾を見せ、枠からはみ出さない
        for _ in 0..80 {
            update(&mut app, Action::InsertChar('a'));
        }
        update(&mut app, Action::InsertChar('Z'));
        let mut terminal = Terminal::new(TestBackend::new(60, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        let rows = render(&app, 60, 24);
        assert!(contains(&rows, "出力先 <aaa"), "{rows:#?}");
        assert!(contains(&rows, "aZ│"), "{rows:#?}");
        let pos = terminal.get_cursor_position().unwrap();
        assert!(pos.x < 59, "{pos:?}");
    }

    #[test]
    fn test_render_too_small_with_modal() {
        use crate::tui::app::Modal;
        let app = App { modal: Some(Modal::new(ModalKind::Help)), ..App::default() };
        let rows = render(&app, 40, MIN_HEIGHT - 1);
        assert!(contains(&rows, "端末が小さすぎます"));
        // 最小の高さ・狭い幅でも描画で panic しない
        render(&app, 20, MIN_HEIGHT);
    }

    #[test]
    fn test_render_part_abilities_follow_selected_part() {
        let mut app = sample_app();
        crate::tui::update::update(&mut app, crate::tui::action::Action::Down);
        app.selected_part = 0;
        let rows = render(&app, 120, 24);
        assert!(contains(&rows, "【部位特殊能力（選択中: 頭部）】"));
        assert!(contains(&rows, "テスト部位能力"));
    }

    #[test]
    fn test_render_no_hits_shows_conditions() {
        let mut app = sample_app();
        app.monster_view.query = "該当なし".into();
        app.refresh_monster_hits();
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "2件中 0件"));
        // 狭いペインでは折り返されるため、行をまたがない部分で確認する
        assert!(contains(&rows, "以下の条件でマッチする"));
        assert!(contains(&rows, "name: \"該当なし\""));
    }

    #[test]
    fn test_render_no_hits_lists_level_and_category() {
        let mut app = sample_app();
        app.monster_view.query = "テスト lv:99 cat:蛮族".into();
        app.refresh_monster_hits();
        let rows = render(&app, 120, 24);
        assert!(contains(&rows, "name: \"テスト\""));
        assert!(contains(&rows, "level: 99"));
        assert!(contains(&rows, "category: \"蛮族\""));
    }

    #[test]
    fn test_render_invalid_query_warning() {
        let mut app = sample_app();
        app.focus = Focus::Search;
        app.monster_view.query = "lv:abc".into();
        app.refresh_monster_hits();
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "検索> lv:abc  [lv: の値は整数で指定してください（abc）]"));
        // 直前（全件）の結果が残る
        assert!(contains(&rows, "2件中 2件"));
    }

    #[test]
    fn test_render_search_placeholder_per_tab() {
        let app = sample_app();
        assert!(contains(&render(&app, 80, 24), "例: テスト lv:6 cat:蛮族"));
        let app = spell_app();
        assert!(contains(&render(&app, 80, 24), "例: テスト lv:3 s:MagicCat_1"));
    }

    #[test]
    fn test_render_search_focus_sets_cursor_and_help() {
        let mut app = sample_app();
        app.focus = Focus::Search;
        app.monster_view.query = "テ".into();
        app.refresh_monster_hits();
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|f| draw(f, &app)).unwrap();
        // 「検索> 」(6 桁) + 「テ」(2 桁) の直後。検索バーはヘッダーの次の行
        terminal.backend_mut().assert_cursor_position((8, 1));
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "Ctrl-u:クリア"));
    }

    #[test]
    fn test_render_empty_data() {
        let app = App::default();
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "モンスターデータがありません"));
    }

    #[test]
    fn test_render_spell_tab_without_data() {
        let app = App { tab: Tab::Spell, ..App::default() };
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "0件中 0件"));
        assert!(contains(&rows, "スペルデータがありません"));
    }

    fn spell_app() -> App {
        use crate::tui::format::tests::{full_spell, spell};
        use serde_json::json;
        let mut a = full_spell("テストスペル", json!({"kind": "value", "value": 9}));
        a.extra.insert("形状".into(), json!("貫通"));
        a.extra.insert("属性".into(), json!("精神効果"));
        let b = spell(
            "サンプル呪文",
            "MagicCat_2",
            json!({"Lv": {"kind": "rank", "rank": 2}, "schoolVariant": "特殊", "god": "神名"}),
        );
        App::new(Vec::new(), vec![a, b], Tab::Spell)
    }

    #[test]
    fn test_render_spell_tab() {
        let mut app = spell_app();
        for width in [80, 120] {
            app.focus = Focus::List;
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "2件中 2件"), "{width}: {rows:#?}");
            assert!(contains(&rows, "検索>"));
            // 結果リスト（Lv 列・系統列）
            assert!(contains(&rows, "系統"));
            assert!(contains(&rows, "> 9"), "{width}: {rows:#?}");
            assert!(contains(&rows, "R2"));
            assert!(contains(&rows, "MagicCat_2"));
            assert!(contains(&rows, "y:パレット"));
            // 詳細ペイン（幅 80 は1ペイン表示なので詳細へ切り替える）
            if is_narrow(width) {
                app.focus = Focus::Detail;
            }
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "Lv.9 MagicCat_1"));
            assert!(contains(&rows, "MP 5"));
            assert!(contains(&rows, "時間 一瞬"));
            assert!(contains(&rows, "対象 1体"));
            assert!(contains(&rows, "射程 術者(30m)"));
            assert!(contains(&rows, "形状 貫通"));
            assert!(contains(&rows, "抵抗 半減"));
            assert!(contains(&rows, "属性 精神効果"));
            assert!(contains(&rows, "【効果】"));
            assert!(contains(&rows, "テスト効果"));
            // チャットパレットはプレビューしない
            assert!(!contains(&rows, "行使修正"));
        }
    }

    #[test]
    fn test_render_spell_detail_with_variant_and_missing_fields() {
        let mut app = spell_app();
        crate::tui::update::update(&mut app, crate::tui::action::Action::Down);
        let rows = render(&app, 120, 24);
        assert!(contains(&rows, "ランク2 MagicCat_2 [特殊] 神名"), "{rows:#?}");
        assert!(contains(&rows, "MP - 時間 -"));
        assert!(!contains(&rows, "【効果】"));
    }

    #[test]
    fn test_render_spell_no_hits_shows_conditions() {
        let mut app = spell_app();
        app.spell_view.query = "該当なし".into();
        app.refresh_spell_hits();
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "2件中 0件"));
        assert!(contains(&rows, "以下の条件でマッチする"));
        assert!(contains(&rows, "name: \"該当なし\""));
    }

    #[test]
    fn test_render_spell_search_query() {
        let mut app = spell_app();
        app.focus = Focus::Search;
        app.spell_view.query = "テスト".into();
        app.refresh_spell_hits();
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "検索> テスト"));
        assert!(contains(&rows, "2件中 1件"));
    }

    #[test]
    fn test_render_status_line() {
        let app = App {
            status: Some(StatusMessage::error("テスト用のエラー")),
            ..App::default()
        };
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "エラー: テスト用のエラー"));
    }

    #[test]
    fn test_help_line_fits_80_columns() {
        let mut app = sample_app();
        for tab in Tab::ALL {
            app.tab = tab;
            for focus in [Focus::List, Focus::Detail, Focus::Search] {
                app.focus = focus;
                let width = Line::from(help_text(&app)).width();
                assert!(width <= 80, "{tab:?} {focus:?}: {width}");
            }
        }
        for kind in [
            ModalKind::Help,
            ModalKind::Json { name: String::new(), text: String::new() },
            ModalKind::Stats { title: String::new(), text: String::new() },
            ModalKind::Export(Default::default()),
        ] {
            app.modal = Some(crate::tui::app::Modal::new(kind));
            let width = Line::from(help_text(&app)).width();
            assert!(width <= 80, "{:?}: {width}", app.modal);
        }
        app.modal = None;
        app.tab = Tab::Monster;
        app.focus = Focus::Detail;
        let rows = render(&app, 80, 24);
        assert!(contains(&rows, "c:ココフォリア"));
        assert!(contains(&rows, "q:終了"));
    }

    #[test]
    fn test_render_copy_success_status() {
        let app = App {
            status: Some(StatusMessage::success(
                "\"テストモンスター\" をココフォリア形式でクリップボードにコピーしました",
            )),
            ..App::default()
        };
        let rows = render(&app, 120, 24);
        assert!(contains(
            &rows,
            "成功: \"テストモンスター\" をココフォリア形式でクリップボードにコピーしました"
        ));
    }

    #[test]
    fn test_render_too_small() {
        let app = App::default();
        let rows = render(&app, 80, MIN_HEIGHT - 1);
        assert!(contains(&rows, "端末が小さすぎます"));
        assert!(!contains(&rows, "1:モンスター"));
    }
}
