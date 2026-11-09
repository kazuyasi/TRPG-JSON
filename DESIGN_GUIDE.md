# TRPG-JSON データ拡張設計ガイド

## 概要

このガイドでは、現在の実装を超えて、追加のデータソースやゲームシステムに対応できるよう TRPG-JSON システムを拡張する方法を説明します。

## 現在のアーキテクチャ

### 単一データソースのモデル
```
CLI (app/) → Core Library (core/) → JSON File (data/systems/monsters.json)
                     ↓
               Query Module
               (find_by_name, find_by_level, etc.)
                     ↓
               I/O Module
               (load_json_array, save_json_array_file)
```

**現在の実装:**
- **データパス**: TOML ファイルで設定: `data/systems/monsters.json`
- **データ構造**: `Vec<Monster>`
- **ファイル形式**: Monster オブジェクトの JSON 配列

## 拡張のシナリオ

### シナリオ 1: 複数のゲームシステム

**目標**: データ構造の異なる複数のゲームシステムに対応する

**設計方針:**

1. `core/` に**システムごとのモジュールを作成**:
   ```rust
   // core/src/systems/
   pub mod system_a;   // ゲームシステム A
   pub mod system_b;   // ゲームシステム B
   pub mod common;     // 共通のトレイト
   ```

2. **共通トレイトを定義**:
   ```rust
   // core/src/systems/common.rs
   pub trait GameEntity: Serialize + Deserialize {
       fn get_name(&self) -> &str;
       fn get_level(&self) -> i32;
       fn get_category(&self) -> &str;
   }
   ```

3. **システムごとに実装**:
   ```rust
   // 既存の Monster 構造体に GameEntity を実装
   impl GameEntity for Monster { ... }
   
   // 新しい D&D の Character 構造体
   pub struct Character { ... }
   impl GameEntity for Character { ... }
   ```

4. **検索モジュールを汎用化**:
   ```rust
   // core/src/query.rs - ジェネリックな関数を追加
   pub fn find_by_name<T: GameEntity>(
       entities: &[T], 
       name: &str
   ) -> Vec<&T> { ... }
   ```

### シナリオ 2: システムごとの複数データファイル

**目標**: 大きなデータセットの分割（例: カテゴリごとのモンスター）に対応する

**設計方針:**

1. 複数のファイルを扱えるよう **I/O モジュールを拡張**:
   ```rust
   // core/src/io.rs
   pub fn load_multiple_json_arrays<P: AsRef<Path>>(
       paths: &[P]
   ) -> Result<Vec<Monster>, IoError> {
       let mut all_monsters = Vec::new();
       for path in paths {
           let monsters = load_json_array(path)?;
           all_monsters.extend(monsters);
       }
       Ok(all_monsters)
   }
   ```

2. **システムの設定を作成**:
    ```rust
    // config/system_a.toml
    [data_files]
    monsters = ["data/systems/monsters.json"]
    items = ["data/systems/items.json"]
    ```

3. 複数のソースを読み込むよう **CLI を更新**:
    ```rust
    // app/src/main.rs
    let config = Config::load("config/default.toml")?;
    let monsters = load_multiple_json_arrays(&config.data_files)?;
    ```

### シナリオ 3: データソースの混在（JSON、データベース、API）

**目標**: 複数のストレージバックエンドからの読み込みに対応する

**設計方針:**

1. **データソースのトレイトを定義**:
   ```rust
   // core/src/io.rs
   pub trait DataSource {
       fn load(&self) -> Result<Vec<Monster>, IoError>;
       fn save(&self, monsters: &[Monster]) -> Result<(), IoError>;
   }
   ```

2. **複数のバックエンドを実装**:
   ```rust
   pub struct JsonSource { path: String }
   pub struct SqliteSource { db_path: String }
   pub struct ApiSource { url: String }
   
   impl DataSource for JsonSource { ... }
   impl DataSource for SqliteSource { ... }
   impl DataSource for ApiSource { ... }
   ```

3. **CLI で使う**:
   ```rust
   let source: Box<dyn DataSource> = match config.backend {
       "json" => Box::new(JsonSource::new(path)),
       "sqlite" => Box::new(SqliteSource::new(path)),
       "api" => Box::new(ApiSource::new(url)),
   };
   
   let monsters = source.load()?;
   ```

## 実装ロードマップ

### Phase 1: 現状 ✅
- 単一のゲームシステム
- 単一のデータファイル（monsters.json）
- 設定によるデータパスの指定
- 基本的な CRUD 操作（find、list、select、add、delete）

### Phase 2: ファイルの柔軟性 ✅（完了）
- 設定ファイル（TOML）への対応
- システムごとの複数データファイル
- ホームディレクトリからの相対パス
- **工数**: 低（2〜3日）✅
- **効果**: 高い柔軟性、容易な移行 ✅

### Phase 2.5: 外部へのデータエクスポート（次の推奨）
- JSON と Google Sheets へのデータのエクスポートに対応
- クラウドサービス向けの OAuth 2.0 認証（ユーザーにとって使いやすい）
- エクスポート時に書式を保持
- 手動で実行（自動化は不要）
- **工数**: 中（3〜4日）
- **効果**: データの共有と共同編集が可能になる
- **対象**: 小規模なチーム、データ管理者が1人の場合

### Phase 2.5 の実装詳細: 外部へのデータエクスポート

**エクスポートのインターフェース:**
```rust
// core/src/export/mod.rs
pub trait DataExporter {
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), String>;
}

pub struct ExportConfig {
    pub destination: String,  // JSON ではファイルパス、Google Sheets ではスプレッドシート ID
    pub format: ExportFormat,
}

pub enum ExportFormat {
    Json,
    GoogleSheets,
    // 今後: 独自の形式
}
```

