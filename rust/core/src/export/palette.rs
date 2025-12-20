/// Spell Chat Palette Generation Module
/// 
/// このモジュールは呪文データをチャットパレット形式に変換します。
/// 補助フラグに応じて異なる出力形式を生成します。

use crate::Spell;

const ERR_MISSING_NAME: &str = "チャットパレット出力のためにはスペル名が必要です。";
const ERR_MISSING_MP: &str = "MPコストは次のどれかで定義してください（value, value+, special）。";
const ERR_MISSING_TARGET: &str = "対象の情報が必要です。魔法の対象は何ですか？";
const ERR_MISSING_RANGE: &str = "射程の情報が必要です。魔法はどこまで届きますか？";
const ERR_MISSING_TIME: &str = "時間の情報が必要です。魔法はどれだけ持続しますか？";
const ERR_MISSING_EFFECT: &str = "効果の情報が必要です。魔法はどんな効果ですか？";
const ERR_MISSING_SCHOOL: &str = "流派もしくはカテゴリの情報が通常魔法には必要です。";
const ERR_INVALID_TARGET_KIND: &str = "対象は個別かエリアかを選択する必要があります。";
const ERR_INVALID_TIME_VALUE: &str = "時間の値は文字列もしくは整数である必要があります。";

/// 魔法系統名を生成する
pub fn format_magic_category(school: &str) -> String {
    if school.chars().count() == 2 {
        format!("{}魔法", school)
    } else {
        school.to_string()
    }
}

// ============================================================================
// Helper Functions（Claude 実装済み予定）
// ============================================================================

/// MP値をフォーマットする
/// 
/// Spell.extra["MP"] から MP値を抽出してフォーマットする
/// 期待される形式: {"value": 3} または {"value+": 3} または {"special": "文字列"}
fn format_mp(spell: &Spell) -> Result<String, String> {
    let mp_obj = spell.extra.get("MP")
        .ok_or_else(|| ERR_MISSING_MP.to_string())?;
    
    let mp_obj = mp_obj.as_object()
        .ok_or_else(|| ERR_MISSING_MP.to_string())?;
    
    // value フィールド（固定MP）
    if let Some(value) = mp_obj.get("value") {
        if let Some(v) = value.as_i64() {
            return Ok(v.to_string());
        }
    }
    
    // value+ フィールド（最小MP）
    if let Some(value_plus) = mp_obj.get("value+") {
        if let Some(v) = value_plus.as_i64() {
            return Ok(format!("{}～", v));
        }
    }
    
    // special フィールド（特殊MP）
    if let Some(special) = mp_obj.get("special") {
        if let Some(s) = special.as_str() {
            return Ok(s.to_string());
        }
    }
    
    // どれにも該当しない
    Err(ERR_MISSING_MP.to_string())
}

/// 対象をフォーマットする
/// 
/// Spell.extra["対象"] から対象情報を抽出
/// 個別: {"kind": "個別", "個別": "1体全"} → "1体全"
/// エリア: {"kind": "エリア", "value": "2エリア", "半径(m)": "10", "末尾": "空間"} → "2エリア(半径10m空間)"
fn format_target(spell: &Spell) -> Result<String, String> {
    let target_obj = spell.extra.get("対象")
        .ok_or_else(|| ERR_MISSING_TARGET.to_string())?;
    
    let target_obj = target_obj.as_object()
        .ok_or_else(|| ERR_MISSING_TARGET.to_string())?;
    
    let kind = target_obj.get("kind")
        .and_then(|k| k.as_str())
        .ok_or_else(|| ERR_INVALID_TARGET_KIND.to_string())?;
    
    match kind {
        "個別" => {
            let individual = target_obj.get("個別")
                .and_then(|v| v.as_str())
                .ok_or_else(|| ERR_MISSING_TARGET.to_string())?;
            Ok(individual.to_string())
        }
        "エリア" => {
            // スキーマに合わせて、エリア対象は {"エリア": {...}} の形式
            let area_obj = target_obj.get("エリア")
                .and_then(|v| v.as_object())
                .ok_or_else(|| ERR_MISSING_TARGET.to_string())?;
            
            let value = area_obj.get("value")
                .and_then(|v| v.as_str())
                .ok_or_else(|| ERR_MISSING_TARGET.to_string())?;
            
            // 半径(m)は整数または文字列の可能性がある
            let radius_m = if let Some(v) = area_obj.get("半径(m)") {
                if let Some(i) = v.as_i64() {
                    i.to_string()
                } else if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    return Err(ERR_MISSING_TARGET.to_string());
                }
            } else {
                return Err(ERR_MISSING_TARGET.to_string());
            };
            
            // 末尾は文字列または整数の可能性がある
            let suffix = if let Some(v) = area_obj.get("末尾") {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else if let Some(i) = v.as_i64() {
                    i.to_string()
                } else {
                    return Err(ERR_MISSING_TARGET.to_string());
                }
            } else {
                return Err(ERR_MISSING_TARGET.to_string());
            };
            
            Ok(format!("{}(半径{}m{})", value, radius_m, suffix))
        }
        _ => Err(ERR_INVALID_TARGET_KIND.to_string()),
    }
}

