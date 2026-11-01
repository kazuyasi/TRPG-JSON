//! TUI の状態（モデル）

use std::cell::Cell;

use trpg_json_core::{Monster, Spell};

use super::format;
use super::query::{MonsterQuery, SpellQuery, parse_monster_query, parse_spell_query};

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

/// モーダルの種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalKind {
    /// 全キーバインドと検索構文の一覧（`?`）
    Help,
    /// 選択中項目の生 JSON（`J`）。開いた時点の内容を保持する
    Json {
        /// 項目名（タイトルとコピー時の文言に使う）
        name: String,
        /// 整形済みの JSON
        text: String,
    },
    /// ヒット中のモンスターをファイルへ書き出すフォーム（`e`）
    Export(ExportForm),
}

/// TUI のエクスポートモーダルで選べる形式
///
/// Google Sheets は OAuth でブラウザを開くため別扱い（T050b）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportChoice {
    #[default]
    Json,
    Udonarium,
}

impl ExportChoice {
    /// 表示順
    pub const ALL: [ExportChoice; 2] = [ExportChoice::Json, ExportChoice::Udonarium];

    /// モーダルに表示する名前
    pub fn label(self) -> &'static str {
        match self {
            ExportChoice::Json => "JSON",
            ExportChoice::Udonarium => "Udonarium (ZIP)",
        }
    }

    /// `export_monsters` に渡す形式名（CLI の `--export` と同じ）
    pub fn format_name(self) -> &'static str {
        match self {
            ExportChoice::Json => "json",
            ExportChoice::Udonarium => "udonarium",
        }
    }

    /// 出力ファイルの拡張子
    pub fn extension(self) -> &'static str {
        match self {
            ExportChoice::Json => ".json",
            ExportChoice::Udonarium => ".zip",
        }
    }

    /// 初めて開いたときの出力先
    pub fn default_path(self) -> String {
        format!("monsters{}", self.extension())
    }

    /// 次の形式（最後の次は先頭）
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|&c| c == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
}

/// エクスポートモーダルの入力内容
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportForm {
    pub format: ExportChoice,
    /// 出力先のパス（入力中の文字列）
    pub path: String,
    /// 書き出す件数（開いた時点のヒット件数）
    pub count: usize,
}

impl ExportForm {
    pub fn new(format: ExportChoice, path: String, count: usize) -> Self {
        Self { format, path, count }
    }

    /// 形式を切り替える。出力先が前の形式の拡張子で終わっていれば拡張子も付け替える
    pub fn cycle_format(&mut self) {
        let old = self.format;
        let new = old.next();
        if let Some(stem) = self.path.strip_suffix(old.extension()) {
            self.path = format!("{}{}", stem, new.extension());
        }
        self.format = new;
    }
}

/// 本文の上に重ねて表示するモーダル（表示中はキー入力をすべて受け取る）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modal {
    pub kind: ModalKind,
    /// 本文のスクロール量
    pub scroll: u16,
    /// 直近の描画でのスクロール量の上限（描画時に書き戻す）
    pub max_scroll: Cell<u16>,
    /// 直近の描画での表示行数（PgUp/PgDn の移動量に使う）
    pub page: Cell<u16>,
}

impl Modal {
    pub fn new(kind: ModalKind) -> Self {
        Self { kind, scroll: 0, max_scroll: Cell::new(0), page: Cell::new(0) }
    }

