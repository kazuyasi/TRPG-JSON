# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2026-11-01 JST (T053 TUI 1行入力のカーソル移動; v0.18.0)
- Responsibilities: kazuyasi (specification/approval/git operations) / Claude (proposal/implementation/testing)
- Status: Phase 2.5 COMPLETE (T013-T027). Phase 3 COMPLETE (T028-T037). Phase 3.5 COMPLETE (T038-1/2/final). Phase 3.6 COMPLETE (T038.6, T040, T040.5). Phase 3.7 COMPLETE (T046, T047). UX improvements complete with enhanced error messages and statistics commands. Workspace version inheritance configured. Test suite: 291 total tests (251 core + 40 app). Ready for commit.

---

## 🔥 Priority Now
- **Phase 3.7: UX Improvements** ✅ COMPLETE
   - Completed: T046 - Error message improvements with filter conditions display (2025-12-28)
   - Completed: T047 - Statistics command for dataset overview and distribution analysis (2025-12-28)
   - Bonus: Workspace version inheritance configured for easier maintenance
   - Status: All Phase 3.7 tasks complete and ready for commit

## 🚧 In Progress
- 実装中のタスクはなし（T053 は 2026-11-01 に `claude/T053` で完了、マージ待ち）
- 次の候補（Phase 4 内、優先度順）: T054 データ再読み込みキー（要検討）→ T051 選択リストペイン

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



- [x] T037 Commit spell system Phase 3 — 2025-12-27
     - Status: COMPLETE ✅
     - All spell functionality implemented and tested ✅
     - Documentation complete (README.md + DESIGN_GUIDE.md) ✅
     - All tests passing (221/221 core tests, 28/28 app tests) ✅
     - Release build successful ✅
     - Git commits: All Phase 3 and 3.5 changes committed and pushed to main
     - Final commit: "refactor(CLI): Remove legacy direct commands, require monster/spell subcommands" (a9bd604)

---

## ✅ Done (Recent 30)
- [x] T047 Statistics command: Dataset overview and distribution analysis — 2025-12-28
         - Description: Implemented dataset statistics commands for monsters and spells. Added stats.rs module with MonsterStats and SpellStats structs. Monster stats include level distribution (1-4/5-8/9-12/13-16/17+), category distribution (Top 5), and numeric field ranges (hit_rate, dodge, damage, armor, life/mental resistance). Spell stats include level distribution (1-5/6-10/11-15/16+), school distribution (Top 5), and level/rank type counts. Added gm monster stats and gm spell stats CLI commands. Implemented 8 comprehensive unit tests. Bonus: Configured workspace version inheritance for easier maintenance (version now managed in single location). All 291 tests passing (251 core + 40 app). Release build successful. Provides quick dataset overview and helps identify data coverage gaps.

- [x] T046 Error message improvements: Display applied filter conditions — 2025-12-28
         - Description: Enhanced error messages to display applied filter conditions for better debugging. Implemented format_spell_filter_conditions() and format_monster_filter_conditions() helper functions. Updated error handling in handle_spell_find_command(), handle_spell_list_command(), handle_spell_palette_command(), handle_monster_find_command(), handle_monster_list_command(), and handle_select_command(). Added 8 new unit tests (4 spell + 4 monster filter formatting tests). All 283 tests passing (243 core + 40 app). Release build successful. Error messages now show which filters were applied when no matches are found, significantly improving UX during data addition work.

- [x] T040.5 Spell schoolVariant/god query support: Implement advanced filtering — 2025-12-28
         - Description: Implemented schoolVariant and god filtering for spell queries with 16 new tests (12 unit + 4 integration). Added spell_find_by_school_variant() and spell_find_by_god() functions in query.rs. Extended spell_find_multi() from 5 to 7 parameters (added schoolVariant, god). Added -v/--school-variant and -g/--god CLI flags to spell find/palette commands. Updated README.md with comprehensive filter examples and options table. All 275 tests passing (243 core + 32 app). Release build successful. Completes DESIGN_GUIDE.md specification implementation.

- [x] T040 Spell level/rank query support: Implement rank-based filtering — 2025-12-27
         - Description: Implemented rank-based filtering for spell queries with 11 new unit tests. Added spell_find_by_rank(), extract_spell_rank(), has_rank_field() functions in query.rs. Extended spell_find_multi() to 5 parameters (added rank). Added -r (rank) CLI flag to spell find/palette commands with mutual exclusivity handling (level takes priority). Level filter (-l) searches Lv.kind: "value"/"value+" only. Rank filter (-r) searches Lv.kind: "rank" only. Added sample rank-based spells (FairyMagic_Rank2, FairyMagic_Rank3). All 231 tests passing (50 query + 28 app). Release build successful.


- [x] T038.6 Level/Rank/God field specification documentation — 2025-12-27
         - Description: Added comprehensive "Spell and Skill Data Structure Specification" section to DESIGN_GUIDE.md (lines 728-816). Documented Lv.kind variants (value/value+/rank) with query behavior, CLI flags, and use cases. Defined schoolVariant and god field query specifications with exact match semantics. Included examples with generic placeholders (神名) following data usage guidelines. Added Data Usage Guidelines section to AGENTS.md to prevent future copyright issues. Ready for T040 implementation.

