//! TUI の状態（モデル）

use std::cell::Cell;

use trpg_json_core::{Monster, Spell};

use super::format;
use super::input::LineInput;
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

impl StatusKind {
    /// 表示時に本文の前に付ける文言（CLI と同じ）
    pub fn prefix(self) -> &'static str {
        match self {
            StatusKind::Success => "成功: ",
            StatusKind::Info => "情報: ",
            StatusKind::Error => "エラー: ",
        }
    }
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
    /// ヒット中の項目と全件の統計（`S`）。開いた時点の内容を保持する
    Stats {
        /// タイトル（「モンスター統計」など）
        title: String,
        /// 整形済みの本文（`【】` で始まる行は節見出し）
        text: String,
    },
    /// ヒット中のモンスターをファイルへ書き出すフォーム（`e`）
    Export(ExportForm),
}

/// TUI のエクスポートモーダルで選べる形式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportChoice {
    #[default]
    Json,
    Udonarium,
    /// Google Sheets（OAuth でブラウザを開くため、実行中は TUI を一時停止する）
    Sheets,
}

impl ExportChoice {
    /// 表示順
    pub const ALL: [ExportChoice; 3] = [ExportChoice::Json, ExportChoice::Udonarium, ExportChoice::Sheets];

    /// モーダルに表示する名前
    pub fn label(self) -> &'static str {
        match self {
            ExportChoice::Json => "JSON",
            ExportChoice::Udonarium => "Udonarium (ZIP)",
            ExportChoice::Sheets => "Google Sheets",
        }
    }

    /// `export_monsters` に渡す形式名（CLI の `--export` と同じ）
    pub fn format_name(self) -> &'static str {
        match self {
            ExportChoice::Json => "json",
            ExportChoice::Udonarium => "udonarium",
            ExportChoice::Sheets => "sheets",
        }
    }

    /// ファイルへ書き出す形式か（Sheets はスプレッドシート ID を入力する）
    pub fn is_file(self) -> bool {
        !matches!(self, ExportChoice::Sheets)
    }

    /// 出力ファイルの拡張子（ファイル形式以外は空）
    pub fn extension(self) -> &'static str {
        match self {
            ExportChoice::Json => ".json",
            ExportChoice::Udonarium => ".zip",
            ExportChoice::Sheets => "",
        }
    }

    /// 入力欄の項目名
    pub fn input_label(self) -> &'static str {
        if self.is_file() { "出力先" } else { "シートID" }
    }

    /// 次の形式（最後の次は先頭）
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|&c| c == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
}

/// エクスポートモーダルの入力内容
///
/// ファイルの出力先とスプレッドシート ID は別々に持ち、形式を切り替えても失わない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportForm {
    pub format: ExportChoice,
    /// ファイルの出力先（JSON / Udonarium）
    pub path: LineInput,
    /// スプレッドシート ID（Sheets）
    pub sheet_id: LineInput,
    /// 書き出す件数（開いた時点のヒット件数）
    pub count: usize,
}

impl Default for ExportForm {
    /// 初めて開いたときの値（JSON / `monsters.json`）
    fn default() -> Self {
        Self {
            format: ExportChoice::Json,
            path: format!("monsters{}", ExportChoice::Json.extension()).into(),
            sheet_id: LineInput::default(),
            count: 0,
        }
    }
}

impl ExportForm {
    /// 選択中の形式の入力欄の内容
    pub fn input(&self) -> &LineInput {
        if self.format.is_file() { &self.path } else { &self.sheet_id }
    }

    /// 選択中の形式の入力欄（変更用）
    pub fn input_mut(&mut self) -> &mut LineInput {
        if self.format.is_file() { &mut self.path } else { &mut self.sheet_id }
    }

