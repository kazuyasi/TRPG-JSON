# T032.5b: Spell Chat Palette Test Specifications

## Test Execution Plan

This document provides detailed test specifications for the `palette.rs` module implementation (T033).

---

## Test Module Location

```rust
// core/src/export/palette.rs (at module bottom)
#[cfg(test)]
mod tests {
    use super::*;
    // All tests defined below
}
```

---

## Category A: MP Value Formatting (4 tests)

### Test A1: test_mp_format_value
**Purpose:** Fixed MP cost formatting

```rust
#[test]
fn test_mp_format_value() {
    let spell = Spell {
        // ... required fields ...
        cost: SpellCost {
            mp: MpCost::Value(5),
        },
        // ... other fields ...
    };
    assert_eq!(format_mp(&spell), Ok("5".to_string()));
}
```

**Input:** `cost.mp = MpCost::Value(5)`
**Expected:** `Ok("5")`
**Failure Condition:** Returns different string

---

### Test A2: test_mp_format_value_plus
**Purpose:** Minimum MP cost (value+) formatting with "～" suffix

```rust
#[test]
fn test_mp_format_value_plus() {
    let spell = Spell {
        // ... required fields ...
        cost: SpellCost {
            mp: MpCost::ValuePlus(3),
        },
        // ... other fields ...
    };
    assert_eq!(format_mp(&spell), Ok("3～".to_string()));
}
```

**Input:** `cost.mp = MpCost::ValuePlus(3)`
**Expected:** `Ok("3～")`
**Failure Condition:** Missing "～" suffix or wrong number

---

### Test A3: test_mp_format_special
**Purpose:** Special/custom MP cost formatting (literal output)

```rust
#[test]
fn test_mp_format_special() {
    let spell = Spell {
        // ... required fields ...
        cost: SpellCost {
            mp: MpCost::Special("複雑な計算式".to_string()),
        },
        // ... other fields ...
    };
    assert_eq!(format_mp(&spell), Ok("複雑な計算式".to_string()));
}
```

**Input:** `cost.mp = MpCost::Special("複雑な計算式")`
**Expected:** `Ok("複雑な計算式")`
**Failure Condition:** String modified or lost

---

### Test A4: test_mp_format_error_missing_all
**Purpose:** Error when all MP cost variants are missing

```rust
#[test]
fn test_mp_format_error_missing_all() {
    let spell = Spell {
        // ... required fields ...
        cost: SpellCost {
            mp: MpCost::None, // Or empty variant
        },
        // ... other fields ...
    };
    assert!(format_mp(&spell).is_err());
    assert_eq!(
        format_mp(&spell),
        Err("MP cost must have one of: value, value+, special".to_string())
    );
}
```

**Expected Behavior:** Return error with descriptive message

---

## Category B: Target Formatting (5 tests)

### Test B1: test_target_format_individual_simple
**Purpose:** Simple individual target

```rust
#[test]
fn test_target_format_individual_simple() {
    let spell = Spell {
        // ... required fields ...
        target: Target {
            kind: TargetKind::Individual("1体".to_string()),
        },
        // ... other fields ...
    };
    assert_eq!(format_target(&spell), Ok("1体".to_string()));
}
```

**Input:** `target.kind = Individual("1体")`
**Expected:** `Ok("1体")`

---

### Test B2: test_target_format_individual_complex
**Purpose:** Complex individual target (multiple targets)

```rust
#[test]
fn test_target_format_individual_complex() {
    let spell = Spell {
        // ... required fields ...
        target: Target {
            kind: TargetKind::Individual("1体全".to_string()),
        },
        // ... other fields ...
    };
    assert_eq!(format_target(&spell), Ok("1体全".to_string()));
}
```

**Input:** `target.kind = Individual("1体全")`
**Expected:** `Ok("1体全")`

---

### Test B3: test_target_format_area_simple
**Purpose:** Area target with simple parameters

```rust
#[test]
fn test_target_format_area_simple() {
    let spell = Spell {
        // ... required fields ...
        target: Target {
            kind: TargetKind::Area {
                value: "1エリア".to_string(),
                radius_m: "5".to_string(),
                suffix: "内".to_string(),
            },
        },
        // ... other fields ...
    };
    assert_eq!(
        format_target(&spell),
        Ok("1エリア(半径5m内)".to_string())
    );
}
```

**Expected:** `"1エリア(半径5m内)"`
**Format Check:** `value(半径radius_mm{suffix})`

---

### Test B4: test_target_format_area_complex
**Purpose:** Area target with larger radius