- [x] T037 Commit spell system Phase 3 — 2025-12-27
         - Description: Final Phase 3 git commit. All spell system implementation, testing, and documentation complete. CLI refactored to require monster/spell subcommands. Legacy direct commands removed. All 221 core tests + 28 app tests passing. Changes pushed to main branch. Phase 3 fully deployed.

- [x] T038 Spell palette: Multi-filter output with integrated select functionality — 2025-12-19
         - Description: Complete redesign of `gm spell palette` command. Removed mandatory positional argument, added `-n/-l/-c/-y` optional filter flags. Refactored to use `spell_find_multi()` for flexible multi-filter queries. Support multi-line output for all matching spells, consolidating select functionality into palette. Clipboard copy (`--copy` or `-y` flag) copies first matched spell only. File output via shell redirection. Require minimum one filter for safety. Added 7 new integration tests covering single/multi-filter scenarios, no-match errors, and precision. All 221 core tests + 28 app tests passing. Release build successful. Eliminates need for separate T035.5 select command.

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

## 📋 Backlog (Phase 3.6 - Advanced Filtering System)

- [x] T040 Spell level/rank query support: Implement rank-based filtering — 2025-12-27
      - Status: COMPLETE ✅
      - Owner: Claude
      - Task: Implement rank filtering in spell query module
      
      **Implementation scope:**
      1. ✅ Add `spell_find_by_rank()` function in query.rs
      2. ✅ Update `spell_find_multi()` to support rank filtering alongside level (5 parameters)
      3. ✅ Update CLI find/palette commands: add `-r` (rank) filter flag
      4. ✅ Handle level vs rank distinction in queries
      5. ✅ Add 11 unit tests for rank queries (all passing)
      6. ✅ Add integration tests for rank filtering in palette command
      7. ✅ Update README.md with rank filtering examples
      
      **New features:**
      - ✅ `gm spell find -r 5` — find spells with rank 5
      - ✅ `gm spell palette -r 3` — show all rank 3 spells
      
      **Impact:**
      - ✅ Enables flexible spell filtering for games using rank-based systems
      - ✅ Release build successful

- [x] T040.5 Spell schoolVariant/god query support: Implement advanced filtering — 2025-12-28
      - Status: COMPLETE ✅
      - Owner: Claude
      - Task: Implement schoolVariant and god filtering in spell query module
      
      **Implementation scope:**
      1. ✅ Add `spell_find_by_school_variant()` function in query.rs
      2. ✅ Add `spell_find_by_god()` function in query.rs
      3. ✅ Update `spell_find_multi()` to 7 parameters (added schoolVariant, god)
      4. ✅ Update CLI find/palette commands: add `-v` (schoolVariant) and `-g` (god) filter flags
      5. ✅ Add 12 unit tests for schoolVariant and god queries
      6. ✅ Add 4 integration tests for schoolVariant/god filtering
      7. ✅ Update README.md with comprehensive filter examples and options table
      
      **New features:**
      - ✅ `gm spell find "name" -v "特殊"` — find spells with schoolVariant "特殊"
      - ✅ `gm spell find "name" -v "特殊" -g "神名"` — find spells with specific god
      - ✅ `gm spell palette -v "特殊"` — show all special variant spells
      - ✅ `gm spell palette -g "神名"` — show all deity-specific spells
      
      **Testing:**
      - ✅ Unit tests: 12 tests (schoolVariant/god exact match, combined filters, no match, extract helpers)
      - ✅ Integration tests: 4 tests (schoolVariant only, god only, palette integration)
      - ✅ All 275 tests passing (243 core + 32 app)
      
      **Impact:**
      - ✅ Completes spell query system with full DESIGN_GUIDE.md specification coverage
      - ✅ Enables filtering for special spell variants and deity-specific spells
      - ✅ Release build successful

## 📋 Future Phases (Post Phase 3)

### Phase 3.7: UX Improvements
- [x] T046 Error message improvements: Display applied filter conditions — 2025-12-28
      - Status: COMPLETE ✅
      - Owner: Claude
      - Priority: Medium (improves user experience during data addition work)
      - Task: Enhance error messages to show which filters were applied
      
      **Previous behavior:**
      ```
      エラー: マッチするスペルが見つかりません
      ```
      
      **Improved behavior:**
      ```
      エラー: 以下の条件でマッチするスペルが見つかりません
        - name: "ファイア"
        - level: 3
        - schoolVariant: "特殊"
        - god: "神名"
      ```
      
      **Implementation scope:**
      1. ✅ Add `format_spell_filter_conditions()` helper function
      2. ✅ Add `format_monster_filter_conditions()` helper function
      3. ✅ Update `handle_spell_find_command()` error handling
      4. ✅ Update `handle_spell_list_command()` error handling
      5. ✅ Update `handle_spell_palette_command()` error handling
      6. ✅ Update `handle_monster_find_command()` error handling
      7. ✅ Update `handle_monster_list_command()` error handling
      8. ✅ Update `handle_select_command()` error handling
      9. ✅ Add 8 unit tests for error message formatting (4 spell + 4 monster)
      10. ✅ All 283 tests passing (243 core + 40 app)
      11. ✅ Release build successful
      
      **Completed:** 2025-12-28
      
      **Benefits:**
      - ✅ Easier debugging when queries return no results
      - ✅ Better UX when working with complex filter combinations
      - ✅ Particularly helpful during spell data addition work (currently 30% complete)

