# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-12-18 JST (T028 in progress; Phase 3 spell system kickoff)
- Responsibilities: kazuyasi (specification/approval/sample data) / Claude (proposal/implementation/testing)
- Status: Phase 2.5 COMPLETE (T013-T027). Phase 3 INITIATED. CLI refactored with nested subcommands (`gm monster`/`gm spell`). Configuration extended. Spell data model with JSON schema validation ready. Sample spell data creation in progress.

---

## 🔥 Priority Now
- **Phase 3: Spell System Implementation (T028-T045)** 📋
   - CLI refactoring complete (nested subcommands: `gm monster`/`gm spell`)
   - Configuration extended with spell paths support
   - Spell data model created with JSON schema validation
   - Ready for spell sample data creation and feature implementation

## 🚧 In Progress
- [ ] T028 JSON Schema for spells - Conditional field validation — IN PROGRESS (kazuyasi)
   - Status: Schema validation rules tested. Using if-then-allOf pattern for kind-based field selection
   - Next: Create sample spell data using schema

---

## Backlog (Phase 3 - Spell System & CLI Enhancement)
- [ ] T029 Sample spell data creation (JSON) — kazuyasi
   - Create 5-10 sample spells covering different categories (真語魔法, 操霊魔法, etc.)
   - Test JSON against schema validation
   - Include examples of all `kind` variants (value/value+/special for MP, value/value+/rank for Lv)

- [ ] T030 Spell query module (search by name/level/school) — Claude
   - Implement query::find_by_school(), find_by_level() for spells
   - Add unit tests for spell search functionality
   - Pattern: follow monster query module design

- [ ] T031 Spell I/O module (load multiple spell files) — Claude
   - Extend io.rs to support Spell deserialization
   - Implement load_multiple_json_arrays() variant for spells
   - Add error handling for invalid spell JSON

- [ ] T032 Spell CLI commands: find/list — Claude
   - Implement `gm spell find <name> [-l <level>] [-s <school>]`
   - Implement `gm spell list <pattern>`
   - Output format: similar to monster commands but without export options

- [ ] T033 Chat palette generation for spells — Claude
   - Design chat palette format for spells (1-2 line format per spec)
   - Implement palette generation with dice rolls (2d+{stat} patterns)
   - Handle variable references from spell data fields

- [ ] T034 Spell CLI command: palette display — Claude
   - Implement `gm spell palette <name> [-c|--copy]`
   - Display formatted chat palette to stdout
   - Optional: copy to clipboard functionality (using copypasta or similar crate)
   - Return error if spell not found

- [ ] T035 Test suite for spell functionality — Claude
   - Unit tests for spell query module (20+ tests)
   - Unit tests for spell palette generation (15+ tests)
   - Integration tests for CLI spell commands (10+ tests)
   - Target: 45+ new tests, all passing

- [ ] T036 Documentation: Spell features in README.md — kazuyasi/Claude
   - Add "Spell Management" section with usage examples
   - Document spell palette output format
   - Add spell query examples with filters

- [ ] T037 Commit spell system Phase 3 — kazuyasi
   - All spell functionality implemented and tested
   - Documentation complete
   - Ready for git commit

---

## ✅ Done (Recent 15)
- [x] T028a CLI Refactoring: Nested subcommand structure (gm monster/spell) — 2025-12-18
       - Description: Refactored Commands enum with MonsterCommands and SpellCommands nested enums. Implemented monster/spell top-level commands. Maintained backward compatibility with direct find/list/select/add/delete for existing users. All existing functionality verified working.

- [x] T028b Configuration: Spell path support — 2025-12-18
       - Description: Extended config.rs with SpellsConfig enum supporting single/multiple spell files. Added resolve_spells_paths() method. Updated default.toml with spell configuration examples. Added 5 new config tests (load single/multiple spells, resolve paths). All 16 config tests passing.

- [x] T028c Data Model: Spell struct implementation — 2025-12-18
       - Description: Implemented Spell struct in core/lib.rs with fields: name, school, level, effect, target, cost, notes, extra. Includes proper serde support with Japanese field name handling. Ready for JSON deserialization.

- [x] T028d JSON Schema: Magic spell schema with conditional field validation — 2025-12-18
       - Description: Created comprehensive magic.json schema. Implemented if-then-allOf pattern for `kind`-based field validation. MP (value/value+/special), Lv (value/value+/rank), and 対象 (個別/エリア) now conditionally require specific fields only. Schema tested and verified with Python validator.

