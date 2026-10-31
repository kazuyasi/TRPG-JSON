//! TUI の状態（モデル）

use std::cell::Cell;

use trpg_json_core::{Monster, Spell, query};

use super::format;

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
    pub fn success(text: impl Into<String>) -> Self {
        Self { kind: StatusKind::Success, text: text.into() }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self { kind: StatusKind::Info, text: text.into() }
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self { kind: StatusKind::Error, text: text.into() }
    }
}

/// キー入力の受け先
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    /// 検索バー（英字キーも文字入力として扱う）
    Search,
    /// 結果リスト
    #[default]
    List,
    /// 詳細ペイン（部位表の選択・本文スクロール）
    Detail,
}

impl Focus {
    /// Tab キーでの次のフォーカス
    pub fn next(self) -> Self {
        match self {
            Focus::Search => Focus::List,
            Focus::List => Focus::Detail,
            Focus::Detail => Focus::Search,
        }
    }

    /// Shift-Tab キーでの前のフォーカス
    pub fn prev(self) -> Self {
        match self {
            Focus::Search => Focus::Detail,
            Focus::List => Focus::Search,
            Focus::Detail => Focus::List,
        }
    }
}

/// 検索結果リストの状態（タブごとに1つ持つ）
///
/// ヒット結果は借用の寿命問題を避けるため、元データへの index で保持する。
#[derive(Debug, Default)]
pub struct ListView {
    /// 検索バーの入力文字列
    pub query: String,
    /// ヒットした項目の index（表示順）
    pub hits: Vec<usize>,
    /// `hits` 内での選択位置
    pub selected: usize,
    /// 先頭に表示している行（描画時に ratatui が調整した値を書き戻す）
    pub offset: Cell<usize>,
    /// 直近の描画での表示可能行数（PgUp/PgDn の移動量に使う）
    pub page_size: Cell<usize>,
}

impl ListView {
    /// 選択中の項目の元データ index
    pub fn selected_index(&self) -> Option<usize> {
        self.hits.get(self.selected).copied()
    }

    /// 選択位置を `delta` だけ動かす（範囲外は端で止める）
    pub fn move_by(&mut self, delta: isize) {
        if self.hits.is_empty() {
            self.selected = 0;
            return;
        }
        let last = self.hits.len() - 1;
        self.selected = self.selected.saturating_add_signed(delta).min(last);
    }

    /// 1ページ分の行数（未描画なら 10）
    pub fn page(&self) -> usize {
        match self.page_size.get() {
            0 => 10,
            n => n,
        }
    }
}

/// TUI 全体の状態
#[derive(Debug, Default)]
pub struct App {
    pub tab: Tab,
    pub focus: Focus,
    /// 起動時に一括ロードしたモンスター
    pub monsters: Vec<Monster>,
    /// 起動時に一括ロードしたスペル（読み込めなかった場合は空）
    pub spells: Vec<Spell>,
    pub monster_view: ListView,
    /// モンスター詳細の部位表で選択中の部位（`part[]` の 0 始まり index）
    pub selected_part: usize,
    /// 詳細ペイン下部（特殊能力など）のスクロール量
    pub detail_scroll: u16,
    /// 直近の描画でのスクロール量の上限（描画時に書き戻す）
    pub detail_max_scroll: Cell<u16>,
    /// 直近の描画での詳細ペイン下部の表示行数（PgUp/PgDn の移動量に使う）
    pub detail_page: Cell<u16>,
    pub status: Option<StatusMessage>,
    pub should_quit: bool,
}

impl App {
    pub fn new(monsters: Vec<Monster>, spells: Vec<Spell>, tab: Tab) -> Self {
        let mut app = Self {
            tab,
            monsters,
            spells,
            ..Self::default()
        };
        app.refresh_monster_hits();
        app
    }

    /// 現在のタブの総件数
    pub fn total_count(&self) -> usize {
        match self.tab {
            Tab::Monster => self.monsters.len(),
            Tab::Spell => self.spells.len(),
        }
    }

    /// 現在のタブのヒット件数（検索未実装のタブは総件数）
    pub fn hit_count(&self) -> usize {
        match self.tab {
            Tab::Monster => self.monster_view.hits.len(),
            Tab::Spell => self.spells.len(),
        }
    }

    /// 選択中のモンスター
    pub fn selected_monster(&self) -> Option<&Monster> {
        self.monster_view.selected_index().and_then(|i| self.monsters.get(i))
    }

    /// 検索語（前後の空白を除く。空なら `None` = 全件）
    pub fn monster_name_query(&self) -> Option<&str> {
        let q = self.monster_view.query.trim();
        (!q.is_empty()).then_some(q)
    }

    /// 検索語でヒット結果を作り直す（Lv 昇順 → 名前順）
    ///
    /// 直前に選んでいた項目がまだヒットしていれば選択を保つ。
    pub fn refresh_monster_hits(&mut self) {
        let previous = self.monster_view.selected_index();
        let name = self.monster_name_query();

        // CLI と同じ判定にするため core の find_multi を1件ずつ適用し、index で保持する
        let monsters = &self.monsters;
        let mut hits: Vec<usize> = (0..monsters.len())
            .filter(|&i| !query::find_multi(std::slice::from_ref(&monsters[i]), name, None, None).is_empty())
            .collect();
        hits.sort_by(|&a, &b| {
            monsters[a]
                .level
                .cmp(&monsters[b].level)
                .then_with(|| monsters[a].name.cmp(&monsters[b].name))
        });

        let view = &mut self.monster_view;
        view.hits = hits;
        view.selected = previous
            .and_then(|p| view.hits.iter().position(|&i| i == p))
            .unwrap_or(0);
        view.offset.set(0);

        if view.selected_index() != previous {
            self.reset_detail();
        }
    }

    /// 選択モンスターが変わったときに部位選択とスクロールを既定に戻す
    pub fn reset_detail(&mut self) {
        self.selected_part = self.selected_monster().map(format::default_part_index).unwrap_or(0);
        self.detail_scroll = 0;
    }
}
