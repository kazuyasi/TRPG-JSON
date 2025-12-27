# TRPG-JSON Data Extension Design Guide

## Overview

This guide describes how to extend the TRPG-JSON system to support additional data sources and game systems beyond the current implementation.

## Current Architecture

### Single Data Source Model
```
CLI (app/) → Core Library (core/) → JSON File (data/systems/monsters.json)
                     ↓
               Query Module
               (find_by_name, find_by_level, etc.)
                     ↓
               I/O Module
               (load_json_array, save_json_array_file)
```

**Current Implementation:**
- **Data Path**: Configured via TOML file: `data/systems/monsters.json`
- **Data Structure**: `Vec<Monster>`
- **File Format**: JSON array of Monster objects

## Extension Scenarios

### Scenario 1: Multiple Game Systems

**Goal**: Support different game systems with different data structures

**Design Approach:**

1. **Create System-Specific Modules** in `core/`:
   ```rust
   // core/src/systems/
   pub mod system_a;   // Game System A
   pub mod system_b;   // Game System B
   pub mod common;     // Shared traits
   ```

2. **Define Common Trait**:
   ```rust
   // core/src/systems/common.rs
   pub trait GameEntity: Serialize + Deserialize {
       fn get_name(&self) -> &str;
       fn get_level(&self) -> i32;
       fn get_category(&self) -> &str;
   }
   ```

3. **Implement for Each System**:
   ```rust
   // Existing Monster struct implements GameEntity
   impl GameEntity for Monster { ... }
   
   // New D&D Character struct
   pub struct Character { ... }
   impl GameEntity for Character { ... }
   ```

4. **Generalize Query Module**:
   ```rust
   // core/src/query.rs - Add generic functions
   pub fn find_by_name<T: GameEntity>(
       entities: &[T], 
       name: &str
   ) -> Vec<&T> { ... }
   ```

### Scenario 2: Multiple Data Files per System

**Goal**: Support splitting large datasets (e.g., monsters by category)

**Design Approach:**

1. **Extend I/O Module** to handle multiple files:
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

2. **Create System Configuration**:
    ```rust
    // config/system_a.toml
    [data_files]
    monsters = ["data/systems/monsters.json"]
    items = ["data/systems/items.json"]
    ```

3. **Update CLI** to load multiple sources:
    ```rust
    // app/src/main.rs
    let config = Config::load("config/default.toml")?;
    let monsters = load_multiple_json_arrays(&config.data_files)?;
    ```

### Scenario 3: Mixed Data Sources (JSON, Database, API)

**Goal**: Support reading from multiple storage backends

**Design Approach:**

1. **Define Data Source Trait**:
   ```rust
   // core/src/io.rs
   pub trait DataSource {
       fn load(&self) -> Result<Vec<Monster>, IoError>;
       fn save(&self, monsters: &[Monster]) -> Result<(), IoError>;
   }
   ```

2. **Implement Multiple Backends**:
   ```rust
   pub struct JsonSource { path: String }
   pub struct SqliteSource { db_path: String }
   pub struct ApiSource { url: String }
   
   impl DataSource for JsonSource { ... }
   impl DataSource for SqliteSource { ... }
   impl DataSource for ApiSource { ... }
   ```

3. **Use in CLI**:
   ```rust
   let source: Box<dyn DataSource> = match config.backend {
       "json" => Box::new(JsonSource::new(path)),
       "sqlite" => Box::new(SqliteSource::new(path)),
       "api" => Box::new(ApiSource::new(url)),
   };
   
   let monsters = source.load()?;
   ```

## Implementation Roadmap

### Phase 1: Current State ✅
- Single game system
- Single data file (monsters.json)
- Configuration-based data path
- Basic CRUD operations (find, list, select, add, delete)

### Phase 2: File Flexibility ✅ (Completed)
- Configuration file support (TOML)
- Multiple data files per system
- Home directory relative paths
- **Effort**: Low (2-3 days) ✅
- **Benefits**: High flexibility, easy migration ✅

### Phase 2.5: External Data Export (Recommended Next)
- Support for exporting data to JSON and Google Sheets
- OAuth 2.0 authentication for cloud services (user-friendly)
- Preserve formatting when exporting
- Manual execution (no automation needed)
- **Effort**: Medium (3-4 days)
- **Benefits**: Enable data sharing and collaborative editing capabilities
- **Target**: Small teams, single data manager