/// 射程をフォーマットする
/// 
/// Spell.extra["射程"] または Spell.extra["射程(m)"] から射程情報を抽出
/// "射程"を優先、なければ"射程(m)"にフォールバック
/// 値をそのまま出力（変換なし）
/// 文字列または整数の両方に対応
fn format_range(spell: &Spell) -> Result<String, String> {
    // "射程"フィールドを優先チェック
    if let Some(range) = spell.extra.get("射程") {
        if let Some(r) = range.as_str() {
            return Ok(r.to_string());
        }
        // 文字列でなくても、as_i64()で試す
        if let Some(i) = range.as_i64() {
            return Ok(i.to_string());
        }
    }
    
    // "射程(m)"フィールドにフォールバック
    if let Some(range) = spell.extra.get("射程(m)") {
        if let Some(r) = range.as_str() {
            return Ok(r.to_string());
        }
        // 文字列でなくても、as_i64()で試す
        if let Some(i) = range.as_i64() {
            return Ok(i.to_string());
        }
    }
    
    Err(ERR_MISSING_RANGE.to_string())
}

/// 時間をフォーマットする
/// 
/// Spell.extra["時間"] から時間情報を抽出
/// 文字列: {"value": "一瞬"} → "一瞬"
/// 整数: {"value": 3, "unit": "年"} → "3年"
fn format_duration(spell: &Spell) -> Result<String, String> {
    let time_obj = spell.extra.get("時間")
        .ok_or_else(|| ERR_MISSING_TIME.to_string())?;
    
    let time_obj = time_obj.as_object()
        .ok_or_else(|| ERR_MISSING_TIME.to_string())?;
    
    // value フィールドが文字列の場合
    if let Some(value) = time_obj.get("value") {
        if let Some(s) = value.as_str() {
            return Ok(s.to_string());
        }
        
        // value フィールドが整数の場合
        if let Some(i) = value.as_i64() {
            let unit = time_obj.get("unit")
                .and_then(|u| u.as_str())
                .unwrap_or("");
            return Ok(format!("{}{}", i, unit));
        }
    }
    
    Err(ERR_INVALID_TIME_VALUE.to_string())
}

// ============================================================================
// Generator Functions（Claude 実装済み予定）
// ============================================================================

/// サポート呪文のパレットを生成する
/// 
/// 出力形式: {name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / {effect}
fn generate_support_palette(spell: &Spell) -> Result<String, String> {
    // 必須フィールドの確認
    let mp = format_mp(spell)?;
    let target = format_target(spell)?;
    let duration = format_duration(spell)?;
    let range = format_range(spell)?;
    
    let effect = spell.extra.get("効果")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ERR_MISSING_EFFECT.to_string())?;
    
    Ok(format!(
        "{} / MP:{} / 対象:{} / 射程:{} / 時間:{} / {}",
        spell.name, mp, target, range, duration, effect
    ))
}

/// 通常呪文のパレットを生成する
/// 
/// 出力形式: 2d+{magic_category}+{行使修正}  {name} / MP:{mp} / 対象:{target} / 射程:{range} / 時間:{duration} / 効果
fn generate_regular_palette(spell: &Spell) -> Result<String, String> {
    // school フィールド確認
    if spell.school.is_empty() {
        return Err(ERR_MISSING_SCHOOL.to_string());
    }
    
    // Helper 関数で必須フィールドの確認
    let mp = format_mp(spell)?;
    let target = format_target(spell)?;
    let duration = format_duration(spell)?;
    let range = format_range(spell)?;
    
    let effect = spell.extra.get("効果")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ERR_MISSING_EFFECT.to_string())?;
    
    let magic_category = format_magic_category(&spell.school);
    
    Ok(format!(
        "2d+{{{}}}+{{行使修正}}  {} / MP:{} / 対象:{} / 射程:{} / 時間:{} / {}",
        magic_category, spell.name, mp, target, range, duration, effect
    ))
}

