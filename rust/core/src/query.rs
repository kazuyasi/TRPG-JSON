use crate::{Monster, Spell};

/// 名前でモンスターを検索（部分マッチ）
pub fn find_by_name<'a>(monsters: &'a [Monster], name: &str) -> Vec<&'a Monster> {
    monsters
        .iter()
        .filter(|m| m.name.contains(name))
        .collect()
}

/// レベルでモンスターを検索（完全一致）
pub fn find_by_level<'a>(monsters: &'a [Monster], level: i32) -> Vec<&'a Monster> {
    monsters
        .iter()
        .filter(|m| m.level == level)
        .collect()
}

/// カテゴリでモンスターを検索（完全一致）
pub fn find_by_category<'a>(monsters: &'a [Monster], category: &str) -> Vec<&'a Monster> {
    monsters
        .iter()
        .filter(|m| m.category == category)
        .collect()
}

/// 複合検索（名前、レベル、カテゴリの条件を組み合わせ）
pub fn find_multi<'a>(
    monsters: &'a [Monster],
    name: Option<&str>,
    level: Option<i32>,
    category: Option<&str>,
) -> Vec<&'a Monster> {
    monsters
        .iter()
        .filter(|m| {
            // 名前フィルタ
            if let Some(n) = name {
                if !m.name.contains(n) {
                    return false;
                }
            }
            // レベルフィルタ
            if let Some(l) = level {
                if m.level != l {
                    return false;
                }
            }
            // カテゴリフィルタ
            if let Some(c) = category {
                if m.category != c {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// 完全一致で名前を検索
pub fn find_by_exact_name<'a>(monsters: &'a [Monster], name: &str) -> Option<&'a Monster> {
    monsters
        .iter()
        .find(|m| m.name == name)
}

// ============================================================================
// Spell Query Functions
// ============================================================================

/// スペルを名前で検索（部分マッチ）
pub fn spell_find_by_name<'a>(spells: &'a [Spell], name: &str) -> Vec<&'a Spell> {
    spells
        .iter()
        .filter(|s| s.name.contains(name))
        .collect()
}

/// スペルを系統で検索（完全一致）
pub fn spell_find_by_school<'a>(spells: &'a [Spell], school: &str) -> Vec<&'a Spell> {
    spells
        .iter()
        .filter(|s| s.school == school)
        .collect()
}

/// スペルをレベルで検索（完全一致）
pub fn spell_find_by_level<'a>(spells: &'a [Spell], level: i32) -> Vec<&'a Spell> {
    spells
        .iter()
        .filter(|s| extract_spell_level(s) == level)
        .collect()
}

/// Spell オブジェクトから Lv を抽出
fn extract_spell_level(spell: &Spell) -> i32 {
    if let Some(lv_obj) = spell.extra.get("Lv") {
        if let Some(obj) = lv_obj.as_object() {
            if let Some(value) = obj.get("value") {
                return value.as_i64().unwrap_or(0) as i32;
            }
            if let Some(value) = obj.get("value+") {
                return value.as_i64().unwrap_or(0) as i32;
            }
            if let Some(rank) = obj.get("rank") {
                return rank.as_i64().unwrap_or(0) as i32;
            }
        }
    }
    0
}