- [x] T047 Statistics command: Dataset overview and distribution analysis — 2025-12-28
      - Status: COMPLETE ✅
      - Owner: Claude
      - Priority: Medium (useful for understanding dataset composition)
      - Task: Implement `stats` command for monsters and spells
      
      **Implemented features:**
      ```bash
      # Monster statistics
      gm monster stats
      # Output example (actual data: 688 monsters):
      # モンスター統計:
      #   総数: 688
      #   レベル分布:
      #     Lv 1-4:  164 (23.8%)
      #     Lv 5-8:  224 (32.6%)
      #     Lv 9-12: 141 (20.5%)
      #     Lv 13-16: 70 (10.2%)
      #     Lv 17+:   83 (12.1%)
      #   カテゴリ分布 (Top 5):
      #     蛮族: 172 (25.0%)
      #     動物:  95 (13.8%)
      #   数値フィールド範囲:
      #     命中力: 5～111
      #     回避力: 7～42
      #     打撃点: -2～38
      #     防護点: 0～34
      #     生命抵抗力: 0～46
      #     精神抵抗力: 0～47
      
      # Spell statistics
      gm spell stats
      # Output example (actual data: 226 spells):
      # スペル統計:
      #   総数: 226
      #   レベル分布:
      #     Lv 1-5:  101 (44.7%)
      #     Lv 6-10:  64 (28.3%)
      #     Lv 11-15: 51 (22.6%)
      #     Lv 16+:    0 (0.0%)
      #   系統分布 (Top 5):
      #     森羅:  60 (26.5%)
      #     真語:  54 (23.9%)
      #     召異:  46 (20.4%)
      #   種別:
      #     レベル型: 216 (95.6%)
      #     ランク型:  10 (4.4%)
      ```
      
      **Implementation scope:**
      1. ✅ Add `stats.rs` module in core for statistics calculation
      2. ✅ Implement `MonsterStats` and `SpellStats` structs
      3. ✅ Add `gm monster stats` CLI command
      4. ✅ Add `gm spell stats` CLI command
      5. ✅ Add distribution calculation functions (level, category, school)
      6. ✅ Add numeric field range calculation (monster hit_rate, dodge, damage, armor, resistances)
      7. ✅ Add formatted output functions
      8. ✅ Add 8 unit tests for statistics calculation
      9. ✅ All 291 tests passing (251 core + 40 app)
      10. ✅ Release build successful
      
      **Completed:** 2025-12-28
      
      **Benefits:**
      - ✅ Quick overview of dataset composition
      - ✅ Identify gaps in data coverage (e.g., no Lv 16+ spells)
      - ✅ Useful for balancing data across levels/categories
      - ✅ Helps plan future data addition work

### Phase 4: Advanced Features
- [ ] T041 Phase 4: Skill system implementation (流派特技)
- [ ] T042 Phase 4: Fairy magic system implementation (妖精魔法)
- [x] T043 Phase 4: Chat palette export to clipboard — 2026-10-31
      - Status: COMPLETE ✅ (covered by T048: CCFolia export copies the chat palette to the clipboard via `commands`; spell palette clipboard copy already available via `gm spell palette --copy`)
- [x] T048 Phase 4: CCFolia (ココフォリア) clipboard export for monsters — 2026-10-31
      - Status: COMPLETE ✅
      - Owner: Claude
      - Version: 0.8.0
      - Task: Copy a monster to the clipboard as a CCFolia character piece

      **Usage:**
      ```bash
      gm monster select -n "モンスター名" --export ccfolia             # copy first core part
      gm monster select -n "モンスター名" --export ccfolia --part 2    # copy 2nd part
      gm monster select -n "モンスター名" --export ccfolia --output piece.json
      ```

      **Implementation scope:**
      1. ✅ Add `export/ccfolia.rs` in core (`{"kind":"character","data":{...}}` per part)
      2. ✅ Field mapping: status = HP/MP/防護点, params = 命中力/回避力/生命抵抗力/精神抵抗力 (-7) + 打撃点 + ダメージ軽減, memo = 知名度/弱点値/先制値/弱点/移動速度/特殊能力
      3. ✅ Reuse Udonarium `PartNamer` / `DataTransformer` and the same chat palette
      4. ✅ CLI: `--export ccfolia` (clipboard by default, `--output` writes file), `--part <n>` (1-based)
      5. ✅ Single monster only (CCFolia accepts one character per paste); multi-part defaults to first core part
      6. ✅ Format verified against official spec: https://docs.ccfolia.com/developer-api/clipboard-api (beta v1.19.0)
      7. ✅ 6 unit tests added; all 260 tests passing (258 core + 2 app)
      8. ✅ Release build successful; manually verified paste into a CCFolia room
