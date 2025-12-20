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

# Search by school
gm spell find "spell name" -s "神聖"

# Combine multiple filters
gm spell find "magic" -l 3 -s "神聖"

# List spells matching a pattern
gm spell list "pattern"

# Generate chat palette for a spell
gm spell palette "spell name"

# Copy spell palette to clipboard (macOS/Linux/Windows)
gm spell palette "spell name" -y
# or
gm spell palette "spell name" --copy
```

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

## Export Features

This tool supports exporting TRPG data to multiple formats for collaboration and data sharing.

### Supported Export Formats

- **JSON**: Native JSON format for data interchange and integration with other tools
- **Google Sheets**: Direct export to Google Sheets for collaborative editing and sharing
- **Udonarium**: Export to Udonarium character format (ZIP file with XML) for use in the Udonarium TRPG tool

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
