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

## 状況と次の手順

### 現在の実装状況（2025-11-30 時点）
- ✅ **Phase 1**: 単一システム、基本的な CRUD 操作
- ✅ **Phase 2**: 複数データファイル、設定ファイルへの対応、ホームディレクトリのパス
- ⏳ **Phase 2.5**: 外部へのデータエクスポート機能（次の優先事項）
- ⏳ **Phase 3**: システムの抽象化（Phase 2.5 の後に予定）
- 📌 **将来**: SQLite バックエンド（任意）、データ形式の柔軟性

### 決定事項
- **優先順位**: ファイル（Phase 2）✓ → データエクスポート（Phase 2.5）→ システム（Phase 3）
- **設定の形式**: TOML（単純さのため YAML ではなくこちらを選択）
- **パスの解決**: ホームディレクトリからの相対パス（`~` の展開に対応）
- **データの統合**: すべてのファイルを1つのコレクションに読み込み、まとめて検索する
- **エクスポート形式**: JSON と Google Sheets に対応
- **手動実行**: データのエクスポートはユーザーが実行し、自動化は不要
- **Phase 4**: 予定なし（範囲は中核の機能に限定）
- **拡張性**: 新しいゲームシステムやエクスポート形式を追加できる設計

### 今後の実装に向けた検討事項
- **Phase 3 の時期**: SW2.5 の対応範囲が安定したら開始する（今回または次のマイルストーン）
- **データベースの選択**: ローカルは SQLite（優先）、リモートは PostgreSQL（将来）
- **キャッシュの戦略**: API から取得するデータには Redis を検討する
- **複数システムの設定**: システムごとに1ファイルか、統合した設定か

### 推奨する次の手順（優先順）
1. **Phase 2.5（工数は中、価値は高い）** - 外部へのデータエクスポート機能
2. **Phase 3（工数は中、価値は高い）** - ゲームシステムを追加するためのシステムの抽象化
3. **検証の強化** - システムごとのより厳密な JSON スキーマの検証
4. **性能の最適化** - 大きなデータセット（1万件を超えるエンティティ）向け
5. **ドキュメント** - 包括的なユーザーガイドと例


## Udonarium エクスポート形式

### 設計の原則
- **出力形式**: すべてのモンスター（部位が1つのものも複数のものも）について、XML ファイルを含む ZIP ファイル
- **CLI のフラグ**: `--export udonarium --output <zip_file_path>` で ZIP の出力先を指定する
- **ファイル名（ZIP の中）**: 個々の XML ファイルは次の命名規則に従う
  - 部位が1つ: `(monster_name).xml`
  - 複数部位: 部位ごとに `(monster_name)_(part_name)(no).xml`
  - 例:
    - 部位が1つ: `ゴブリン.zip` の中に `ゴブリン.xml`
    - 複数部位: `トレント.zip` の中に `トレント_幹.xml`、`トレント_根0.xml`、`トレント_根1.xml`
- **テンプレートの選択**: すべての部位（コア=true と コア=false の両方）でコアのテンプレートを使う
- **XML の構造**: ネストした `<data>` 要素を持つ Udonarium のキャラクター形式に基づく
- **チャットパレット**: 命中、回避、抵抗の判定だけを自動生成する（特殊能力は含めない）
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
      <data name="移動速度"></data>
    </data>
    <data name="魔物知識">
      <data name="生態" type="note"><!-- カテゴリ、レベル --></data>
    </data>
  </data>
</data>

<chat-palette dicebot="SwordWorld2.5">
<!-- 自動生成するコマンド: 命中力,打撃点, 回避力, 生命抵抗力, 精神抵抗力 -->
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
| `part.hit_rate` | `part.命中力` | `character/detail/ステータス・バフ・デバフ/命中力` | hit_rate - 7 | 7 を引く（期待値から基準値へ） |
| `part.damage` | `part.打撃点` | `character/detail/ステータス・バフ・デバフ/打撃点` | 数値 | 補正は不要（すでに基準値） |
| `part.dodge` | `part.回避力` | `character/detail/ステータス・バフ・デバフ/回避力` | dodge - 7 | 7 を引く（期待値から基準値へ） |
| `monster.life_resistance` | `生命抵抗力` | `character/detail/ステータス・バフ・デバフ/生命抵抗力` | life_resistance - 7 | 7 を引く。コア部位のみ |
| `monster.mental_resistance` | `精神抵抗力` | `character/detail/ステータス・バフ・デバフ/精神抵抗力` | mental_resistance - 7 | 7 を引く。コア部位のみ |
| `monster.fame` | `知名度` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\r先制値` | **コア部位のみ** |
| `monster.weakness_value` | `弱点値` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\r先制値` | **コア部位のみ** |
| `monster.weakness` | `弱点` | `character/detail/情報/弱点` | 文字列 | **コア部位のみ** |
| `monster.initiative` | `先制値` | `character/detail/戦闘準備/魔物知識・先制判定` | 形式: `知名度/弱点値\r先制値` | **コア部位のみ** |
| `monster.common_abilities` | `共通特殊能力` | `character/detail/特殊能力/特殊能力1` | テキスト | 特殊能力の1つ目の枠 |
| `part.special_abilities` | `part.部位特殊能力` | `character/detail/特殊能力/特殊能力2` | テキスト | 部位ごとの能力 |

### チャットパレットの自動生成