- [x] T049 Phase 4: TUI mode (`gm tui`) — design drafted 2026-10-31、完了 2026-11-01
      - Status: ✅ 完了（T049a〜T049g すべて実装済み。後続は T050 / T051）
      - Owner: Claude
      - Design doc: TUI_DESIGN.md
      - Goal: search → inspect → copy/export with keyboard only (ratatui + crossterm)
      - Planned subtasks: T049a refactor (Result-returning export helpers) / T049b skeleton / T049c monster tab / T049d CCFolia copy / T049e spell tab / T049f modals & docs / T049g level & category filters
      - Follow-up: T050 export modal, stats view, add/delete in TUI
      - [x] T049a 前準備リファクタ（0.8.1）— 2026-10-31
        1. ✅ `commands::monster::ccfolia_json(monster, part) -> Result<CcfoliaPiece, CcfoliaError>` を切り出し
        2. ✅ `commands::monster::export_monsters(monsters, format, output) -> Result<(), ExportError>` を切り出し
        3. ✅ CLI ハンドラは上記を呼んで表示・終了するだけに変更（出力文言・終了コードは不変）
        4. ✅ app に `thiserror` を追加。エラーの `Display` は CLI 文言と同一
        5. ✅ 単体テスト 7 件追加。全 267 件通過（core 258 + app 9）
      - [x] T049b TUI 骨格（0.9.0）— 2026-10-31
        1. ✅ `gm tui [monster|spell]` サブコマンドを追加（既存 CLI サブコマンドは無変更）
        2. ✅ 依存に `ratatui 0.30.2` を追加（crossterm は `ratatui::crossterm` 経由）
        3. ✅ `app/src/tui/`: `mod.rs`（起動・イベントループ）/ `app.rs`（状態）/ `action.rs`（キー→Action）/ `update.rs`（純粋な状態遷移）/ `ui/mod.rs`（描画）
        4. ✅ `ratatui::init`/`restore` で raw mode・alternate screen・panic hook を処理。stdin/stdout が TTY でなければ起動せずエラー終了
        5. ✅ データは端末切替前に読み込む（モンスター失敗は stderr に出して終了、スペル失敗はステータス行に表示しスペルタブを「データなし」）
        6. ✅ 空の2タブ画面（ヘッダー: タブ＋件数、本文、ステータス行、キーヘルプ行）。`1`/`2` でタブ切替、`q`/`Ctrl-c` で終了。高さ 15 行未満は警告表示
        7. ✅ 単体テスト 10 件追加（Action 変換・update・TestBackend 描画 80/120 桁）。全 277 件通過（core 258 + app 19）
        8. ✅ 疑似端末で起動 → タブ切替 → 終了、alternate screen からの復帰を確認
      - [x] T049c モンスタータブ（0.10.0）— 2026-10-31
        1. ✅ 検索バー: 名前の部分一致（`core::query::find_multi` を使用）で入力のたびに即時再検索。`/` でフォーカス、`Enter`/`Esc` でリストへ、`Backspace`・`Ctrl-u`（クリア）。フォーカス中は端末カーソルを入力位置に置く
        2. ✅ 結果リスト: Lv 昇順 → 名前順。カテゴリ列は「アンデッド」（全角5文字）が収まる 10 桁、残りを名前列に割り当て。`↑↓`/`jk`・`PgUp`/`PgDn`・`g`/`G`。ヒット 0 件は `format_monster_filter_conditions` の条件一覧を表示。再検索後も選択中の項目がヒットしていれば選択を保持
        3. ✅ 詳細ペイン: 見出し・知名度/弱点値・先制値・抵抗・移動（-1 は `-`）・弱点、部位表（MP -1 は `--`、コアは `*`）、選択部位の部位特殊能力・共通特殊能力・備考（空の節は省略、折り返し＋`PgUp`/`PgDn`/`g`/`G` でスクロール）
        4. ✅ 部位選択: 詳細ペインで `↑↓`/`jk`。既定はココフォリア出力と同じ最初のコア部位。`selected_part` は `part[]` の index で、T049d では `+1` して `ccfolia_json` に渡せばよい
        5. ✅ `Tab`/`Shift-Tab` でペイン移動（検索 → リスト → 詳細）、フォーカス中のペインは枠を強調。キーヘルプ行はフォーカスごとに切り替え。ヘッダーに「総件数中 ヒット件数」
        6. ✅ 新規: `tui/format.rs`（表示用整形の純粋関数）、`tui/ui/{search,list,detail}.rs`。曖昧幅の記号を避け、節見出しは `【】`、選択記号は `>`
        7. ✅ 単体テスト 22 件追加（整形・Action 変換・update・TestBackend 描画 80/120 桁）。全 299 件通過（core 258 + app 41）
        8. ✅ 疑似端末で起動 → 検索 → 詳細ペイン移動 → 終了を確認
      - [x] T049d ココフォリアコピー（0.11.0）— 2026-10-31
        1. ✅ `c`（結果リスト・詳細ペイン）で選択中モンスターの選択部位をココフォリア形式でクリップボードへコピー。`selected_part + 1` を `ccfolia_json` に渡す。検索バーでは文字入力、スペルタブでは無効
        2. ✅ `update` を `Option<Effect>` 返却に変更し、新規 `tui/effect.rs`（`Effect::CopyToClipboard`）でクリップボード書き込みを実行。`update` は純粋なまま
        3. ✅ ステータス行: 成功 `"…" をココフォリア形式でクリップボードにコピーしました`、未選択は情報、`CcfoliaError`・クリップボード失敗はエラー（CLI と同じ文言）
        4. ✅ キーヘルプ行に `c:ココフォリア` を追加（80 桁に収まるよう短縮）
        5. ✅ 単体テスト 10 件追加（Action 変換・update のコピー要求・Effect 実行の成功/失敗・描画）。全 309 件通過（core 258 + app 51）
        6. ✅ 疑似端末で起動 → 詳細ペインで `c` → ステータス行の成功表示とクリップボードの JSON を確認
      - [x] T049e スペルタブ（0.12.0）— 2026-10-31
        1. ✅ スペルタブにモンスタータブと同じ検索バー・結果リスト・詳細ペインを実装（名前の部分一致は `core::query::spell_find_multi` を使用）。検索語・選択・詳細スクロールはタブごとに保持（`ListView` に `detail_scroll` を移動）
        2. ✅ 結果リスト: Lv 列は `9` / `5+` / `R2`、並びはレベル系昇順 → ランク昇順 → 名前順、右端は系統。ヒット 0 件は `format_spell_filter_conditions` の条件一覧、データなしは「スペルデータがありません」。ヘッダーは「総件数中 ヒット件数」
        3. ✅ 詳細ペイン: 見出し（Lv/ランク・系統・schoolVariant・god）、MP・時間／対象／射程(`術者(30m)` 併記)・形状／抵抗・属性、【効果】。チャットパレットはプレビューしない。`↑↓`/`jk`/`PgUp`/`PgDn`/`g`/`G` でスクロール
        4. ✅ `y`: 選択中スペルのパレットをコピー（ステータス行に先頭部分を表示）。`Y`: ヒット全件を改行区切りでコピー（生成失敗は除外して件数・名前を表示、全件失敗はエラー）。モンスタータブでは無効、`c` はスペルタブで無効
        5. ✅ core: 表示に再利用するため `export::palette::{format_mp, format_target, format_duration}` を `pub` に
        6. ✅ 新規: `tui/format.rs` に `SpellLevel` / `spell_title` / `spell_summary` / `spell_range` / `spell_sections`。キーヘルプ行（スペル用）を追加し 80 桁に収まることをテスト
        7. ✅ 単体テスト 20 件追加（整形・Action 変換・update のコピー要求/失敗/タブ別無効・描画 80/120 桁）。全 329 件通過（core 258 + app 71）
        8. ✅ 疑似端末（サンプル設定）で起動 → 検索 → `y` / `Y` → ステータス行の成功表示とクリップボードのパレットを確認
      - [x] T049f モーダル・狭い端末対応・README（0.13.0）— 2026-10-31
        1. ✅ ヘルプモーダル（`?`）: 全キーバインドと検索構文（段階1）を節ごとに表示。内容は新規 `tui/help.rs` の定数で持ち、キー列の桁をそろえる
        2. ✅ JSON ビューモーダル（`J`）: 選択中のモンスター／スペルを `serde_json::to_string_pretty` で整形して表示。`y` で JSON 全文をコピー。未選択は情報表示
        3. ✅ モーダル共通: 本文領域に重ねて表示（ステータス行・キーヘルプ行は隠さない）。`↑↓`/`jk`/`PgUp`/`PgDn`/`Space`/`g`/`G` でスクロール、`Esc`/`q`/`Enter`/開いたキーで閉じる、`Ctrl-c` で終了。表示中は他のキーを受け付けない。新規 `tui/ui/modal.rs`、`App.modal: Option<Modal>`
        4. ✅ 狭い端末（幅 100 桁未満）: 結果リスト ⇔ 詳細の1ペイン表示。`Enter`（結果リスト → 詳細）/ `Esc`（詳細 → リスト）で切り替え（`Action::FocusDetail` を追加）
        5. ✅ キーヘルプ行に `Enter:詳細` `J:JSON` `?:ヘルプ` を追加し、80 桁に収まるよう `PgUp/Dn`・`Tab`・`1/2` はヘルプへ移動。モーダル用のキーヘルプ行を追加
        6. ✅ 見出し（`【】`）と折り返し行数の見積もりを `ui/mod.rs` に共通化（詳細ペインとモーダルで共用）
        7. ✅ README に「TUI モード（`gm tui`）」の節を追加（日本語）。TUI_DESIGN.md §4.3 / §4.6 / §6 / §7.1 を更新
        8. ✅ 単体テスト 15 件追加（Action 変換・update のモーダル開閉/スクロール/コピー/操作の遮断・描画 60/80/99/100/120 桁）。全 344 件通過（core 258 + app 86）
        9. ✅ 疑似端末（サンプル設定、120×30 と 80×24）で `?` → `G` → `q` → `J` → `y` → `Esc` → `Enter` → `2` → `J` → `q` → `q` を確認（ステータス行の成功表示とクリップボードの JSON を確認）
      - [x] T049g 検索構文 段階2・`←→` タブ切替（0.14.0）— 2026-11-01
        1. ✅ 新規 `tui/query.rs`: `parse_monster_query` / `parse_spell_query`（純粋関数）と `MonsterQuery` / `SpellQuery`。モンスターは `lv:` `cat:`/`c:`、スペルは `lv:` `school:`/`s:`。判定は core の `find_multi` / `spell_find_multi`（CLI と同じ）
        2. ✅ フリーワードは名前の部分一致（複数は AND）。キーは英字のみ・大文字小文字無視・全角 `：` も可。英字以外の接頭辞（`テスト:改` など）はフリーワード
        3. ✅ 解釈できない条件（空の値・整数でない `lv:`・重複・タブで使えないキー）は検索バーを黄色にして `[理由]` を表示し、直前の結果を残す（`ListView.query_error`）
        4. ✅ ヒット 0 件の条件一覧に level / category / school を表示。未入力時の案内をタブ別の入力例に変更
        5. ✅ 設計書 §6 のうち未実装だった `←`/`→` のタブ切り替えを追加（結果リスト・詳細ペイン。検索バー・モーダルでは無効）
        6. ✅ ヘルプモーダルの「検索構文」節を段階2の内容に更新。README に「検索構文」の節を追加。TUI_DESIGN.md §4.3 / §5 / §6 を更新
        7. ✅ 単体テスト 15 件追加（パーサ 9・update 3・描画 3。Action 変換のテストに `←→` を追記）。全 359 件通過（core 258 + app 101）
        8. ✅ 疑似端末（サンプル設定、100×24）で `lv:abc` の警告 → `c:アンデッド` → `テ lv:6`（0 件の条件一覧）→ `→` でスペルタブ → `s:x` → `?` を確認