```rust
#[test]
fn test_target_format_area_complex() {
    let spell = Spell {
        // ... required fields ...
        target: Target {
            kind: TargetKind::Area {
                value: "2エリア".to_string(),
                radius_m: "10".to_string(),
                suffix: "空間".to_string(),
            },
        },
        // ... other fields ...
    };
    assert_eq!(
        format_target(&spell),
        Ok("2エリア(半径10m空間)".to_string())
    );
}
```

**Expected:** `"2エリア(半径10m空間)"`

---

### Test B5: test_target_format_error_invalid_kind
**Purpose:** Error handling for invalid target kind

```rust
#[test]
fn test_target_format_error_invalid_kind() {
    let spell = Spell {
        // ... required fields ...
        target: Target {
            kind: TargetKind::Unknown, // Invalid variant
        },
        // ... other fields ...
    };
    assert!(format_target(&spell).is_err());
}
```

**Expected:** Return error for unknown target kind

---

## Category C: Duration Formatting (3 tests)

### Test C1: test_duration_format_string
**Purpose:** String-based duration (instant/round-based)

```rust
#[test]
fn test_duration_format_string() {
    let spell = Spell {
        // ... required fields ...
        time: Duration {
            value: TimeValue::String("一瞬".to_string()),
            unit: "".to_string(), // Ignored for string
        },
        // ... other fields ...
    };
    assert_eq!(format_duration(&spell), Ok("一瞬".to_string()));
}
```

**Input:** `time.value = String("一瞬")`
**Expected:** `Ok("一瞬")`
**Key Point:** Unit is ignored for string values

---

### Test C2: test_duration_format_integer
**Purpose:** Numeric duration with unit

```rust
#[test]
fn test_duration_format_integer() {
    let spell = Spell {
        // ... required fields ...
        time: Duration {
            value: TimeValue::Integer(3),
            unit: "年".to_string(),
        },
        // ... other fields ...
    };
    assert_eq!(format_duration(&spell), Ok("3年".to_string()));
}
```

**Input:** `time.value = Integer(3)`, `unit = "年"`
**Expected:** `Ok("3年")`
**Format Check:** `{value}{unit}`

---

### Test C3: test_duration_format_integer_round
**Purpose:** Numeric duration in rounds

```rust
#[test]
fn test_duration_format_integer_round() {
    let spell = Spell {
        // ... required fields ...
        time: Duration {
            value: TimeValue::Integer(10),
            unit: "ラウンド".to_string(),
        },
        // ... other fields ...
    };
    assert_eq!(format_duration(&spell), Ok("10ラウンド".to_string()));
}
```

**Input:** `time.value = Integer(10)`, `unit = "ラウンド"`
**Expected:** `Ok("10ラウンド")`

---

## Category D: Magic Category Formatting (4 tests)

### Test D1: test_magic_category_2char_holy
**Purpose:** 2-character category gets "魔法" suffix

```rust
#[test]
fn test_magic_category_2char_holy() {
    let school = "神聖";
    assert_eq!(format_magic_category(school), "神聖魔法");
}
```

**Input:** `"神聖"` (2 full-width chars)
**Expected:** `"神聖魔法"`
**Logic:** Character count == 2 → append "魔法"

---

### Test D2: test_magic_category_2char_fairy
**Purpose:** Another 2-character category

```rust
#[test]
fn test_magic_category_2char_fairy() {
    let school = "妖精";
    assert_eq!(format_magic_category(school), "妖精魔法");
}
```

**Input:** `"妖精"` (2 full-width chars)
**Expected:** `"妖精魔法"`

---

### Test D3: test_magic_category_long
**Purpose:** Long category name (not exactly 2 chars) - no suffix

```rust
#[test]
fn test_magic_category_long() {
    let school = "ハイテクノロジー";
    assert_eq!(
        format_magic_category(school),
        "ハイテクノロジー"
    );
}
```

**Input:** `"ハイテクノロジー"` (8 full-width chars)
**Expected:** `"ハイテクノロジー"` (unchanged)
**Logic:** Character count != 2 → use as-is

---

### Test D4: test_magic_category_1char
**Purpose:** 1-character category - no suffix

```rust
#[test]
fn test_magic_category_1char() {
    let school = "聖";
    assert_eq!(format_magic_category(school), "聖");
}
```

**Input:** `"聖"` (1 full-width char)
**Expected:** `"聖"` (unchanged)
**Logic:** Character count != 2 → use as-is

---

## Category E: Support Spell Palette Generation (3 tests)

### Test E1: test_support_palette_basic
**Purpose:** Complete support spell palette output

