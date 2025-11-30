use crate::Monster;

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
}