- [x] T050 Phase 4: TUI エクスポートモーダル・stats 画面 — 2026-11-01 着手、完了 2026-11-01
      - Status: ✅ 完了（T050a〜T050c すべて実装済み。後続は T051）
      - Owner: Claude
      - Design doc: TUI_DESIGN.md §4.3 / §6 / §11
      - 範囲の決定（2026-11-01、Operator と合意）: 追加・削除は TUI に入れない。追加は JSON を作って CLI の `gm monster add`、削除も破壊的操作なので CLI の `gm monster delete` で行う（TUI_DESIGN.md Q7）
      - Planned subtasks: T050a エクスポートモーダル（JSON / Udonarium）/ T050b Sheets エクスポート（TUI 一時停止 → OAuth → 復帰）/ T050c stats 画面
      - [x] T050a エクスポートモーダル（0.15.0）— 2026-11-01
        1. ✅ モンスタータブの結果リスト・詳細ペインで `e`: ヒット中の全件（結果リストの並び順）を書き出すモーダル。ヒット 0 件・スペルタブは情報表示で開かない
        2. ✅ 形式は JSON / Udonarium (ZIP)。`Tab`/`Shift-Tab`/`↑↓` で切り替え、出力先が前の形式の拡張子で終わっていれば付け替え
        3. ✅ 出力先は文字入力（`Backspace`・`Ctrl-u`）。入力フォーム専用のキー解釈 `action::from_form_key`（`q` なども文字として入る）。`Enter` で実行、`Esc` でキャンセル、空なら「出力先のパスを入力してください」
        4. ✅ `Effect::Export` → `commands::monster::export_monsters`（CLI と共通）。先頭の `~` / `~/` はホームへ展開。成功・失敗の文言は CLI と同じ。前回の形式と出力先を `App.last_export` に保持
        5. ✅ Udonarium エクスポーターが stderr に書く進捗で画面が崩れないよう、実行後に全面を描き直す（`clear_region(All)` + `swap_buffers`。`Terminal::clear` はカーソル位置の問い合わせに応答しない端末でエラーになるため使わない）
        6. ✅ キーヘルプ行に `e:出力`（80 桁に収めるため詳細ペインの `PgUp/Dn:本文` はヘルプへ移動）。ヘルプモーダルに「エクスポート」節。README に「エクスポート（`e`）」の節、TUI_DESIGN.md §4.3 / §6 / §11 / §13 を更新
        7. ✅ 単体テスト 16 件追加（Action 変換 2・update 8・Effect 実行 5・描画 1）。全 375 件通過（core 258 + app 117）
        8. ✅ 疑似端末（サンプル設定、100×24）で `e` → パス入力 → `Enter`（JSON 20 件）→ `e`（前回のパスを引き継ぎ）→ `Tab`（`.zip` に付け替え）→ `Enter`（ZIP に XML 22 件）を確認
      - [x] T050c 統計モーダル（0.16.0）— 2026-11-01
        1. ✅ 結果リスト・詳細ペインで `S`: 現在のタブの統計を core の `MonsterStats` / `SpellStats` で集計して表示（CLI の `stats` と同じ内容。core の見出し行は除く）
        2. ✅ 検索で絞り込み中は `【ヒット中 N 件（検索: …）】` → `【全件 M 件】` の順。検索構文エラーで直前の結果を残しているときは検索語を出さない。データ 0 件は情報表示
        3. ✅ スクロールは JSON ビューと共通。`Esc`/`q`/`Enter`/`S` で閉じる。`ModalKind::Stats` を追加
        4. ✅ ヘルプモーダル・キーヘルプ行（モーダル用）・README・TUI_DESIGN.md §4.3 / §6 を更新
        5. ✅ 単体テスト 7 件追加（Action 変換 1・update 5・描画 1）。全 382 件通過（core 258 + app 124）
        6. ✅ 疑似端末（サンプル設定、100×30）で `/` → `ゴ` → `Enter` → `S` → `G` → `q` を確認
      - [x] T050b Sheets エクスポート（0.17.0）— 2026-11-01
        1. ✅ エクスポートモーダルの形式に Google Sheets を追加（JSON → Udonarium → Sheets の順）。入力欄は「シートID」で、ファイルの出力先とは別に保持（`ExportForm.path` / `sheet_id`、`input()` / `input_mut()`）
        2. ✅ ID が空なら「スプレッドシート ID を入力してください」。`~` の展開はファイル形式のみ。`App.last_export` を `ExportForm` に変更し ID も引き継ぐ
        3. ✅ 実行は TUI を一時停止（`tui::run_suspended`: alternate screen を抜けて raw mode 解除 → `export_monsters` → 結果を表示 → `Enter` 待ち → 復帰・全面再描画）。OAuth の案内・進捗は通常の画面で読める。結果はステータス行にも表示（`Effect::needs_suspend`）
        4. ✅ `StatusKind::prefix()` を追加し、ステータス行と一時停止中の結果表示で共用
        5. ✅ README のエクスポート節、TUI_DESIGN.md §2 / §4.3 / §7.4 / §12 を更新
        6. ✅ 単体テスト 3 件追加（update 1・Effect 2）、既存テスト 2 件を Sheets 込みに更新。全 385 件通過（core 258 + app 127）
        7. ✅ 疑似端末（サンプル設定、100×24）で `e` → `Tab` `Tab` → 短い ID → `Enter` → 一時停止画面にエラーと案内 → `Enter` で TUI に戻りステータス行にエラーを確認
        8. ⚠️ 実際の OAuth 認証・Sheets API への書き込みは未確認（認証情報とスプレッドシートが必要。Operator による手動確認を推奨）