```rust
#[test]
fn test_support_palette_basic() {
    let spell = Spell {
        name: "ライト".to_string(),
        補助: true,
        cost: SpellCost {
            mp: MpCost::Value(3),
        },
        target: Target {
            kind: TargetKind::Individual("任意の地点".to_string()),
        },
        range: "10m(起点指定)".to_string(),
        time: Duration {
            value: TimeValue::String("一瞬".to_string()),
            unit: "".to_string(),
        },
        effect: "光源を生成する。".to_string(),
        // ... other fields ...
    };
    
    let result = generate_spell_palette(&spell)?;
    assert_eq!(
        result,
        "ライト / MP:3 / 対象:任意の地点 / 射程:10m(起点指定) / 時間:一瞬 / 光源を生成する。"
    );
}
```

**Format Check:**
```
{name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / {effect}
```

---

### Test E2: test_support_palette_with_value_plus_mp
**Purpose:** Support spell with value+ MP cost

```rust
#[test]
fn test_support_palette_with_value_plus_mp() {
    let spell = Spell {
        name: "魔法解除".to_string(),
        補助: true,
        cost: SpellCost {
            mp: MpCost::ValuePlus(8), // Will format as "8～"
        },
        target: Target {
            kind: TargetKind::Individual("魔法1つ".to_string()),
        },
        range: "接触".to_string(),
        time: Duration {
            value: TimeValue::String("一瞬".to_string()),
            unit: "".to_string(),
        },
        effect: "魔法を打ち消す。".to_string(),
        // ... other fields ...
    };
    
    let result = generate_spell_palette(&spell)?;
    assert_eq!(
        result,
        "魔法解除 / MP:8～ / 対象:魔法1つ / 射程:接触 / 時間:一瞬 / 魔法を打ち消す。"
    );
}
```

**Note:** MP should be "8～" (with value+ formatting)

---

### Test E3: test_support_palette_error_missing_effect
**Purpose:** Error when effect field is missing

```rust
#[test]
fn test_support_palette_error_missing_effect() {
    let spell = Spell {
        name: "テスト".to_string(),
        補助: true,
        effect: "".to_string(), // Missing/empty effect
        // ... other required fields ...
    };
    
    assert!(generate_support_palette(&spell).is_err());
    assert_eq!(
        generate_support_palette(&spell),
        Err("Effect description is required".to_string())
    );
}
```

---

## Category F: Regular Spell Palette Generation (3 tests)

### Test F1: test_regular_palette_basic
**Purpose:** Complete regular spell palette with dice roll

```rust
#[test]
fn test_regular_palette_basic() {
    let spell = Spell {
        name: "ゴッド・ジャッジメント".to_string(),
        補助: false,
        school: "神聖".to_string(), // 2-char → becomes "神聖魔法"
        cost: SpellCost {
            mp: MpCost::Value(15),
        },
        target: Target {
            kind: TargetKind::Area {
                value: "1エリア".to_string(),
                radius_m: "4".to_string(),
                suffix: "すべて".to_string(),
            },
        },
        range: "術者".to_string(),
        time: Duration {
            value: TimeValue::String("一瞬".to_string()),
            unit: "".to_string(),
        },
        effect: "物理的に神の捌きを下す。".to_string(),
        // ... other fields ...
    };
    
    let result = generate_spell_palette(&spell)?;
    assert_eq!(
        result,
        "2d+{神聖魔法}+{行使修正}  ゴッド・ジャッジメント / MP:15 / 対象:1エリア(半径4mすべて) / 射程:術者 / 時間:一瞬 / 物理的に神の捌きを下す。"
    );
}
```

**Format Check:**
```
2d+{magic_category}+{行使修正}  {name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / {effect}
```

**Key Points:**
- Magic category derived from school (神聖 → 神聖魔法)
- Literal `{行使修正}` preserved
- Two spaces after closing brace before spell name

---

### Test F2: test_regular_palette_execution_bonus_preserved
**Purpose:** Verify `{行使修正}` is literal output (not substituted)

```rust
#[test]
fn test_regular_palette_execution_bonus_preserved() {
    let spell = Spell {
        name: "テストスペル".to_string(),
        補助: false,
        school: "プリエスト".to_string(), // Long → no "魔法" suffix
        cost: SpellCost {
            mp: MpCost::Value(10),
        },
        // ... other required fields ...
    };
    
    let result = generate_regular_palette(&spell)?;
    assert!(result.contains("{行使修正}"));
    assert!(!result.contains("{execution_bonus}")); // Should NOT be translated
}
```

**Verification:** Output contains literal `{行使修正}` for dicebot substitution

---

### Test F3: test_regular_palette_error_missing_school
**Purpose:** Error when school is missing for regular spell