**生成するコマンド（命中、回避、抵抗の判定のみ）:**
```
2d+{命中力}　命中判定
2d+{回避力}　回避判定
2d+{打撃点}　ダメージロール
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
gm export --export udonarium --output <zip_file_path>
# 例:
gm export --export udonarium --output ゴブリン.zip
gm export --export udonarium --output /path/to/monsters.zip
```

**部位が1つのモンスター:**
- 出力形式: XML ファイルを1つ含む ZIP ファイル
- ZIP の中のファイル名: `(monster_name).xml`
- 例: `--output ゴブリン.zip` で `ゴブリン.xml` を含む ZIP ファイルを作成する

**複数部位のモンスター（コア部位が複数あるものを含む）:**
- 出力形式: 複数の XML ファイルを含む ZIP ファイル
- ZIP の中のファイル名の規則: `(monster_name)_(part_name)(no).xml`
- 例: `--output トレント.zip` で次のファイルを含む ZIP ファイルを作成する
  ```
  トレント_幹.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  トレント_根0.xml (コア以外の部位)
  トレント_根1.xml (コア以外の部位)
  ```
- コア部位が複数ある例: `--output アンシェント・ドラゴン.zip` で次のファイルを作成する
  ```
  アンシェント・ドラゴン_頭0.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  アンシェント・ドラゴン_頭1.xml (コア部位 - 戦闘準備 情報 魔物知識 を含む)
  アンシェント・ドラゴン_防護膜.xml (コア以外の部位)
  ```

### 部位の命名アルゴリズム

1. `part.name` が空なら、コア=true には "core" を使い、それ以外には連番を使う
2. `part.name` があれば、その名前を使う
3. 名前が重複する場合（例: "根" の部位が複数ある）は、連番（0, 1, 2...）を付ける

**例:**
- `name=""` で `コア=true` の部位 → `monster.xml`
- `name=""` で `コア=false` の部位（最初のもの）→ `monster_0.xml`
- `name="幹"` で `コア=true` の部位 → `monster_幹.xml`
- `name="根"` の部位 → `monster_根0.xml`、`monster_根1.xml`

### 実装上の注意

- 位置の属性（location.x、location.y）は既定で "0" にする
- その他のキャラクター属性（posZ、rotate、roll）は "0" にする
- ダイスボットは "SwordWorld2.5" にする
- numberResource 要素では、利便性のために `currentValue` 属性を使う
- データの変換では、必要に応じて空や null の値を "-" として扱う

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
  検索: `gm spell find -l 3` → このレコードに一致する

**2. `Lv.kind: "value+"`（最低レベル）**
- **CLI のフラグ**: `-l <level>`
- **検索の動作**: `Lv.value+` が指定のレベル以下であるレコードを返す
- **用途**: 最低レベル以上で使えるスキル（例: 「レベル5以上で使えるスキル」）
- **例**:
  ```json
  { "name": "上級剣術", "Lv": { "kind": "value+", "value+": 5 } }
  ```
  検索: `gm skill find -l 7` → 一致する（5 ≤ 7 のため）
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
  検索: `gm spell find -r 2` → このレコードに一致する

### 系統バリアントのフィールド（`schoolVariant`）

**CLI のフラグ**: `-sv <variant>`

**検索の動作**:
- `schoolVariant` が指定の値と完全に一致するレコードを返す（完全一致のみ）
- `-sv` フラグを指定すると、`schoolVariant` フィールドが無い（または値が `null` の）レコードは結果から除外される

**例**:
```json
{ "name": "特殊甲系魔法", "school": "甲系", "schoolVariant": "特殊" }
```
検索: `gm spell find -sv 特殊` → このレコードに一致する

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
検索: `gm spell find -sv 特殊 -g 神名` → このレコードに一致する

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
ライト / MP:3 / 対象:任意の地点 / 射程:10m(起点指定) / 光源を生成する。
魔法解除 / MP:8 / 対象:魔法1つ / 射程:接触 / 魔法を打ち消す。
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
2d+{甲系魔法}+{行使判定} 呪文名 / MP:15 / 対象:1エリア(半径4mすべて) / 射程:術者 / 時間:一瞬 / 対象に効果を与える。
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
- `category`: 魔法カテゴリ名の導出（別の項を参照）
- `効果`: 効果の説明。そのまま出力する
- `時間` のフィールド: 持続時間の情報（別の項を参照）

#### 魔法カテゴリ名
**`category` が全角2文字ちょうどではないとき:**
- 魔法カテゴリ → `category` をそのまま使う
- 例: `category` が "ハイテクノロジー" なら、魔法カテゴリは "ハイテクノロジー"

**`category` が全角2文字ちょうどのとき:**
- 魔法カテゴリ → `category` + "魔法"
- 例: `category` が "丙系" なら、魔法カテゴリは "丙系魔法"

#### 対象
**`対象.kind == "個別"` のとき:**
- 出力: `対象.個別` をそのまま
- 例: `対象.個別` が "1体全" なら "1体全" と出力

**`対象.kind == "エリア"` のとき:**
- 出力形式: `対象.value`(半径`対象.半径(m)`m`対象.末尾`)
- 例: `対象.value` が "2エリア"、`対象.半径(m)` が "10"、`対象.末尾` が "空間" なら "2エリア(半径10m空間)" と出力

#### 時間
**`時間.value` が文字列のとき:**
- 出力: `時間.value` をそのまま
- 例: `時間.value` が "一瞬" なら "一瞬" と出力

**`時間.value` が整数のとき:**
- 出力: `時間.value` + `時間.unit`
- 例: `時間.value` が "3"、`時間.unit` が "年" なら "3年" と出力

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