- [x] T052 CLI `monster add` / `delete` の保存先修正（fix, 0.17.1）— 2026-11-01 登録、完了 2026-11-01
      - Status: ✅ 完了（Known Issues の該当項目を解消）
      - 背景: Known Issues「`gm monster add` / `delete` が全ファイルの内容を最初のファイルへ保存する」
      - 実装:
        1. ✅ データファイルを1つずつ `io::load_json_array` で読み込み、対象のモンスターを含むファイルだけを書き戻す。`load_multiple_json_arrays` の結果は保存に使わない
        2. ✅ 削除: 同名が複数ファイルにあるときは、確認メッセージにファイルを列挙し、承認されたらすべてから削除（従来の「その名前をすべて消す」挙動を維持）
        3. ✅ 追加: 新規は最初のファイルの末尾へ（従来どおり）。同名の上書きは、該当ファイルそれぞれで同じ位置に置き換える（同じファイル内の重複は1件にまとめる）
        4. ✅ 処理本体を `add_monster_to_files` / `delete_monster_from_files`（確認はクロージャで受け取る）と `MonsterFileError` に分離し、CLI ハンドラは表示と終了だけを行う。読み込みエラーにはファイルのパスを含める。データファイル未設定でも panic しない
        5. ✅ 成功メッセージに書き換えたファイルを表示。README に保存先の説明、TUI_DESIGN.md §Q7 の既知の注意を更新
        6. ✅ 単体テスト 11 件追加（2ファイル構成: add 新規・add 上書き・add キャンセル・複数ファイルの上書き・delete 2 件・複数ファイルの削除・未発見とキャンセル・データファイルなし・置き換え関数）。全 395 件通過（core 258 + app 137）
        7. ✅ 実機（サンプルデータを 3 件ずつ 2 ファイルに分けた設定）で delete（2つ目のファイル）・add 上書き（2つ目）・add 新規（1つ目）・delete キャンセル・未発見を確認。他のファイルは変化なし
      - 見送り: `add` の保存先を選ぶオプション（`--file` など）は今回は追加しない（必要になったら別タスク）
