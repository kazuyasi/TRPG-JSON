//! TUI モード（`gm tui`）
//!
//! 構成は Elm 風: キー入力 → `Action` → `update(&mut App)` → `ui::draw(&App)`。
//! 設計の詳細はリポジトリ直下の `TUI_DESIGN.md` を参照。

mod action;
mod app;
mod effect;
mod format;
mod help;
mod input;
mod query;
mod ui;
mod update;

use std::io::{self, IsTerminal};

use ratatui::DefaultTerminal;
use ratatui::backend::{Backend, ClearType};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::crossterm::event::{self, Event};
use thiserror::Error;
use trpg_json_core::io as core_io;

pub use app::Tab;
use app::{App, DataSources, StatusMessage};

/// TUI 起動・実行時のエラー（`Display` は CLI の「エラー: 」以降の文言）
#[derive(Debug, Error)]
pub enum TuiError {
    #[error("TUI は端末（TTY）上でのみ起動できます（パイプやリダイレクトには対応していません）")]
    NotATerminal,

    #[error("{0}")]
    Load(#[from] core_io::IoError),

    #[error("端末の操作に失敗しました: {0}")]
    Terminal(#[from] io::Error),
}

/// TUI を起動し、終了するまでブロックする
///
/// データ読み込みは端末を切り替える前に行い、失敗したら `Err` を返す
/// （呼び出し側が stderr に出して終了する）。スペルの読み込み失敗は致命的にせず、
/// スペルタブを「データなし」にしてステータス行に表示する。
pub fn run(monster_paths: &[String], spell_paths: &[String], initial_tab: Tab) -> Result<(), TuiError> {
    if !io::stdout().is_terminal() || !io::stdin().is_terminal() {
        return Err(TuiError::NotATerminal);
    }

    let monsters = core_io::load_multiple_json_arrays(monster_paths)?;
    let (spells, status) = match effect::load_spells(spell_paths) {
        Ok(s) => (s, None),
        Err(e) => (Vec::new(), Some(StatusMessage::error(format!("スペルデータを読み込めません: {}", e)))),
    };

    let mut app = App::new(monsters, spells, initial_tab);
    app.status = status;
    app.sources = DataSources {
        monster_paths: monster_paths.to_vec(),
        spell_paths: spell_paths.to_vec(),
    };

    // raw mode + alternate screen に入る。panic 時に端末を戻す hook も ratatui::init が設定する
    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> Result<(), TuiError> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        // 入力フォームのモーダルでは英字も文字入力として扱う
        let action = match &app.modal {
            Some(m) if m.is_form() => action::from_form_key(key),
            m => action::from_key(key, app.focus, m.is_some()),
        };
        if let Some(action) = action
            && let Some(effect) = update::update(app, action)
        {
            let redraw = effect.needs_full_redraw();
            let status = if effect.needs_suspend() {
                run_suspended(terminal, effect, app)?
            } else {
                effect::execute(effect, app)
            };
            app.status = Some(status);
            if redraw {
                force_full_redraw(terminal)?;
            }
        }
    }
    Ok(())
}

/// TUI を一時停止（raw mode・alternate screen を抜ける）して副作用を実行し、Enter で TUI に戻る
///
/// Google Sheets の OAuth はブラウザを開き、案内や進捗を stderr に出して完了までブロックするため、
/// 通常の画面で実行して利用者が読めるようにする。結果も同じ画面に出し、Enter を待ってから戻る。
fn run_suspended(terminal: &mut DefaultTerminal, effect: effect::Effect, app: &mut App) -> io::Result<StatusMessage> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    eprintln!("Google Sheets へエクスポートします（TUI は一時停止中です）");
    let status = effect::execute(effect, app);
    eprintln!();
    eprintln!("{}{}", status.kind.prefix(), status.text);
    eprint!("Enter キーで TUI に戻ります");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    Ok(status)
}

/// 画面を消し、次の描画で全セルを描き直させる（エクスポート処理が stderr に書いた文字を消す）
///
/// `Terminal::clear` は端末へカーソル位置を問い合わせ、応答しない端末ではエラーになるため使わない。
/// 描画直後の現在バッファは空なので、`swap_buffers` で直前の内容も捨てると次の描画が全面の差分になる。
fn force_full_redraw(terminal: &mut DefaultTerminal) -> io::Result<()> {
    terminal.backend_mut().clear_region(ClearType::All)?;
    terminal.swap_buffers();
    Ok(())
}