- [x] T027 Documentation: Udonarium export examples in README.md — 2025-12-17
       - Description: Updated README.md and DESIGN_GUIDE.md with complete Udonarium export documentation. Added single-part and multi-part monster export examples, feature descriptions, and usage examples.
- [x] T026 Unit and integration tests: Udonarium export (22 tests) — 2025-12-17
       - Description: All XML generator tests passing (22/22). Tests cover core/non-core part generation, chat palette commands, value adjustments (-7 for hit_rate/dodge/resistance), weakness transformation, and multi-part monsters.
       - Tests verified: CorePartXmlGeneration, NonCorePartXmlGeneration, ChatPaletteGeneration (11 comprehensive tests), value adjustments, proper newline handling.
- [x] T025 CLI Integration: Udonarium export flag and help text — 2025-12-17
       - Description: Integrated Udonarium exporter into core export module. Updated CLI help text for select command with Udonarium export examples. Added ExportFormat::Udonarium enum and factory support.
- [x] T024 Udonarium exporter: Chat palette auto-generation — 2025-12-17
       - Description: Implemented auto-generation of chat palette with 5 dice roll commands (命中力, 回避力, 打撃点, 生命抵抗力, 精神抵抗力). Commands correctly reference adjusted values (-7 applied where needed). No special abilities included in chat palette per spec.
- [x] T023 Udonarium exporter: ZIP compression for multi-part monsters — 2025-12-17
       - Description: Implemented ZipFileWriter with proper multi-file packaging. Supports single-part (one XML) and multi-part (multiple XML files) monsters. File naming follows convention: monster_name.xml (single), monster_name_part_name#.xml (multi-part).
- [x] T022 Udonarium exporter: XML generation for each part — 2025-12-17
       - Description: Implemented XmlGenerator with separate core and non-core part XML templates. Proper XML structure with image section, size field, status values with -7 adjustment, chat palette, and section visibility control.
- [x] T021 Udonarium exporter: Core module design and data transformer — 2025-12-17
       - Description: Implemented data transformation pipeline: Monster → TransformedMonster/TransformedPart. Includes value adjustments (hit_rate/dodge/resistance -7), MP handling (-1→0), weakness text transformation, and part-specific data segregation.
- [x] T020 Documentation: README.md Export Features Section — 2025-12-14
      - Description: Updated README.md with comprehensive Export Features documentation. Added detailed Google Sheets setup instructions (Google Cloud Project setup, OAuth configuration via environment variables or config file, authentication flow explanation). Included supported export formats (JSON and Google Sheets), export command examples with `--export` and `--output` flags, and practical export examples with actual use cases.
- [x] T019 Phase 2.5: Google Sheets API Integration (P25-3c) — 2025-12-13
      - Description: Complete OAuth 2.0 authentication flow implementation with browser-based authorization. Full Google Sheets API v4 integration: find_empty_row() to locate empty spreadsheet rows, write_rows_to_sheet() for data writing via batchUpdate. GoogleSheetsExporter fully implemented with async/await support. Weakness field output fixed (AW column even row). Data transformer improvements for empty string handling. Total tests: 94 (80 core + 8 app + 6 new sheets tests). All passing.
      - Subtasks completed:
         1. [x] T019a: OAuth 2.0 authentication flow (browser-based with token management)
         2. [x] T019b: find_empty_row() using Google Sheets API values.get
         3. [x] T019c: write_rows_to_sheet() using Google Sheets API values.batchUpdate
         4. [x] T019d: Integration tests - full Google Sheets export workflow
         5. [x] T019e: GoogleSheetsExporter::export implementation complete
      - Dependencies added: tiny_http, webbrowser, uuid, reqwest, urlencoding
- [x] T018 Phase 2.5: Integration tests and build verification (P25-6, P25-7) — 2025-12-13
      - Description: Full test suite verification (88 tests: 80 core + 8 app). Release build success. All compilation warnings resolved. Phase 2.5 core implementation complete and ready for API integration.
- [x] T017 Phase 2.5: Export module tests (P25-5) — 2025-12-13
      - Description: Added 8 CLI integration tests in app/src/main.rs. Tests cover export format parsing, exporter factory creation, config handling, JSON export functionality, and error scenarios (empty data, invalid IDs).
- [x] T015b-g Phase 2.5: Google Sheets exporter implementation (P25-3b-g) — 2025-12-13
      - Description: Completed exporter implementation: auth.rs (OAuth credentials), sheets.rs (data transformation), sheets_api.rs (API skeleton), google_sheets.rs (error handling). 37 export tests, all passing.