```rust
#[test]
fn test_regular_palette_error_missing_school() {
    let spell = Spell {
        name: "テスト".to_string(),
        補助: false,
        school: "".to_string(), // Missing school
        // ... other required fields ...
    };
    
    assert!(generate_regular_palette(&spell).is_err());
    assert_eq!(
        generate_regular_palette(&spell),
        Err("School/category is required for non-support spells".to_string())
    );
}
```

---

## Category G: Entry Point Tests (3 tests)

### Test G1: test_generate_palette_delegates_to_support
**Purpose:** Entry point delegates to support generator when `補助=true`

```rust
#[test]
fn test_generate_palette_delegates_to_support() {
    let spell = Spell {
        name: "ライト".to_string(),
        補助: true,
        // ... other fields for support spell ...
    };
    
    let result = generate_spell_palette(&spell);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.contains("2d+")); // Support spells have no dice roll
}
```

**Verification:** No dice roll (2d+) in output

---

### Test G2: test_generate_palette_delegates_to_regular
**Purpose:** Entry point delegates to regular generator when `補助=false`

```rust
#[test]
fn test_generate_palette_delegates_to_regular() {
    let spell = Spell {
        name: "ゴッド・ジャッジメント".to_string(),
        補助: false,
        school: "神聖".to_string(),
        // ... other fields for regular spell ...
    };
    
    let result = generate_spell_palette(&spell);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.starts_with("2d+")); // Regular spells start with dice roll
}
```

**Verification:** Output starts with "2d+"

---

### Test G3: test_generate_palette_error_propagation
**Purpose:** Errors from helpers are propagated through entry point

```rust
#[test]
fn test_generate_palette_error_propagation() {
    let spell = Spell {
        name: "テスト".to_string(),
        補助: true,
        effect: "".to_string(), // Will cause error in support generator
        // ... other fields ...
    };
    
    let result = generate_spell_palette(&spell);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err("Effect description is required".to_string())
    );
}
```

---

## Category H: Integration Tests (2+ tests)

### Test H1: test_palette_with_real_sample_spell_1
**Purpose:** Integration test with actual sample spell data (from data/sample/spells_sample.json)

```rust
#[test]
fn test_palette_with_real_sample_spell_1() {
    // Load spell from sample data
    let spells = load_spells_json_array("data/sample/spells_sample.json").unwrap();
    let spell = spells.iter().find(|s| s.name == "ライト").unwrap();
    
    let result = generate_spell_palette(spell);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.contains("ライト"));
    assert!(output.contains("MP:"));
    assert!(output.contains("対象:"));
}
```

**Real-world verification** with actual spell data

---

### Test H2: test_palette_with_real_sample_spell_2
**Purpose:** Integration test with different sample spell

```rust
#[test]
fn test_palette_with_real_sample_spell_2() {
    let spells = load_spells_json_array("data/sample/spells_sample.json").unwrap();
    // Choose a regular (non-support) spell
    let spell = spells.iter().find(|s| s.補助 == false).unwrap();
    
    let result = generate_spell_palette(spell);
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert!(output.starts_with("2d+")); // Regular spell must have dice roll
    assert!(output.contains("{行使修正}"));
}
```

---

### Test H3: test_palette_japanese_character_preservation
**Purpose:** Verify Japanese characters are preserved correctly

```rust
#[test]
fn test_palette_japanese_character_preservation() {
    let spell = Spell {
        name: "魔法解除".to_string(),
        補助: true,
        effect: "呪いを打ち消す。".to_string(),
        // ... other fields ...
    };
    
    let result = generate_spell_palette(&spell);
    let output = result.unwrap();
    
    assert!(output.contains("魔法解除"));
    assert!(output.contains("呪いを打ち消す。"));
}
```

**No character encoding issues**

---

## Test Execution Order Recommendation

1. **Phase 1 (Helper functions):** Run tests A1-A4, B1-B5, C1-C3, D1-D4 first
2. **Phase 2 (Generators):** Run E1-E3, F1-F3, G1-G3
3. **Phase 3 (Integration):** Run H1-H3

---

## Total Test Count

- Category A: 4 tests
- Category B: 5 tests
- Category C: 3 tests
- Category D: 4 tests
- Category E: 3 tests
- Category F: 3 tests
- Category G: 3 tests
- Category H: 3 tests

**Total: 28 unit + integration tests**

---

## Success Criteria

✅ All 28 tests must pass
✅ No panics or unwrap() failures
✅ Error messages are user-friendly
✅ Japanese characters preserved correctly
✅ Output format matches specification exactly

---

Last updated: 2025-12-19
Ready for implementation in T033.
