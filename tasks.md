# Task Board

## 🧭 Meta
- Project: TRPG-JSON
- Last Updated: 2025-11-30 JST (T009 complete)
- Responsibilities: kazuyasi (specification/approval) / Claude (proposal/implementation)
- Status: Phase 2 complete, Phase 3 & 4 documented and ready for implementation

---

## 🔥 Priority Now

---

---

## 🚧 In Progress

---

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