- [x] T053 TUI 1行入力のカーソル移動（検索バー・エクスポートの入力欄）（feat, 0.18.0）— 2026-11-01 登録、完了 2026-11-01
      - Status: ✅ 完了
      - 背景: 今の入力欄は末尾への追加・削除しかできず、長いパスや検索語の途中を直せない。エクスポートモーダルの形式切り替えは `↑↓` / `Tab` のままにし、`←→` はカーソル移動に使う（2026-11-01 合意）
      - 未決事項の扱い: 範囲は検索バーとエクスポートの入力欄の両方（方針案1の共用部品に合わせた）。新しいキー操作の追加なので feat で 0.18.0 とした（Operator の確認待ち）
      - 実装:
        1. ✅ 1行入力の部品 `tui/input.rs` の `LineInput`（入力文字列＋文字単位のカーソル位置＋表示のずらし量）。`ListView.query` と `ExportForm.path` / `sheet_id` をこれに置き換え
        2. ✅ キー: `←` / `→` で1文字移動（全角も1文字）、`Home` / `End`（`Ctrl-a` / `Ctrl-e`）で先頭・末尾、`Backspace` はカーソルの前、`Delete` はカーソル位置を削除、文字はカーソル位置に挿入。解釈は `action::from_line_input_key` で検索バーとフォームが共用。Action に `DeleteCharForward` / `CursorLeft` / `CursorRight` / `CursorHome` / `CursorEnd` を追加
        3. ✅ 検索バーは入力が変わったときだけ再検索（カーソル移動では検索し直さない）。検索バー以外の `←→`（タブ切り替え）・`Home`/`End`（先頭・末尾）は従来どおり
        4. ✅ 枠に収まらない入力は、カーソルが見える位置まで横にずらして表示（ずらし量は描画時に書き戻し、なるべく動かさない）。省いた側に `<` / `>`。検索バーにも適用（従来は右端で切れていた）
        5. ✅ 端末カーソル（IME の変換窓の位置）を入力のカーソル位置に合わせる
        6. ✅ キーヘルプ行に `←→:カーソル`（80 桁に収めるため検索中の `↑↓:選択`、エクスポート中の `文字入力:出力先` は省略。ヘルプモーダルには記載）。ヘルプモーダル・README（「入力欄の編集」節）・TUI_DESIGN.md §4.3 / §6 を更新
        7. ✅ 単体テスト 11 件追加（LineInput 7・Action 変換 1・update 2・描画 1）、既存テスト 2 件を更新。全 406 件通過（core 258 + app 148）
        8. ✅ 疑似端末（サンプル設定、100×24）で `/` → `ゴリン`（0 件）→ `←←` → `ブ`（「ゴブリン」に絞り込み）→ `Enter` → `e` → `←`×5 → `_2` → `Home` → `x` → `Delete` → `Enter` で `xonsters_2.json`（ゴブリン 1 件）が書き出されることを確認
      - Operator 確認（実端末、2026-11-01）:
        1. ✅ 検索バーで日本語入力中、IME の変換窓が入力のカーソル位置に出る（`←` で途中に戻してから入力した場合も）
        2. ✅ 検索バーで `←` `→` `Home` `End` `Ctrl-a` `Ctrl-e` `Delete` が効き、検索バー以外では `←→` がタブ切り替えのまま
        3. ✅ 枠より長い入力が枠からはみ出さず、入力した分だけ末尾側から表示される（長い入力の途中・先頭へ戻したときの `<` / `>` とずれ方は明示的には未確認）
        4. ✅ エクスポートモーダルで `←→` はカーソル移動、`Tab` / `↑↓` は形式切り替え
