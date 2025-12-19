# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-12-19 JST (T032 complete; Phase 3 spell CLI & palette design)
- Responsibilities: kazuyasi (specification/approval/sample data) / Claude (proposal/implementation/testing)
- Status: Phase 2.5 COMPLETE (T013-T027). Phase 3 IN PROGRESS (T028-T032 complete). Spell system core implementation done (query, I/O, CLI find/list). Chat palette specification documented in DESIGN_GUIDE.md. Ready for palette generation implementation.

---

## 🔥 Priority Now
- **Phase 3: Spell System Implementation (T033-T037)** 📋
   - ✅ CLI refactoring complete (nested subcommands: `gm monster`/`gm spell`)
   - ✅ Configuration extended with spell paths support
   - ✅ Spell data model created with JSON schema validation
   - ✅ Sample spell data created & validated (9 spells, all schema-compliant)
   - ✅ Query module implemented (20+ tests passing)
   - ✅ I/O module implemented (10+ tests passing)
   - ✅ CLI commands implemented: `gm spell find/list` working
   - 🔄 Chat palette specification documented in DESIGN_GUIDE.md
   - Next: Implement palette generation (T033) with補助フラグ support

## 🚧 In Progress
- [ ] T033 Chat palette generation for spells — READY TO START
     - Status: Implementation plan complete, ready to begin coding
     - Owner: Claude
     - Task: Implement palette.rs module with補助フラグ conditional logic
     - Reference Docs: SPELL_PALETTE_IMPLEMENTATION.md + SPELL_PALETTE_TESTS.md
     - Expected: 28 tests passing, all 10 functions implemented
     - Blockers: None
     - Next: After T033, start T034 (CLI palette command)

- [x] T033 Chat palette generation for spells — 2025-12-19
     - Status: COMPLETE ✅
     - Owner: Claude + kazuyasi (collaborative)
     - Task: Implement palette.rs module with補助フラグ conditional logic
     - Completed:
       * Helper functions: format_mp(), format_target(), format_duration() ✅
       * Generator functions: generate_support_palette(), generate_regular_palette() ✅
       * Entry point: generate_spell_palette() ✅
       * 28 unit + integration tests (all PASSED) ✅
       * Error constants: 9個 (kazuyasi実装 + Claude追加) ✅
     - Test results: 49 passed; 0 failed
     - Lines of code: ~400 (functions + tests)

---

## Backlog (Phase 3 - Spell System & CLI Enhancement)
- [x] T029 Sample spell data creation (JSON) — 2025-12-19
   - Created 9 sample spells covering different categories (MagicCat_1, MagicCat_2, MagicCat_3)
   - Validated against JSON schema - all compliant
   - Includes examples of all `kind` variants (value/value+/rank for Lv, value/value+ for MP)
   - Fixed: 「一瞬」 unit removal, 属性 値修正

- [x] T030 Spell query module (search by name/level/school) — 2025-12-19
   - Implemented spell_find_by_name(), spell_find_by_school(), spell_find_by_level()
   - Added spell_find_multi() for combined filters
   - Added spell_find_by_exact_name() for exact matching
   - 20 unit tests implemented and passing

- [x] T031 Spell I/O module (load multiple spell files) — 2025-12-19
   - Extended io.rs with load_spells_json_array() for single file loading
   - Implemented load_multiple_spells_json_arrays() for multiple files
   - Added save_spells_json_array_stdout() for output
   - 10 unit tests implemented and passing

- [x] T032 Spell CLI commands: find/list/palette — 2025-12-19
    - Implemented `gm spell find <name> [-l <level>] [-s <school>]` with multi-filter support
    - Implemented `gm spell list <pattern>` with name matching
    - Implemented `gm spell palette <name>` showing basic spell info
    - All commands working and tested; release build successful

- [x] T032.5a Chat palette specification review & revision — 2025-12-19
     - Reviewed and finalized specification in DESIGN_GUIDE.md (728-802行)
     - Clarified MP field structure: exactly one of value/value+/special exists
     - Marked {行使修正} as literal output for dicebot replacement
     - Kept field names in backticks untranslated per documentation rules
     - Confirmed input/output data remain in Japanese
     - All spell search commands (find/list/palette) verified working
     - Ready for T032.5b implementation planning

- [x] T032.5b Chat palette specification translation & implementation plan — 2025-12-19
      - Status: COMPLETE
      - Task: Translate finalized specification to implementation requirements
      - Output: Detailed implementation checklist covering:
        1. Data field extraction logic from Spell struct
        2. Conditional logic for補助フラグ (true/false branching)
        3. Output format generation (single-line format)
        4. Error handling for missing/invalid fields
      - Deliverable: Implementation plan document + unit test specifications (28 tests)
      - Documents Created:
        * SPELL_PALETTE_IMPLEMENTATION.md (8 parts: spec, checklist, error handling, tests)
        * SPELL_PALETTE_TESTS.md (28 tests across 8 categories with full specifications)

