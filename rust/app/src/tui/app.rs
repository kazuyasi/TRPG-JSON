//! TUI の状態（モデル）

use trpg_json_core::{Monster, Spell};

/// 表示中のタブ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Monster,
    Spell,
}

impl Tab {
    /// すべてのタブ（表示順）
    pub const ALL: [Tab; 2] = [Tab::Monster, Tab::Spell];

    /// タブ行に表示するラベル
    pub fn label(self) -> &'static str {
        match self {
            Tab::Monster => "1:モンスター",
            Tab::Spell => "2:スペル",
        }
    }

    /// `Tab::ALL` 内での位置
    pub fn index(self) -> usize {
        match self {
            Tab::Monster => 0,
            Tab::Spell => 1,
        }
    }
}

/// ステータス行に出すメッセージの種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Success / Info はコピー操作（T049d 以降）で使用する
pub enum StatusKind {
    Success,
    Info,
    Error,
}

/// ステータス行のメッセージ（CLI の `process::exit` の代わりに使う）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMessage {
    pub kind: StatusKind,
    pub text: String,
}

impl StatusMessage {
    pub fn error(text: impl Into<String>) -> Self {
        Self { kind: StatusKind::Error, text: text.into() }
    }
}

/// TUI 全体の状態
#[derive(Debug, Default)]
pub struct App {
    pub tab: Tab,
    /// 起動時に一括ロードしたモンスター
    pub monsters: Vec<Monster>,
    /// 起動時に一括ロードしたスペル（読み込めなかった場合は空）
    pub spells: Vec<Spell>,
    pub status: Option<StatusMessage>,
    pub should_quit: bool,
}

impl App {
    pub fn new(monsters: Vec<Monster>, spells: Vec<Spell>, tab: Tab) -> Self {
        Self {
            tab,
            monsters,
            spells,
            status: None,
            should_quit: false,
        }
    }

    /// 現在のタブの総件数
    pub fn total_count(&self) -> usize {
        match self.tab {
            Tab::Monster => self.monsters.len(),
            Tab::Spell => self.spells.len(),
        }
    }
}
