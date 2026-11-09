# TRPG-JSON レポジトリ

TRPG（テーブルトーク・ロールプレイングゲーム）のゲームシステム向けの、JSON ベースのデータ管理システムです。

## 概要

このレポジトリは、TRPG のゲームデータ（モンスター、キャラクター、アイテムなど）を JSON 形式で管理・検索するためのツールを提供します。ゲームデータを検索・絞り込みするコマンドラインインターフェースを含みます。

## インストール

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install) 1.85 以降

### インストール手順

```bash
# 方法1: レポジトリのルートから
cargo install --path rust/app

# 方法2: rust/app ディレクトリから
cd rust/app
cargo install --path .
```

`gm` コマンドは Cargo の bin ディレクトリ（通常は `~/.cargo/bin`）にインストールされます。このディレクトリが `PATH` に含まれていることを確認してください。

## 使い方

JSON 形式の TRPG ゲームデータを管理するコマンドラインツールです。

### モンスターの基本コマンド

```bash
# 名前でモンスターを検索
gm monster find "monster name"

# パターンに一致するモンスターを一覧表示
gm monster list "pattern"

# フィルタを指定して検索
gm monster select -l 6 -c "Category"

# モンスターを追加
gm monster add monster.json

# モンスターを削除
gm monster delete "monster name"
```

データファイルを複数設定している場合の保存先:
- `add`（新規）: 設定の最初のファイルの末尾に追加します
- `add`（同名あり）: 確認のうえ、そのモンスターを含むファイルだけを書き換えます（位置はそのまま）
- `delete`: 確認のうえ、そのモンスターを含むファイルだけを書き換えます。同名が複数のファイルにあるときは、確認メッセージにファイルを列挙し、すべてから削除します

### スペルの管理

スペルデータを検索・操作するためのスペル管理機能を備えています。

#### スペルの検索コマンド

```bash
# 名前でスペルを検索
gm spell find "spell name"

# レベルで検索
gm spell find "spell name" -l 5

# ランクで検索（ランク制の魔法システム向け）
gm spell find "spell name" -r 3

# 系統で検索
gm spell find "spell name" -s "甲系"

# schoolVariant で検索
gm spell find "spell name" -v "特殊"

# 神で検索（神聖魔法向け）
gm spell find "spell name" -g "神名"

# 複数のフィルタを組み合わせる
gm spell find "magic" -l 3 -s "甲系"
gm spell find "甲系" -v "特殊" -g "神名"

# パターンに一致するスペルを一覧表示
gm spell list "pattern"

# スペルのチャットパレットを生成（複数のフィルタに対応）
gm spell palette -n "spell name"
gm spell palette -l 5              # レベル5のスペルすべて
gm spell palette -r 3              # ランク3のスペルすべて
gm spell palette -s "甲系"         # 指定した系統のスペルすべて
gm spell palette -v "特殊"         # 特殊バリアントのスペルすべて
gm spell palette -g "神名"         # 指定した神のスペルすべて

# フィルタを組み合わせて対象を絞ってパレットを生成
gm spell palette -n "magic" -s "甲系" -l 5
gm spell palette -v "特殊" -g "神名"

# スペルのパレットをクリップボードへコピー（macOS/Linux/Windows）
gm spell palette -n "spell name" -y
# または
gm spell palette -n "spell name" --copy
```

**フィルタのオプション:**
- `-n`: スペル名（部分一致）
- `-l`: レベル（完全一致。`Lv.kind: "value"` または `"value+"` が対象）
- `-r`: ランク（完全一致。`Lv.kind: "rank"` が対象）
- `-s`: 系統 / カテゴリ（完全一致）
- `-v` / `--school-variant`: 系統のバリアント（完全一致）
- `-g` / `--god`: 神の名前（完全一致）
- `-y` / `--copy`: 最初の結果をクリップボードへコピー

**注意:** レベル（`-l`）とランク（`-r`）のフィルタは同時に使えません。両方を指定した場合はレベルが優先されます。