- [ ] T033 Chat palette generation for spells — Claude (T032.5b dependency)
    - Implement palette.rs module with補助フラグ conditional logic
    - Support for補助=true (no dice rolls) format
    - Support for補助=false (with dice roll commands) format
    - Handle judgment stat selection based on対象/抵抗 patterns
    - Unit tests: 20+ for palette generation logic
    - Dependencies: T032.5a (spec finalized) → T032.5b (implementation plan)

- [ ] T034 Spell CLI command: palette display — Claude
   - Implement `gm spell palette <name>` with formatted output
   - Display multi-line palette for regular spells, single-line for support spells
   - Optional: copy to clipboard functionality (--copy flag)
   - Integration test with T033 palette generation

- [ ] T035 Test suite for spell functionality — Claude
   - Unit tests for palette generation (15+ tests)
   - Integration tests for CLI spell commands (10+ tests)
   - Total: 25+ new tests targeting palette feature

- [ ] T035.5 Spell select command: Query by category and level — Claude (Low Priority)
    - Implement `gm spell select -l <level> -c <category>` command
    - Similar to monster select but for spells
    - Support filters: -l (level), -c (category)
    - Return JSON array of matching spells
    - Unit tests: 10+ for spell select command
    - Effort: Low (building on existing spell query functions)

- [ ] T036 Documentation: README.md update — kazuyasi/Claude (Low Priority)
    - Update "Basic Commands" section to show monster subcommand syntax
      - Change `gm find` → `gm monster find`
      - Change `gm list` → `gm list`
      - Change `gm select` → `gm monster select`
      - Change `gm add` → `gm monster add`
      - Change `gm delete` → `gm monster delete`
    - Add new "Spell Management" section with usage examples:
      - `gm spell find/list/select` command examples
      - Document spell select filters (-l level, -c category)
      - Document spell palette output format (補助/通常 patterns)
      - Add spell query examples with filters
    - Add chat palette command examples (after T033 completion)

- [ ] T037 Commit spell system Phase 3 — kazuyasi
    - All spell functionality implemented and tested
    - Documentation complete (DESIGN_GUIDE.md + README.md)
    - Ready for git commit

---

## ✅ Done (Recent 15)
- [x] T033 Chat palette generation for spells — 2025-12-19
        - Description: Implemented palette.rs module with 7 functions: format_mp(), format_target(), format_duration(), format_magic_category() (kazuyasi), generate_support_palette(), generate_regular_palette(), generate_spell_palette(). 28 comprehensive tests covering MP/target/duration/category formatting, support/regular/entry-point palette generation, and integration tests. All tests passing (49/49). Support for both support spells (補助=true) and regular spells (補助=false) with correct output formats and dice roll handling.

- [x] T032.5b Chat palette specification translation & implementation plan — 2025-12-19
        - Description: Translated DESIGN_GUIDE.md spell palette specification into detailed implementation plan. Created SPELL_PALETTE_IMPLEMENTATION.md with 10 function definitions, error handling for 8+ scenarios, and implementation checklist. Created SPELL_PALETTE_TESTS.md with 28 unit/integration tests across 8 categories (MP formatting, target formatting, duration formatting, magic category, support/regular spell generation, entry point, integration). Ready for T033 palette.rs implementation.

- [x] T032.5a Chat palette specification review & revision — 2025-12-19
        - Description: Finalized spell chat palette specification in DESIGN_GUIDE.md. Translated section to English/Japanese mixed format following documentation rules. Clarified MP field structure (value/value+/special). Marked {行使修正} as literal output for dicebot replacement. Verified all spell search commands (find/list/palette) work correctly. Ready for T032.5b implementation planning.

- [x] T032 Spell CLI commands: find/list/palette (partial) — 2025-12-19
        - Description: Implemented all three spell CLI commands with working functionality. `gm spell find` supports multi-filter search (-l level, -s school). `gm spell list` shows matching spells with metadata. `gm spell palette` displays spell information. Release build successful. Ready for next phase (palette generation refinement).

- [x] T031 Spell I/O module (load multiple spell files) — 2025-12-19
       - Description: Extended io.rs with Spell-specific I/O functions. Implemented load_spells_json_array() for single/multiple file loading, save_spells_json_array_stdout() for output. All 10 tests passing. Spell data now fully loadable from JSON files.

- [x] T030 Spell query module (search by name/level/school) — 2025-12-19
       - Description: Implemented spell_find_by_name(), spell_find_by_school(), spell_find_by_level(), spell_find_multi() for combined filtering, spell_find_by_exact_name() for exact matching. All 20 query tests passing. Pattern matches Monster query module design.

- [x] T029 Sample spell data creation (JSON) & validation — 2025-12-19
       - Description: Created 9 sample spells covering MagicCat_1/2/3 categories with all kind variants (value/value+/rank for Lv, value/value+ for MP). Fixed schema compliance issues (「一瞬」unit removal, 属性 value correction). JSON schema validation: all spells pass. Sample data ready for testing.

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
