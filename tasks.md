# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-12-13 JST (T013, T014, T015, T016 complete; Google Sheets exporter implementation ready to begin)
- Responsibilities: kazuyasi (specification/approval) / Claude (proposal/implementation)
- Status: Phase 2 complete, Phase 2.5 specification phase COMPLETE. Implementation phase: core + JSON exporter + CLI integration + Google Sheets format spec done. Next: Google Sheets exporter implementation (T015b-g)

---

## 🔥 Priority Now
- [x] T015 Phase 2.5: Google Sheets export format specification (P25-3a) — **SPECIFICATION COMPLETE** — 2025-12-13
      - Status: ✅ Complete. Specification defined in DESIGN_GUIDE.md (rows 561-703)
      - Details: Column mapping (A-AJ, AM, AM+1, AW, AW+1), cell merge structure, output format for 2-row-per-part layout, field transformations documented
      - Ready for: T015b-g (Google Sheets exporter implementation)

## 🚧 In Progress

---

## Backlog (Phase 2.5)
- [x] T013 Phase 2.5: Core export module structure (P25-1) — 2025-12-03
       - Description: Implemented core/src/export/mod.rs with DataExporter trait, ExportFormat enum (Json, GoogleSheets), ExportConfig struct. ExporterFactory for creating exporters. 9 unit tests for format parsing, config creation, factory methods.
- [x] T014 Phase 2.5: JSON exporter implementation (P25-2) — 2025-12-03
       - Description: Implemented core/src/export/json.rs with JsonExporter struct. Export Monster array to JSON file with pretty-printing. All 8 unit tests passing: single/multiple monsters, empty data, roundtrip, invalid directory, formatting preservation.
- [x] T015 Phase 2.5: Google Sheets export format specification (P25-3a) — 2025-12-13
       - Description: Completed detailed Google Sheets export format specification in DESIGN_GUIDE.md (lines 561-729). Defined: column mapping (A for monster name with part name, L-AJ for core stats, AM-AW+1 for special abilities and weakness), cell merge structure (vertical+horizontal merge for A-AJ with data in odd rows only; horizontal-only merge for AM-AW+1 with different data per row), 2-row-per-part output layout, field transformations (replace エネルギー→E, ダメージ→ダメ, remove 属性). Included complete Trent example with JSON input and expected output rows 3-8.
       - Owner: kazuyasi (specification definition and approval)
- [ ] T015b Phase 2.5: Google Sheets exporter implementation (P25-3b) — **READY FOR IMPLEMENTATION**
      - Description: Implement core/src/export/sheets.rs with GoogleSheetsExporter struct following format specification from T015. Dependencies: google-sheets4 crate, oauth2, yup-oauth2. Create GoogleSheetsExporter with OAuth 2.0 authentication flow. Implement data transformation: Monster → spreadsheet rows with headers (per specification). Create/update rows in specified spreadsheet ID. Handle auth errors (invalid token, 401, 403) gracefully. 5+ unit tests for auth, data transform, API interaction.
      - Subtasks:
        1. T015b: Add dependencies (google-sheets4, oauth2, yup-oauth2, tokio) to core/Cargo.toml
        2. T015c: Create OAuth 2.0 credential flow (local token cache in ~/.config/trpg-json/credentials.json)
        3. T015d: Implement data transformation (Monster → spreadsheet row format per spec)
        4. T015e: Implement API write operations (append/update rows to specified sheet)
        5. T015f: Add error handling and user-friendly messages
        6. T015g: Write unit tests for each component
- [x] T016 Phase 2.5: CLI integration - export flags (P25-4) — 2025-12-03
      - Description: Updated app/src/main.rs Select command with --export and --output flags. Created export_results() helper function. Integrated ExporterFactory and error handling. All functionality tested: JSON export with multiple filters, error messages for missing --output and unsupported formats, sheets format correctly reports "not yet implemented".
- [ ] T017 Phase 2.5: Export module tests (P25-5)
      - Description: Add comprehensive unit tests for JSON and Google Sheets exporters. Test error handling, empty data, large datasets. Mock Google Sheets API for testing. Target 10+ new tests.
      - Subtasks:
        1. T017a: Unit tests for JSON exporter (edge cases: empty data, special chars, large datasets)
        2. T017b: Unit tests for Google Sheets authentication flow
        3. T017c: Mock tests for Sheets API interaction
        4. T017d: Integration tests: JSON export with filters, error message validation
- [ ] T018 Phase 2.5: Integration tests and build verification (P25-6, P25-7)
      - Description: Add integration tests for select command with export functionality. Run full test suite (target: 50+ tests passing). Verify release build succeeds. Document any breaking changes.
      - Subtasks:
        1. T018a: E2E test for JSON export with filters
        2. T018b: E2E test for Sheets export (mock OAuth)
        3. T018c: Run full test suite and verify 50+ tests pass
        4. T018d: Verify release build (cargo build --release) succeeds
        5. T018e: Document new error codes and auth setup steps

---

## ✅ Done (Recent 10)
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
