# TRPG-JSON Repository

A JSON-based data management system for TRPG (Tabletop Role-Playing Game) game systems.

## Overview

This repository provides tools for managing and querying TRPG game data (such as monsters, characters, and items) in JSON format. It includes a command-line interface for searching and filtering game data.

## Usage

This is a command-line tool for managing TRPG game data in JSON format.

### Basic Commands

```bash
# Search for a monster by name
gm find "monster name"

# List monsters matching a pattern
gm list "pattern"

# Query with filters
gm select -l 6 -c "Category"

# Add a new monster
gm add monster.json

# Delete a monster
gm delete "monster name"
```

## Export Features

This tool supports exporting TRPG data to multiple formats for collaboration and data sharing.

### Supported Export Formats

- **JSON**: Native JSON format for data interchange and integration with other tools
- **Google Sheets**: Direct export to Google Sheets for collaborative editing and sharing

### Export Commands

Export data using the `select` command with the `--export` and `--output` flags:

```bash
# Export query results to JSON file
gm select -l 6 --export json --output results.json

# Export query results to Google Sheets
gm select -l 6 -c "Category" --export sheets --output <spreadsheet-id>

# Export single monster to JSON
gm select -n "monster name" --export json --output monster.json
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

### Export Examples

```bash
# Export all level 6 monsters
gm select -l 6 --export json --output level6_monsters.json

# Export category-specific monsters to Google Sheets
gm select -c "動物" --export sheets --output <spreadsheet-id>

# Export with multiple filters
gm select -l 6 -c "蛮族" --export json --output filtered_results.json
```