**CLI の使い方:**
```bash
# 検索結果を JSON ファイルへエクスポート
gm select -l 6 --export json --output results.json

# Google Sheets へエクスポート（OAuth 2.0 の設定が必要）
gm select -c "Category" --export sheets --output "Spreadsheet ID"
```

**JSON エクスポートの手順:**
1. ユーザーが実行: `gm select ... --export json --output <file>`
2. システムがデータを検証
3. データを JSON ファイルへエクスポート
4. 確認メッセージを表示

**Google Sheets エクスポートの手順:**
1. ユーザーが Google OAuth 2.0 で認証（初回のみ）
2. ユーザーが実行: `gm select ... --export sheets --output <spreadsheet-id>`
3. システムが指定の Google Sheet の行を作成または更新
4. シートの URL を含む確認メッセージを表示

**この方式の利点:**
- JSON エクスポート: 認証が不要で、ファイルベースのためバージョン管理しやすい
- Google Sheets: クラウドベースで、共同編集とリアルタイムの同期ができる
- データはユーザーの管理下に置かれる
- 出力先を柔軟に選べる

**設計上の考慮点:**
- JSON エクスポート: Monster オブジェクトの標準的な JSON 配列形式
- Google Sheets: OAuth 2.0 認証、Google Sheets API との連携
- エクスポート前のデータ検証
- 既存のコマンド構造との統合
- 認証と API の失敗に対するエラー処理

### Phase 3: システムの抽象化
- トレイトによるシステムの分離
- 複数のゲームシステムへの対応
- 再利用できる query / io モジュール
- **工数**: 中（5〜7日）
- **効果**: 将来のシステムへの拡張性

**実装の詳細:**
```rust
// core/src/systems/common.rs
pub trait GameEntity: Serialize + Deserialize + Clone {
    fn get_name(&self) -> &str;
    fn get_level(&self) -> i32;
    fn get_category(&self) -> &str;
}

// システムごとに実装
impl GameEntity for Monster { ... }  // SW2.5
impl GameEntity for DndCharacter { ... }  // D&D
```

**手順:**
1. `core/src/systems/` のモジュール構成を作成
2. システムに依存しないメソッドを持つ `GameEntity` トレイトを定義
3. 既存の `Monster` 構造体にトレイトを実装
4. 新しいシステムのモジュール（dnd、pathfinder など）を作成
5. トレイトオブジェクトで動くよう検索関数を汎用化
6. `--system` フラグで CLI からシステムを選べるようにする

## 今後の検討事項

### SQLite のローカルバックエンド（任意）
より大きなデータセットや高度な検索のために:
- JSON ファイルの代わりにローカルの SQLite データベースを使う
- 1万件を超えるエンティティのインデックス検索
- 複雑な検索への対応（結合、集計）
- **注意**: レポジトリをサンプルデータ付きで公開する場合に検討する

### データ形式の柔軟性
現在の実装は JSON を使っています。他の形式も検討できます。
- YAML: TRPG のデータ構造をより読みやすく書ける
- TOML: すでに設定に使っている
- **保留**: Phase 3 のシステム抽象化の後で再検討する

### オープンソースとデータのプライバシー
- **コード**: オープンソースのライセンス（Apache 2.0、MIT など）にできる
- **データ**: 書籍に由来するデータは著作権の管理が必要
- **解決策**: サンプルデータと本番データを分ける
- **状況**: サンプルデータが用意できるまで保留

## 当面の次の手順（必要な場合）

1. **設定ファイルへの対応を追加**:
   - `config/default.toml` を作成する
   - `config` クレートか `toml` クレートを使う
   - CLI のフラグでパスを上書きできるようにする: `gm --config config/custom.toml find "name"`

2. **データパスをパラメータにする**:
   - ハードコードされたパスを関数の引数に変える
   - `main()` からハンドラ関数へ渡す
   - 代わりに設定ファイルから読み込む

3. 新しいシナリオの**単体テストを追加**:
   - 複数ファイルの読み込み
   - 設定の解析
   - ファイルが無い場合のエラー処理

## コード構成のベストプラクティス

```
trpg-json/
├── core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models.rs          # Monster、Part など
│   │   ├── query.rs           # 検索の関数
│   │   ├── io.rs              # ファイルの入出力
│   │   ├── systems/
│   │   │   ├── mod.rs
│   │   │   ├── system_a.rs    # システム A 固有
│   │   │   ├── common.rs      # 共通のトレイト
│   │   └── config.rs          # 設定の読み込み
│   └── Cargo.toml
├── app/
│   ├── src/
│   │   ├── main.rs            # エントリポイントとルーティング
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── monster.rs     # モンスターのコマンドのハンドラ
│   │   │   └── spell.rs       # スペルのコマンドのハンドラ
│   │   └── utils.rs           # 共通の入出力と UI のユーティリティ
│   └── Cargo.toml
├── config/
│   └── default.toml           # 既定の設定
├── data/
│   ├── systems/
│   │   ├── monsters.json      # 暗号化済み
│   │   └── items.json         # 将来
│   └── samples/               # サンプルデータ
└── README.md
```

## セキュリティの考慮事項

### 暗号化されたデータ
- 本番のデータファイルはすべて `git-crypt` で暗号化したままにする
- 開発環境: 必要なときだけ復号する
- CI/CD: 暗号化した認証情報を使うか、オフラインで処理する

