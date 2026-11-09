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

### Phase 2.5: 外部へのデータエクスポート ✅（完了）
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
gm monster select -l 6 --export json --output results.json

# Google Sheets へエクスポート（OAuth 2.0 の設定が必要）
gm monster select -c "Category" --export sheets --output "Spreadsheet ID"
```

**JSON エクスポートの手順:**
1. ユーザーが実行: `gm monster select ... --export json --output <file>`
2. システムがデータを検証
3. データを JSON ファイルへエクスポート
4. 確認メッセージを表示

**Google Sheets エクスポートの手順:**
1. ユーザーが Google OAuth 2.0 で認証（初回のみ）
2. ユーザーが実行: `gm monster select ... --export sheets --output <spreadsheet-id>`
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
- 1万件を超える項目のインデックス検索
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
   - CLI のフラグでパスを上書きできるようにする: `gm --config config/custom.toml monster find "name"`

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
gm monster find "Monster Name"  # 引き続きモンスターが見つかるはず
gm monster select -l 6          # レベル6のモンスターがすべて返るはず
```

**重要な注意点:**
- すべてのファイルは JSON 配列でなければならない
- ファイルの順序は検索結果に影響しない（1つのコレクションにまとめられる）
- 新規の追加はリストの最初のファイルに保存し、同名の更新と削除は対象のモンスターを含むファイルだけを書き換える
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
**注意:** 新規の追加は、設定のリストの最初のファイルに保存されます。同名の更新と削除は、そのモンスターを含むファイルだけを書き換えます（T052）。
**解決策:** 新規の追加先を変えたい場合:
1. ときどきファイルを統合することを検討する
2. または「編集する」ファイルがリストの最初に来るよう設定を組み替える

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
gm monster find "Monster Name" -l 3         # System A のモンスターを検索（既定）
```

**5. 後方互換性を確保する** - 指定がなければ SW2.5 を既定にする

**この方式の利点:**
- システムをプラグインのように追加できる
- 検索ロジックを再利用できる（`find_by_name()` はどの `GameEntity` でも動く）
- システムごとにデータとコードを分けられる
- 既存のコードを変更せずに新しいシステムを追加しやすい

## 参考資料

### 現在の実装
- **CLI のエントリポイント**: `rust/app/src/main.rs`
- **コマンドのハンドラ**: `rust/app/src/commands/{monster,spell}.rs`
- **ユーティリティ**: `rust/app/src/utils.rs`（入出力、整形）
- **ビジネスロジック**: `rust/core/src/query.rs`（検索の関数）
- **ファイルの入出力**: `rust/core/src/io.rs`（読み込みと保存）
- **設定**: `rust/core/src/config.rs`（設定の解析）
- **データモデル**: `rust/core/src/lib.rs`（Monster、Part の構造体）
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

## 状況と次の手順

### 現在の実装状況（2026-11-09 時点）
- ✅ **Phase 1**: 単一システム、基本的な CRUD 操作
- ✅ **Phase 2**: 複数データファイル、設定ファイルへの対応、ホームディレクトリのパス
- ✅ **Phase 2.5**: 外部へのデータエクスポート機能（JSON、Google Sheets、Udonarium、CCFolia）
- ⏳ **Phase 3**: システムの抽象化（未着手。`tasks.md` の T044 に相当。`tasks.md` の Phase 3 はスペル機能のことで、別の区分）
- 📌 **将来**: SQLite バックエンド（任意）、データ形式の柔軟性

### 決定事項
- **優先順位**: ファイル（Phase 2）✓ → データエクスポート（Phase 2.5）✓ → システム（Phase 3）
- **設定の形式**: TOML（単純さのため YAML ではなくこちらを選択）
- **パスの解決**: ホームディレクトリからの相対パス（`~` の展開に対応）
- **データの統合**: すべてのファイルを1つのコレクションに読み込み、まとめて検索する
- **エクスポート形式**: JSON、Google Sheets、Udonarium、CCFolia に対応
- **手動実行**: データのエクスポートはユーザーが実行し、自動化は不要
- **Phase 4**: `tasks.md` の Phase 4 として進行中（TUI など）
- **拡張性**: 新しいゲームシステムやエクスポート形式を追加できる設計

### 今後の実装に向けた検討事項
- **Phase 3 の時期**: SW2.5 の対応範囲が安定したら開始する（今回または次のマイルストーン）
- **データベースの選択**: ローカルは SQLite（優先）、リモートは PostgreSQL（将来）
- **キャッシュの戦略**: API から取得するデータには Redis を検討する
- **複数システムの設定**: システムごとに1ファイルか、統合した設定か

### 推奨する次の手順（優先順）
1. **Phase 3（工数は中、価値は高い）** - ゲームシステムを追加するためのシステムの抽象化
2. **検証の強化** - システムごとのより厳密な JSON スキーマの検証
3. **性能の最適化** - 大きなデータセット（1万件を超える項目）向け
4. **ドキュメント** - 包括的なユーザーガイドと例


## Udonarium エクスポート形式

### 設計の原則
- **出力形式**: すべてのモンスター（部位が1つのものも複数のものも）について、XML ファイルを含む ZIP ファイル
- **CLI のフラグ**: `--export udonarium --output <zip_file_path>` で ZIP の出力先を指定する
- **ファイル名（ZIP の中）**: 個々の XML ファイルは次の命名規則に従う
  - 部位が1つ: `(monster_name).xml`
  - 複数部位: 部位ごとに `(monster_name)_(part_name).xml`。同じ名前の部位が複数あるときは `(monster_name)_(part_name)_(no).xml`
  - 例:
    - 部位が1つ: `ゴブリン.zip` の中に `ゴブリン.xml`
    - 複数部位: `モンスターA.zip` の中に `モンスターA_部位A.xml`、`モンスターA_部位B_0.xml`、`モンスターA_部位B_1.xml`
- **テンプレートの選択**: コア部位（コア=true）とコア以外の部位（コア=false）で別のテンプレートを使う（違いは「XML 出力の違い」の項を参照）
- **XML の構造**: ネストした `<data>` 要素を持つ Udonarium のキャラクター形式に基づく
- **チャットパレット**: 計算、固定値判定、ダイス判定のコマンドを自動生成する（特殊能力は含めない）
- **フィールドの対応**: TRPG-JSON の Monster → Udonarium の XML データ構造

### XML の構造

**ルート要素:**
```xml
<character location.name="table" location.x="0" location.y="0" posZ="0" rotate="0" roll="0">
```

**データの階層:**
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
      <!-- 命中力、打撃点、回避力、抵抗力 -->
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
    </data>
    <data name="魔物知識">
      <data name="生態" type="note"><!-- カテゴリ、レベル --></data>
    </data>
  </data>
</data>

<chat-palette dicebot="SwordWorld2.5">
<!-- 自動生成するコマンド: 計算、固定値判定、ダイス判定（「チャットパレットの自動生成」の項を参照） -->
</chat-palette>
```

