# TRPG-JSON Repository

A JSON-based data management system for TRPG (Tabletop Role-Playing Game) game systems.

## Overview

This repository provides tools for managing and querying TRPG game data (such as monsters, characters, and items) in JSON format. It includes a command-line interface for searching and filtering game data.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later

### Install Steps

```bash
# Option 1: From repository root
cargo install --path rust/app

# Option 2: From rust/app directory
cd rust/app
cargo install --path .
```

The `gm` command will be installed to your Cargo bin directory (typically `~/.cargo/bin`). Make sure this directory is in your `PATH`.

## Usage

This is a command-line tool for managing TRPG game data in JSON format.

### Basic Monster Commands

```bash
# Search for a monster by name
gm monster find "monster name"

# List monsters matching a pattern
gm monster list "pattern"

# Query with filters
gm monster select -l 6 -c "Category"

# Add a new monster
gm monster add monster.json

# Delete a monster
gm monster delete "monster name"
```

### Spell Management

The tool includes comprehensive spell management capabilities for querying and manipulating spell data.

#### Spell Search Commands

```bash
# Search for a spell by name
gm spell find "spell name"

# Search by level
gm spell find "spell name" -l 5

# Search by rank (for rank-based magic systems)
gm spell find "spell name" -r 3

# Search by school
gm spell find "spell name" -s "神聖"

# Search by schoolVariant
gm spell find "spell name" -v "特殊"

# Search by god (for divine magic)
gm spell find "spell name" -g "神名"

# Combine multiple filters
gm spell find "magic" -l 3 -s "神聖"
gm spell find "神聖" -v "特殊" -g "神名"

# List spells matching a pattern
gm spell list "pattern"

# Generate chat palette for spells (supports multiple filters)
gm spell palette -n "spell name"
gm spell palette -l 5              # All level 5 spells
gm spell palette -r 3              # All rank 3 spells
gm spell palette -s "神聖"         # All spells of a school
gm spell palette -v "特殊"         # All special variant spells
gm spell palette -g "神名"         # All spells of a deity

# Combine filters for precise palette generation
gm spell palette -n "magic" -s "神聖" -l 5
gm spell palette -v "特殊" -g "神名"

# Copy spell palette to clipboard (macOS/Linux/Windows)
gm spell palette -n "spell name" -y
# or
gm spell palette -n "spell name" --copy
```

**Filter Options:**
- `-n`: Spell name (partial match)
- `-l`: Level (exact match, for `Lv.kind: "value"` or `"value+"`)
- `-r`: Rank (exact match, for `Lv.kind: "rank"`)
- `-s`: School/Category (exact match)
- `-v` / `--school-variant`: School variant (exact match)
- `-g` / `--god`: God/Deity name (exact match)
- `-y` / `--copy`: Copy first result to clipboard

**Note:** Level (`-l`) and rank (`-r`) filters are mutually exclusive. If both are specified, level takes priority.

#### Spell Palette Output Format

The `gm spell palette` command generates formatted spell data for use in TRPG chat systems:

**Support Spells** (`補助=true`):
```
spell name / MP:cost / 対象:target / 射程:range / 時間:duration / effect description
```

**Regular Spells** (`補助=false`):
```
2d+{magic_category}+{行使修正}  spell name / MP:cost / 対象:target / 射程:range / 時間:duration / effect description
```

**Examples:**

```bash
# Generate palette for support spell
gm spell palette "ライト"
# Output: ライト / MP:3 / 対象:任意の地点 / 射程:10m(起点指定) / 時間:一瞬 / 光源を生成する。

# Generate palette for regular spell (with copy to clipboard)
gm spell palette "ゴッド・ジャッジメント" -y
# Output: 2d+{神聖魔法}+{行使修正}  ゴッド・ジャッジメント / MP:15 / 対象:1エリア(半径4m) / 射程:術者 / 時間:一瞬 / ...
```

#### Magic Category Formatting