### 設定ファイル
- ローカルの設定の上書き用に `.gitignore` の記載を維持する
- 例: `config/local.toml`（コミットしない）
- 本番の設定は暗号化してコミットする

### 複数システムへの対応
- システムごとにデータディレクトリを分ける
- システムごとに独立した暗号化の方針を持てる
- 読み込む前にシステムの互換性を検証する

## テスト戦略

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_load_multiple_files() { ... }
    
    #[test]
    fn test_config_loading() { ... }
    
    #[test]
    fn test_system_detection() { ... }
}
```

### 統合テスト
- 異なるデータソースからの読み込みをテストする
- エラー処理（ファイルが無い、形式が不正）をテストする
- システムが混在するシナリオをテストする

## 移行ガイド: 単一ファイル → 複数ファイル

大きな `monsters.json` が1つあり、それを分割したい場合の手順です。

**手順 1: 分割したデータファイルを用意する**
```bash
# monsters.json をカテゴリやレベルの範囲で分割する
# 例: カテゴリごとに別のファイルを作成する
jq '.[] | select(.Category == "Category1")' data/systems/monsters.json > data/systems/monsters_part1.json
jq '.[] | select(.Category == "Category2")' data/systems/monsters.json > data/systems/monsters_part2.json
```

**手順 2: 設定ファイルを更新する**
```toml
[data]
# 従来の書き方（引き続き使える）:
# monsters = "data/systems/monsters.json"

# 新しい書き方（複数ファイル）:
monsters = [
    "data/systems/monsters_part1.json",
    "data/systems/monsters_part2.json"
]
```

**手順 3: データが失われていないことを確認する**
```bash
gm find "Monster Name"  # 引き続きエンティティが見つかるはず
gm select -l 6          # レベル6のエンティティがすべて返るはず
```

**重要な注意点:**
- すべてのファイルは JSON 配列でなければならない
- ファイルの順序は検索結果に影響しない（1つのコレクションにまとめられる）
- 追加や削除の変更は、リストの最初のファイルに保存される
- ファイルが増えすぎないよう、定期的な統合を検討する

## トラブルシューティング

### 問題: "Error: Data file not found"
**解決策:**
1. パスがホームディレクトリからの相対パスになっているか確認する: `~/playground/TRPG-JSON/...`
2. ファイルが存在するか確認する: `ls ~/path/to/file.json`
3. ファイルの権限を確認する: `chmod 644 data/systems/monsters.json`
4. 代替策として絶対パスを使う: `/Users/username/playground/TRPG-JSON/...`

### 問題: 正しい JSON なのに "JSON parsing error" が出る
**解決策:**
1. ファイルが正しい JSON か確認する: `jq empty data/systems/monsters.json`
2. 文字コードの問題がないか確認する: `file data/systems/monsters.json`
3. ルート要素が配列か確認する: `jq '. | type' data/systems/monsters.json` → "array" と出力されるはず
4. 壊れたレコードがないか確認する: `jq '.[] | has("name")' data/systems/monsters.json`

### 問題: 複数のファイルでモンスター名が衝突する
**解決策:**
1. 重複しないファイル名を付ける: "monsters_part1.json"、"monsters_part2.json"
2. 出どころを示すメタデータを加える: Monster の追加フィールドに "source" を入れる
3. または1つのファイルに統合する: `jq -s 'add' monsters_part*.json > monsters_consolidated.json`

### 問題: add/delete が意図しないファイルに反映される
**注意:** 複数のファイルを使う場合、変更は設定のリストの最初のファイルに保存されます。
**解決策:** 保存の動作を変えたい場合:
1. ときどきファイルを統合することを検討する
2. または「編集する」ファイルがリストの最初に来るよう設定を組み替える
3. 将来の Phase 4 のバックエンドでは、ファイルごとの追跡に対応する予定

## 例: 新しいシステムの追加（Phase 3 以降）

Phase 3 の実装後に、ゲームシステムを追加で対応させる手順です。

**1. システムのモジュールを作成する**（`core/src/systems/system_b.rs`）:
```rust
use serde::{Deserialize, Serialize};
use super::common::GameEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: i32,
    pub hit_points: i32,
    pub armor_class: i32,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl GameEntity for Character {
    fn get_name(&self) -> &str { &self.name }
    fn get_level(&self) -> i32 { self.level }
    fn get_category(&self) -> &str { &self.class }
}
```

**2. システムのファクトリを作成する**（`core/src/systems/mod.rs`）:
```rust
pub fn load_system(name: &str, config: &Config) -> Result<Box<dyn GameEntity>, String> {
     match name {
         "system_a" => load_system_a_entities(config),
         "system_b" => load_system_b_characters(config),
         _ => Err(format!("Unknown system: {}", name)),
     }
 }
```

**3. システムを選べるよう設定のスキーマを更新する**:
```toml
[data]
monsters = ["data/systems/monsters.json"]

[system]
name = "system_a"  # "system_a"、"system_b" などを指定できる