#### スペルのパレットの出力形式

`gm spell palette` コマンドは、TRPG のチャットシステムで使える形式のスペルデータを生成します。

**補助スペル**（`補助=true`）:
```
spell name / MP:cost / 対象:target / 射程:range / 時間:duration / effect description
```

**通常のスペル**（`補助=false`）:
```
2d+{magic_category}+{行使修正}  spell name / MP:cost / 対象:target / 射程:range / 時間:duration / effect description
```

**例:**

```bash
# 補助スペルのパレットを生成
gm spell palette -n "補助呪文名"
# 出力: 補助呪文名 / MP:2 / 対象:1体 / 射程:術者 / 時間:10秒 / 効果の説明。

# 通常のスペルのパレットを生成（クリップボードへコピー）
gm spell palette -n "呪文名" -y
# 出力: 2d+{甲系魔法}+{行使修正}  呪文名 / MP:15 / 対象:1エリア(半径4m) / 射程:術者 / 時間:一瞬 / ...
```

#### 魔法カテゴリの整形

魔法カテゴリは自動的に整形されます。
- 2文字のカテゴリ（例: "甲系"、"丙系"）は "甲系魔法"、"丙系魔法" になります
- それより長いカテゴリ（例: "ハイテクノロジー"）は変更されません
- `{行使修正}` のプレースホルダは、ダイスボットでの置換のためにそのまま残ります

## TUI モード（`gm tui`）

キー操作だけで「検索 → 確認 → コピー」を行える対話画面です。既存の CLI サブコマンドはそのまま使えます。
設計の詳細は [TUI_DESIGN.md](TUI_DESIGN.md) を参照してください。

```bash
gm tui                       # モンスタータブで起動
gm tui spell                 # スペルタブで起動
gm tui --config path.toml    # 設定ファイルを指定（データパスは CLI と共通）
```

- 端末（TTY）上でのみ起動します（パイプ・リダイレクト時はエラー終了）
- 画面: ヘッダー（タブ・件数）、検索バー、結果リスト、詳細ペイン、ステータス行、キーヘルプ行
- 検索バーは入力のたびに絞り込みます（空なら全件）。書き方は下の「検索構文」を参照
- 幅 100 桁未満の端末では結果リストと詳細を1ペインずつ表示し、`Enter`（詳細へ）/ `Esc`（リストへ）で切り替えます
- 高さ 15 行未満の端末では「端末が小さすぎます」と表示します

### 主なキー

| キー | 動作 |
|------|------|
| `1` / `2` / `←` `→` | モンスター / スペルのタブへ切り替え |
| `/` | 検索バーへ（入力中は英字も文字として入力。`Enter`/`Esc` でリストへ、`Ctrl-u` でクリア。入力の編集は下記） |
| `↑↓` / `j` `k` | 選択の移動（モンスター詳細では部位の選択、スペル詳細では本文のスクロール） |
| `PgUp` `PgDn` / `g` `G` | ページ移動 / 先頭・末尾へ |
| `Tab` / `Shift-Tab` | ペイン移動（検索 → 結果 → 詳細） |
| `Enter` / `Esc` | 結果リスト → 詳細へ / 詳細 → 結果リストへ |
| `c` | モンスター: 選択中の部位をココフォリア形式でクリップボードへコピー |
| `y` / `Y` | スペル: 選択中 / ヒット全件のチャットパレットをコピー |
| `e` | モンスター: ヒット中の全件をファイルへエクスポート（下記） |
| `Space` | 結果リストのカーソル行を選択リストへ追加 / 外す（下記） |
| `v` | 選択リストのペインへ移動 / 結果リストへ戻る |
| `C` | スペル: 選択リストの全件のチャットパレットをコピー |
| `E` | モンスター: 選択リストの全件をエクスポート |
| `J` | 選択中の項目の JSON を表示（表示中に `y` でコピー） |
| `S` | 統計（CLI の `stats` と同じ集計）。検索で絞り込み中はヒット中の統計を先に、続けて全件の統計を表示 |
| `R` | 設定のデータファイルを読み直す（CLI の `add` / `delete` やエディタでの変更を反映）。検索語・タブ・選択中の項目は保つ。読み込みに失敗したら今のデータのままにして、ステータス行にエラーを表示 |
| `?` | ヘルプ（全キーバインドと検索構文） |
| `q` / `Ctrl-c` | 終了（ヘルプ・JSON・統計の表示中の `q` / `Esc` は閉じるだけ） |