Magic categories are automatically formatted:
- 2-character categories (e.g., "神聖", "妖精") become "神聖魔法", "妖精魔法"
- Longer categories (e.g., "ハイテクノロジー") remain unchanged
- The `{行使修正}` placeholder is preserved for dicebot substitution

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
| `/` | 検索バーへ（入力中は英字も文字として入力。`Enter`/`Esc` でリストへ、`Ctrl-u` でクリア） |
| `↑↓` / `j` `k` | 選択の移動（モンスター詳細では部位の選択、スペル詳細では本文のスクロール） |
| `PgUp` `PgDn` / `g` `G` | ページ移動 / 先頭・末尾へ |
| `Tab` / `Shift-Tab` | ペイン移動（検索 → 結果 → 詳細） |
| `Enter` / `Esc` | 結果リスト → 詳細へ / 詳細 → 結果リストへ |
| `c` | モンスター: 選択中の部位をココフォリア形式でクリップボードへコピー |
| `y` / `Y` | スペル: 選択中 / ヒット全件のチャットパレットをコピー |
| `e` | モンスター: ヒット中の全件をファイルへエクスポート（下記） |
| `J` | 選択中の項目の JSON を表示（表示中に `y` でコピー） |
| `S` | 統計（CLI の `stats` と同じ集計）。検索で絞り込み中はヒット中の統計を先に、続けて全件の統計を表示 |
| `?` | ヘルプ（全キーバインドと検索構文） |
| `q` / `Ctrl-c` | 終了（ヘルプ・JSON・統計の表示中の `q` / `Esc` は閉じるだけ） |

### エクスポート（`e`）

モンスタータブで `e` を押すと、ヒット中の全件（結果リストの並び順）を書き出すモーダルが開きます。

- 形式: `Tab` / `↑↓` で JSON / Udonarium (ZIP) / Google Sheets を切り替え（出力先が `.json` / `.zip` で終わっていれば拡張子も付け替え）
- 出力先: そのまま文字入力（`Backspace` で1文字削除、`Ctrl-u` でクリア）。相対パスは `gm` を起動したディレクトリが基準、先頭の `~/` はホームディレクトリ
- `Enter` で出力、`Esc` でキャンセル。結果はステータス行に表示。同じ名前のファイルは上書きされます（CLI と同じ）
- 次に開いたときは前回の形式と出力先（スプレッドシート ID）が入っています
- Google Sheets: 出力先の代わりにスプレッドシート ID を入力します。`Enter` で TUI を一時停止して通常の画面で実行し（認証が必要ならブラウザが開きます）、結果を表示したあと `Enter` で TUI に戻ります。認証の設定は CLI の `--export sheets` と同じです（下の「Google Sheets Setup」を参照）
- スペルのエクスポートは未対応です
- モンスターの追加・削除は TUI では行いません。CLI の `gm monster add` / `gm monster delete` を使ってください

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

## Export Features

This tool supports exporting TRPG data to multiple formats for collaboration and data sharing.

### Supported Export Formats

- **JSON**: Native JSON format for data interchange and integration with other tools
- **Google Sheets**: Direct export to Google Sheets for collaborative editing and sharing
- **Udonarium**: Export to Udonarium character format (ZIP file with XML) for use in the Udonarium TRPG tool
- **CCFolia (ココフォリア)**: Copy a monster to the clipboard as a CCFolia character piece; paste it into a CCFolia room to add it

### Monster Export Commands

Export monster data using the `monster select` command with the `--export` and `--output` flags:

```bash
# Export query results to JSON file
gm monster select -l 6 --export json --output results.json

# Export query results to Google Sheets
gm monster select -l 6 -c "Category" --export sheets --output <spreadsheet-id>

# Export to Udonarium format (ZIP file with XML)
gm monster select -l 6 --export udonarium --output monsters.zip

# Export single monster to JSON
gm monster select -n "monster name" --export json --output monster.json

# Export single monster to Udonarium format
gm monster select -n "monster name" --export udonarium --output monster.zip

# Copy single monster to clipboard in CCFolia format
gm monster select -n "monster name" --export ccfolia
```

### CCFolia (ココフォリア) Export

`--export ccfolia` converts one monster into CCFolia's clipboard character format
(`{"kind":"character","data":{...}}`) and copies it to the clipboard
(macOS: `pbcopy`, Linux: `xclip`, Windows: `clip`). Open a CCFolia room and paste (Ctrl+V / Cmd+V) to create the piece.