# System B の例:
# [data]
# characters = ["data/systems/characters.json"]
# [system]
# name = "system_b"
```

**4. システムを選べるよう CLI を更新する**:
```bash
gm --system system_b find "Character Name"  # System B のキャラクターを検索
gm find "Monster Name" -l 3                 # System A のモンスターを検索（既定）
```

**5. 後方互換性を確保する** - 指定がなければ SW2.5 を既定にする

**この方式の利点:**
- システムをプラグインのように追加できる
- 検索ロジックを再利用できる（`find_by_name()` はどの `GameEntity` でも動く）
- システムごとにデータとコードを分けられる
- 既存のコードを変更せずに新しいシステムを追加しやすい

## 参考資料

### 現在の実装
- **CLI のエントリポイント**: `app/src/main.rs`
- **コマンドのハンドラ**: `app/src/commands/{monster,spell}.rs`
- **ユーティリティ**: `app/src/utils.rs`（入出力、整形）
- **ビジネスロジック**: `core/src/query.rs`（検索の関数）
- **ファイルの入出力**: `core/src/io.rs`（読み込みと保存）
- **設定**: `core/src/config.rs`（設定の解析）
- **データモデル**: `core/src/lib.rs`（Monster、Part の構造体）
- **設定ファイル**: `config/default.toml`

### ビルドと開発
- ビルドとテストのコマンドは `AGENTS.md` を参照
- Rust 2024 edition、エラー処理は thiserror、シリアライズは serde

### 関連ドキュメント
- `tasks.md` - プロジェクトのタスクボードと状況
- `README.md` - ユーザー向けのドキュメント
- `AGENTS.md` - 開発のガイドラインと git の方針

### 使っている設計パターン
- **トレイトベースの設計**（将来の Phase 3 の拡張性のため）
- **enum のデシリアライズ**（設定の柔軟性のための `#[serde(untagged)]`）
- **エラーの伝播**（thiserror クレートによる扱いやすいエラー処理）
- **設定の管理**（TOML ベース、環境変数に対応）

### 性能の考慮事項
- **現在**: 読み込んだ全モンスターに対する O(n) の検索
- **将来**: 大きなデータセット向けに Phase 4 以降でインデックスを検討する
- **メモリ**: 全モンスターをメモリに読み込む（10万件未満なら十分）
- **ディスク**: JSON 形式は人が読めるが圧縮されていない（将来の最適化を検討する）

## Status & Next Steps

### Current Implementation Status (as of 2025-11-30)
- ✅ **Phase 1**: Single system, basic CRUD operations
- ✅ **Phase 2**: Multiple data files, configuration file support, home directory paths
- ⏳ **Phase 2.5**: External data export capabilities (next priority)
- ⏳ **Phase 3**: System abstraction (planned after Phase 2.5)
- 📌 **Future**: SQLite backend (optional), data format flexibility

### Decisions Made
- **Priority**: Files (Phase 2) ✓ → Data Export (Phase 2.5) → Systems (Phase 3)
- **Config format**: TOML (chose over YAML for simplicity)
- **Path resolution**: Home directory relative (supports `~` expansion)
- **Data merge**: Load all files into single collection for unified query
- **Export format**: JSON and Google Sheets supported
- **Manual execution**: Data export is user-triggered, no automation needed
- **Phase 4**: Not planned (scope limited to core functionality)
- **Extensibility**: Design supports adding new game systems and export formats

### Questions for Future Implementation
- **Phase 3 Timing**: Begin when SW2.5 coverage is stable (current or next milestone)
- **Database choice**: SQLite for local (preferred), PostgreSQL for remote (future)
- **Caching strategy**: Consider Redis for API-backed data
- **Multi-system config**: One file per system or unified config?

### Recommended Next Steps (Priority Order)
1. **Phase 2.5 (Medium effort, high value)** - External data export capabilities
2. **Phase 3 (Medium effort, high value)** - System abstraction for additional game systems
3. **Enhanced validation** - Stricter JSON schema validation per system
4. **Performance optimization** - For large datasets (>10k entities)
5. **Documentation** - Comprehensive user guides and examples


## Udonarium Export Format

### Design Principles
- **Output format**: ZIP file containing XML file(s) for all monsters (single-part and multi-part)
- **CLI flag**: `--export udonarium --output <zip_file_path>` specifies the ZIP output file path
- **File naming (inside ZIP)**: Individual XML files follow naming convention:
  - Single-part: `(monster_name).xml`
  - Multi-part: `(monster_name)_(part_name)(no).xml` for each part
  - Examples:
    - Single-part: `ゴブリン.xml` inside `ゴブリン.zip`
    - Multi-part: `トレント_幹.xml`, `トレント_根0.xml`, `トレント_根1.xml` inside `トレント.zip`
- **Template selection**: Core template used for all parts (both コア=true and コア=false)
- **XML structure**: Based on Udonarium character format with nested `<data>` elements
- **Chat palette**: Auto-generated with hit, dodge, and resistance checks only (no special abilities)
- **Field mapping**: TRPG-JSON Monster → Udonarium XML data structure

### XML Structure

**Root element:**
```xml
<character location.name="table" location.x="0" location.y="0" posZ="0" rotate="0" roll="0">
```

**Data hierarchy:**
```xml
<data name="character">
  <data name="image">
    <data type="image" name="imageIdentifier"></data>
  </data>
  <data name="common">
    <data name="name">モンスター名 (or モンスター名\n(部位名))</data>
    <data name="size">1</data>
  </data>
  <data name="detail">
    <data name="リソース">
      <data type="numberResource" currentValue="X" name="HP">X</data>
      <data type="numberResource" currentValue="X" name="MP">X</data>
      <data type="numberResource" currentValue="X" name="防護点">X</data>
    </data>
    <data name="ステータス・バフ・デバフ">
      <!-- Hit rate, Damage, Dodge, Resistances -->
    </data>
    <data name="特殊能力">
      <data name="特殊能力1" type="note"></data>
      <data name="特殊能力2" type="note"></data>
    </data>
    <data name="戦闘準備">
      <data name="魔物知識・先制判定" type="note"></data>
    </data>
    <data name="情報">
      <data name="弱点" type="note"></data>
      <data name="移動速度"></data>
    </data>
    <data name="魔物知識">
      <data name="生態" type="note"><!-- Category, Level --></data>
    </data>
  </data>
</data>

<chat-palette dicebot="SwordWorld2.5">
<!-- Auto-generated commands: 命中力,打撃点, 回避力, 生命抵抗力, 精神抵抗力 -->
</chat-palette>
```

