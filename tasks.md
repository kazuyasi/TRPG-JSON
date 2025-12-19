# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-12-19 JST (T033-T036.5 complete; Phase 3 implementation and documentation complete)
- Responsibilities: kazuyasi (specification/approval/git operations) / Claude (proposal/implementation/testing)
- Status: Phase 2.5 COMPLETE (T013-T027). Phase 3 COMPLETE (T028-T036.5 done). Spell system fully implemented with chat palette generation, CLI commands, comprehensive test suite, documentation, and all I/O tests fixed. Ready for T037 (final commit).

---

## 🔥 Priority Now
- **Phase 3: Spell System Implementation (T033-T037)** 📋 READY FOR FINAL COMMIT
   - ✅ CLI refactoring complete (nested subcommands: `gm monster`/`gm spell`)
   - ✅ Configuration extended with spell paths support
   - ✅ Spell data model created with JSON schema validation
   - ✅ Sample spell data created & validated (9 spells, all schema-compliant, aligned with schema)
   - ✅ Query module implemented (20+ tests passing)
   - ✅ I/O module implemented (ALL tests passing, 49/49 ✅)
   - ✅ CLI commands implemented: `gm spell find/list/palette` all working
   - ✅ Chat palette generation with補助フラグ support fully implemented (28 tests)
   - ✅ Spell palette CLI command with clipboard support (T034 complete)
   - ✅ Spell CLI integration test suite (10 tests, 18 total app tests passing)
   - ✅ Documentation: README.md updated with new syntax and Spell Management section (T036)
   - ✅ I/O tests fixed: Multiple spell file loading tests now passing (T036.5)
   - ✅ Release build: SUCCESS
   - Next: T037 (final Phase 3 commit by kazuyasi)

## 🚧 In Progress
- [ ] T037 Commit spell system Phase 3 — READY FOR kazuyasi
      - Status: All implementation, testing, and documentation complete
      - Owner: kazuyasi (git operations)
      - Task: Final git commit for Phase 3
      - Blockers: None
      - Changes to commit:
        * README.md (T036): Updated command syntax and added Spell Management section
        * io.rs (T036.5): Fixed spell I/O multiple file loading tests
        * tasks.md: Updated task tracking and completion status
      - Expected: All tests pass, release build succeeds

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

- [x] T034 Spell CLI command: palette display — 2025-12-19
      - Status: COMPLETE ✅

- [x] T035 Spell CLI integration test suite — 2025-12-19
      - Status: COMPLETE ✅

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



- [x] T036 Documentation: README.md update — 2025-12-19
      - Status: COMPLETE ✅
      - Task: Updated README.md with new command syntax and Spell Management section
      - Completed:
        * Refactored "Basic Commands" → "Basic Monster Commands" ✅
        * Updated all monster commands: `gm find` → `gm monster find`, etc. ✅
        * Added comprehensive "Spell Management" section ✅
        * Documented spell search commands (find/list/palette) with examples ✅
        * Documented spell palette output format (補助/通常 patterns) ✅
        * Explained magic category formatting rules ✅
        * Updated all export examples to `gm monster select` syntax ✅
      - Impact: README fully up-to-date with Phase 3 implementation

- [x] T036.5 Fix spell I/O multiple file loading tests — 2025-12-19
      - Status: COMPLETE ✅
      - Location: rust/core/src/io.rs (lines 746-821)
      - Fixed tests: 2 (test_load_multiple_spells_json_arrays, test_load_multiple_spells_json_arrays_with_empty_file)
      - Root cause: Test JSON used old field names (School/Lv/Target/Cost)
      - Solution: Updated test JSON data to match actual Spell struct definition
        1. Changed "School" → "category"
        2. Changed flat "Lv" → { "kind": "value", "value": N }
        3. Changed flat "MP"/"Cost" → { "kind": "value", "value": N }
        4. Changed "Target" → "対象" with proper nested structure
      - Test results: Both tests now PASS ✅
      - Release build: SUCCESS ✅
      - All I/O tests: 49/49 PASS ✅

