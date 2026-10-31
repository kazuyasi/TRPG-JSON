//! 描画（ratatui）

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};

use super::app::{App, Focus, StatusKind, Tab};

mod detail;
mod list;
mod search;

/// 描画を行う最小の端末の高さ（これ未満は警告のみ表示）
pub const MIN_HEIGHT: u16 = 15;

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

/// ヘッダー右端の件数表示（検索のあるタブは「総件数中 ヒット件数」）
fn count_label(app: &App) -> String {
    match app.tab {
        Tab::Monster => format!("{}件中 {}件", app.total_count(), app.hit_count()),
        Tab::Spell => format!("{}件", app.total_count()),
    }
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

fn draw_body(frame: &mut Frame, area: Rect, app: &App) {
    match app.tab {
        Tab::Monster => draw_monster_tab(frame, area, app),
        Tab::Spell => draw_spell_tab(frame, area, app),
    }
}

fn draw_monster_tab(frame: &mut Frame, area: Rect, app: &App) {
    let [search_area, panes] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
    search::draw(frame, search_area, app);

    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(panes);
    list::draw(frame, list_area, app);
    detail::draw(frame, detail_area, app);
}

fn draw_spell_tab(frame: &mut Frame, area: Rect, app: &App) {
    let lines = if app.spells.is_empty() {
        vec![Line::from(Span::styled(
            "スペルデータなし",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        vec![
            Line::from(format!("スペルデータ {} 件を読み込みました。", app.spells.len())),
            Line::from(""),
            Line::from("検索・一覧・詳細表示は今後のバージョンで追加されます。"),
        ]
    };
    let block = Block::default().borders(Borders::ALL).title(" スペル ");
    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let Some(status) = &app.status else {
        return;
    };
    let (prefix, color) = match status.kind {
        StatusKind::Success => ("成功: ", Color::Green),
        StatusKind::Info => ("情報: ", Color::Cyan),
        StatusKind::Error => ("エラー: ", Color::Red),
    };
    let line = Line::from(vec![
        Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::raw(status.text.as_str()),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

/// 現在のフォーカスで使える主要キー
fn help_text(app: &App) -> &'static str {
    match (app.tab, app.focus) {
        (Tab::Spell, _) => "1/2:タブ切替  q:終了",
        (Tab::Monster, Focus::Search) => {
            "文字入力:名前で絞り込み  Enter/Esc:リストへ  Ctrl-u:クリア  ↑↓:選択  Ctrl-c:終了"
        }
        (Tab::Monster, Focus::List) => {
            "↑↓/jk:選択  PgUp/PgDn/g/G:移動  /:検索  Tab:ペイン移動  1/2:タブ  q:終了"
        }
        (Tab::Monster, Focus::Detail) => {
            "↑↓/jk:部位選択  PgUp/PgDn/g/G:スクロール  Tab:ペイン移動  Esc:リストへ  q:終了"
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
        let other = monster("サンプル動物", 2, "動物", vec![part("", true)]);
        App::new(vec![m, other], Vec::new(), Tab::Monster)
    }

    #[test]
    fn test_render_monster_tab() {
        let mut app = sample_app();
        // Lv 昇順なので先頭はサンプル動物。テストモンスターを選ぶ
        crate::tui::update::update(&mut app, crate::tui::action::Action::Down);
        for width in [80, 120] {
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "1:モンスター"));
            assert!(contains(&rows, "2:スペル"));
            assert!(contains(&rows, "2件中 2件"));
            assert!(contains(&rows, "検索>"));
            // 結果リスト
            assert!(contains(&rows, "サンプル動物"));
            assert!(contains(&rows, "> 6"), "{width}: {rows:#?}");
            // 詳細ペイン
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
            assert!(contains(&rows, "q:終了"));
        }
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
        assert!(contains(&rows, "スペルデータなし"));
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
    fn test_render_too_small() {
        let app = App::default();
        let rows = render(&app, 80, MIN_HEIGHT - 1);
        assert!(contains(&rows, "端末が小さすぎます"));
        assert!(!contains(&rows, "1:モンスター"));
    }
}