- [x] T009 Data extension design guide — 2025-11-30
     - Description: Completed DESIGN_GUIDE.md with Phase 2.5 (Data Export) priority. Added extensibility patterns for multiple game systems. Migration Guide, Troubleshooting (7 scenarios), system integration examples included.
- [x] T012 Phase 2.5: Config Integration - Multiple Data Files — 2025-11-30
     - Description: Extended config.rs to support both single and multiple data files using MonstersConfig enum with #[serde(untagged)]. Updated main.rs to resolve paths from home directory. All CLI commands (find, list, select, add, delete) work with multiple files. Config accepts both formats. All 43 tests passing, release build successful.
- [x] T008d CLI Tool: I/O policy enforcement — 2025-11-30
     - Description: All commands correctly implement stdout/stderr policy. Errors to stderr (eprintln!), results to stdout (println!), exit code 1 on failure. Verified: find, list, select, add, delete commands all compliant.
- [x] T008c CLI Tool: Query/Update commands (select/add/delete) — 2025-11-30
     - Description: All three commands already implemented. `select` executes queries with -n, -l, -c filters returning JSON array. `add` supports new/overwrite confirmation dialogue. `delete` supports exact match deletion with confirmation. All functionality working as specified.
- [x] T011 Phase 2: File Flexibility (Configuration & Multiple Data Files) — 2025-11-30
    - Description: Implemented `load_multiple_json_arrays()` in `core/src/io.rs` to support multiple data files. Config file support and `--config` CLI flag were already in place. Added 3 comprehensive unit tests for multiple file loading scenarios (normal case, with empty files, with missing files). All 41 unit tests passing, release build successful.
- [x] T010 Configuration file path management — 2025-11-29
    - Description: Updated config file search to prioritize `~/.config/trpg-json/default.toml`, with fallback to `config/default.toml`. Fixed macOS path resolution using `HOME` environment variable instead of `dirs` crate. Updated help message to reflect new default path.
- [x] T008b-3c app: list command implementation — 2025-11-29
   - Description: Implemented `gm list <pattern>` with pattern matching, returns JSON for 1 hit, name list for multiple hits, error to stderr + exit 1 for 0 hits
- [x] T008b-3b app: find command implementation — 2025-11-29
   - Description: Implemented `gm find <name> [-l <Lv>] [-c <Category>]` with multi-filter support, returns JSON for 1 hit, count for multiple hits, error to stderr + exit 1 for 0 hits
- [x] T008b-3a core: query module implementation — 2025-11-24
  - Description: Implemented `find_by_name()`, `find_by_level()`, `find_by_category()` functions with unit tests and pattern matching
- [x] T008b-2 core: io module implementation — 2025-11-24
  - Description: Implemented `load_json_array()` and `save_json_array_stdout()` with `IoError` using thiserror
- [x] T008b-1 core: model definition — 2025-11-24
  - Description: Completed serde implementation of `Monster`/`Part` structs, Japanese key support, 5 unit tests passing
- [x] T008a CLI Tool: Foundation implementation — 2025-09-14
  - Description: clap integration, `gm --help`/`gm find`/`gm list` scaffolding completed
- [x] T004 Project specification Rust module design — 2025-09-14
- [x] T002 Task system organization — 2025-09-14
- [x] T005 Security operations policy implementation — 2025-09-14
- [x] T001 Initial specification review (specification agreement) — 2025-09-14
- [x] T003 Rust environment minimal setup (using Zed) — 2025-09-14
- [x] T006 JSON formatting and normalization processing — 2025-09-14
- [x] T000 task.md format migration — 2025-09-14

---

## 📋 Future Phases (Post Phase 3)
- [ ] T038 Phase 4: Skill system implementation (流派特技)
- [ ] T039 Phase 4: Fairy magic system implementation (妖精魔法)
- [ ] T040 Phase 4: Chat palette export to clipboard
- [ ] T041 Phase 4: Multi-system support (extend beyond SW2.5)
- [ ] T042 Phase 4: Skill/Fairy magic CLI commands (gm skill find/list/palette)

## 🚮 Canceled
- [ ] T007 Data analysis feature implementation (deemed unnecessary) — 2025-09-14

---

## 🧪 Decisions & Links
- CLI command name: `gm`
- I/O policy: **stdout as default**, errors to stderr
- Security: Decrypt during development, enforce encryption for sensitive data
- Data target: JSON-based entity files, **future expansion under consideration**
- Phase 2.5 Export Targets: JSON (file-based) + Google Sheets (cloud-based with OAuth 2.0)
- No CSV export (JSON is preferred for data interchange)
- Export is user-triggered (manual), no automation needed
- OAuth 2.0: One-time setup for Google Sheets, credentials stored locally