- **One character per paste**: the query must match exactly one monster (an exact name match is preferred)
- **Multi-part monsters**: the first core part is copied by default; use `--part <n>` (1-based) to pick another part. The list of parts is printed to stderr
- **status**: HP / MP / 防護点 (MP `-1` becomes `0`)
- **params**: 命中力 / 回避力 / 生命抵抗力 / 精神抵抗力 (each `-7`, same as Udonarium), 打撃点, ダメージ軽減 (`0`)
- **memo**: category and level, 知名度/弱点値, 先制値, 弱点, movement, special abilities (knowledge info on core parts only)
- **commands**: the same chat palette as the Udonarium export
- `--output <file>` writes the JSON to a file instead of the clipboard

```bash
gm monster select -n "モンスター名" --export ccfolia            # copy the core part
gm monster select -n "モンスター名" --export ccfolia --part 2   # copy the 2nd part
gm monster select -n "モンスター名" --export ccfolia --output piece.json
```

### Google Sheets Setup

To use the Google Sheets export feature, you need to set up Google OAuth 2.0 authentication:

#### 1. Set up Google Cloud Project

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the **Google Sheets API**
4. Create OAuth 2.0 credentials (Desktop application)
5. Download the credentials JSON file

#### 2. Configure Authentication

The tool supports two methods for OAuth configuration:

**Option A: Environment Variables (Recommended)**

```bash
export GOOGLE_CLIENT_ID="your-client-id.apps.googleusercontent.com"
export GOOGLE_CLIENT_SECRET="your-client-secret"
export GOOGLE_REDIRECT_URI="http://localhost:8080/callback"  # Optional, defaults to this value
```

**Option B: Configuration File**

Create `~/.config/trpg-json/oauth_config.json`:

```json
{
  "client_id": "your-client-id.apps.googleusercontent.com",
  "client_secret": "your-client-secret",
  "redirect_uri": "http://localhost:8080/callback",
  "scopes": ["https://www.googleapis.com/auth/spreadsheets"]
}
```

#### 3. Authentication Flow

On first export to Google Sheets:

1. The tool will open your browser to Google's OAuth consent screen
2. Grant permission for the application to access your Google Sheets
3. You'll be redirected to `http://localhost:8080/callback`
4. The tool automatically receives the authorization code and saves credentials
5. Your data is exported to the specified spreadsheet

The credentials are automatically saved and reused for future exports. They are stored securely in `~/.config/trpg-json/credentials.json`.

### Udonarium Export

The Udonarium export feature converts TRPG-JSON monster data to Udonarium character XML format, packaged in a ZIP file for easy import.

#### How It Works

1. **Single-part monsters**: Creates a ZIP file containing one XML file
   - Example: `gm select -n "ゴブリン" --export udonarium --output ゴブリン.zip`
   - Creates: `ゴブリン.zip` containing `ゴブリン.xml`

2. **Multi-part monsters**: Creates a ZIP file containing multiple XML files (one per part)
   - Example: `gm select -n "トレント" --export udonarium --output トレント.zip`
   - Creates: `トレント.zip` containing:
     - `トレント_幹.xml` (core part with full stats)
     - `トレント_根0.xml` (non-core part)
     - `トレント_根1.xml` (non-core part)

#### Features

- **Automatic XML generation**: Converts monster stats to Udonarium XML format
- **Chat palette**: Auto-generates dice roll commands for combat checks
- **Part handling**: Correctly handles core and non-core monster parts
- **File organization**: Multiple parts automatically organized in ZIP archive

#### Usage Examples

```bash
# Export a single monster to Udonarium format
gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip

# Export all level 6 monsters
gm monster select -l 6 --export udonarium --output level6_monsters.zip

# Export monsters by category
gm monster select -c "蛮族" --export udonarium --output barbarians.zip
```

### Complete Export Examples

```bash
# JSON Export Examples
gm monster select -l 6 --export json --output level6_monsters.json
gm monster select -c "蛮族" --export json --output barbarians.json

# Google Sheets Export Examples
gm monster select -l 2 -c "動物" --export sheets --output <spreadsheet-id>
gm monster select -l 6 -c "蛮族" --export sheets --output <spreadsheet-id>

# Udonarium Export Examples
gm monster select -l 6 --export udonarium --output level6_monsters.zip
gm monster select -c "蛮族" --export udonarium --output barbarians.zip
gm monster select -n "ゴブリン" --export udonarium --output ゴブリン.zip
```
