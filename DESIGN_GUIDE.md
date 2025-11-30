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
- Support for exporting data to external formats
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
    pub destination: String,
    pub format: ExportFormat,
}

pub enum ExportFormat {
    Json,
    Csv,
    // Future: Custom formats
}
```

**CLI Usage:**
```bash
# Export search results to file
gm select -l 6 --export json --output results.json

# Export to CSV format
gm select -c "Category" --export csv --output results.csv
```

**Setup Steps:**
1. User runs: `gm select ... --export <format> --output <file>`
2. System validates data
3. Data exported to specified format
4. Confirmation message displayed

**Advantages of this approach:**
- Flexible export options
- No external authentication complexity
- File-based, easy to version control
- Data remains under user control

**Design Considerations:**
- Support for multiple output formats
- Validation of exported data
- Integration with existing command structure

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
- **Export format**: Multiple formats supported (JSON, CSV, etc.)
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