### Field Mapping

**Rust Monster struct → Udonarium XML:**

| Rust Field | JSON Key | Udonarium XML Location | Format | Notes |
|------------|----------|------------------------|--------|-------|
| `monster.name` | `name` | `character/common/name` | `name` or `name\n(part.name)` | If part.name is empty, output only name; if exists, concatenate |
| `monster.category` | `Category` | `character/detail/魔物知識/生態` | Included in ecology note | Part of knowledge field |
| `monster.level` | `Lv` | `character/detail/魔物知識/生態` | Included in ecology note | Part of knowledge field |
| `part.hp` | `part.HP` | `character/detail/リソース/HP` | numeric value | |
| `part.mp` | `part.MP` | `character/detail/リソース/MP` | numeric value | Output 0 if mp == -1 |
| `part.armor` | `part.防護点` | `character/detail/リソース/防護点` | numeric value | |
| `part.hit_rate` | `part.命中力` | `character/detail/ステータス・バフ・デバフ/命中力` | hit_rate - 7 | Subtract 7 (expected value to base value) |
| `part.damage` | `part.打撃点` | `character/detail/ステータス・バフ・デバフ/打撃点` | numeric value | No adjustment needed (already base value) |
| `part.dodge` | `part.回避力` | `character/detail/ステータス・バフ・デバフ/回避力` | dodge - 7 | Subtract 7 (expected value to base value) |
| `monster.life_resistance` | `生命抵抗力` | `character/detail/ステータス・バフ・デバフ/生命抵抗力` | life_resistance - 7 | Subtract 7; core parts only |
| `monster.mental_resistance` | `精神抵抗力` | `character/detail/ステータス・バフ・デバフ/精神抵抗力` | mental_resistance - 7 | Subtract 7; core parts only |
| `monster.fame` | `知名度` | `character/detail/戦闘準備/魔物知識・先制判定` | Format: `知名度/弱点値\r先制値` | **Core parts only** |
| `monster.weakness_value` | `弱点値` | `character/detail/戦闘準備/魔物知識・先制判定` | Format: `知名度/弱点値\r先制値` | **Core parts only** |
| `monster.weakness` | `弱点` | `character/detail/情報/弱点` | string | **Core parts only** |
| `monster.initiative` | `先制値` | `character/detail/戦闘準備/魔物知識・先制判定` | Format: `知名度/弱点値\r先制値` | **Core parts only** |
| `monster.common_abilities` | `共通特殊能力` | `character/detail/特殊能力/特殊能力1` | text value | First special ability slot |
| `part.special_abilities` | `part.部位特殊能力` | `character/detail/特殊能力/特殊能力2` | text value | Part-specific abilities |

### Chat Palette Auto-Generation

**Generated commands include (hit/dodge/resistance checks only):**
```
2d+{命中力}　命中判定
2d+{回避力}　回避判定
2d+{打撃点}　ダメージロール
2d+{生命抵抗力}　生命抵抗判定
2d+{精神抵抗力}　精神抵抗判定
```

**Notes:**
- Commands reference Rust field names with -7 adjustment applied (命中力, 回避力, 生命抵抗力, 精神抵抗力 values already adjusted)
- Special abilities are NOT included in chat palette
- Dicebot: SwordWorld2.5

### XML Output Differences: Core vs Non-Core Parts

**Core parts (コア=true):**
- Include `character/detail/戦闘準備/魔物知識・先制判定`, `character/detail/情報/弱点`, and `character/detail/魔物知識/生態` sections
- Format in 戦闘準備: `知名度/弱点値\n先制値`
- Output all status values (命中力-7, 回避力-7, 生命抵抗力-7, 精神抵抗力-7)

**Non-core parts (コア=false):**
- **DO NOT include** `character/detail/戦闘準備`, `character/detail/情報`, `character/detail/魔物知識` sections
- Output part-specific HP, MP, armor, hit_rate-7, dodge-7, life_resistance-7, mental_resistance-7
- Include resistance values (for resistance judgment rolls)

### File Output Strategy

**CLI Usage:**
```bash
gm export --export udonarium --output <zip_file_path>
# Example:
gm export --export udonarium --output ゴブリン.zip
gm export --export udonarium --output /path/to/monsters.zip
```

**Single-part monsters:**
- Output format: ZIP file containing single XML file
- File naming inside ZIP: `(monster_name).xml`
- Example: `--output ゴブリン.zip` creates a ZIP file containing `ゴブリン.xml`

**Multi-part monsters (including multiple core parts):**
- Output format: ZIP file containing multiple XML files
- File naming convention inside ZIP: `(monster_name)_(part_name)(no).xml`
- Example: `--output トレント.zip` creates a ZIP file containing:
  ```
  トレント_幹.xml (core part - includes 戦闘準備 情報 魔物知識)
  トレント_根0.xml (non-core part)
  トレント_根1.xml (non-core part)
  ```
- Example with multiple core parts: `--output アンシェント・ドラゴン.zip` creates:
  ```
  アンシェント・ドラゴン_頭0.xml (core part - includes 戦闘準備 情報 魔物知識)
  アンシェント・ドラゴン_頭1.xml (core part - includes 戦闘準備 情報 魔物知識)
  アンシェント・ドラゴン_防護膜.xml (non-core part)
  ```

