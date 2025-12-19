# T032.5b: Spell Chat Palette Implementation Plan

## Overview
This document translates the Spell Chat Palette specification (DESIGN_GUIDE.md lines 728-805) into detailed implementation requirements for the `palette.rs` module.

## Task Objectives
- ✅ Define data field extraction logic
- ✅ Define conditional logic for `補助` flag (true/false branching)
- ✅ Define output format generation
- ✅ Define error handling for missing/invalid fields
- ✅ Specify 20+ unit tests
- Deliverable: Implementation checklist + test specifications

---

## Part 1: Output Format Specification

### Support Spells (`補助 == true`)

**Output Format:**
```
{name} / MP:{mp_value} / 対象:{target} / 射程:{range} / 時間:{duration} / {effect}
```

**No dice rolls, information confirmation format only.**

#### Field Extraction:

| Field | Source | Format Rules |
|-------|--------|--------------|
| `name` | `Spell.name` | Use as-is |
| `mp_value` | `Spell.cost.mp` (value/value+/special) | See MP Value Format below |
| `target` | `Spell.target` | See Target Format below |
| `range` | `Spell.range` | Use as-is |
| `duration` | `Spell.time` | See Duration Format below |
| `effect` | `Spell.effect` | Use as-is |

### Regular Spells (`補助 == false`)

**Output Format:**
```
2d+{magic_category}+{execution_bonus}  {name} / MP:{mp_value} / 対象:{target} / 射程:{range} / 時間:{duration} / 効果
```

**Dice rolls required (category serves as judgment check name).**

#### Field Extraction:

| Field | Source | Format Rules |
|-------|--------|--------------|
| `magic_category` | `Spell.school` | See Magic Category Format below |
| `execution_bonus` | Literal | Output as literal string `{行使修正}` (for dicebot substitution) |
| `name` | `Spell.name` | Use as-is |
| `mp_value` | `Spell.cost.mp` | See MP Value Format below |
| `target` | `Spell.target` | See Target Format below |
| `range` | `Spell.range` | Use as-is |
| `duration` | `Spell.time` | See Duration Format below |
| `effect` | `Spell.effect` | Use as-is (note: singular form "効果", not "効果複数") |

---

## Part 2: Field Format Rules

### 1. MP Value Format

**Rule:** Spell.cost.mp has exactly one of: `value`, `value+`, or `special`

| Kind | Input | Output |
|------|-------|--------|
| `value` | 3 | `3` |
| `value+` | 3 | `3～` (minimum cost, append "～") |
| `special` | "複雑な計算式" | Use value as-is |

**Implementation:**
```rust
fn format_mp(spell: &Spell) -> Result<String, String> {
    match &spell.cost.mp {
        MpCost::Value(v) => Ok(v.to_string()),
        MpCost::ValuePlus(v) => Ok(format!("{}～", v)),
        MpCost::Special(s) => Ok(s.clone()),
    }
}
```

### 2. Target Format

**Rule:** `Spell.target` has `kind` field determining format

#### Target Kind: `個別` (Individual)

| Source | Output Rule |
|--------|------------|
| `Spell.target.individual` | Output as-is |

**Example:** If `個別` is "1体全", output "1体全"

#### Target Kind: `エリア` (Area)

| Source | Output Format | Example |
|--------|---------------|---------|
| `Spell.target.value` | `value` + "(" | "2エリア(" |
| `Spell.target.radius_m` | "半径" + value + "m" | "半径10m" |
| `Spell.target.suffix` | suffix | "空間)" |

**Combined format:** `{value}(半径{radius_m}m{suffix})`

**Example:** value="2エリア", radius_m="10", suffix="空間" → Output "2エリア(半径10m空間)"

**Implementation:**
```rust
fn format_target(spell: &Spell) -> Result<String, String> {
    match &spell.target.kind {
        TargetKind::Individual(ind) => Ok(ind.clone()),
        TargetKind::Area { value, radius_m, suffix } => {
            Ok(format!("{}(半径{}m{})", value, radius_m, suffix))
        }
    }
}
```

### 3. Duration Format

**Rule:** `Spell.time` has `value` field (string or integer) + optional `unit`

| Value Type | Input | Output Format |
|------------|-------|----------------|
| String | "一瞬" | "一瞬" (as-is) |
| Integer | 3 (with unit="年") | "3年" (value + unit) |

**Implementation:**
```rust
fn format_duration(spell: &Spell) -> Result<String, String> {
    match &spell.time.value {
        TimeValue::String(s) => Ok(s.clone()),
        TimeValue::Integer(i) => {
            let unit = &spell.time.unit; // Assume unit exists
            Ok(format!("{}{}", i, unit))
        }
    }
}
```

### 4. Magic Category Format

**Rule:** Derives from `Spell.school` (category name)

