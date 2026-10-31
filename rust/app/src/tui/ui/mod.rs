//! 描画（ratatui）

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};

use super::app::{App, StatusKind, Tab};

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
    draw_help(frame, help);
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
        Constraint::Length(16),
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
        Paragraph::new(format!("{}件", app.total_count())).alignment(Alignment::Right),
        count,
    );
}

fn draw_body(frame: &mut Frame, area: Rect, app: &App) {
    let (title, lines) = match app.tab {
        Tab::Monster => (
            " モンスター ",
            vec![
                Line::from(format!("モンスターデータ {} 件を読み込みました。", app.monsters.len())),
                Line::from(""),
                Line::from("検索・一覧・詳細表示は今後のバージョンで追加されます。"),
            ],
        ),
        Tab::Spell if app.spells.is_empty() => (
            " スペル ",
            vec![Line::from(Span::styled(
                "スペルデータなし",
                Style::default().fg(Color::DarkGray),
            ))],
        ),
        Tab::Spell => (
            " スペル ",
            vec![
                Line::from(format!("スペルデータ {} 件を読み込みました。", app.spells.len())),
                Line::from(""),
                Line::from("検索・一覧・詳細表示は今後のバージョンで追加されます。"),
            ],
        ),
    };

    let block = Block::default().borders(Borders::ALL).title(title);
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

fn draw_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new("1/2:タブ切替  q:終了").style(Style::default().fg(Color::DarkGray));
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

    #[test]
    fn test_render_monster_tab() {
        let app = App::default();
        for width in [80, 120] {
            let rows = render(&app, width, 24);
            assert!(contains(&rows, "1:モンスター"));
            assert!(contains(&rows, "2:スペル"));
            assert!(contains(&rows, "モンスターデータ 0 件を読み込みました。"));
            assert!(contains(&rows, "q:終了"));
        }
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