### Part Naming Algorithm

1. If `part.name` is empty, use "core" for コア=true, use sequential number for others
2. If `part.name` is provided, use that name
3. For duplicate names (e.g., multiple "根" parts), append sequential number (0, 1, 2...)

**Examples:**
- Part with `name=""` and `コア=true` → `monster.xml`
- Part with `name=""` and `コア=false` (first) → `monster_0.xml`
- Part with `name="幹"` and `コア=true` → `monster_幹.xml`
- Parts with `name="根"` → `monster_根0.xml`, `monster_根1.xml`

### Implementation Notes

- Location attributes (location.x, location.y) set to "0" by default
- All other character attributes (posZ, rotate, roll) set to "0"
- Dicebot set to "SwordWorld2.5"
- numberResource elements use `currentValue` attribute for convenience
- Data transformation handles empty/null values as "-" where appropriate

## Spell and Skill Data Structure Specification

This section defines data structures and query behavior specific to Spells and Skills.

### Level Field (`Lv` struct)

**Scope**: Applies to Spells and Skills only (Monsters use a simple `level: i32` field instead of the `Lv` struct)

**Mutual Exclusivity**: The `Lv.kind` field must contain exactly one of three mutually exclusive variants:
- `"value"`: Fixed level (e.g., "spell acquired at level 3")
- `"value+"`: Minimum level (e.g., "skill available at level 5 or higher")
- `"rank"`: Rank-based progression (e.g., "rank 2 ability")

**CLI Query Flags**: 
- `-l` (level filter) and `-r` (rank filter) cannot be used simultaneously
- Querying by level (`-l`) searches records with `Lv.kind: "value"` or `"value+"`
- Querying by rank (`-r`) searches records with `Lv.kind: "rank"`

#### `Lv.kind` Variants and Query Behavior

**1. `Lv.kind: "value"` (Fixed Level)**
- **CLI flag**: `-l <level>`
- **Query behavior**: Returns records where `Lv.value` exactly matches the specified level
- **Use case**: Spells with fixed acquisition level (e.g., "Fire Ball" acquired at level 3)
- **Example**:
  ```json
  { "name": "ファイア・ボール", "Lv": { "kind": "value", "value": 3 } }
  ```
  Query: `gm spell find -l 3` → matches this record

**2. `Lv.kind: "value+"` (Minimum Level)**
- **CLI flag**: `-l <level>`
- **Query behavior**: Returns records where `Lv.value+` is less than or equal to the specified level
- **Use case**: Skills available from a minimum level onward (e.g., "Skill available at level 5 or higher")
- **Example**:
  ```json
  { "name": "上級剣術", "Lv": { "kind": "value+", "value+": 5 } }
  ```
  Query: `gm skill find -l 7` → matches (since 5 ≤ 7)
  Query: `gm skill find -l 3` → does NOT match (since 5 > 3)

**3. `Lv.kind: "rank"` (Rank-based)**
- **CLI flag**: `-r <rank>`
- **Query behavior**: Returns records where `Lv.rank` exactly matches the specified rank
- **Use case**: Rank-based progression systems (e.g., "Rank 2 magic")
- **Value range**: Variable; depends on `schoolVariant` and system implementation
- **Example**:
  ```json
  { "name": "丙系魔法ランク2", "Lv": { "kind": "rank", "rank": 2 } }
  ```
  Query: `gm spell find -r 2` → matches this record

### School Variant Field (`schoolVariant`)

**CLI flag**: `-sv <variant>`

**Query behavior**: 
- Returns records where `schoolVariant` exactly matches the specified value (exact match only)
- If `-sv` flag is specified, records without a `schoolVariant` field (or with `null` value) are excluded from results

**Example**:
```json
{ "name": "特殊甲系魔法", "school": "甲系", "schoolVariant": "特殊" }
```
Query: `gm spell find -sv 特殊` → matches this record

### God Field (`god`)

**Applicability**: Only applicable when `schoolVariant == "特殊"` AND `school == "甲系"`

**CLI flag**: `-g <god_name>`

**Query behavior**:
- Returns records where `god` exactly matches the specified value (exact match only)
- If `-g` flag is specified, records without a `god` field (or with `null` value) are excluded from results

**Example**:
```json
{ 
  "name": "特殊甲系魔法の例", 
  "school": "甲系", 
  "schoolVariant": "特殊",
  "god": "神名" 
}
```
Query: `gm spell find -sv 特殊 -g 神名` → matches this record

**Note**: If `schoolVariant` is not "特殊" or `school` is not "甲系", the `god` field is not applicable and should not exist in the data.

## Spell Chat Palette Format

### Design Principles
- **Output format**: Text-based chat palette (one command per line)
- **Support spell flag**: `補助 == true` or `補助 == false` determines output content
- **Dice rolls**: Not required for support spells; required for regular spells
- **Field mapping**: TRPG-JSON Spell → Chat palette text format

### Support Spells (`補助 == true`)
**Output format:**
```
Spell Name / MP:X / 対象:Y / 射程:Z / 時間:T / Effect Description
```

**Examples:**
```
ライト / MP:3 / 対象:任意の地点 / 射程:10m(起点指定) / 光源を生成する。
魔法解除 / MP:8 / 対象:魔法1つ / 射程:接触 / 魔法を打ち消す。
```

**Characteristics:**
- No dice rolls
- Spell information confirmation format
- Target and effect are primary information

### Regular Spells (`補助 == false`)
**Output format:**
```
2d+{Magic Category}+{行使修正}  Spell Name / MP:X / 対象:Y / 射程:Z / 時間:T / 効果
```