### Phase 2.5 Implementation Details: External Data Export

**Export Interface:**
```rust
// core/src/export/mod.rs
pub trait DataExporter {
    fn export(&self, data: &[Monster], config: &ExportConfig) -> Result<(), String>;
}

pub struct ExportConfig {
    pub destination: String,  // File path for JSON, Spreadsheet ID for Google Sheets
    pub format: ExportFormat,
}

pub enum ExportFormat {
    Json,
    GoogleSheets,
    // Future: Custom formats
}
```

**CLI Usage:**
```bash
# Export search results to JSON file
gm select -l 6 --export json --output results.json

# Export to Google Sheets (requires OAuth 2.0 setup)
gm select -c "Category" --export sheets --output "Spreadsheet ID"
```

**Setup Steps for JSON Export:**
1. User runs: `gm select ... --export json --output <file>`
2. System validates data
3. Data exported to JSON file
4. Confirmation message displayed

**Setup Steps for Google Sheets Export:**
1. User authenticates with Google OAuth 2.0 (one-time setup)
2. User runs: `gm select ... --export sheets --output <spreadsheet-id>`
3. System creates/updates rows in the specified Google Sheet
4. Confirmation message with sheet URL displayed

**Advantages of this approach:**
- JSON export: No authentication needed, file-based, easy to version control
- Google Sheets: Cloud-based, collaborative editing, real-time synchronization
- Data remains under user control
- Flexible destination options

**Design Considerations:**
- JSON export: Standard JSON array format with Monster objects
- Google Sheets: OAuth 2.0 authentication, Google Sheets API integration
- Data validation before export
- Integration with existing command structure
- Error handling for authentication and API failures

### Phase 3: System Abstraction
- Trait-based system separation
- Support for multiple game systems
- Reusable query/io modules
- **Effort**: Medium (5-7 days)
- **Benefits**: Extensibility for future systems

**Implementation Details:**
```rust
// core/src/systems/common.rs
pub trait GameEntity: Serialize + Deserialize + Clone {
    fn get_name(&self) -> &str;
    fn get_level(&self) -> i32;
    fn get_category(&self) -> &str;
}

// Implement for each system
impl GameEntity for Monster { ... }  // SW2.5
impl GameEntity for DndCharacter { ... }  // D&D
```

**Steps:**
1. Create `core/src/systems/` module structure
2. Define `GameEntity` trait with system-agnostic methods
3. Implement trait for existing `Monster` struct
4. Create new system modules (dnd, pathfinder, etc.)
5. Generalize query functions to work with trait objects
6. Add system selection to CLI via `--system` flag

## Future Considerations

### SQLite Local Backend (Optional)
For larger datasets or advanced querying:
- Local SQLite database instead of JSON files
- Indexed search for >10k entities
- Complex query support (joins, aggregations)
- **Note**: Consider if repository becomes public with sample data

### Data Format Flexibility
Current implementation uses JSON. Other formats could be evaluated:
- YAML: More readable for TRPG data structures
- TOML: Already used for configuration
- **Deferred**: Revisit after Phase 3 system abstraction

### Open Source & Data Privacy
- **Code**: Can be licensed as open source (Apache 2.0, MIT, etc.)
- **Data**: Book-derived data requires copyright management
- **Solution**: Separate sample data from production data
- **Status**: Hold until sample data available

## Immediate Next Steps (If Needed)

1. **Add Configuration File Support**:
   - Create `config/default.toml`
   - Use `config` crate or `toml` crate
   - Allow path override via CLI flag: `gm --config config/custom.toml find "name"`

2. **Parameterize Data Path**:
   - Change hardcoded path to function parameter
   - Pass from `main()` through handler functions
   - Load from config file instead

3. **Add Unit Tests** for new scenarios:
   - Multiple file loading
   - Configuration parsing
   - Error handling for missing files

## Code Organization Best Practices

```
trpg-json/
├── core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models.rs          # Monster, Part, etc.
│   │   ├── query.rs           # Search functions
│   │   ├── io.rs              # File I/O
│   │   ├── systems/
│   │   │   ├── mod.rs
│   │   │   ├── system_a.rs    # System A specific
│   │   │   ├── common.rs      # Shared traits
│   │   └── config.rs          # Configuration loading
│   └── Cargo.toml
├── app/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── find.rs        # find handler
│   │   │   ├── list.rs        # list handler
│   │   │   └── select.rs      # select handler
│   │   └── config.rs          # Load config
│   └── Cargo.toml
├── config/
│   └── default.toml           # Default configuration
├── data/
│   ├── systems/
│   │   ├── monsters.json      # Encrypted
│   │   └── items.json         # Future
│   └── samples/               # Sample data
└── README.md
```