### 入力欄の編集（検索バー・エクスポートの出力先）

| キー | 動作 |
|------|------|
| 文字入力 | カーソル位置に挿入 |
| `←` `→` | カーソルを1文字移動（全角も1文字） |
| `Home` `End` / `Ctrl-a` `Ctrl-e` | カーソルを先頭 / 末尾へ |
| `Backspace` / `Delete` | カーソルの前 / カーソル位置の1文字を削除 |
| `Ctrl-u` | 入力をクリア |

枠に収まらない長い入力は、カーソルが見える位置まで横にずらして表示します（省いた側に `<` / `>` を表示）。検索バーにフォーカスがある間の `←` `→` はタブ切り替えではなくカーソル移動です。

### エクスポート（`e`）

モンスタータブで `e` を押すと、ヒット中の全件（結果リストの並び順）を書き出すモーダルが開きます。

- 形式: `Tab` / `↑↓` で JSON / Udonarium (ZIP) / Google Sheets を切り替え（出力先が `.json` / `.zip` で終わっていれば拡張子も付け替え）
- 出力先: そのまま文字入力（編集のキーは上の「入力欄の編集」と同じ。`←` `→` はカーソル移動で、形式の切り替えには使いません）。相対パスは `gm` を起動したディレクトリが基準、先頭の `~/` はホームディレクトリ
- `Enter` で出力、`Esc` でキャンセル。結果はステータス行に表示。同じ名前のファイルは上書きされます（CLI と同じ）
- 次に開いたときは前回の形式と出力先（スプレッドシート ID）が入っています
- Google Sheets: 出力先の代わりにスプレッドシート ID を入力します。`Enter` で TUI を一時停止して通常の画面で実行し（認証が必要ならブラウザが開きます）、結果を表示したあと `Enter` で TUI に戻ります。認証の設定は CLI の `--export sheets` と同じです（下の「Google Sheets の設定」を参照）
- スペルのエクスポートは未対応です
- モンスターの追加・削除は TUI では行いません。CLI の `gm monster add` / `gm monster delete` を使ってください

### 選択リスト（`Space`）

別々の検索で見つけた項目を1つのリストに集め、まとめてコピーやエクスポートができます。

- 結果リストの `Space` で追加・解除します。選択済みの行には `✓` が付きます
- タブごと（モンスター / スペル）に追加順で保持します。検索条件を変えても残り、TUI を終了すると消えます
- 項目があると、幅 120 桁以上の端末では右端にペインが出ます。それ未満では `v` で表示を切り替えます
- ペインでは `↑↓` / `jk` で移動し、`x` で外します。`v` / `Esc` で結果リストへ戻ります
- `C`（スペル）: 全件のチャットパレットを追加順に改行区切りでコピーします。`Y` はヒット全件のコピーのままです
- `E`（モンスター）: 全件を追加順にエクスポートします。形式と操作は `e` と同じです。ココフォリアは1回に1体しか貼り付けられないため、これまでどおり `c` を使います
- `R` で読み直すと名前で引き直します。データから消えた項目はリストから外し、件数をステータス行に表示します

### 検索構文

フリーワードと `キー:値` を空白区切りで混在できます（CLI のオプションと対応）。

| タブ | 書き方 | 意味（対応する CLI） |
|------|--------|---------------------|
| 共通 | フリーワード | 名前の部分一致（`-n`）。複数あればすべてを含むもの |
| 共通 | `lv:<n>` | レベルが n（`-l`）。スペルのランク表記は対象外 |
| モンスター | `cat:<s>` / `c:<s>` | カテゴリが s（`-c`） |
| スペル | `school:<s>` / `s:<s>` | 系統が s（`-s`） |