**Examples:**
```
2d+{甲系魔法}+{行使判定} 呪文名 / MP:15 / 対象:1エリア(半径4mすべて) / 射程:術者 / 時間:一瞬 / 対象に効果を与える。
```

**Characteristics:**
- Dice rolls required (category serves as the judgment check name)
- `{行使修正}` is literal output (replaced by dicebot system at runtime)

### Schema Field References
- `name`: Spell name
- `MP.value` OR `MP.value+` OR `MP.special`: MP consumption (exactly one exists)
  - `value`: Fixed MP cost
  - `value+`: Minimum MP cost (output as "3～" etc.)
  - `special`: Special consumption (output string as-is)
- `射程` OR `射程(m)`: Range information (either field name is acceptable)
  - Flexible field naming to accommodate different data sources
  - Output value as-is regardless of field name used
  - Example: `射程` value "10m(起点指定)" or `射程(m)` value "10"
- `対象` fields: Target information (see separate section)
- `category`: Magic category name derivation (see separate section)
- `効果`: Effect description, output as-is
- `時間` fields: Duration information (see separate section)

#### Magic Category Name
**When `category` is NOT exactly 2 full-width characters:**
- Magic category → `category` as-is
- Example: If `category` is "ハイテクノロジー", magic category is "ハイテクノロジー"

**When `category` IS exactly 2 full-width characters:**
- Magic category → `category` + "魔法"
- Example: If `category` is "丙系", magic category is "丙系魔法"

#### 対象 (Target)
**When `対象.kind == "個別"`:**
- Output: `対象.個別` as-is
- Example: If `対象.個別` is "1体全", output "1体全"

**When `対象.kind == "エリア"`:**
- Output format: `対象.value`(半径`対象.半径(m)`m`対象.末尾`)
- Example: If `対象.value` is "2エリア", `対象.半径(m)` is "10", `対象.末尾` is "空間", output "2エリア(半径10m空間)"

#### 時間 (Duration)
**When `時間.value` is a string:**
- Output: `時間.value` as-is
- Example: If `時間.value` is "一瞬", output "一瞬"

**When `時間.value` is an integer:**
- Output: `時間.value` + `時間.unit`
- Example: If `時間.value` is "3" and `時間.unit` is "年", output "3年"

---

## Google Sheets Export Format

### Design Principles
- **Text only**: No formatting, cell formulas, or styling
- **Cell merging**: Preserve existing cell merges; cells have mixed merging patterns (some vertical, some horizontal)
- **Line breaks**: Represented as `\n` in specification; actual cell content uses real line breaks
- **Language**: All output field names in Japanese
- **Row structure**: 2 rows per part (part A line + part B line)
- **Multiple parts**: Output n parts × 2 rows each
  - Example: Trent with 3 parts (幹, 根, 根) → 2 rows + 2 rows + 2 rows = 6 rows total
  - Example: Single-part monster → 2 rows total
- **Missing part data**: Fill with "-" except for `name` and `共通特殊能力` columns
- **Empty/negative values**: Convert to "-"
- **Output target**: "search" sheet only
- **Insertion point**: Scan from row 3, find first empty cell in odd rows (row 3, 5, 7, 9...) of column A; insert there
- **Error handling**: If no empty row found, display error and cancel export
- **Weakness field transformation**: In `弱点` column, replace "エネルギー" with "E" and "ダメージ" with "ダメ"; remove "属性"
- **`\n()` handling**: If content inside `\n()` is empty, omit the entire `\n()` wrapper

### Row Output Structure
**For each part in the monster's `part` array:**

1. **First line (odd-numbered row in output)**:
   - Columns with single-part data: Output the value
   - Common data (`name`, `共通特殊能力`, etc.): Output the value
   - Example: Row 3 for first part

2. **Second line (even-numbered row in output)**:
   - Output Part-specific special abilities(`部位特殊能力`)
   - Output Weakness debuff with field transformations(`弱点`)
   - Output Only two above values
   - Example: Row 4 for first part

**For Trent example (2 parts: 幹 + 根(x2)):**
- Row 3: First part (幹) data + common data
- Row 4: First part (幹) Part-specific special abilities + Weakness
- Row 5: Second part (根) data + common data
- Row 6: Second part (根) Part-specific special abilities + Weakness
- Row 7: Second part (根) data + common data
- Row 8: Second part (根) Part-specific special abilities + Weakness
- Common data fields repeat in all rows (3, 4, 5, 6, 7, 8)

### Column Mapping

| Cell | Output Data | Notes |
|------|------------|-------|
| A | `name` with `part.name` | Format: `name` if `part.name` is empty; `name\n(part.name)` if present. Prefix "★" if `コア==true` |
| L | `part.HP` | Part hit points |
| P | `part.MP` | Part magic points; "-" if negative or -1 |
| R | `part.防護点` | Part defense rating |
| T | `先制値` | Initiative; "-" for non-first parts |
| V | `生命抵抗力` | Life resistance; "-" for non-first parts |
| X | `精神抵抗力` | Spirit resistance; "-" for non-first parts |
| Z | "3" | Fixed constant value (rule constraint) |
| AB | `moveon\n(moveon_des)` | Ground movement speed and description; "-" if `moveon==-1` |
| AD | `movein\n(movein_des)` | Aerial movement speed and description; "-" if `movein==-1` |
| AF | `part.命中力` | Part accuracy |
| AH | `part.回避力` | Part evasion |
| AJ | `data` | Rulebook reference page |
| AM（odd row） | `共通特殊能力` | Common special abilities (repeats for all rows of same monster) |
| AM+1(even row) | `part.部位特殊能力` | Part-specific special abilities |
| AW(odd row) | `知名度 / 弱点値` | Knowledge rating / weakness value; "-" for non-first parts |
| AW+1(even row) | `弱点` (transformed) | Weakness debuff with field transformations; "-" for non-first parts |