    /// 形式を切り替える。ファイル形式に移るとき、出力先が他のファイル形式の拡張子で終わっていれば付け替える
    pub fn cycle_format(&mut self) {
        let new = self.format.next();
        if new.is_file() {
            let other = ExportChoice::ALL
                .iter()
                .filter(|c| c.is_file() && **c != new)
                .find_map(|c| self.path.strip_suffix(c.extension()));
            if let Some(stem) = other {
                self.path = format!("{}{}", stem, new.extension()).into();
            }
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
    /// 検索バーの入力（文字列とカーソル位置）
    pub query: LineInput,
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
    /// 選択リストの項目の元データ index（追加順）。検索条件を変えても保持する
    pub picks: Vec<usize>,
}

impl ListView {
    /// 元データ index の項目が選択リストにあるか
    pub fn is_picked(&self, index: usize) -> bool {
        self.picks.contains(&index)
    }

    /// 選択リストへ追加する。すでにあれば外す。追加したら `true`
    fn toggle_pick(&mut self, index: usize) -> bool {
        match self.picks.iter().position(|&i| i == index) {
            Some(pos) => {
                self.picks.remove(pos);
                false
            }
            None => {
                self.picks.push(index);
                true
            }
        }
    }

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

    /// データを読み直したあとのヒット結果に差し替える（元データの index は読み直しで変わる）
    ///
    /// `keep` が直前の選択と同じ項目のヒット内位置を返せばそこを選ぶ。見つからなければ
    /// 同じ行（末尾を超えれば末尾）を選ぶ。同じ項目を選べたら `true` を返す。
    fn replace_hits_after_reload(&mut self, hits: Vec<usize>, keep: Option<usize>) -> bool {
        let previous_row = self.selected;
        self.hits = hits;
        let kept = keep.is_some();
        self.selected = keep.unwrap_or_else(|| previous_row.min(self.hits.len().saturating_sub(1)));
        // 表示位置はなるべく動かさない（選択が見えるよう描画時に ratatui が調整する）
        self.offset.set(self.offset.get().min(self.selected));
        kept
    }
}

/// データファイルの設定（`R` での再読み込みに使う）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DataSources {
    pub monster_paths: Vec<String>,
    pub spell_paths: Vec<String>,
}

/// データファイルを読み直した結果（失敗は CLI の「エラー: 」以降の文言）
#[derive(Debug)]
pub struct ReloadedData {
    pub monsters: Result<Vec<Monster>, String>,
    pub spells: Result<Vec<Spell>, String>,
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
    /// 直前に実行したエクスポートの入力内容（次に開いたときの初期値）
    pub last_export: Option<ExportForm>,
    /// 読み込んだデータファイル（`R` で読み直す）
    pub sources: DataSources,
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

    /// カーソル行の項目を選択リストへ追加する／外す（結果はステータス行に出す）
    pub fn toggle_pick(&mut self) {
        let Some(index) = self.view().selected_index() else {
            self.status = Some(StatusMessage::info("選択リストに入れる項目がありません"));
            return;
        };
        let name = match self.tab {
            Tab::Monster => self.monsters[index].name.clone(),
            Tab::Spell => self.spells[index].name.clone(),
        };
        let added = self.view_mut().toggle_pick(index);
        let verb = if added { "に追加しました" } else { "から外しました" };
        let count = self.view().picks.len();
        self.status = Some(StatusMessage::info(format!(
            "\"{}\" を選択リスト{}（{} 件）",
            name, verb, count
        )));
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

        let hits = monster_hits(&self.monsters, &filter);
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

        let hits = spell_hits(&self.spells, &filter);
        self.spell_filter = filter;
        if self.spell_view.replace_hits(hits) {
            self.spell_view.detail_scroll = 0;
        }
    }

    /// 読み直したデータに差し替え、検索語・タブ・選択中の項目を保ったまま結果を作り直す
    ///
    /// 読み込みに失敗した種類は今のデータを残す。結果をステータス行のメッセージで返す。
    pub fn apply_reload(&mut self, data: ReloadedData) -> StatusMessage {
        let mut errors = Vec::new();
        let mut dropped = 0;
        match data.monsters {
            Ok(monsters) => dropped += self.replace_monsters(monsters),
            Err(e) => errors.push(format!("モンスターデータを読み込めません（前のデータのままです）: {}", e)),
        }
        match data.spells {
            Ok(spells) => dropped += self.replace_spells(spells),
            Err(e) => errors.push(format!("スペルデータを読み込めません（前のデータのままです）: {}", e)),
        }
        let note = match dropped {
            0 => String::new(),
            n => format!("。データから消えた {} 件を選択リストから外しました", n),
        };
        if errors.is_empty() {
            StatusMessage::success(format!(
                "データを再読み込みしました（モンスター {} 件・スペル {} 件）{}",
                self.monsters.len(),
                self.spells.len(),
                note
            ))
        } else {
            StatusMessage::error(format!("{}{}", errors.join(" / "), note))
        }
    }