**Decision Logic:**
1. Count full-width characters in `school` string
2. If **exactly 2 full-width characters**: append "魔法"
3. Otherwise: use `school` as-is

**Examples:**
- "神聖" (2 chars) → "神聖魔法"
- "妖精" (2 chars) → "妖精魔法"
- "ハイテクノロジー" (8 chars) → "ハイテクノロジー" (as-is)

**Implementation:**
```rust
fn format_magic_category(school: &str) -> String {
    if school.chars().count() == 2 {
        format!("{}魔法", school)
    } else {
        school.to_string()
    }
}
```

---

## Part 3: Implementation Checklist

### Module Structure
```rust
// core/src/export/palette.rs
pub mod palette {
    pub fn generate_spell_palette(spell: &Spell) -> Result<String, String>;
    
    // Helper functions
    fn format_mp(spell: &Spell) -> Result<String, String>;
    fn format_target(spell: &Spell) -> Result<String, String>;
    fn format_duration(spell: &Spell) -> Result<String, String>;
    fn format_magic_category(school: &str) -> String;
    fn generate_support_palette(spell: &Spell) -> Result<String, String>;
    fn generate_regular_palette(spell: &Spell) -> Result<String, String>;
}
```

### Required Implementations

- [ ] **1. Main entry point:** `generate_spell_palette(spell: &Spell) -> Result<String, String>`
  - Checks `spell.補助` flag
  - Delegates to appropriate generator function
  - Error handling for missing required fields

- [ ] **2. Support spell generator:** `generate_support_palette(spell: &Spell) -> Result<String, String>`
  - Format: `{name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / {effect}`
  - Requires: name, cost.mp, target, range, time, effect
  - Error on missing fields

- [ ] **3. Regular spell generator:** `generate_regular_palette(spell: &Spell) -> Result<String, String>`
  - Format: `2d+{magic_category}+{行使修正}  {name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / 効果`
  - Requires: school, name, cost.mp, target, range, time, effect
  - Magic category derived from school
  - Execution bonus always literal `{行使修正}`

- [ ] **4. MP formatter:** `format_mp(spell: &Spell) -> Result<String, String>`
  - Handles value/value+/special cases
  - Appends "～" for value+ kind

- [ ] **5. Target formatter:** `format_target(spell: &Spell) -> Result<String, String>`
  - Handles 個別 and エリア kinds
  - Assembles area format: `value(半径radius_mm{suffix})`

- [ ] **6. Duration formatter:** `format_duration(spell: &Spell) -> Result<String, String>`
  - Handles string and integer value types
  - Appends unit for integers

- [ ] **7. Magic category formatter:** `format_magic_category(school: &str) -> String`
  - Pure function (no Result needed)
  - Counts full-width characters
  - Appends "魔法" if exactly 2 characters

---

## Part 4: Error Handling

### Missing Field Errors

| Field | Error Message | Handling |
|-------|---------------|----------|
| `name` | "Missing spell name" | Return Err |
| `cost.mp` | "Missing MP cost" | Return Err |
| `target` | "Missing target information" | Return Err |
| `range` | "Missing range information" | Return Err |
| `time` | "Missing duration information" | Return Err |
| `effect` | "Missing effect description" | Return Err |
| `school` (for regular spells) | "Missing school for regular spell" | Return Err |

### Field Validation Errors

| Field | Error Condition | Handling |
|-------|-----------------|----------|
| `cost.mp` | All three (value/value+/special) missing | Return Err("MP cost must have one of: value, value+, special") |
| `target.kind` | Neither 個別 nor エリア | Return Err("Invalid target kind") |
| `target.value` (area) | Missing for area target | Return Err("Missing area value") |
| `time.value` | Invalid type | Return Err("Invalid time value") |

---

## Part 5: Unit Test Specifications

### Test Categories (20+ tests)

#### A. MP Value Formatting (4 tests)
- [ ] **test_mp_format_value**: Fixed cost (value=5) → "5"
- [ ] **test_mp_format_value_plus**: Minimum cost (value+=3) → "3～"
- [ ] **test_mp_format_special**: Special cost → "複雑な計算"
- [ ] **test_mp_format_missing**: Missing all MP kinds → Error

#### B. Target Formatting (5 tests)
- [ ] **test_target_format_individual_simple**: Individual target "1体" → "1体"
- [ ] **test_target_format_individual_complex**: Individual target "1体全" → "1体全"
- [ ] **test_target_format_area_simple**: Area (value="1エリア", radius_m="5", suffix="内") → "1エリア(半径5m内)"
- [ ] **test_target_format_area_complex**: Area with multiple meters (radius_m="10") → "2エリア(半径10m空間)"
- [ ] **test_target_format_invalid**: Invalid target kind → Error