    /// 文字入力を受け付けるフォームか（キーの解釈を切り替える）
    pub fn is_form(&self) -> bool {
        matches!(self.kind, ModalKind::Export(_))
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
    /// 詳細ペイン下部（特殊能力・効果など）のスクロール量
    pub detail_scroll: u16,
    /// 検索構文を解釈できなかったときの理由（検索バーを警告色にし、直前の結果を残す）
    pub query_error: Option<String>,
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

    /// ヒット結果を差し替える。直前に選んでいた項目がまだヒットしていれば選択を保つ
    ///
    /// 選択中の項目が変わったら `true` を返す（詳細ペインの状態を戻す判断に使う）。
    fn replace_hits(&mut self, hits: Vec<usize>) -> bool {
        let previous = self.selected_index();
        self.hits = hits;
        self.selected = previous
            .and_then(|p| self.hits.iter().position(|&i| i == p))
            .unwrap_or(0);
        self.offset.set(0);
        self.selected_index() != previous
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
    pub spell_view: ListView,
    /// 直近に解釈できたモンスターの検索条件（ヒット 0 件の条件一覧に使う）
    pub monster_filter: MonsterQuery,
    /// 直近に解釈できたスペルの検索条件（ヒット 0 件の条件一覧に使う）
    pub spell_filter: SpellQuery,
    /// モンスター詳細の部位表で選択中の部位（`part[]` の 0 始まり index）
    pub selected_part: usize,
    /// 直近の描画でのスクロール量の上限（描画時に書き戻す）
    pub detail_max_scroll: Cell<u16>,
    /// 直近の描画での詳細ペイン下部の表示行数（PgUp/PgDn の移動量に使う）
    pub detail_page: Cell<u16>,
    /// 表示中のモーダル（なければ `None`）
    pub modal: Option<Modal>,
    /// 直前にエクスポートした形式と出力先（次に開いたときの初期値）
    pub last_export: Option<(ExportChoice, String)>,
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
        app.refresh_spell_hits();
        app
    }

    /// 現在のタブの総件数
    pub fn total_count(&self) -> usize {
        match self.tab {
            Tab::Monster => self.monsters.len(),
            Tab::Spell => self.spells.len(),
        }
    }

    /// 現在のタブのヒット件数
    pub fn hit_count(&self) -> usize {
        self.view().hits.len()
    }

    /// 現在のタブの結果リストの状態
    pub fn view(&self) -> &ListView {
        match self.tab {
            Tab::Monster => &self.monster_view,
            Tab::Spell => &self.spell_view,
        }
    }

    /// 現在のタブの結果リストの状態（変更用）
    pub fn view_mut(&mut self) -> &mut ListView {
        match self.tab {
            Tab::Monster => &mut self.monster_view,
            Tab::Spell => &mut self.spell_view,
        }
    }

    /// 選択中のモンスター
    pub fn selected_monster(&self) -> Option<&Monster> {
        self.monster_view.selected_index().and_then(|i| self.monsters.get(i))
    }

    /// 選択中のスペル
    pub fn selected_spell(&self) -> Option<&Spell> {
        self.spell_view.selected_index().and_then(|i| self.spells.get(i))
    }

    /// 現在のタブのヒット結果を検索バーの条件で作り直す
    pub fn refresh_hits(&mut self) {
        match self.tab {
            Tab::Monster => self.refresh_monster_hits(),
            Tab::Spell => self.refresh_spell_hits(),
        }
    }

    /// モンスターのヒット結果を作り直す（Lv 昇順 → 名前順）
    ///
    /// 検索構文を解釈できなければ `query_error` を設定し、直前の結果をそのまま残す。
    pub fn refresh_monster_hits(&mut self) {
        let filter = match parse_monster_query(&self.monster_view.query) {
            Ok(q) => q,
            Err(e) => {
                self.monster_view.query_error = Some(e.to_string());
                return;
            }
        };
        self.monster_view.query_error = None;

        let monsters = &self.monsters;
        let mut hits: Vec<usize> = (0..monsters.len()).filter(|&i| filter.matches(&monsters[i])).collect();
        hits.sort_by(|&a, &b| {
            monsters[a]
                .level
                .cmp(&monsters[b].level)
                .then_with(|| monsters[a].name.cmp(&monsters[b].name))
        });

        self.monster_filter = filter;
        if self.monster_view.replace_hits(hits) {
            self.reset_monster_detail();
        }
    }

    /// スペルのヒット結果を作り直す（Lv 昇順 → ランク昇順 → 名前順）
    ///
    /// 検索構文を解釈できなければ `query_error` を設定し、直前の結果をそのまま残す。
    pub fn refresh_spell_hits(&mut self) {
        let filter = match parse_spell_query(&self.spell_view.query) {
            Ok(q) => q,
            Err(e) => {
                self.spell_view.query_error = Some(e.to_string());
                return;
            }
        };
        self.spell_view.query_error = None;

        let spells = &self.spells;
        let mut hits: Vec<usize> = (0..spells.len()).filter(|&i| filter.matches(&spells[i])).collect();
        hits.sort_by(|&a, &b| {
            format::spell_level(&spells[a])
                .sort_key()
                .cmp(&format::spell_level(&spells[b]).sort_key())
                .then_with(|| spells[a].name.cmp(&spells[b].name))
        });

        self.spell_filter = filter;
        if self.spell_view.replace_hits(hits) {
            self.spell_view.detail_scroll = 0;
        }
    }

    /// 現在のタブで選択項目が変わったときに詳細ペインの状態を既定に戻す
    pub fn reset_detail(&mut self) {
        match self.tab {
            Tab::Monster => self.reset_monster_detail(),
            Tab::Spell => self.spell_view.detail_scroll = 0,
        }
    }

    /// モンスターの部位選択とスクロールを既定に戻す
    fn reset_monster_detail(&mut self) {
        self.selected_part = self.selected_monster().map(format::default_part_index).unwrap_or(0);
        self.monster_view.detail_scroll = 0;
    }
}