### フィールドの対応

**Rust の Monster 構造体 → Udonarium の XML:**

| Rust のフィールド | JSON キー | Udonarium XML の位置 | 形式 | 備考 |
|------------|----------|------------------------|--------|-------|
| `monster.name` | `name` | `character/common/name` | `name` または `name\n(part.name)` | part.name が空なら name だけを出力し、あれば連結する |
| `monster.category` | `Category` | `character/detail/魔物知識/生態` | 生態のノートに含める | 魔物知識のフィールドの一部 |
| `monster.level` | `Lv` | `character/detail/魔物知識/生態` | 生態のノートに含める | 魔物知識のフィールドの一部 |
| `part.hp` | `part.HP` | `character/detail/リソース/HP` | 数値 | |
| `part.mp` | `part.MP` | `character/detail/リソース/MP` | 数値 | mp == -1 のときは 0 を出力 |
| `part.armor` | `part.防護点` | `character/detail/リソース/防護点` | 数値 | |
| `part.hit_rate` | `part.命中力` | `character/detail/ステータス・バフ・デバフ/命中力` | hit_rate - 7 | 7 を引く（期待値から基準値へ。下限は 0） |
| `part.damage` | `part.打撃点` | `character/detail/ステータス・バフ・デバフ/打撃点` | 数値 | 補正は不要（すでに基準値） |
| `part.dodge` | `part.回避力` | `character/detail/ステータス・バフ・デバフ/回避力` | dodge - 7 | 7 を引く（期待値から基準値へ。下限は 0） |
| `monster.life_resistance` | `生命抵抗力` | `character/detail/ステータス・バフ・デバフ/生命抵抗力` | life_resistance - 7 | 7 を引く（下限は 0）。コア以外の部位にも同じ値を出力する |
| `monster.mental_resistance` | `精神抵抗力` | `character/detail/ステータス・バフ・デバフ/精神抵抗力` | mental_resistance - 7 | 7 を引く（下限は 0）。コア以外の部位にも同じ値を出力する |
| `monster.fame` | `知名度` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\n先制値` | **コア部位のみ** |
| `monster.weakness_value` | `弱点値` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\n先制値` | **コア部位のみ** |
| `monster.weakness` | `弱点` | `character/detail/情報/弱点` | 変換後の文字列 | **コア部位のみ**。"エネルギー" → "E"、"ダメージ" → "ダメ"、"属性" は削除 |
| `monster.initiative` | `先制値` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\n先制値` | **コア部位のみ** |
| `monster.common_abilities` | `共通特殊能力` | `character/detail/特殊能力/特殊能力1` | テキスト | 特殊能力の1つ目の枠 |
| `part.special_abilities` | `part.部位特殊能力` | `character/detail/特殊能力/特殊能力2` | テキスト | 部位ごとの能力 |

### チャットパレットの自動生成

**生成するコマンド（コア部位もコア以外の部位も同じ）:**
```
//-----計算
C({HP}+{防護点}+{ダメージ軽減}-()) 　【残HP（物理ダメージ）】
C({HP}+{ダメージ軽減}-())　【残HP（魔法ダメージ）】
C({MP}-())　【MP消費】
C{HP}　【現在HP】
C{MP}　【現在MP】