```
テスト lv:6 cat:蛮族      # 名前に「テスト」を含む Lv6 の蛮族
lv:3 s:MagicCat_1         # 系統 MagicCat_1 の Lv3 のスペル
```

- キーの大文字小文字は区別せず、区切りは全角 `：` も使えます
- 解釈できない条件（`lv:abc`、値の無い `lv:`、タブで使えないキー、同じキーの重複）は検索バーを黄色にして理由を表示し、直前の結果を残します

## エクスポート機能

共同作業やデータの共有のために、TRPG のデータを複数の形式へエクスポートできます。

### 対応するエクスポート形式

- **JSON**: データの受け渡しや他ツールとの連携に使える JSON 形式
- **Google Sheets**: 共同編集や共有のために Google Sheets へ直接エクスポート
- **Udonarium**: TRPG ツールの Udonarium で使えるキャラクター形式（XML を含む ZIP ファイル）へエクスポート
- **CCFolia（ココフォリア）**: モンスターを CCFolia のキャラクターコマとしてクリップボードへコピー。CCFolia のルームに貼り付けると追加できます

### モンスターのエクスポートコマンド

`monster select` コマンドに `--export` と `--output` のフラグを付けて、モンスターのデータをエクスポートします。

```bash
# 検索結果を JSON ファイルへエクスポート
gm monster select -l 6 --export json --output results.json

# 検索結果を Google Sheets へエクスポート
gm monster select -l 6 -c "Category" --export sheets --output <spreadsheet-id>

# Udonarium 形式（XML を含む ZIP ファイル）へエクスポート
gm monster select -l 6 --export udonarium --output monsters.zip

# モンスター1体を JSON へエクスポート
gm monster select -n "monster name" --export json --output monster.json

# モンスター1体を Udonarium 形式へエクスポート
gm monster select -n "monster name" --export udonarium --output monster.zip

# モンスター1体を CCFolia 形式でクリップボードへコピー
gm monster select -n "monster name" --export ccfolia
```

### CCFolia（ココフォリア）へのエクスポート

`--export ccfolia` は、モンスター1体を CCFolia のクリップボード用キャラクター形式
（`{"kind":"character","data":{...}}`）に変換して、クリップボードへコピーします
（macOS: `pbcopy`、Linux: `xclip`、Windows: `clip`）。CCFolia のルームを開いて貼り付ける（Ctrl+V / Cmd+V）とコマが作られます。

- **貼り付け1回につき1キャラクター**: 検索結果がちょうど1体になる必要があります（名前の完全一致を優先します）
- **複数部位のモンスター**: 既定では最初のコア部位をコピーします。別の部位を選ぶには `--part <n>`（1 始まり）を使います。部位の一覧は stderr に出力されます
- **status**: HP / MP / 防護点（MP の `-1` は `0` になります）
- **params**: 命中力 / 回避力 / 生命抵抗力 / 精神抵抗力（いずれも `-7`、Udonarium と同じ）、打撃点、ダメージ軽減（`0`）
- **memo**: カテゴリとレベル、知名度/弱点値、先制値、弱点、移動、特殊能力（知識の情報はコア部位のみ）
- **commands**: Udonarium へのエクスポートと同じチャットパレット
- `--output <file>` を付けると、クリップボードの代わりにファイルへ JSON を書き出します

```bash
gm monster select -n "モンスター名" --export ccfolia            # コア部位をコピー
gm monster select -n "モンスター名" --export ccfolia --part 2   # 2番目の部位をコピー
gm monster select -n "モンスター名" --export ccfolia --output piece.json
```

### Google Sheets の設定

Google Sheets へのエクスポートを使うには、Google OAuth 2.0 の認証を設定する必要があります。

#### 1. Google Cloud プロジェクトの準備