/// スペルを複合検索（名前、系統、レベルの条件を組み合わせ）
pub fn spell_find_multi<'a>(
    spells: &'a [Spell],
    name: Option<&str>,
    school: Option<&str>,
    level: Option<i32>,
) -> Vec<&'a Spell> {
    spells
        .iter()
        .filter(|s| {
            // 名前フィルタ
            if let Some(n) = name {
                if !s.name.contains(n) {
                    return false;
                }
            }
            // 系統フィルタ
            if let Some(sch) = school {
                if s.school != sch {
                    return false;
                }
            }
            // レベルフィルタ
            if let Some(l) = level {
                if extract_spell_level(s) != l {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// スペルを完全一致で名前から検索
pub fn spell_find_by_exact_name<'a>(spells: &'a [Spell], name: &str) -> Option<&'a Spell> {
    spells
        .iter()
        .find(|s| s.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn sample_monsters() -> Vec<Monster> {
        let json_data = r#"[
            {
                "Category": "蛮族",
                "Lv": 6,
                "Revision": 2.5,
                "data": "TEST001",
                "illust": "",
                "movein": 22,
                "movein_des": "飛行",
                "moveon": 22,
                "moveon_des": "",
                "name": "魔法使い系の敵",
                "part": [
                    {
                        "HP": 48,
                        "MP": 75,
                        "name": "",
                        "コア": true,
                        "命中力": 15,
                        "回避力": 15,
                        "打撃点": 6,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 5
                    }
                ],
                "備考": "",
                "先制値": 14,
                "共通特殊能力": "飛行",
                "弱点": "属性ダメージ",
                "弱点値": 17,
                "生命抵抗力": 16,
                "知名度": 14,
                "精神抵抗力": 16
            },
            {
                "Category": "蛮族",
                "Lv": 3,
                "Revision": 2.5,
                "data": "TEST002",
                "illust": "",
                "movein": 10,
                "movein_des": "歩行",
                "moveon": 10,
                "moveon_des": "",
                "name": "小型人型敵",
                "part": [
                    {
                        "HP": 20,
                        "MP": 10,
                        "name": "",
                        "コア": true,
                        "命中力": 10,
                        "回避力": 10,
                        "打撃点": 3,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 2
                    }
                ],
                "備考": "",
                "先制値": 8,
                "共通特殊能力": "",
                "弱点": "属性ダメージ",
                "弱点値": 12,
                "生命抵抗力": 10,
                "知名度": 8,
                "精神抵抗力": 10
            },
            {
                "Category": "魔法生物",
                "Lv": 6,
                "Revision": 2.5,
                "data": "TEST003",
                "illust": "",
                "movein": -1,
                "movein_des": "",
                "moveon": -1,
                "moveon_des": "",
                "name": "元素系の敵",
                "part": [
                    {
                        "HP": 40,
                        "MP": -1,
                        "name": "",
                        "コア": true,
                        "命中力": 12,
                        "回避力": 14,
                        "打撃点": 8,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 3
                    }
                ],
                "備考": "",
                "先制値": 15,
                "共通特殊能力": "属性",
                "弱点": "属性ダメージ",
                "弱点値": 15,
                "生命抵抗力": 14,
                "知名度": 12,
                "精神抵抗力": 14
            }
        ]"#;

        serde_json::from_str(json_data).expect("Failed to parse sample monsters")
    }

    #[test]
    fn test_find_by_name_single_match() {
        let monsters = sample_monsters();
        let results = find_by_name(&monsters, "小型人型敵");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "小型人型敵");
    }

    #[test]
    fn test_find_by_name_partial_match() {
        let monsters = sample_monsters();
        let results = find_by_name(&monsters, "魔法");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "魔法使い系の敵");
    }

    #[test]
    fn test_find_by_name_no_match() {
        let monsters = sample_monsters();
        let results = find_by_name(&monsters, "存在しないモンスター");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_find_by_level_single_match() {
        let monsters = sample_monsters();
        let results = find_by_level(&monsters, 3);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].level, 3);
        assert_eq!(results[0].name, "小型人型敵");
    }

    #[test]
    fn test_find_by_level_multiple_matches() {
        let monsters = sample_monsters();
        let results = find_by_level(&monsters, 6);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|m| m.level == 6));
    }

    #[test]
    fn test_find_by_level_no_match() {
        let monsters = sample_monsters();
        let results = find_by_level(&monsters, 99);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_find_by_category_single_match() {
        let monsters = sample_monsters();
        let results = find_by_category(&monsters, "魔法生物");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "元素系の敵");
        assert_eq!(results[0].category, "魔法生物");
    }

    #[test]
    fn test_find_by_category_multiple_matches() {
        let monsters = sample_monsters();
        let results = find_by_category(&monsters, "蛮族");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|m| m.category == "蛮族"));
    }

    #[test]
    fn test_find_by_category_no_match() {
        let monsters = sample_monsters();
        let results = find_by_category(&monsters, "存在しないカテゴリ");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_find_multi_name_only() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, Some("魔法"), None, None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "魔法使い系の敵");
    }

    #[test]
    fn test_find_multi_level_only() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, None, Some(6), None);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_multi_category_only() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, None, None, Some("蛮族"));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_multi_name_and_level() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, Some("魔法"), Some(6), None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "魔法使い系の敵");
    }

    #[test]
    fn test_find_multi_name_and_level_no_match() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, Some("小型人型敵"), Some(6), None);
        assert_eq!(results.len(), 0); // 小型人型敵はLv3なので、Lv6での検索ではヒットしない
    }

    #[test]
    fn test_find_multi_all_filters() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, Some("魔法"), Some(6), Some("蛮族"));
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "魔法使い系の敵");
    }

    #[test]
    fn test_find_multi_all_filters_no_match() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, Some("魔法"), Some(6), Some("魔法生物"));
        assert_eq!(results.len(), 0); // 魔法使い系は蛮族カテゴリなので、魔法生物での検索ではヒットしない
    }

    #[test]
    fn test_find_multi_no_filters() {
        let monsters = sample_monsters();
        let results = find_multi(&monsters, None, None, None);
        assert_eq!(results.len(), 3); // すべての条件がNoneの場合、すべてのモンスターを返す
    }

    #[test]
    fn test_find_by_exact_name_match() {
        let monsters = sample_monsters();
        let result = find_by_exact_name(&monsters, "小型人型敵");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "小型人型敵");
    }

    #[test]
    fn test_find_by_exact_name_no_match() {
        let monsters = sample_monsters();
        let result = find_by_exact_name(&monsters, "存在しないモンスター");
        assert!(result.is_none());
    }

     #[test]
     fn test_find_by_exact_name_partial_no_match() {
         let monsters = sample_monsters();
         let result = find_by_exact_name(&monsters, "小型");
         assert!(result.is_none()); // 部分マッチはしない
     }

     // ========================================================================
     // Spell Query Tests
     // ========================================================================

       fn sample_spells() -> Vec<Spell> {
           let json_data = r#"[
               {
                   "name": "Magic_47438",
                   "school": "MagicCat_1",
                   "Lv": { "kind": "value", "value": 9 },
                   "MP": { "kind": "value", "value": 86 },
                   "effect": "EffectDescription_41410",
                   "対象": { "kind": "個別", "個別": "1体全" }
               },
               {
                   "name": "Magic_33778",
                   "school": "MagicCat_2",
                   "Lv": { "kind": "value", "value": 13 },
                   "MP": { "kind": "value", "value": 15 },
                   "effect": "EffectDescription_75723",
                   "対象": { "kind": "エリア", "エリア": { "value": "1エリア", "半径(m)": 35 } }
               },
               {
                   "name": "Magic_83071",
                   "school": "MagicCat_2",
                   "Lv": { "kind": "value", "value": 3 },
                   "MP": { "kind": "value", "value": 72 },
                   "effect": "EffectDescription_37348",
                   "対象": { "kind": "個別", "個別": "接触点" }
               },
               {
                   "name": "Magic_16470",
                   "school": "MagicCat_1",
                   "Lv": { "kind": "value", "value": 7 },
                   "MP": { "kind": "value", "value": 40 },
                   "effect": "EffectDescription_32293",
                   "対象": { "kind": "個別", "個別": "弾丸" }
               },
               {
                   "name": "Magic_88250",
                   "school": "MagicCat_2",
                   "Lv": { "kind": "value", "value": 7 },
                   "MP": { "kind": "value", "value": 82 },
                   "effect": "EffectDescription_14305",
                   "対象": { "kind": "個別", "個別": "魔法1つ" }
               }
           ]"#;

          serde_json::from_str(json_data).expect("Failed to parse sample spells")
      }

     #[test]
     fn test_spell_find_by_name_single_match() {
         let spells = sample_spells();
         let results = spell_find_by_name(&spells, "Magic_47438");
         assert_eq!(results.len(), 1);
         assert_eq!(results[0].name, "Magic_47438");
     }

     #[test]
     fn test_spell_find_by_name_partial_match() {
         let spells = sample_spells();
         let results = spell_find_by_name(&spells, "47438");
         assert_eq!(results.len(), 1); // Magic_47438
     }

     #[test]
     fn test_spell_find_by_name_no_match() {
         let spells = sample_spells();
         let results = spell_find_by_name(&spells, "NonExistent");
         assert_eq!(results.len(), 0);
     }

     #[test]
     fn test_spell_find_by_school_single_match() {
         let spells = sample_spells();
         let results = spell_find_by_school(&spells, "MagicCat_1");
         assert_eq!(results.len(), 2); // Magic_47438, Magic_16470
         assert!(results.iter().all(|s| s.school == "MagicCat_1"));
     }

     #[test]
     fn test_spell_find_by_school_multiple_match() {
         let spells = sample_spells();
         let results = spell_find_by_school(&spells, "MagicCat_2");
         assert_eq!(results.len(), 3); // Magic_33778, Magic_83071, Magic_88250
         assert!(results.iter().all(|s| s.school == "MagicCat_2"));
     }

     #[test]
     fn test_spell_find_by_school_no_match() {
         let spells = sample_spells();
         let results = spell_find_by_school(&spells, "NonExistent");
         assert_eq!(results.len(), 0);
     }

     #[test]
     fn test_spell_find_by_level_single_match() {
         let spells = sample_spells();
         let results = spell_find_by_level(&spells, 13);
         assert_eq!(results.len(), 1);
         assert_eq!(results[0].name, "Magic_33778");
     }

     #[test]
     fn test_spell_find_by_level_multiple_match() {
         let spells = sample_spells();
         let results = spell_find_by_level(&spells, 7);
         assert_eq!(results.len(), 2); // Magic_16470, Magic_88250
     }

     #[test]
     fn test_spell_find_by_level_no_match() {
         let spells = sample_spells();
         let results = spell_find_by_level(&spells, 99);
         assert_eq!(results.len(), 0);
     }

     #[test]
     fn test_spell_find_multi_name_only() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, Some("47438"), None, None);
         assert_eq!(results.len(), 1); // Magic_47438
     }

     #[test]
     fn test_spell_find_multi_school_only() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, None, Some("MagicCat_1"), None);
         assert_eq!(results.len(), 2);
     }

     #[test]
     fn test_spell_find_multi_level_only() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, None, None, Some(7));
         assert_eq!(results.len(), 2);
     }

     #[test]
     fn test_spell_find_multi_name_and_school() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, Some("47438"), Some("MagicCat_1"), None);
         assert_eq!(results.len(), 1); // Magic_47438 MagicCat_1
         assert!(results.iter().all(|s| s.school == "MagicCat_1"));
     }

     #[test]
     fn test_spell_find_multi_name_and_school_no_match() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, Some("Magic_4"), Some("MagicCat_2"), None);
         assert_eq!(results.len(), 0); // Magic_47438は MagicCat_1
     }

     #[test]
     fn test_spell_find_multi_all_filters() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, Some("Magic_"), Some("MagicCat_1"), Some(9));
         assert_eq!(results.len(), 1);
         assert_eq!(results[0].name, "Magic_47438");
     }

     #[test]
     fn test_spell_find_multi_all_filters_no_match() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, Some("Magic_"), Some("MagicCat_1"), Some(13));
         assert_eq!(results.len(), 0); // MagicCat_1 の Lv 13 はない
     }

     #[test]
     fn test_spell_find_multi_no_filters() {
         let spells = sample_spells();
         let results = spell_find_multi(&spells, None, None, None);
         assert_eq!(results.len(), 5); // すべてのスペルを返す
     }

     #[test]
     fn test_spell_find_by_exact_name_match() {
         let spells = sample_spells();
         let result = spell_find_by_exact_name(&spells, "Magic_47438");
         assert!(result.is_some());
         assert_eq!(result.unwrap().name, "Magic_47438");
     }

     #[test]
     fn test_spell_find_by_exact_name_no_match() {
         let spells = sample_spells();
         let result = spell_find_by_exact_name(&spells, "NonExistent");
         assert!(result.is_none());
     }

     #[test]
     fn test_spell_find_by_exact_name_partial_no_match() {
         let spells = sample_spells();
         let result = spell_find_by_exact_name(&spells, "Magic_3");
         assert!(result.is_none()); // 部分マッチはしない
     }
}
