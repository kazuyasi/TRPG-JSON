//! モーダル（ヘルプ・JSON ビュー）
//!
//! 本文の領域の上に重ねて描画する。ヘッダー・ステータス行・キーヘルプ行は隠さない
//! （JSON のコピー結果をステータス行で確認できるように）。

use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use super::super::app::{ExportChoice, ExportForm, Modal, ModalKind};
use super::super::help::SECTIONS;
use super::{heading, wrapped_height};

/// モーダルを描画する
pub fn draw(frame: &mut Frame, area: Rect, modal: &Modal) {
    let (title, hint, lines) = match &modal.kind {
        ModalKind::Help => (" ヘルプ ".to_string(), " Esc/q:閉じる ", help_lines()),
        ModalKind::Json { name, text } => (
            format!(" JSON: {} ", name),
            " y:コピー  Esc/q:閉じる ",
            text.lines().map(|l| Line::from(l.to_string())).collect(),
        ),
        ModalKind::Stats { title, text } => (
            format!(" {} ", title),
            " Esc/q:閉じる ",
            text.lines()
                .map(|l| match l.strip_prefix('【').and_then(|r| r.strip_suffix('】')) {
                    Some(h) => heading(h),
                    None => Line::from(format!(" {}", l)),
                })
                .collect(),
        ),
        ModalKind::Export(form) => {
            draw_export(frame, area, form);
            return;
        }
    };
    let rect = modal_rect(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(title, Style::default().add_modifier(Modifier::BOLD)))
        .title_bottom(Line::from(hint).right_aligned());
    let inner = block.inner(rect);

    // 折り返し後の行数からスクロール上限を求め、update に伝える
    let max_scroll = wrapped_height(&lines, inner.width).saturating_sub(inner.height as usize);
    let max_scroll = max_scroll.min(u16::MAX as usize) as u16;
    modal.max_scroll.set(max_scroll);
    modal.page.set(inner.height.max(1));

    frame.render_widget(Clear, rect);
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((modal.scroll.min(max_scroll), 0));
    frame.render_widget(para, rect);
}

/// エクスポートモーダルの項目名の幅（「シートID」の 8 桁）
const EXPORT_LABEL_WIDTH: usize = 8;

/// エクスポートモーダルの本文。入力欄（出力先 / シートID）の行は `EXPORT_PATH_ROW` 行目
pub fn export_lines(form: &ExportForm) -> Vec<Line<'static>> {
    let label = |text: &str| {
        let pad = EXPORT_LABEL_WIDTH.saturating_sub(Line::from(text).width());
        Span::styled(
            format!("{}{}  ", text, " ".repeat(pad)),
            Style::default().fg(Color::Yellow),
        )
    };

    let mut formats = vec![label("形式")];
    for (i, choice) in ExportChoice::ALL.iter().enumerate() {
        if i > 0 {
            formats.push(Span::raw("  "));
        }
        let style = if *choice == form.format {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        formats.push(Span::styled(format!(" {} ", choice.label()), style));
    }

    let dim = Style::default().fg(Color::DarkGray);
    let notes: [&str; 2] = if form.format.is_file() {
        ["相対パスは gm を起動したディレクトリが基準。~/ はホーム", "同じ名前のファイルがあれば上書きします"]
    } else {
        [
            "実行中は TUI を一時停止します（認証が必要ならブラウザが開きます）",
            "設定は CLI の --export sheets と同じ（README 参照）",
        ]
    };
    vec![
        Line::from(vec![
            label("対象"),
            Span::raw(format!("ヒット中のモンスター {} 件（結果リストの並び順）", form.count)),
        ]),
        Line::from(formats),
        Line::from(vec![label(form.format.input_label()), Span::raw(form.input().to_string())]),
        Line::from(""),
        Line::from(Span::styled(notes[0], dim)),
        Line::from(Span::styled(notes[1], dim)),
    ]
}

/// `export_lines` のうち出力先の行（カーソルを置く）
const EXPORT_PATH_ROW: u16 = 2;

/// エクスポートモーダル: 本文領域の中央に小さく出し、出力先の末尾にカーソルを置く
fn draw_export(frame: &mut Frame, area: Rect, form: &ExportForm) {
    let lines = export_lines(form);
    let width = area.width.min(72);
    let height = area.height.min(lines.len() as u16 + 2);
    let rect = Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(" エクスポート ", Style::default().add_modifier(Modifier::BOLD)))
        .title_bottom(Line::from(" Enter:出力  Esc:キャンセル ").right_aligned());
    let inner = block.inner(rect);

    let mut lines = lines;
    // 出力先が長いときは折り返さず、末尾（入力位置）が見えるよう先頭を `<` で省く
    let path_row = EXPORT_PATH_ROW as usize;
    let avail = (inner.width as usize).saturating_sub(1);
    if lines[path_row].width() > avail {
        let label = lines[path_row].spans[0].clone();
        let room = avail.saturating_sub(label.width() + 1);
        lines[path_row] = Line::from(vec![label, Span::styled("<", Style::default().fg(Color::DarkGray)), Span::raw(tail_fitting(form.input(), room))]);
    }
    let cursor_x = lines[path_row].width() as u16;

    frame.render_widget(Clear, rect);
    frame.render_widget(Paragraph::new(lines).block(block), rect);
    if inner.height > EXPORT_PATH_ROW {
        let x = inner.x + cursor_x.min(inner.width.saturating_sub(1));
        frame.set_cursor_position(Position::new(x, inner.y + EXPORT_PATH_ROW));
    }
}

/// 表示幅が `width` 以下に収まる `s` の末尾部分
fn tail_fitting(s: &str, width: usize) -> String {
    let mut used = 0;
    let mut start = s.len();
    for (i, c) in s.char_indices().rev() {
        let w = Line::from(c.to_string()).width();
        if used + w > width {
            break;
        }
        used += w;
        start = i;
    }
    s[start..].to_string()
}

/// 本文領域の中央に置く。狭い端末では本文領域いっぱいに広げる
fn modal_rect(area: Rect) -> Rect {
    let width = if area.width >= 100 { area.width * 4 / 5 } else { area.width };
    let height = if area.height >= 20 { area.height * 9 / 10 } else { area.height };
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}

/// ヘルプの本文。キー列は全節で桁をそろえる
pub fn help_lines() -> Vec<Line<'static>> {
    let key_width = SECTIONS
        .iter()
        .flat_map(|s| s.entries.iter())
        .map(|(key, _)| Line::from(*key).width())
        .max()
        .unwrap_or(0);

    let mut lines = Vec::new();
    for section in SECTIONS {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(heading(section.title));
        for (key, desc) in section.entries {
            let pad = key_width - Line::from(*key).width();
            lines.push(Line::from(vec![
                Span::raw(" "),
                Span::styled(*key, Style::default().fg(Color::Yellow)),
                Span::raw(" ".repeat(pad + 2)),
                Span::raw(*desc),
            ]));
        }
    }
    lines
}