## Security Considerations

### Encrypted Data
- All production data files must remain under `git-crypt` encryption
- Development environment: decrypt only when needed
- CI/CD: Use encrypted credentials or offline processing

### Configuration Files
- Keep `.gitignore` entry for local config overrides
- Example: `config/local.toml` (not committed)
- Production config committed with encryption

### Multi-System Support
- Separate data directories per system
- Each system can have independent encryption policy
- Validate system compatibility before loading

## Testing Strategy

### Unit Tests
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

### Integration Tests
- Test loading from different data sources
- Test error handling (missing files, invalid format)
- Test mixed system scenarios

## Migration Guide: Single File → Multiple Files

If you currently have one large `monsters.json` and want to split it:

**Step 1: Prepare the split data files**
```bash
# Split monsters.json by category or level range
# Example: Create separate files for each category
jq '.[] | select(.Category == "Category1")' data/systems/monsters.json > data/systems/monsters_part1.json
jq '.[] | select(.Category == "Category2")' data/systems/monsters.json > data/systems/monsters_part2.json
```

**Step 2: Update config file**
```toml
[data]
# Old (still works):
# monsters = "data/systems/monsters.json"

# New (multiple files):
monsters = [
    "data/systems/monsters_part1.json",
    "data/systems/monsters_part2.json"
]
```

**Step 3: Verify no data loss**
```bash
gm find "Monster Name"  # Should still find entities
gm select -l 6          # Should return all level 6 entities
```

**Important Notes:**
- All files must contain JSON arrays
- File order doesn't affect search results (merged into single collection)
- When adding/deleting, changes save to first file in list
- Consider consolidation periodically to avoid too many files

## Troubleshooting

### Problem: "Error: Data file not found"
**Solutions:**
1. Check path is relative to home directory: `~/playground/TRPG-JSON/...`
2. Verify file exists: `ls ~/path/to/file.json`
3. Check file permissions: `chmod 644 data/systems/monsters.json`
4. Use absolute path as fallback: `/Users/username/playground/TRPG-JSON/...`

### Problem: "JSON parsing error" on valid JSON
**Solutions:**
1. Verify file is valid JSON: `jq empty data/systems/monsters.json`
2. Check for encoding issues: `file data/systems/monsters.json`
3. Ensure root element is array: `jq '. | type' data/systems/monsters.json` → should output "array"
4. Check for corrupt records: `jq '.[] | has("name")' data/systems/monsters.json`

### Problem: Multiple files with conflicting monster names
**Solutions:**
1. Use unique naming: "monsters_part1.json", "monsters_part2.json"
2. Add metadata to identify source: Include "source" field in Monster extra fields
3. Or consolidate into single file: `jq -s 'add' monsters_part*.json > monsters_consolidated.json`

### Problem: add/delete affects wrong file
**Note:** When using multiple files, modifications save to the first file in config list. 
**Solution:** If you need different save behavior:
1. Consider consolidating files occasionally
2. Or reorganize config to put "editable" file first
3. Future Phase 4 backends will support per-file tracking

## Example: Adding a New System (Phase 3+)

To add support for an additional game system after Phase 3 implementation:

**1. Create system module** in `core/src/systems/system_b.rs`:
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

**2. Create system factory** in `core/src/systems/mod.rs`:
```rust
pub fn load_system(name: &str, config: &Config) -> Result<Box<dyn GameEntity>, String> {
     match name {
         "system_a" => load_system_a_entities(config),
         "system_b" => load_system_b_characters(config),
         _ => Err(format!("Unknown system: {}", name)),
     }
 }
```

**3. Update config schema** to support system selection:
```toml
[data]
monsters = ["data/systems/monsters.json"]

[system]
name = "system_a"  # Can be "system_a", "system_b", etc.

# System B example:
# [data]
# characters = ["data/systems/characters.json"]
# [system]
# name = "system_b"
```

**4. Update CLI** to accept system selection:
```bash
gm --system system_b find "Character Name"  # Search System B characters
gm find "Monster Name" -l 3                 # Search System A monsters (default)
```

**5. Ensure backward compatibility** - default to SW2.5 if not specified