- [ ] T054 TUI データ再読み込みキー — 2026-11-01 登録（要検討）
      - Status: 未着手・採否未定
      - 背景: TUI はデータを起動時に一度だけ読む。CLI の `add` / `delete` やエディタでデータを直したあと、反映するには TUI の再起動が必要
      - 方針案: `R` で設定のデータファイルを読み直し、検索語・タブ・できれば選択中の項目を保ったまま結果を作り直す。読み込みに失敗したら今のデータを残し、ステータス行にエラーを出す
- [ ] T051 Phase 4: TUI selection list pane (multi-select → bulk copy/export) — future, after T050 (see TUI_DESIGN.md §4.5)
- [ ] T044 Phase 4: Multi-system support (extend beyond SW2.5)
- [ ] T045 Phase 4: Skill/Fairy magic CLI commands (gm skill find/list/palette)

## 🚮 Canceled / Superseded
- [ ] T007 Data analysis feature implementation (deemed unnecessary) — 2025-09-14
- [ ] T035.5 Spell select command (superseded by T038) — Consolidated into palette command with multi-filter output

---

## ⚠️ Known Issues & Tech Debt

### ✅ RESOLVED: `gm monster add` / `delete` が全ファイルの内容を最初のファイルへ保存する
- **Location**: `rust/app/src/commands/monster.rs` - `handle_add` / `handle_delete`
- **Status**: ✅ 解消（2026-11-01、T052 / 0.17.1。対象のモンスターを含むファイルだけを読み書きする）。2026-11-01 に T050 の検討中に発見・再現確認
- **内容**: 設定の全モンスターファイルを `load_multiple_json_arrays` でまとめて読み込み、その配列を `data_paths[0]` に保存している。モンスターのデータファイルを複数設定していると、2つ目以降の内容が1つ目のファイルにも書き込まれ重複する（2ファイル構成で `delete` を実行し、2つ目の内容が1つ目へ複製されることを確認）
- **影響**: データファイルを1つだけ使っている場合は問題なし
- **対応案**: 対象のモンスターを含むファイルだけを読み書きする（追加は最初のファイル、または指定したファイルへ）

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