// ============================================================================
// Entry Point（Claude 実装済み予定）
// ============================================================================

/// 呪文チャットパレットを生成する（メイン関数）
/// 
/// 補助フラグに応じて適切な Generator に委譲
pub fn generate_spell_palette(spell: &Spell) -> Result<String, String> {
    // 名前フィールドを確認
    if spell.name.is_empty() {
        return Err(ERR_MISSING_NAME.to_string());
    }
    
    // 補助フラグの確認
    let is_support = spell.extra.get("補助")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    if is_support {
        generate_support_palette(spell)
    } else {
        generate_regular_palette(spell)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // その他のテスト（Claude 実装予定）
    // ========================================================================

    /// MP値フォーマット: 固定コスト
    #[test]
    fn test_mp_format_value() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("MP".to_string(), serde_json::json!({"value": 5}));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_mp(&spell), Ok("5".to_string()));
    }

    /// MP値フォーマット: 最小コスト（value+）
    #[test]
    fn test_mp_format_value_plus() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("MP".to_string(), serde_json::json!({"value+": 3}));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_mp(&spell), Ok("3～".to_string()));
    }

    /// MP値フォーマット: 特殊コスト
    #[test]
    fn test_mp_format_special() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("MP".to_string(), serde_json::json!({"special": "複雑な計算式"}));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_mp(&spell), Ok("複雑な計算式".to_string()));
    }

    /// MP値フォーマット: エラー（すべて欠落）
    #[test]
    fn test_mp_format_error_missing_all() {
        let extra = std::collections::HashMap::new();
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert!(format_mp(&spell).is_err());
    }

    // ========================================================================
    // Target Format Tests (5)
    // ========================================================================

    /// 対象フォーマット: 個別（シンプル）
    #[test]
    fn test_target_format_individual_simple() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_target(&spell), Ok("1体".to_string()));
    }

    /// 対象フォーマット: 個別（複合）
    #[test]
    fn test_target_format_individual_complex() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体全"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_target(&spell), Ok("1体全".to_string()));
    }

    /// 対象フォーマット: エリア（シンプル）
    #[test]
    fn test_target_format_area_simple() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "エリア",
            "エリア": {
                "value": "1エリア",
                "半径(m)": 5,
                "末尾": "すべて"
            }
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_target(&spell), Ok("1エリア(半径5mすべて)".to_string()));
    }

    /// 対象フォーマット: エリア（複合）
    #[test]
    fn test_target_format_area_complex() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "エリア",
            "エリア": {
                "value": "2エリア",
                "半径(m)": 10,
                "末尾": "空間"
            }
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_target(&spell), Ok("2エリア(半径10m空間)".to_string()));
    }

    /// 対象フォーマット: エラー（不正なkind）
    #[test]
    fn test_target_format_error_invalid_kind() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "不正"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert!(format_target(&spell).is_err());
    }

    // ========================================================================
    // Duration Format Tests (3)
    // ========================================================================

    /// 時間フォーマット: 文字列型
    #[test]
    fn test_duration_format_string() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("時間".to_string(), serde_json::json!({
            "value": "一瞬"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_duration(&spell), Ok("一瞬".to_string()));
    }

    /// 時間フォーマット: 整数型
    #[test]
    fn test_duration_format_integer() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("時間".to_string(), serde_json::json!({
            "value": 3,
            "unit": "年"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_duration(&spell), Ok("3年".to_string()));
    }

    /// 時間フォーマット: 整数型（ラウンド）
    #[test]
    fn test_duration_format_integer_round() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("時間".to_string(), serde_json::json!({
            "value": 10,
            "unit": "ラウンド"
        }));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_duration(&spell), Ok("10ラウンド".to_string()));
    }

    // ========================================================================
    // Magic Category Format Tests (4) - 3個は kazuyasi が実装予定
    // ========================================================================

    /// 魔法系統フォーマット: 2文字（神聖）
    #[test]
    fn test_magic_category_2char() {
        let school = "神聖";
        assert_eq!(format_magic_category(school), "神聖魔法");
    }

    /// 魔法系統フォーマット: 2文字（妖精）
    #[test]
    fn test_magic_category_2char_fairy() {
        let school = "妖精";
        assert_eq!(format_magic_category(school), "妖精魔法");
    }

    /// 魔法系統フォーマット: 長い名前
    #[test]
    fn test_magic_category_long() {
        let school = "ハイテクノロジー";
        assert_eq!(format_magic_category(school), "ハイテクノロジー");
    }

    /// 魔法系統フォーマット: 1文字
    #[test]
    fn test_magic_category_1char() {
        let school = "禅";
        assert_eq!(format_magic_category(school), "禅");
    }

    // ========================================================================
    // Range Format Tests (5)
    // ========================================================================

    /// 射程フォーマット: 「射程」フィールド（基本）
    #[test]
    fn test_range_format_with_seishou_field() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_range(&spell), Ok("接触".to_string()));
    }

    /// 射程フォーマット: 「射程」フィールド（複雑な値）
    #[test]
    fn test_range_format_with_seishou_field_complex() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程".to_string(), serde_json::json!("10m(起点指定)"));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_range(&spell), Ok("10m(起点指定)".to_string()));
    }

    /// 射程フォーマット: 「射程(m)」フィールド（フォールバック）
    #[test]
    fn test_range_format_with_seishou_m_field() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程(m)".to_string(), serde_json::json!("20"));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_range(&spell), Ok("20".to_string()));
    }

    /// 射程フォーマット: 「射程」優先（両方フィールドがある場合）
    #[test]
    fn test_range_format_prefers_seishou() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("射程(m)".to_string(), serde_json::json!("20"));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        // 「射程」を優先
        assert_eq!(format_range(&spell), Ok("接触".to_string()));
    }

    /// 射程フォーマット: エラー（両方のフィールドが欠落）
    #[test]
    fn test_range_format_error_missing_both() {
        let extra = std::collections::HashMap::new();
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert!(format_range(&spell).is_err());
    }

    /// 射程フォーマット: 「射程(m)」が整数型
    #[test]
    fn test_range_format_with_seishou_m_integer() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程(m)".to_string(), serde_json::json!(30));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_range(&spell), Ok("30".to_string()));
    }

    /// 射程フォーマット: 「射程」が整数型
    #[test]
    fn test_range_format_with_seishou_integer() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("射程".to_string(), serde_json::json!(50));
        let spell = Spell {
            name: "テスト".to_string(),
            school: "魔法".to_string(),
            extra,
        };
        assert_eq!(format_range(&spell), Ok("50".to_string()));
    }

    // ========================================================================
    // Support Spell Palette Tests (3)
    // ========================================================================

    /// サポート呪文パレット: 基本
    #[test]
    fn test_support_palette_basic() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value": 3}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "任意の地点"
        }));
        extra.insert("射程".to_string(), serde_json::json!("術者"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("光源を生成する。"));

        let spell = Spell {
            name: "ライト".to_string(),
            school: "".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("ライト"));
        assert!(result.contains("MP:3"));
        assert!(result.contains("対象:任意の地点"));
        assert!(!result.contains("2d+"));  // No dice roll for support spells
    }

    /// サポート呪文パレット: value+ MP
    #[test]
    fn test_support_palette_with_value_plus_mp() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value+": 8}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "魔法1つ"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("魔法を打ち消す。"));

        let spell = Spell {
            name: "魔法解除".to_string(),
            school: "".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("MP:8～"));
    }

    /// サポート呪文パレット: 効果欠落エラー
    #[test]
    fn test_support_palette_error_missing_effect() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value": 3}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "テスト"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        // 効果フィールドを欠落

        let spell = Spell {
            name: "テスト".to_string(),
            school: "".to_string(),
            extra,
        };
        assert!(generate_spell_palette(&spell).is_err());
    }

    // ========================================================================
    // Regular Spell Palette Tests (3)
    // ========================================================================

    /// 通常呪文パレット: 基本
    #[test]
    fn test_regular_palette_basic() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 15}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "エリア",
            "エリア": {
                "value": "1エリア",
                "半径(m)": 4,
                "末尾": "すべて"
            }
        }));
        extra.insert("射程".to_string(), serde_json::json!("術者"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("物理的に神の捌きを下す。"));

        let spell = Spell {
            name: "ゴッド・ジャッジメント".to_string(),
            school: "神聖".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.starts_with("2d+"));
        assert!(result.contains("神聖魔法"));
        assert!(result.contains("{行使修正}"));
        assert!(result.contains("MP:15"));
    }

    /// 通常呪文パレット: 行使修正の保持
    #[test]
    fn test_regular_palette_execution_bonus_preserved() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 10}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("テスト効果"));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "プリエスト".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("{行使修正}"));
    }

    /// 通常呪文パレット: 魔法系統欠落エラー
    #[test]
    fn test_regular_palette_error_missing_school() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 5}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("テスト"));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "".to_string(),  // 空の school
            extra,
        };
        assert!(generate_spell_palette(&spell).is_err());
    }

    // ========================================================================
    // Entry Point Tests (3)
    // ========================================================================

    /// エントリーポイント: サポート呪文への委譲
    #[test]
    fn test_generate_palette_delegates_to_support() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value": 3}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "テスト"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("テスト効果"));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(!result.contains("2d+"));  // Support spells don't have dice rolls
    }

    /// エントリーポイント: 通常呪文への委譲
    #[test]
    fn test_generate_palette_delegates_to_regular() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 5}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("テスト効果"));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "神聖".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.starts_with("2d+"));  // Regular spells start with dice roll
    }

    /// エントリーポイント: エラー伝播
    #[test]
    fn test_generate_palette_error_propagation() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        // 効果フィールド欠落
        extra.insert("MP".to_string(), serde_json::json!({"value": 3}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "テスト"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "".to_string(),
            extra,
        };
        assert!(generate_spell_palette(&spell).is_err());
    }

    // ========================================================================
    // Integration Tests (3)
    // ========================================================================

    /// 統合テスト: サンプル呪文1（妖精魔法）
    #[test]
    fn test_palette_with_real_sample_spell_1() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 8}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "エリア",
            "エリア": {
                "value": "3エリア",
                "半径(m)": 6,
                "末尾": "空間"
            }
        }));
        extra.insert("射程".to_string(), serde_json::json!("術者"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("妖精の祝福を与える。"));

        let spell = Spell {
            name: "フェアリーズブレッシング".to_string(),
            school: "妖精".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("妖精魔法"));
        assert!(result.contains("フェアリーズブレッシング"));
        assert!(result.contains("3エリア(半径6m空間)"));
    }

    /// 統合テスト: サンプル呪文2（神聖魔法）
    #[test]
    fn test_palette_with_real_sample_spell_2() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 20}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        extra.insert("射程".to_string(), serde_json::json!("術者"));
        extra.insert("時間".to_string(), serde_json::json!({"value": 5, "unit": "ラウンド"}));
        extra.insert("効果".to_string(), serde_json::json!("神聖な力で対象を強化する。"));

        let spell = Spell {
            name: "ホーリーシールド".to_string(),
            school: "神聖".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("神聖魔法"));
        assert!(result.contains("ホーリーシールド"));
        assert!(result.contains("5ラウンド"));
    }

    /// 統合テスト: 日本語文字の保持
    #[test]
    fn test_palette_japanese_character_preservation() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value": 5}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "任意の標的"
        }));
        extra.insert("射程".to_string(), serde_json::json!("接触"));
        extra.insert("時間".to_string(), serde_json::json!({"value": "永続"}));
        extra.insert("効果".to_string(), serde_json::json!("対象に日本語の呪いを与える。特に危険な技能。"));

        let spell = Spell {
            name: "日本語呪い".to_string(),
            school: "".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("日本語呪い"));
        assert!(result.contains("任意の標的"));
        assert!(result.contains("対象に日本語の呪いを与える。特に危険な技能。"));
    }

    /// 統合テスト: 「射程(m)」フィールド使用（サポート呪文）
    #[test]
    fn test_palette_with_seishou_m_field_support() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(true));
        extra.insert("MP".to_string(), serde_json::json!({"value": 3}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "任意の地点"
        }));
        extra.insert("射程(m)".to_string(), serde_json::json!("30"));  // 「射程(m)」を使用
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("光源を生成する。"));

        let spell = Spell {
            name: "ライト".to_string(),
            school: "".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("ライト"));
        assert!(result.contains("射程:30"));  // 「射程(m)」の値が使われること
    }

    /// 統合テスト: 「射程(m)」フィールド使用（通常呪文）
    #[test]
    fn test_palette_with_seishou_m_field_regular() {
        let mut extra = std::collections::HashMap::new();
        extra.insert("補助".to_string(), serde_json::json!(false));
        extra.insert("MP".to_string(), serde_json::json!({"value": 10}));
        extra.insert("対象".to_string(), serde_json::json!({
            "kind": "個別",
            "個別": "1体"
        }));
        extra.insert("射程(m)".to_string(), serde_json::json!("50"));  // 「射程(m)」を使用
        extra.insert("時間".to_string(), serde_json::json!({"value": "一瞬"}));
        extra.insert("効果".to_string(), serde_json::json!("テスト効果"));

        let spell = Spell {
            name: "テスト".to_string(),
            school: "神聖".to_string(),
            extra,
        };
        let result = generate_spell_palette(&spell).unwrap();
        assert!(result.contains("射程:50"));  // 「射程(m)」の値が使われること
        assert!(result.contains("神聖魔法"));
    }
}