**Benefits of this approach:**
- Plug-and-play system support
- Reusable query logic (`find_by_name()` works for any `GameEntity`)
- Separate data and code for each system
- Easy to add new systems without modifying existing code

## References & Further Reading

### Current Implementation
- **CLI entry point**: `app/src/main.rs`
- **Business logic**: `core/src/query.rs` (search functions)
- **File I/O**: `core/src/io.rs` (load/save operations)
- **Configuration**: `core/src/config.rs` (config parsing)
- **Data models**: `core/src/lib.rs` (Monster, Part structs)
- **Configuration file**: `config/default.toml`

### Build & Development
- See `AGENTS.md` for build/test commands
- Rust 2024 edition, thiserror for error handling, serde for serialization

### Related Documentation
- `tasks.md` - Project task board and status
- `README.md` - User-facing documentation
- `AGENTS.md` - Development guidelines and git policy

### Design Patterns Used
- **Trait-based design** (for future Phase 3 extensibility)
- **Enum deserialization** (`#[serde(untagged)]` for config flexibility)
- **Error propagation** (thiserror crate for ergonomic error handling)
- **Configuration management** (TOML-based, environment variable support)

### Performance Considerations
- **Current**: O(n) search over all loaded monsters
- **Future**: Consider indexing for Phase 4+ with large datasets
- **Memory**: All monsters loaded in memory (sufficient for <100k entities)
- **Disk**: JSON format is human-readable but not compressed (consider future optimization)

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
- **Use case**: Rank-based progression systems (e.g., "Rank 2 fairy magic")
- **Value range**: Variable; depends on `schoolVariant` and system implementation
- **Example**:
  ```json
  { "name": "妖精魔法ランク2", "Lv": { "kind": "rank", "rank": 2 } }
  ```
  Query: `gm spell find -r 2` → matches this record

### School Variant Field (`schoolVariant`)

**CLI flag**: `-sv <variant>`

**Query behavior**: 
- Returns records where `schoolVariant` exactly matches the specified value (exact match only)
- If `-sv` flag is specified, records without a `schoolVariant` field (or with `null` value) are excluded from results

**Example**:
```json
{ "name": "特殊神聖魔法", "school": "神聖", "schoolVariant": "特殊" }
```
Query: `gm spell find -sv 特殊` → matches this record

### God Field (`god`)

**Applicability**: Only applicable when `schoolVariant == "特殊"` AND `school == "神聖"`

**CLI flag**: `-g <god_name>`

**Query behavior**:
- Returns records where `god` exactly matches the specified value (exact match only)
- If `-g` flag is specified, records without a `god` field (or with `null` value) are excluded from results

**Example**:
```json
{ 
  "name": "特殊神聖魔法の例", 
  "school": "神聖", 
  "schoolVariant": "特殊",
  "god": "神名" 
}
```
Query: `gm spell find -sv 特殊 -g 神名` → matches this record

**Note**: If `schoolVariant` is not "特殊" or `school` is not "神聖", the `god` field is not applicable and should not exist in the data.

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
2d+{神聖魔法}+{行使判定} ゴッド・ジャッジメント / MP:15 / 対象:1エリア(半径4mすべて) / 射程:術者 / 時間:一瞬 / 物理的に神の捌きを下す。
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
- Example: If `category` is "妖精", magic category is "妖精魔法"

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
      "部位特殊能力": "再生＝5",
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
      "部位特殊能力": "拘束攻撃",
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
| 4 |||||||||||||| 再生＝5 | 炎ダメ+3 |
| 5 | トレント\n(根) | 75 | 20 | 7 | - | - | - | 3 | - | - | 19 | 15 | SAMPLE | 特殊系統魔法8Lv／11、魔法の才能 | - |
| 6 |||||||||||||| 拘束攻撃 | - |
| 7 | トレント\n(根) | 75 | 20 | 7 | - | - | - | 3 | - | - | 19 | 15 | SAMPLE | 特殊系統魔法8Lv／11、魔法の才能 | - |
| 8 |||||||||||||| 拘束攻撃 | - |

**Notes:**
- Rows 3-4: First part (幹, コア=true) → "★" prefix, common values output, all fields populated
- Rows 5-8: Second part (根, コア=false) → no "★" prefix, `先制値`/`生命抵抗力`/`精神抵抗力`/`知名度`/`弱点値`/`弱点` → "-"