### Cell Merge Structure

**Cells merged both vertically and horizontally (across rows and columns):**
- Multiple small grid cells are merged both vertically across rows and horizontally across columns (Example for rows 3-4: A3:G4, H3:I4, J3:K4, N3:O4, P3:Q4, R3:S4, T3:U4, V3:W4, X3:Y4, Z3:AA4, AB3:AC4, AD3:AE4, AF3:AG4, AH3:AI4, AJ3:AL4)
   - **Data placement**: Only fill cells in odd-numbered rows (first line of each part)
   - **Even rows**: Leave completely empty; the cell merge automatically spans from the odd row above
   - When outputting to Google Sheets API, do not write any data to even-row cells in these columns

**Cells merged only horizontally (across columns only):**
- Merging pattern is as shown in parentheses (Example for rows 3-4: AM3:AV3, AM4:AV4, AW3:AX3, AW4:AX4)
   - Column AM (`共通特殊能力`): Write to target odd rows only (if multiple parts, copy the same value to all rows)
   - Column AM+1 (`part.部位特殊能力`): Write to target even rows only (if multiple parts, copy the same value to all rows, but values differ per part)
   - Column AW (`知名度 / 弱点値`): Write only for the first part in the target rows (to odd rows); fill other odd rows with "-/-"
   - Column AW+1 (`弱点`): Write only for the first part in the target rows (to even rows); fill other even rows with "-"

**Implementation Note:**
When using Google Sheets API to insert data:
1. Cells merged both vertically and horizontally (A-AJ): Write data only to specified odd rows; do not write to even rows or unspecified odd rows
2. Cells merged only horizontally (AM-AW+1): Output different data to specified odd and even rows respectively; do not write to unspecified sections
3. Preserve existing cell merge formatting; API call must maintain merge structure

### Field Transformation Rules

1. **`part.name` concatenation**:
   - If `part.name` is empty → output only `name`
   - If `part.name` has value → output `name\n(part.name)`
   - Add "★" prefix if `コア == true`

2. **Movement fields** (`moveon`, `movein`):
   - If value is -1 → output "-"
   - Otherwise → output as `value\n(description)`
   - If description is empty → omit `\n()` wrapper

3. **Weakness field**:
   - Apply text replacements: "エネルギー" → "E", "ダメージ" → "ダメ"
   - Delete text: "属性"
   - Example: "炎属性ダメージ+2" → "炎ダメ+2"

4. **Common data across rows**:
   - `name`, `共通特殊能力` columns repeat in all rows of same monster
   - Example: For Trent (2 parts, 6 rows), columns AM, AM+1, AW, AW+1 appear in rows 3, 4, 5, 6, 7, 8

5. **Part-specific data**:
   - `part.HP`, `part.MP`, `part.防護点`, etc. → only change when moving to different part
   - Non-first parts: Replace single-part common fields with "-"
   - Exception: `name` and `共通特殊能力` always output actual values (not "-")

### Example: Trent (Multiple Parts)

**JSON Data:**
```json
{
  "name": "トレント",
  "先制値": 13,
  "共通特殊能力": "特殊系統魔法8Lv／11、魔法の才能",
  "知名度": 16,
  "弱点値": 21,
  "弱点": "炎属性ダメージ+3",
  "moveon": -1,
  "moveon_des": "",
  "movein": -1,
  "movein_des": "",
  "data": "SAMPLE",
  "part": [
    {
      "name": "幹",
      "コア": true,
      "HP": 105,
      "MP": 45,
      "命中力": 21,
      "回避力": 18,
      "部位特殊能力": "能力F＝5",
      "防護点": 9,
      "部位数": 1
    },
    {
      "name": "根",
      "コア": false,
      "HP": 75,
      "MP": 20,
      "命中力": 19,
      "回避力": 15,
      "部位特殊能力": "能力AC",
      "防護点": 7,
      "部位数": 2
    }
  ]
}
```

**Expected Output (rows 3-8):**

| Row | A | L | P | R | T | V | X | Z | AB | AD | AF | AH | AJ | AM | AW |
|-----|---|---|---|---|---|---|---|---|----|----|----|----|----|----|------|
| 3 | ★トレント\n(幹) | 105 | 45 | 9 | 13 | 21 | 19 | 3 | - | - | 21 | 18 | SAMPLE | 特殊系統魔法8Lv／11、魔法の才能 | 16/21 |
| 4 |||||||||||||| 能力F＝5 | 炎ダメ+3 |
| 5 | トレント\n(根) | 75 | 20 | 7 | - | - | - | 3 | - | - | 19 | 15 | SAMPLE | 特殊系統魔法8Lv／11、魔法の才能 | - |
| 6 |||||||||||||| 能力AC | - |
| 7 | トレント\n(根) | 75 | 20 | 7 | - | - | - | 3 | - | - | 19 | 15 | SAMPLE | 特殊系統魔法8Lv／11、魔法の才能 | - |
| 8 |||||||||||||| 能力AC | - |

**Notes:**
- Rows 3-4: First part (幹, コア=true) → "★" prefix, common values output, all fields populated
- Rows 5-8: Second part (根, コア=false) → no "★" prefix, `先制値`/`生命抵抗力`/`精神抵抗力`/`知名度`/`弱点値`/`弱点` → "-"