- [ ] T035.5 Spell select command: Query by category and level — Claude (Low Priority)
     - Implement `gm spell select -l <level> -c <category>` command
     - Similar to monster select but for spells
     - Support filters: -l (level), -c (category)
     - Return JSON array of matching spells
     - Unit tests: 10+ for spell select command
     - Effort: Low (building on existing spell query functions)

- [ ] T038 Spell palette: Support partial name matching & flexible range field — Claude (Low Priority)
     - Status: Enhancement based on specification clarifications
     - Owner: Claude
     - Task: Modify `gm spell palette` command with 2 improvements:
     
     **1. Partial name matching:**
     - Current behavior: Exact name match only (returns error if not found)
     - Requested behavior: Partial name match like `gm spell find` command
     - Implementation:
       1. Refactor palette command to use spell_find_by_name() for name resolution
       2. Handle multiple matches: return error with suggestions or first match
       3. Add tests for partial match scenarios
     
     **2. Flexible range field handling (射程):**
     - Current: Assumes exact field name "射程"
     - Required: Support both "射程" and "射程(m)" field names
     - Implementation:
       1. Add format_range() helper function
       2. Check for "射程" field first, fallback to "射程(m)"
       3. Output value as-is (no transformation)
       4. Add tests for both field name variations
     
     - DESIGN_GUIDE.md: Updated with flexible range field documentation
     - Impact: Better data source compatibility, UX consistency
     - Estimated tests: 5-10 for partial matching + 3-5 for range field variations
     - Effort: Low (reuse existing query logic + simple field detection)

- [ ] T037 Commit spell system Phase 3 — kazuyasi
     - Status: READY FOR COMMIT ✅
     - All spell functionality implemented and tested ✅
     - Documentation complete (README.md + DESIGN_GUIDE.md) ✅
     - All tests passing (212/212 core tests, 18/18 app tests) ✅
     - Release build successful ✅
     - Ready for final git commit

---

## ✅ Done (Recent 18)
- [x] T035 Spell CLI integration test suite — 2025-12-19
        - Description: Added 10 comprehensive spell functionality tests in app/src/main.rs. Tests cover spell search (by name exact/partial, by category, multi-filter), palette generation (support/regular/area targets), data persistence, and schema compliance. All 18 app tests passing (8 export + 10 spell tests). Test data paths fixed for cross-directory execution.

- [x] T034 Spell CLI command: palette display — 2025-12-19
        - Description: Implemented `gm spell palette <name> [-c/--copy]` command with full integration. Added copy_to_clipboard() helper supporting macOS (pbcopy), Linux (xclip), and Windows (clip). Integrated palette.rs generate_spell_palette() into CLI. Sample spell data aligned with schema requirements (射程/末尾 values compliant). All tests passing; release build successful.

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

## ⚠️ Known Issues & Tech Debt

### ✅ RESOLVED: I/O Tests - Spell Multiple File Loading (T036.5)
- **Location**: `rust/core/src/io.rs` - `test_load_multiple_spells_json_arrays*` (2 tests)
- **Status**: ✅ FIXED (2025-12-19)
- **Resolution Details**: 
  - Root cause: Test JSON used outdated field names (School, flat Lv/MP, Target)
  - Fix: Updated test data to match actual Spell struct schema with proper nesting
  - Test results: Both tests now PASS ✅
  - Impact: All 49 I/O tests passing, 212/212 core tests passing
- **Notes**: Schema compliance now verified. Spell I/O operations fully tested and working.

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

### Spell Palette Range Field Specification
- **Issue**: Palette output includes "射程" field but implementation assumes exact field match
- **Clarification**: Either "射程" or "射程(m)" is acceptable - no exact field name requirement
- **Impact**: Sample data validation and palette generation should accept either format
- **Status**: ✅ DESIGN_GUIDE.md updated (line 774); T038 will implement flexible range field detection
- **Implementation**: format_range() helper to check both "射程" and "射程(m)" field names

### Spell Palette Search Behavior
- **Current**: `gm spell palette <name>` requires exact spell name match
- **Requested**: Change to partial/fuzzy name matching (like `gm spell find`)
- **Impact**: Better user experience, consistent with find/list command behavior
- **Status**: ⏳ Planned for T038 implementation (combined with range field enhancement)
