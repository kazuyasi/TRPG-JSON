# AGENTS.md

## Build/Test Commands
```bash
cd trpg-json
cargo build --release    # Build optimized binary
cargo build              # Build debug version
cargo test               # Run all tests
cargo test --lib         # Run library tests only
cargo test <test_name>   # Run single test
cargo run --bin gm -- --help  # Run CLI
```

## Code Style Guidelines
- Use Rust 2024 edition
- Follow core/app separation pattern
- Use thiserror for error handling
- Use serde + serde_json for JSON serialization
- Japanese comments and CLI messages expected
- Use serde field rename attributes for Japanese field names
- Use Option<T> for nullable fields in structs

## Data Usage Guidelines
- **Public Repository Policy**: This repository may be published on GitHub; all committed files must contain only non-copyrighted data
- **Examples and Documentation**: Use generic placeholder names (e.g., "モンスター名", "神名", "キャラクター名")
- **Test Data**: Use only fictional/generic data not derived from published rulebooks
- **Sample Data**: Must be created independently or with explicit permission
- **If Uncertain**: Always use generic placeholders instead of specific names from rulebooks

## Architecture
- Workspace: `trpg-json/Cargo.toml` with `core` and `app` members
- Core library: data models, I/O, business logic
- CLI app (`app`):
  - `main.rs`: Entry point and routing
  - `commands/`: Command handlers (monster, spell)
  - `utils.rs`: Common I/O and UI utilities
- Main data: `data/sample/monsters_sample.json`

## Data Models
- Entity struct with field names (may use Japanese or other languages via serde rename)
- Part struct for entity sub-components
- Use HashMap<String, serde_json::Value> for extra fields
- -1 values represent "none/unknown" for optional numeric fields

## CLI Commands

### Implemented Commands

#### find command
```bash
gm find <name> [-l <Lv>] [-c <Category>]
```
- **Function**: Search monsters by name with optional level and category filters
- **Arguments**:
  - `<name>` - Monster name (partial match)
  - `-l, --level <Lv>` - Filter by level (exact match, optional)
  - `-c, --category <Category>` - Filter by category (exact match, optional)
- **Output**:
  - 0 hits → Error to stderr + exit 1
  - 1 hit → JSON object with full monster details
  - Multiple hits → Hit count + exact match JSON if available

#### list command
```bash
gm list <pattern>
```
- **Function**: List monsters matching a name pattern
- **Arguments**:
  - `<pattern>` - Monster name pattern (partial match)
- **Output**:
  - 0 hits → Error to stderr + exit 1
  - 1 hit → JSON object with full monster details
  - Multiple hits → Name list (Lv, name, category) + hit count + exact match JSON if available

### Backlog Commands
- `gm select <query>` - Execute query returning JSON array
- `gm add <json-file>` - Add monster with duplicate check
- `gm delete <name>` - Delete monster with confirmation

## File Editing Rules
- Always request confirmation before editing any file

## Git Operations Policy
- **Claude's role**: Implementation, testing, documentation (never perform git operations)
- **User's role**: All git operations (commit, push, branch management, etc.)
- Claude will prepare changes and show diffs, but user must execute all git commands
- When implementation is complete, Claude will notify user and wait for git instructions
