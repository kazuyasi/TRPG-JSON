//! TUI モード（`gm tui`）
//!
//! 構成は Elm 風: キー入力 → `Action` → `update(&mut App)` → `ui::draw(&App)`。
//! 設計の詳細はリポジトリ直下の `TUI_DESIGN.md` を参照。

mod action;
mod app;
mod ui;
mod update;

use std::io::{self, IsTerminal};

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event};
use thiserror::Error;
use trpg_json_core::io as core_io;

pub use app::Tab;
use app::{App, StatusMessage};

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
    let (spells, status) = if spell_paths.is_empty() {
        (Vec::new(), None)
    } else {
        match core_io::load_multiple_spells_json_arrays(spell_paths) {
            Ok(s) => (s, None),
            Err(e) => (
                Vec::new(),
                Some(StatusMessage::error(format!("スペルデータを読み込めません: {}", e))),
            ),
        }
    };

    let mut app = App::new(monsters, spells, initial_tab);
    app.status = status;

    // raw mode + alternate screen に入る。panic 時に端末を戻す hook も ratatui::init が設定する
    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> Result<(), TuiError> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if let Event::Key(key) = event::read()?
            && let Some(action) = action::from_key(key)
        {
            update::update(app, action);
        }
    }
    Ok(())
}
