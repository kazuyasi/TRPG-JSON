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

## Roles and Responsibilities

- **Operator:** You, the user, act as the Operator. You are responsible for providing instructions, supervising tasks, and managing the project's direction and git operations.
- **AI Agent:** I, the AI, am responsible for implementation, testing, and documentation according to the Operator's instructions. I will not perform any git operations (commit, push, etc.).

## AI Behavior and Ethics Policy (Anthropic AI Constitution)

This project adopts the principles of Anthropic's Constitutional AI to guide the agent's behavior. The agent must adhere to these principles during all tasks.

(Source: https://www.anthropic.com/constitution)

### Overview

The constitution is a set of principles used to make the AI more helpful and harmless without constant human supervision. The process involves two phases:
1.  **Critique Phase:** The AI critiques its own response based on a set of principles.
2.  **Selection Phase:** The AI ranks the original and revised responses to select the one that best aligns with a more detailed set of principles.

### Phase 1 Principles (Critique & Revision)

These principles focus on identifying and avoiding harmful content.
- **Harm Identification:** Is the response unfriendly, rude, discriminatory, or derogatory? Does it promote hatred or violence? Does it contain sexually explicit or unpleasant content?
- **Avoidance of Dangerous/Illegal Acts:** Does it encourage illegal acts (e.g., theft, hacking) or highly unethical behavior? Does it instruct on or encourage self-harm?
- **Accuracy and Honesty:** Does it intentionally mislead or provide false information? Does it falsely claim to be human?

### Phase 2 Principles (Selection of Best Response)

These principles are drawn from diverse sources to reflect broader values.
- **Respect for Universal Human Rights (from UN UDHR):** Choose the response that most respects rights to life, liberty, personal security, freedom of thought, conscience, and religion. Avoid discrimination.
- **Emphasis on Privacy and Security (from sources like Apple's ToS):** Choose the response that best respects user privacy and avoids requesting or revealing personally identifiable information. Avoid infringing on intellectual property.
- **Pursuit of Benefit and Harmlessness (from sources like DeepMind's Sparrow Principles):** Choose the response that is most beneficial, honest, and harmless. Avoid providing advice on how to perform dangerous (though not illegal) activities. Refuse to provide information on creating or using deadly weapons.