//-----固定値判定
C({命中力}+7) 命中判定（固定値）
C({回避力}+7) 回避判定（固定値）
C({生命抵抗力}+7) 生命抵抗判定（固定値）
C({精神抵抗力}+7) 精神抵抗判定（固定値）

//-----ダイス判定
2d+{命中力}　命中判定
2d+{打撃点}　ダメージロール
2d+{回避力}　回避判定
2d+{生命抵抗力}　生命抵抗判定
2d+{精神抵抗力}　精神抵抗判定
```

**注意点:**
- コマンドは -7 の補正を適用した Rust のフィールド名を参照する（命中力、回避力、生命抵抗力、精神抵抗力 の値は補正済み）
- 特殊能力はチャットパレットに**含めない**
- ダイスボット: SwordWorld2.5

### XML 出力の違い: コア部位とコア以外の部位

**コア部位（コア=true）:**
- `character/detail/戦闘準備/魔物知識・先制判定`、`character/detail/情報/弱点`、`character/detail/魔物知識/生態` のセクションを含める
- 戦闘準備の形式: `知名度/弱点値\n先制値`
- すべてのステータス値を出力する（命中力-7、回避力-7、生命抵抗力-7、精神抵抗力-7）

**コア以外の部位（コア=false）:**
- `character/detail/戦闘準備`、`character/detail/情報`、`character/detail/魔物知識` のセクションは**含めない**
- 部位ごとの HP、MP、armor、hit_rate-7、dodge-7、life_resistance-7、mental_resistance-7 を出力する
- 抵抗値を含める（抵抗判定のロールのため）

### ファイル出力の方針

**CLI の使い方:**
```bash
gm monster select -n "モンスター名" --export udonarium --output <zip_file_path>
# 例:
gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip
gm monster select -l 6 --export udonarium --output /path/to/monsters.zip
```

**部位が1つのモンスター:**
- 出力形式: XML ファイルを1つ含む ZIP ファイル
- ZIP の中のファイル名: `(monster_name).xml`
- 例: `--output ゴブリン.zip` で `ゴブリン.xml` を含む ZIP ファイルを作成する

**複数部位のモンスター（コア部位が複数あるものを含む）:**
- 出力形式: 複数の XML ファイルを含む ZIP ファイル
- ZIP の中のファイル名の規則: `(monster_name)_(part_name).xml`。同じ名前の部位が複数あるときは `(monster_name)_(part_name)_(no).xml`
- 例: `--output モンスターA.zip` で次のファイルを含む ZIP ファイルを作成する
  ```
  モンスターA_部位A.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  モンスターA_部位B_0.xml (コア以外の部位)
  モンスターA_部位B_1.xml (コア以外の部位)
  ```
- コア部位が複数ある例: `--output モンスターB.zip` で次のファイルを作成する
  ```
  モンスターB_部位A_0.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  モンスターB_部位A_1.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  モンスターB_部位B.xml (コア以外の部位)
  ```

### 部位の命名アルゴリズム

1. `part.name` が空なら、コア=true にはモンスター名だけを使い、それ以外にはモンスター名に `_` と番号を付ける（番号は、名前が空の部位を先頭から 0, 1, 2... と数えたもの）
2. `part.name` があれば、その名前を使う
3. 名前が重複する場合（例: "部位B" の部位が複数ある）は、部位名のあとに `_` と連番（0, 1, 2...）を付ける

**例:**
- `name=""` で `コア=true` の部位 → `monster.xml`
- `name=""` で `コア=false` の部位 → `monster_0.xml`（名前が空のコア部位が先にあれば `monster_1.xml` から）
- `name="部位A"` で `コア=true` の部位 → `monster_部位A.xml`
- `name="部位B"` の部位 → `monster_部位B_0.xml`、`monster_部位B_1.xml`（1つだけなら `monster_部位B.xml`）

### 実装上の注意

- 位置の属性（location.x、location.y）は既定で "0" にする
- その他のキャラクター属性（posZ、rotate、roll）は "0" にする
- ダイスボットは "SwordWorld2.5" にする
- numberResource 要素では、利便性のために `currentValue` 属性を使う
- データの変換では、HP、命中力、回避力、打撃点の値がなければ 0 として出力する

## スペルとスキルのデータ構造の仕様

この節では、スペルとスキルに固有のデータ構造と検索の動作を定義します。

### レベルのフィールド（`Lv` 構造体）

**適用範囲**: スペルとスキルのみ（モンスターは `Lv` 構造体ではなく単純な `level: i32` フィールドを使う）

**排他性**: `Lv.kind` フィールドには、互いに排他的な次の3種類のうち、ちょうど1つが入らなければならない
- `"value"`: 固定のレベル（例: 「レベル3で習得するスペル」）
- `"value+"`: 最低レベル（例: 「レベル5以上で使えるスキル」）
- `"rank"`: ランク制の成長（例: 「ランク2の能力」）

**CLI の検索フラグ**:
- `-l`（レベルのフィルタ）と `-r`（ランクのフィルタ）は同時に使えない
- レベル（`-l`）での検索は、`Lv.kind: "value"` または `"value+"` のレコードを対象にする
- ランク（`-r`）での検索は、`Lv.kind: "rank"` のレコードを対象にする

#### `Lv.kind` の種類と検索の動作

**1. `Lv.kind: "value"`（固定のレベル）**
- **CLI のフラグ**: `-l <level>`
- **検索の動作**: `Lv.value` が指定のレベルと完全に一致するレコードを返す
- **用途**: 習得レベルが固定のスペル（例: レベル3で習得する "Fire Ball"）
- **例**:
  ```json
  { "name": "ファイア・ボール", "Lv": { "kind": "value", "value": 3 } }
  ```
  検索: `gm spell find "ファイア" -l 3` → このレコードに一致する

**2. `Lv.kind: "value+"`（最低レベル）**
- **CLI のフラグ**: `-l <level>`
- **検索の動作**: `Lv.value+` が指定のレベル以下であるレコードを返す
- **用途**: 最低レベル以上で使えるスキル（例: 「レベル5以上で使えるスキル」）
- **例**:
  ```json
  { "name": "上級剣術", "Lv": { "kind": "value+", "value+": 5 } }
  ```
  検索: `gm skill find -l 7` → 一致する（5 ≤ 7 のため。`gm skill` は未実装で、T045 の予定）
  検索: `gm skill find -l 3` → 一致しない（5 > 3 のため）

**3. `Lv.kind: "rank"`（ランク制）**
- **CLI のフラグ**: `-r <rank>`
- **検索の動作**: `Lv.rank` が指定のランクと完全に一致するレコードを返す
- **用途**: ランク制の成長システム（例: 「ランク2の魔法」）
- **値の範囲**: 可変。`schoolVariant` とシステムの実装による
- **例**:
  ```json
  { "name": "丙系魔法ランク2", "Lv": { "kind": "rank", "rank": 2 } }
  ```
  検索: `gm spell find "魔法" -r 2` → このレコードに一致する

### 系統バリアントのフィールド（`schoolVariant`）

**CLI のフラグ**: `-v <variant>` / `--school-variant <variant>`

**検索の動作**:
- `schoolVariant` が指定の値と完全に一致するレコードを返す（完全一致のみ）
- `-v` フラグを指定すると、`schoolVariant` フィールドが無い（または値が `null` の）レコードは結果から除外される

**例**:
```json
{ "name": "特殊甲系魔法", "school": "甲系", "schoolVariant": "特殊" }
```
検索: `gm spell find "魔法" -v 特殊` → このレコードに一致する

### 神のフィールド（`god`）

**適用範囲**: `schoolVariant == "特殊"` かつ `school == "甲系"` の場合にのみ適用される

**CLI のフラグ**: `-g <god_name>`

**検索の動作**:
- `god` が指定の値と完全に一致するレコードを返す（完全一致のみ）
- `-g` フラグを指定すると、`god` フィールドが無い（または値が `null` の）レコードは結果から除外される

**例**:
```json
{ 
  "name": "特殊甲系魔法の例", 
  "school": "甲系", 
  "schoolVariant": "特殊",
  "god": "神名" 
}
```
検索: `gm spell find "魔法" -v 特殊 -g 神名` → このレコードに一致する

**注意**: `schoolVariant` が "特殊" でない、または `school` が "甲系" でない場合、`god` フィールドは適用されず、データに存在すべきではない。

## スペルのチャットパレット形式

### 設計の原則
- **出力形式**: テキストのチャットパレット（1行に1コマンド）
- **補助スペルのフラグ**: `補助 == true` か `補助 == false` かで出力内容が決まる
- **ダイスロール**: 補助スペルには不要、通常のスペルには必要
- **フィールドの対応**: TRPG-JSON の Spell → チャットパレットのテキスト形式

### 補助スペル（`補助 == true`）
**出力形式:**
```
Spell Name / MP:X / 対象:Y / 射程:Z / 時間:T / Effect Description
```

**例:**
```
補助呪文名 / MP:2 / 対象:1体 / 射程:術者 / 時間:10秒 / 効果の説明。
補助呪文名2 / MP:5 / 対象:1体 / 射程:接触 / 時間:一瞬 / 効果の説明。
```

**特徴:**
- ダイスロールなし
- スペルの情報を確認するための形式
- 対象と効果が主な情報

### 通常のスペル（`補助 == false`）
**出力形式:**
```
2d+{Magic Category}+{行使修正}  Spell Name / MP:X / 対象:Y / 射程:Z / 時間:T / 効果
```

**例:**
```
2d+{甲系魔法}+{行使修正}  呪文名 / MP:15 / 対象:1エリア(半径4mすべて) / 射程:術者 / 時間:一瞬 / 対象に効果を与える。
```

**特徴:**
- ダイスロールが必要（カテゴリが判定の名前になる）
- `{行使修正}` は文字どおりに出力する（実行時にダイスボットのシステムが置換する）

### スキーマのフィールドの参照
- `name`: スペル名
- `MP.value` または `MP.value+` または `MP.special`: 消費 MP（ちょうど1つが存在する）
  - `value`: 固定の消費 MP
  - `value+`: 最低の消費 MP（"3～" などと出力）
  - `special`: 特殊な消費（文字列をそのまま出力）
- `射程` または `射程(m)`: 射程の情報（どちらのフィールド名でもよい）
  - 異なるデータソースに対応するための柔軟なフィールド名
  - 使われているフィールド名によらず、値をそのまま出力する
  - 例: `射程` の値が "10m(起点指定)"、または `射程(m)` の値が "10"
- `対象` のフィールド: 対象の情報（別の項を参照）
- `school`: 魔法カテゴリ名の導出（別の項を参照）
- `効果`: 効果の説明。そのまま出力する
- `時間` のフィールド: 持続時間の情報（別の項を参照）

#### 魔法カテゴリ名
**`school` が2文字ちょうどではないとき:**
- 魔法カテゴリ → `school` をそのまま使う
- 例: `school` が "ハイテクノロジー" なら、魔法カテゴリは "ハイテクノロジー"

**`school` が2文字ちょうどのとき（全角・半角は問わない）:**
- 魔法カテゴリ → `school` + "魔法"
- 例: `school` が "丙系" なら、魔法カテゴリは "丙系魔法"

#### 対象
**`対象.kind == "個別"` のとき:**
- 出力: `対象.個別` をそのまま
- 例: `対象.個別` が "1体全" なら "1体全" と出力

**`対象.kind == "エリア"` のとき:**
- 出力形式: `対象.エリア.value`(半径`対象.エリア.半径(m)`m`対象.エリア.末尾`)
- 例: `対象.エリア.value` が "2エリア"、`対象.エリア.半径(m)` が "10"、`対象.エリア.末尾` が "空間" なら "2エリア(半径10m空間)" と出力

#### 時間
**`時間.value` が文字列のとき:**
- 出力: `時間.value` をそのまま
- 例: `時間.value` が "一瞬" なら "一瞬" と出力

**`時間.value` が整数のとき:**
- 出力: `時間.value` + `時間.unit`
- 例: `時間.value` が 3、`時間.unit` が "年" なら "3年" と出力

---

## Google Sheets エクスポート形式

### 設計の原則
- **テキストのみ**: 書式、セルの数式、スタイルは使わない
- **セルの結合**: 既存のセルの結合を保持する。結合のパターンは混在している（縦方向のものも横方向のものもある）
- **改行**: 仕様では `\n` で表すが、実際のセルの内容では本物の改行を使う
- **言語**: 出力するフィールド名はすべて日本語
- **行の構成**: 部位ごとに2行（部位の A 行 + 部位の B 行）
- **複数部位**: n 部位 × 各2行を出力する
  - 例: 部位が3つ（部位A, 部位B, 部位B）のモンスターA → 2行 + 2行 + 2行 = 合計6行
  - 例: 部位が1つのモンスター → 合計2行
- **部位のデータが無い場合**: `HP`、`命中力`、`回避力` の値がなければ、そのセルは空のままにする
- **負の値**: `MP` の負の値と移動の `-1` だけを "-" に変換する
- **出力先**: "search" シートのみ
- **挿入位置**: A 列の3〜1000行目を読み、最後に値がある行より後の最初の奇数行から挿入する（途中の空き行は使わない）
- **エラー処理**: 今は空きがなくてもエラーにならず、1001 行目に書き込む（T060 で直す予定）
- **弱点フィールドの変換**: `弱点` の列で "エネルギー" を "E"、"ダメージ" を "ダメ" に置き換え、"属性" を削除する
- **`\n()` の扱い**: `\n()` の中身が空なら、`\n()` 全体を省く

### 行の出力構造
**モンスターの `part` 配列の各部位について:**

1. **1行目（出力の奇数行）**:
   - 部位ごとのデータの列: 値を出力する
   - 共通のデータ（`name`、`共通特殊能力` など）: 値を出力する
   - 例: 最初の部位なら3行目

2. **2行目（出力の偶数行）**:
   - 部位ごとの特殊能力（`部位特殊能力`）を出力する
   - フィールド変換を適用した弱点のデバフ（`弱点`）を出力する
   - 上の2つの値だけを出力する
   - 例: 最初の部位なら4行目

**モンスターAの例（2部位: 部位A + 部位B(x2)）:**
- 3行目: 1つ目の部位（部位A）のデータ + 共通のデータ
- 4行目: 1つ目の部位（部位A）の部位ごとの特殊能力 + 弱点
- 5行目: 2つ目の部位（部位B）のデータ + 共通のデータ
- 6行目: 2つ目の部位（部位B）の部位ごとの特殊能力 + 弱点
- 7行目: 2つ目の部位（部位B）のデータ + 共通のデータ
- 8行目: 2つ目の部位（部位B）の部位ごとの特殊能力 + 弱点
- 共通のデータのフィールドはすべての奇数行（3, 5, 7）で繰り返す

### 列の対応

| セル | 出力するデータ | 備考 |
|------|------------|-------|
| A | `name` と `part.name` | 形式: `part.name` が空なら `name`、あれば `name\n(part.name)`。`コア==true` なら先頭に "★" を付ける |
| L | `part.HP` | 部位の HP |
| P | `part.MP` | 部位の MP。負の値または -1 なら "-" |
| R | `part.防護点` | 部位の防護点 |
| T | `先制値` | 先制値。最初の部位以外は "-" |
| V | `生命抵抗力` | 生命抵抗力。最初の部位以外は "-" |
| X | `精神抵抗力` | 精神抵抗力。最初の部位以外は "-" |
| Z | "3" | 固定の定数（ルール上の制約） |
| AB | `moveon\n(moveon_des)` | 地上の移動速度と説明。`moveon==-1` なら "-" |
| AD | `movein\n(movein_des)` | 地上以外の移動速度と説明（飛行・水中・土中など、種類はモンスターによる）。`movein==-1` なら "-" |
| AF | `part.命中力` | 部位の命中力 |
| AH | `part.回避力` | 部位の回避力 |
| AJ | `data` | ルールブックの参照ページ |
| AM（奇数行） | `part.打撃点` と `共通特殊能力` | `2d+打撃点、共通特殊能力` の形（例: `2d+6、飛行`）。打撃点が 0 なら `2d`、打撃点がなければ共通特殊能力だけ。同じモンスターのすべての奇数行で繰り返す |
| AM+1（偶数行） | `part.部位特殊能力` | 部位ごとの特殊能力 |
| AW（奇数行） | `知名度 / 弱点値` | 知名度 / 弱点値。最初の部位以外は "-/-" |
| AW+1（偶数行） | `弱点`（変換後） | フィールド変換を適用した弱点のデバフ。最初の部位以外は "-" |

### セル結合の構造

**縦と横の両方向に結合されたセル（行と列をまたぐ）:**
- 複数の小さなグリッドセルが、行をまたいで縦に、列をまたいで横に結合されている（3〜4行目の例: A3:G4, H3:I4, J3:K4, N3:O4, P3:Q4, R3:S4, T3:U4, V3:W4, X3:Y4, Z3:AA4, AB3:AC4, AD3:AE4, AF3:AG4, AH3:AI4, AJ3:AL4）
   - **データの配置**: 奇数行（各部位の1行目）のセルだけに書き込む
   - **偶数行**: 完全に空のままにする。セルの結合により、上の奇数行から自動的に広がる
   - Google Sheets API へ出力するとき、これらの列の偶数行のセルには何も書き込まない

**横方向にだけ結合されたセル（列のみをまたぐ）:**
- 結合のパターンは括弧内のとおり（3〜4行目の例: AM3:AV3, AM4:AV4, AW3:AX3, AW4:AX4）
   - AM 列（`共通特殊能力`）: 対象の奇数行だけに書き込む（複数部位なら、すべての行に同じ値をコピーする）
   - AM+1 列（`part.部位特殊能力`）: 対象の偶数行だけに書き込む（複数部位なら、すべての行に同じ値をコピーするが、値は部位ごとに異なる）
   - AW 列（`知名度 / 弱点値`）: 対象の行のうち最初の部位についてだけ（奇数行へ）書き込む。他の奇数行は "-/-" で埋める
   - AW+1 列（`弱点`）: 対象の行のうち最初の部位についてだけ（偶数行へ）書き込む。他の偶数行は "-" で埋める

**実装上の注意:**
Google Sheets API でデータを挿入するとき:
1. 縦と横の両方向に結合されたセル（A-AJ）: 指定の奇数行だけにデータを書き込む。偶数行や指定外の奇数行には書き込まない
2. 横方向にだけ結合されたセル（AM-AW+1）: 指定の奇数行と偶数行にそれぞれ別のデータを出力する。指定外の部分には書き込まない
3. 既存のセル結合の書式を保持する。API の呼び出しは結合の構造を維持しなければならない

### フィールドの変換規則

1. **`part.name` の連結**:
   - `part.name` が空 → `name` だけを出力する
   - `part.name` に値がある → `name\n(part.name)` を出力する
   - `コア == true` なら先頭に "★" を付ける

2. **移動のフィールド**（`moveon`、`movein`）:
   - 値が -1 → "-" を出力する
   - それ以外 → `value\n(description)` の形で出力する
   - description が空 → `\n()` の部分を省く

3. **弱点のフィールド**:
   - 文字列を置換する: "エネルギー" → "E"、"ダメージ" → "ダメ"
   - 文字列を削除する: "属性"
   - 例: "炎属性ダメージ+2" → "炎ダメ+2"

4. **行をまたぐ共通のデータ**:
   - `name`、`共通特殊能力` の列は、同じモンスターのすべての奇数行で繰り返す
   - 例: モンスターA（2部位、6行）では、`name` と `共通特殊能力` の列が 3, 5, 7 行目に現れる

5. **部位ごとのデータ**:
   - `part.HP`、`part.MP`、`part.防護点` など → 別の部位に移るときだけ変わる
   - 最初の部位以外: 単一部位用の共通フィールドを "-" に置き換える
   - 例外: `name` と `共通特殊能力` は常に実際の値を出力する（"-" にしない）

### 例: モンスターA（複数部位）

**JSON データ:**
```json
{
  "name": "モンスターA",
  "先制値": 10,
  "生命抵抗力": 9,
  "精神抵抗力": 8,
  "共通特殊能力": "共通能力の例",
  "知名度": 12,
  "弱点値": 15,
  "弱点": "炎属性ダメージ+1",
  "moveon": -1,
  "moveon_des": "",
  "movein": -1,
  "movein_des": "",
  "data": "SAMPLE",
  "part": [
    {
      "name": "部位A",
      "コア": true,
      "HP": 50,
      "MP": 20,
      "命中力": 12,
      "回避力": 11,
      "部位特殊能力": "能力A",
      "防護点": 4,
      "部位数": 1
    },
    {
      "name": "部位B",
      "コア": false,
      "HP": 30,
      "MP": 10,
      "命中力": 10,
      "回避力": 9,
      "部位特殊能力": "能力B",
      "防護点": 2,
      "部位数": 2
    }
  ]
}
```

**期待される出力（3〜8行目）:**

| 行 | A | L | P | R | T | V | X | Z | AB | AD | AF | AH | AJ | AM | AW |
|-----|---|---|---|---|---|---|---|---|----|----|----|----|----|----|------|
| 3 | ★モンスターA\n(部位A) | 50 | 20 | 4 | 10 | 9 | 8 | 3 | - | - | 12 | 11 | SAMPLE | 共通能力の例 | 12/15 |
| 4 |||||||||||||| 能力A | 炎ダメ+1 |
| 5 | モンスターA\n(部位B) | 30 | 10 | 2 | - | - | - | 3 | - | - | 10 | 9 | SAMPLE | 共通能力の例 | -/- |
| 6 |||||||||||||| 能力B | - |
| 7 | モンスターA\n(部位B) | 30 | 10 | 2 | - | - | - | 3 | - | - | 10 | 9 | SAMPLE | 共通能力の例 | -/- |
| 8 |||||||||||||| 能力B | - |

**注意点:**
- 3〜4行目: 1つ目の部位（部位A、コア=true）→ 先頭に "★" を付け、共通の値を出力し、すべてのフィールドを埋める
- 5〜8行目: 2つ目の部位（部位B、コア=false）→ "★" は付けず、`先制値`/`生命抵抗力`/`精神抵抗力`/`弱点` は "-"、`知名度`/`弱点値` は "-/-"
