# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-12-28 JST (T046 completed; Phase 3.7 UX improvements in progress)
- Responsibilities: kazuyasi (specification/approval/git operations) / Claude (proposal/implementation/testing)
- Status: Phase 2.5 COMPLETE (T013-T027). Phase 3 COMPLETE (T028-T037). Phase 3.5 COMPLETE (T038-1/2/final). Phase 3.6 COMPLETE (T038.6, T040, T040.5). Phase 3.7 STARTED (T046 done). Enhanced error messages with filter condition display implemented. Test suite: 283 total tests (243 core + 40 app). Ready for commit.

---

## 🔥 Priority Now
- **Phase 3.7: UX Improvements** 🚧 IN PROGRESS
   - Completed: T046 - Error message improvements with filter conditions display (2025-12-28)
   - Next: T047 - Statistics command for dataset overview and distribution analysis
   - Status: T046 complete and ready for commit

## 🚧 In Progress
- Currently no active implementation tasks
- T046 complete; ready for git commit

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

- [ ] T047 Statistics command: Dataset overview and distribution analysis
      - Status: Backlog
      - Owner: Claude
      - Priority: Medium (useful for understanding dataset composition)
      - Task: Implement `stats` command for monsters and spells
      
      **New features:**
      ```bash
      # Monster statistics
      gm monster stats
      # Output:
      # モンスター統計:
      #   総数: 150
      #   レベル分布:
      #     Lv 1-5:  45 (30.0%)
      #     Lv 6-10: 60 (40.0%)
      #     Lv 11+:  45 (30.0%)
      #   カテゴリ分布:
      #     蛮族:     50 (33.3%)
      #     魔法生物: 40 (26.7%)
      #     アンデッド: 30 (20.0%)
      #     動物:     30 (20.0%)
      
      # Spell statistics
      gm spell stats
      # Output:
      # スペル統計:
      #   総数: 200
      #   レベル分布:
      #     Lv 1-3:  80 (40.0%)
      #     Lv 4-7:  90 (45.0%)
      #     Lv 8+:   30 (15.0%)
      #   系統分布:
      #     神聖: 60 (30.0%)
      #     操霊: 50 (25.0%)
      #     妖精: 40 (20.0%)
      #     他:   50 (25.0%)
      #   種別:
      #     レベル型: 180 (90.0%)
      #     ランク型:  20 (10.0%)
      ```
      
      **Implementation scope:**
      1. Add `stats.rs` module in core for statistics calculation
      2. Implement `MonsterStats` and `SpellStats` structs
      3. Add `gm monster stats` CLI command
      4. Add `gm spell stats` CLI command
      5. Add distribution calculation functions (level, category, school)
      6. Add formatted output functions
      7. Add unit tests for statistics calculation
      8. Add integration tests for CLI commands
      
      **Estimated effort:** Small-Medium (2-3 hours)
      
      **Benefits:**
      - Quick overview of dataset composition
      - Identify gaps in data coverage
      - Useful for balancing data across levels/categories
      - Helps plan future data addition work

### Phase 4: Advanced Features
- [ ] T041 Phase 4: Skill system implementation (流派特技)
- [ ] T042 Phase 4: Fairy magic system implementation (妖精魔法)
- [ ] T043 Phase 4: Chat palette export to clipboard
- [ ] T044 Phase 4: Multi-system support (extend beyond SW2.5)
- [ ] T045 Phase 4: Skill/Fairy magic CLI commands (gm skill find/list/palette)

## 🚮 Canceled / Superseded
- [ ] T007 Data analysis feature implementation (deemed unnecessary) — 2025-09-14
- [ ] T035.5 Spell select command (superseded by T038) — Consolidated into palette command with multi-filter output

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