1. [Google Cloud Console](https://console.cloud.google.com/) を開きます
2. 新しいプロジェクトを作成するか、既存のプロジェクトを選択します
3. **Google Sheets API** を有効にします
4. OAuth 2.0 の認証情報（デスクトップアプリケーション）を作成します
5. 認証情報の JSON ファイルをダウンロードします

#### 2. 認証の設定

OAuth の設定方法は2つあります。

**方法 A: 環境変数（推奨）**

```bash
export GOOGLE_CLIENT_ID="your-client-id.apps.googleusercontent.com"
export GOOGLE_CLIENT_SECRET="your-client-secret"
export GOOGLE_REDIRECT_URI="http://localhost:8080/callback"  # 省略可。省略時はこの値
```

**方法 B: 設定ファイル**

`~/.config/trpg-json/oauth_config.json` を作成します。

```json
{
  "client_id": "your-client-id.apps.googleusercontent.com",
  "client_secret": "your-client-secret",
  "redirect_uri": "http://localhost:8080/callback",
  "scopes": ["https://www.googleapis.com/auth/spreadsheets"]
}
```

#### 3. 認証の流れ

Google Sheets へ初めてエクスポートするときの流れです。

1. ブラウザで Google の OAuth 同意画面が開きます
2. アプリケーションが Google Sheets にアクセスすることを許可します
3. `http://localhost:8080/callback` へリダイレクトされます
4. ツールが認可コードを自動で受け取り、認証情報を保存します
5. 指定したスプレッドシートへデータがエクスポートされます

認証情報は自動で保存され、次回以降のエクスポートで再利用されます。保存先は `~/.config/trpg-json/credentials.json` で、安全に保管されます。

### Udonarium へのエクスポート

TRPG-JSON のモンスターデータを Udonarium のキャラクター XML 形式に変換し、インポートしやすいよう ZIP ファイルにまとめます。

#### 仕組み

1. **部位が1つのモンスター**: XML ファイルを1つ含む ZIP ファイルを作成します
   - 例: `gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip`
   - 作成されるもの: `ゴブリン.xml` を含む `ゴブリン.zip`

2. **複数部位のモンスター**: 部位ごとの XML ファイルを含む ZIP ファイルを作成します
   - 例: `gm monster select -n "モンスターA" --export udonarium --output モンスターA.zip`
   - 作成されるもの: 次のファイルを含む `モンスターA.zip`
     - `モンスターA_部位A.xml`（全ステータスを持つコア部位）
     - `モンスターA_部位B0.xml`（コア以外の部位）
     - `モンスターA_部位B1.xml`（コア以外の部位）

#### 機能

- **XML の自動生成**: モンスターのステータスを Udonarium の XML 形式に変換します
- **チャットパレット**: 戦闘の判定に使うダイスロールのコマンドを自動生成します
- **部位の処理**: コア部位とコア以外の部位を正しく扱います
- **ファイルの整理**: 複数の部位を自動で ZIP アーカイブにまとめます

#### 使用例

```bash
# モンスター1体を Udonarium 形式へエクスポート
gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip

# レベル6のモンスターをすべてエクスポート
gm monster select -l 6 --export udonarium --output level6_monsters.zip

# カテゴリを指定してモンスターをエクスポート
gm monster select -c "蛮族" --export udonarium --output barbarians.zip
```

### エクスポートの例のまとめ

```bash
# JSON へのエクスポート
gm monster select -l 6 --export json --output level6_monsters.json
gm monster select -c "蛮族" --export json --output barbarians.json

# Google Sheets へのエクスポート
gm monster select -l 2 -c "動物" --export sheets --output <spreadsheet-id>
gm monster select -l 6 -c "蛮族" --export sheets --output <spreadsheet-id>

# Udonarium へのエクスポート
gm monster select -l 6 --export udonarium --output level6_monsters.zip
gm monster select -c "蛮族" --export udonarium --output barbarians.zip
gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip
```