    /// モンスターを差し替える。選択中の項目は名前で探し、部位の選択とスクロールも保つ
    ///
    /// 選択リストも名前で引き直し、データから消えて外した件数を返す。
    fn replace_monsters(&mut self, monsters: Vec<Monster>) -> usize {
        let previous = self.selected_monster().map(|m| m.name.clone());
        let picked: Vec<String> = self.monster_view.picks.iter().map(|&i| self.monsters[i].name.clone()).collect();
        self.monsters = monsters;
        self.monster_view.picks = remap_picks(&picked, self.monsters.iter().map(|m| m.name.as_str()));
        let filter = self.reload_monster_filter();
        let hits = monster_hits(&self.monsters, &filter);
        let keep = position_by_name(&hits, previous.as_deref(), |i| &self.monsters[i].name);
        self.monster_filter = filter;
        let kept = self.monster_view.replace_hits_after_reload(hits, keep);
        let part_count = self.selected_monster().map_or(0, |m| m.part.len());
        if !kept || self.selected_part >= part_count {
            self.reset_monster_detail();
        }
        picked.len() - self.monster_view.picks.len()
    }

    /// スペルを差し替える。選択中の項目は名前で探し、本文のスクロールも保つ
    ///
    /// 選択リストも名前で引き直し、データから消えて外した件数を返す。
    fn replace_spells(&mut self, spells: Vec<Spell>) -> usize {
        let previous = self.selected_spell().map(|s| s.name.clone());
        let picked: Vec<String> = self.spell_view.picks.iter().map(|&i| self.spells[i].name.clone()).collect();
        self.spells = spells;
        self.spell_view.picks = remap_picks(&picked, self.spells.iter().map(|s| s.name.as_str()));
        let filter = self.reload_spell_filter();
        let hits = spell_hits(&self.spells, &filter);
        let keep = position_by_name(&hits, previous.as_deref(), |i| &self.spells[i].name);
        self.spell_filter = filter;
        if !self.spell_view.replace_hits_after_reload(hits, keep) {
            self.spell_view.detail_scroll = 0;
        }
        picked.len() - self.spell_view.picks.len()
    }

    /// 再読み込みで使うモンスターの検索条件（検索語を解釈できなければ直前に解釈できた条件）
    fn reload_monster_filter(&mut self) -> MonsterQuery {
        match parse_monster_query(&self.monster_view.query) {
            Ok(q) => {
                self.monster_view.query_error = None;
                q
            }
            Err(_) => self.monster_filter.clone(),
        }
    }

    /// 再読み込みで使うスペルの検索条件（検索語を解釈できなければ直前に解釈できた条件）
    fn reload_spell_filter(&mut self) -> SpellQuery {
        match parse_spell_query(&self.spell_view.query) {
            Ok(q) => {
                self.spell_view.query_error = None;
                q
            }
            Err(_) => self.spell_filter.clone(),
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

/// 条件に合うモンスターの index（Lv 昇順 → 名前順）
fn monster_hits(monsters: &[Monster], filter: &MonsterQuery) -> Vec<usize> {
    let mut hits: Vec<usize> = (0..monsters.len()).filter(|&i| filter.matches(&monsters[i])).collect();
    hits.sort_by(|&a, &b| {
        monsters[a]
            .level
            .cmp(&monsters[b].level)
            .then_with(|| monsters[a].name.cmp(&monsters[b].name))
    });
    hits
}

/// 条件に合うスペルの index（Lv 昇順 → ランク昇順 → 名前順）
fn spell_hits(spells: &[Spell], filter: &SpellQuery) -> Vec<usize> {
    let mut hits: Vec<usize> = (0..spells.len()).filter(|&i| filter.matches(&spells[i])).collect();
    hits.sort_by(|&a, &b| {
        format::spell_level(&spells[a])
            .sort_key()
            .cmp(&format::spell_level(&spells[b]).sort_key())
            .then_with(|| spells[a].name.cmp(&spells[b].name))
    });
    hits
}

/// 読み直したデータの中で選択リストの名前を探し、新しい index の列を返す（順序は保つ）
///
/// 同名の項目を複数選んでいたら、まだ使っていない同名の項目へ順に割り当てる。見つからない名前は外す。
fn remap_picks<'a>(picked: &[String], names: impl Iterator<Item = &'a str>) -> Vec<usize> {
    let names: Vec<&str> = names.collect();
    let mut picks = Vec::new();
    for name in picked {
        if let Some(i) = (0..names.len()).find(|i| names[*i] == name && !picks.contains(i)) {
            picks.push(i);
        }
    }
    picks
}

/// ヒット中で名前が `name` の最初の項目の位置
fn position_by_name<'a>(hits: &[usize], name: Option<&str>, name_of: impl Fn(usize) -> &'a String) -> Option<usize> {
    let name = name?;
    hits.iter().position(|&i| name_of(i) == name)
}