#### C. Duration Formatting (3 tests)
- [ ] **test_duration_format_string**: String value "一瞬" → "一瞬"
- [ ] **test_duration_format_integer**: Integer value 3 with unit "年" → "3年"
- [ ] **test_duration_format_integer_round**: Integer value 10 with unit "ラウンド" → "10ラウンド"

#### D. Magic Category Formatting (4 tests)
- [ ] **test_magic_category_2char**: "神聖" → "神聖魔法"
- [ ] **test_magic_category_2char_fairy**: "妖精" → "妖精魔法"
- [ ] **test_magic_category_long**: "ハイテクノロジー" → "ハイテクノロジー" (as-is)
- [ ] **test_magic_category_1char**: "聖" → "聖" (as-is)

#### E. Support Spell Palette Generation (3 tests)
- [ ] **test_support_palette_basic**: Basic spell with all fields → Correct format
- [ ] **test_support_palette_with_special_mp**: value+ MP cost → "3～" in output
- [ ] **test_support_palette_missing_effect**: Missing effect field → Error

#### F. Regular Spell Palette Generation (3 tests)
- [ ] **test_regular_palette_basic**: Basic spell with school → Correct format with dice roll
- [ ] **test_regular_palette_execution_bonus**: Literal `{行使修正}` preserved in output
- [ ] **test_regular_palette_missing_school**: Missing school field → Error

#### G. Entry Point Tests (3 tests)
- [ ] **test_generate_palette_support**: `補助=true` → Delegates to support generator
- [ ] **test_generate_palette_regular**: `補助=false` → Delegates to regular generator
- [ ] **test_generate_palette_error_propagation**: Field error in helper → Error propagated

#### H. Integration Tests (2+ tests)
- [ ] **test_palette_with_sample_spell_1**: Full sample spell (妖精魔法 category) → Correct output
- [ ] **test_palette_with_sample_spell_2**: Different spell (神聖 category) → Correct output
- [ ] **test_palette_japanese_characters**: Japanese characters in name/effect → Preserved correctly

---

## Part 6: Implementation Order

### Phase 1: Helper Functions
1. Implement `format_mp()`
2. Implement `format_magic_category()` (pure function)
3. Implement `format_duration()`
4. Implement `format_target()`

**Tests for Phase 1:** Categories A, B, C, D (14 tests)

### Phase 2: Generator Functions
5. Implement `generate_support_palette()`
6. Implement `generate_regular_palette()`
7. Implement `generate_spell_palette()` (entry point)

**Tests for Phase 2:** Categories E, F, G (9 tests)

### Phase 3: Integration & Polish
8. Add integration tests (Category H: 2+ tests)
9. Error handling refinement
10. Documentation comments

---

## Part 7: Error Messages (User-Friendly)

```rust
const ERR_MISSING_NAME: &str = "Spell name is required for palette generation";
const ERR_MISSING_MP: &str = "MP cost must be specified (value, value+, or special)";
const ERR_MISSING_TARGET: &str = "Target information is required";
const ERR_MISSING_RANGE: &str = "Range information is required";
const ERR_MISSING_TIME: &str = "Duration information is required";
const ERR_MISSING_EFFECT: &str = "Effect description is required";
const ERR_MISSING_SCHOOL: &str = "School/category is required for non-support spells";
const ERR_INVALID_TARGET_KIND: &str = "Target must be either individual (個別) or area (エリア)";
const ERR_INVALID_TIME_VALUE: &str = "Duration value must be string or integer";
```

---

## Part 8: Integration Points (for T033)

### When implementing `palette.rs` in T033:

1. **Module location:** `core/src/export/palette.rs`
2. **Public interface:** `pub fn generate_spell_palette(spell: &Spell) -> Result<String, String>`
3. **CLI integration:** `gm spell palette <name>` command
4. **Error propagation:** CLI converts Err to stderr output

### Data structures required (verify in existing Spell struct):
- `spell.name: String`
- `spell.school: String`
- `spell.補助: bool` (or `spell.support_flag`)
- `spell.cost.mp: MpCost` (enum: Value, ValuePlus, Special)
- `spell.target: Target` (struct with kind field)
- `spell.range: String`
- `spell.time: Duration` (struct with value + unit)
- `spell.effect: String`

---

## Summary: Deliverables for T032.5b

✅ **Document created:** SPELL_PALETTE_IMPLEMENTATION.md
✅ **Implementation checklist:** 10 functions defined
✅ **Error handling:** Specified for 8+ error cases
✅ **Test specifications:** 22 unit tests + integration tests

**Output Format Reference:**
- Support spells (補助=true): `name / MP:X / 対象:Y / 射程:Z / 時間:T / effect`
- Regular spells (補助=false): `2d+{category}+{行使修正}  name / MP:X / 対象:Y / 射程:Z / 時間:T / 効果`

**Next phase (T033):** Implement `core/src/export/palette.rs` using this specification.

---

Last updated: 2025-12-19
Ready for T033 implementation.
