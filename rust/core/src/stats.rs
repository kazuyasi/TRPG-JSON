// stats.rs - データセット統計情報の計算

use crate::{Monster, Spell};
#[cfg(test)]
use crate::Part;
use std::collections::HashMap;

/// モンスター統計情報
#[derive(Debug)]
pub struct MonsterStats {
    pub total_count: usize,
    pub level_distribution: Vec<(String, usize, f64)>, // (範囲, 件数, パーセント)
    pub category_distribution: Vec<(String, usize, f64)>, // Top 5
    pub numeric_ranges: NumericRanges,
}

/// 数値フィールドの範囲
#[derive(Debug)]
pub struct NumericRanges {
    pub hit_rate: (i32, i32),       // (最小, 最大)
    pub dodge: (i32, i32),
    pub damage: (i32, i32),
    pub armor: (i32, i32),
    pub life_resistance: (i32, i32),
    pub mental_resistance: (i32, i32),
}

/// スペル統計情報
#[derive(Debug)]
pub struct SpellStats {
    pub total_count: usize,
    pub level_distribution: Vec<(String, usize, f64)>, // (範囲, 件数, パーセント)
    pub school_distribution: Vec<(String, usize, f64)>, // Top 5
    pub level_type_count: usize,
    pub rank_type_count: usize,
}

impl MonsterStats {
    /// モンスターデータから統計情報を計算
    pub fn calculate(monsters: &[Monster]) -> Self {
        let total_count = monsters.len();
        
        // レベル分布を計算 (1-4, 5-8, 9-12, 13-16, 17+)
        let level_distribution = Self::calculate_level_distribution(monsters);
        
        // カテゴリ分布を計算 (Top 5)
        let category_distribution = Self::calculate_category_distribution(monsters);
        
        // 数値フィールドの範囲を計算
        let numeric_ranges = Self::calculate_numeric_ranges(monsters);
        
        MonsterStats {
            total_count,
            level_distribution,
            category_distribution,
            numeric_ranges,
        }
    }
    
    /// レベル分布を計算
    fn calculate_level_distribution(monsters: &[Monster]) -> Vec<(String, usize, f64)> {
        let ranges = vec![
            ("Lv 1-4", 1, 4),
            ("Lv 5-8", 5, 8),
            ("Lv 9-12", 9, 12),
            ("Lv 13-16", 13, 16),
            ("Lv 17+", 17, i32::MAX),
        ];
        
        let total = monsters.len() as f64;
        let mut distribution = Vec::new();
        
        for (label, min, max) in ranges {
            let count = monsters.iter()
                .filter(|m| m.level >= min && m.level <= max)
                .count();
            let percentage = if total > 0.0 { (count as f64 / total) * 100.0 } else { 0.0 };
            distribution.push((label.to_string(), count, percentage));
        }
        
        distribution
    }
    
    /// カテゴリ分布を計算 (Top 5)
    fn calculate_category_distribution(monsters: &[Monster]) -> Vec<(String, usize, f64)> {
        let mut category_counts: HashMap<String, usize> = HashMap::new();
        
        for monster in monsters {
            *category_counts.entry(monster.category.clone()).or_insert(0) += 1;
        }
        
        let total = monsters.len() as f64;
        let mut distribution: Vec<_> = category_counts.into_iter()
            .map(|(category, count)| {
                let percentage = if total > 0.0 { (count as f64 / total) * 100.0 } else { 0.0 };
                (category, count, percentage)
            })
            .collect();
        
        // 件数で降順ソート、Top 5を取得
        distribution.sort_by(|a, b| b.1.cmp(&a.1));
        distribution.truncate(5);
        
        distribution
    }
    
    /// 数値フィールドの範囲を計算
    fn calculate_numeric_ranges(monsters: &[Monster]) -> NumericRanges {
        // 各パートから数値を収集
        let mut hit_rates = Vec::new();
        let mut dodges = Vec::new();
        let mut damages = Vec::new();
        let mut armors = Vec::new();
        
        for monster in monsters {
            for part in &monster.part {
                if let Some(hr) = part.hit_rate {
                    hit_rates.push(hr);
                }
                if let Some(d) = part.dodge {
                    dodges.push(d);
                }
                if let Some(dmg) = part.damage {
                    damages.push(dmg);
                }
                armors.push(part.armor);
            }
        }
        
        // モンスターレベルの生命抵抗力と精神抵抗力を収集
        let life_resistances: Vec<i32> = monsters.iter().map(|m| m.life_resistance).collect();
        let mental_resistances: Vec<i32> = monsters.iter().map(|m| m.mental_resistance).collect();
        
        NumericRanges {
            hit_rate: Self::min_max(&hit_rates),
            dodge: Self::min_max(&dodges),
            damage: Self::min_max(&damages),
            armor: Self::min_max(&armors),
            life_resistance: Self::min_max(&life_resistances),
            mental_resistance: Self::min_max(&mental_resistances),
        }
    }
    
    /// 最小値と最大値を取得
    fn min_max(values: &[i32]) -> (i32, i32) {
        if values.is_empty() {
            return (0, 0);
        }
        let min = *values.iter().min().unwrap();
        let max = *values.iter().max().unwrap();
        (min, max)
    }
    
    /// 統計情報を整形して出力
    pub fn format(&self) -> String {
        let mut output = String::new();
        
        output.push_str("モンスター統計:\n");
        output.push_str(&format!("  総数: {}\n", self.total_count));
        
        output.push_str("  レベル分布:\n");
        for (range, count, percentage) in &self.level_distribution {
            output.push_str(&format!("    {}: {:3} ({:5.1}%)\n", range, count, percentage));
        }
        
        output.push_str("  カテゴリ分布 (Top 5):\n");
        for (category, count, percentage) in &self.category_distribution {
            output.push_str(&format!("    {}: {:3} ({:5.1}%)\n", category, count, percentage));
        }
        
        output.push_str("  数値フィールド範囲:\n");
        let ranges = &self.numeric_ranges;
        output.push_str(&format!("    命中力: {}～{}\n", ranges.hit_rate.0, ranges.hit_rate.1));
        output.push_str(&format!("    回避力: {}～{}\n", ranges.dodge.0, ranges.dodge.1));
        output.push_str(&format!("    打撃点: {}～{}\n", ranges.damage.0, ranges.damage.1));
        output.push_str(&format!("    防護点: {}～{}\n", ranges.armor.0, ranges.armor.1));
        output.push_str(&format!("    生命抵抗力: {}～{}\n", ranges.life_resistance.0, ranges.life_resistance.1));
        output.push_str(&format!("    精神抵抗力: {}～{}\n", ranges.mental_resistance.0, ranges.mental_resistance.1));
        
        output
    }
}

impl SpellStats {
    /// スペルデータから統計情報を計算
    pub fn calculate(spells: &[Spell]) -> Self {
        let total_count = spells.len();
        
        // レベル分布を計算 (1-5, 6-10, 11-15, 16+)
        let level_distribution = Self::calculate_level_distribution(spells);
        
        // 系統分布を計算 (Top 5)
        let school_distribution = Self::calculate_school_distribution(spells);
        
        // レベル型/ランク型の件数を計算
        let (level_type_count, rank_type_count) = Self::calculate_type_counts(spells);
        
        SpellStats {
            total_count,
            level_distribution,
            school_distribution,
            level_type_count,
            rank_type_count,
        }
    }
    
    /// レベル分布を計算
    fn calculate_level_distribution(spells: &[Spell]) -> Vec<(String, usize, f64)> {
        let ranges = vec![
            ("Lv 1-5", 1, 5),
            ("Lv 6-10", 6, 10),
            ("Lv 11-15", 11, 15),
            ("Lv 16+", 16, i32::MAX),
        ];
        
        let total = spells.len() as f64;
        let mut distribution = Vec::new();
        
        for (label, min, max) in ranges {
            let count = spells.iter()
                .filter(|s| {
                    if let Some(lv_obj) = s.extra.get("Lv") {
                        if let Some(kind) = lv_obj.get("kind").and_then(|k| k.as_str()) {
                            if kind == "value" || kind == "value+" {
                                if let Some(value) = lv_obj.get("value").and_then(|v| v.as_i64()) {
                                    let level = value as i32;
                                    return level >= min && level <= max;
                                }
                            }
                        }
                    }
                    false
                })
                .count();
            let percentage = if total > 0.0 { (count as f64 / total) * 100.0 } else { 0.0 };
            distribution.push((label.to_string(), count, percentage));
        }
        
        distribution
    }
    
    /// 系統分布を計算 (Top 5)
    fn calculate_school_distribution(spells: &[Spell]) -> Vec<(String, usize, f64)> {
        let mut school_counts: HashMap<String, usize> = HashMap::new();
        
        for spell in spells {
            *school_counts.entry(spell.school.clone()).or_insert(0) += 1;
        }
        
        let total = spells.len() as f64;
        let mut distribution: Vec<_> = school_counts.into_iter()
            .map(|(school, count)| {
                let percentage = if total > 0.0 { (count as f64 / total) * 100.0 } else { 0.0 };
                (school, count, percentage)
            })
            .collect();
        
        // 件数で降順ソート、Top 5を取得
        distribution.sort_by(|a, b| b.1.cmp(&a.1));
        distribution.truncate(5);
        
        distribution
    }
    
    /// レベル型/ランク型の件数を計算
    fn calculate_type_counts(spells: &[Spell]) -> (usize, usize) {
        let mut level_type = 0;
        let mut rank_type = 0;
        
        for spell in spells {
            if let Some(lv_obj) = spell.extra.get("Lv") {
                if let Some(kind) = lv_obj.get("kind").and_then(|k| k.as_str()) {
                    match kind {
                        "value" | "value+" => level_type += 1,
                        "rank" => rank_type += 1,
                        _ => {}
                    }
                }
            }
        }
        
        (level_type, rank_type)
    }
    
    /// 統計情報を整形して出力
    pub fn format(&self) -> String {
        let mut output = String::new();
        
        output.push_str("スペル統計:\n");
        output.push_str(&format!("  総数: {}\n", self.total_count));
        
        output.push_str("  レベル分布:\n");
        for (range, count, percentage) in &self.level_distribution {
            output.push_str(&format!("    {}: {:3} ({:5.1}%)\n", range, count, percentage));
        }
        
        output.push_str("  系統分布 (Top 5):\n");
        for (school, count, percentage) in &self.school_distribution {
            output.push_str(&format!("    {}: {:3} ({:5.1}%)\n", school, count, percentage));
        }
        
        output.push_str("  種別:\n");
        let total = self.total_count as f64;
        let level_pct = if total > 0.0 { (self.level_type_count as f64 / total) * 100.0 } else { 0.0 };
        let rank_pct = if total > 0.0 { (self.rank_type_count as f64 / total) * 100.0 } else { 0.0 };
        output.push_str(&format!("    レベル型: {} ({:.1}%)\n", self.level_type_count, level_pct));
        output.push_str(&format!("    ランク型: {} ({:.1}%)\n", self.rank_type_count, rank_pct));
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    fn create_test_monster(level: i32, category: &str) -> Monster {
        Monster {
            category: category.to_string(),
            level,
            revision: 2.5,
            data: "TEST001".to_string(),
            illust: "".to_string(),
            movein: -1,
            movein_description: "".to_string(),
            moveon: -1,
            moveon_description: "".to_string(),
            name: format!("テストモンスター{}", level),
            part: vec![Part {
                hp: Some(50),
                mp: 50,
                name: "".to_string(),
                core: Some(true),
                hit_rate: Some(15),
                dodge: Some(15),
                damage: Some(10),
                part_count: 1,
                special_abilities: "".to_string(),
                armor: 5,
            }],
            notes: "".to_string(),
            initiative: 14,
            common_abilities: "".to_string(),
            weakness: "".to_string(),
            weakness_value: 17,
            life_resistance: 16,
            fame: 14,
            mental_resistance: 16,
            extra: HashMap::new(),
        }
    }
    
    #[test]
    fn test_monster_stats_total_count() {
        let monsters = vec![
            create_test_monster(3, "蛮族"),
            create_test_monster(7, "蛮族"),
            create_test_monster(12, "魔法生物"),
        ];
        
        let stats = MonsterStats::calculate(&monsters);
        assert_eq!(stats.total_count, 3);
    }
    
    #[test]
    fn test_monster_level_distribution() {
        let monsters = vec![
            create_test_monster(2, "蛮族"),  // 1-4
            create_test_monster(4, "蛮族"),  // 1-4
            create_test_monster(6, "蛮族"),  // 5-8
            create_test_monster(10, "魔法生物"), // 9-12
            create_test_monster(18, "動物"), // 17+
        ];
        
        let stats = MonsterStats::calculate(&monsters);
        
        // Lv 1-4: 2件
        assert_eq!(stats.level_distribution[0].1, 2);
        // Lv 5-8: 1件
        assert_eq!(stats.level_distribution[1].1, 1);
        // Lv 9-12: 1件
        assert_eq!(stats.level_distribution[2].1, 1);
        // Lv 17+: 1件
        assert_eq!(stats.level_distribution[4].1, 1);
    }
    
    #[test]
    fn test_monster_category_top5() {
        let monsters = vec![
            create_test_monster(3, "蛮族"),
            create_test_monster(3, "蛮族"),
            create_test_monster(3, "蛮族"),
            create_test_monster(5, "魔法生物"),
            create_test_monster(5, "魔法生物"),
            create_test_monster(7, "動物"),
        ];
        
        let stats = MonsterStats::calculate(&monsters);
        
        // Top 1: 蛮族 3件
        assert_eq!(stats.category_distribution[0].0, "蛮族");
        assert_eq!(stats.category_distribution[0].1, 3);
        
        // Top 2: 魔法生物 2件
        assert_eq!(stats.category_distribution[1].0, "魔法生物");
        assert_eq!(stats.category_distribution[1].1, 2);
    }
    
    #[test]
    fn test_monster_numeric_ranges() {
        let mut m1 = create_test_monster(3, "蛮族");
        m1.part[0].hit_rate = Some(10);
        m1.part[0].dodge = Some(12);
        m1.part[0].damage = Some(5);
        m1.part[0].armor = 3;
        m1.life_resistance = 14;
        m1.mental_resistance = 13;
        
        let mut m2 = create_test_monster(10, "魔法生物");
        m2.part[0].hit_rate = Some(20);
        m2.part[0].dodge = Some(18);
        m2.part[0].damage = Some(15);
        m2.part[0].armor = 8;
        m2.life_resistance = 20;
        m2.mental_resistance = 19;
        
        let monsters = vec![m1, m2];
        let stats = MonsterStats::calculate(&monsters);
        
        assert_eq!(stats.numeric_ranges.hit_rate, (10, 20));
        assert_eq!(stats.numeric_ranges.dodge, (12, 18));
        assert_eq!(stats.numeric_ranges.damage, (5, 15));
        assert_eq!(stats.numeric_ranges.armor, (3, 8));
        assert_eq!(stats.numeric_ranges.life_resistance, (14, 20));
        assert_eq!(stats.numeric_ranges.mental_resistance, (13, 19));
    }
    
    fn create_test_spell(level: i32, school: &str, kind: &str) -> Spell {
        let mut extra = HashMap::new();
        
        let mut lv_obj = serde_json::Map::new();
        lv_obj.insert("kind".to_string(), serde_json::Value::String(kind.to_string()));
        if kind == "value" || kind == "value+" {
            lv_obj.insert("value".to_string(), serde_json::Value::Number(level.into()));
        } else if kind == "rank" {
            lv_obj.insert("rank".to_string(), serde_json::Value::Number(level.into()));
        }
        extra.insert("Lv".to_string(), serde_json::Value::Object(lv_obj));
        
        Spell {
            name: format!("テストスペル{}", level),
            school: school.to_string(),
            extra,
        }
    }
    
    #[test]
    fn test_spell_stats_total_count() {
        let spells = vec![
            create_test_spell(3, "神聖魔法", "value"),
            create_test_spell(7, "操霊魔法", "value"),
        ];
        
        let stats = SpellStats::calculate(&spells);
        assert_eq!(stats.total_count, 2);
    }
    
    #[test]
    fn test_spell_level_distribution() {
        let spells = vec![
            create_test_spell(2, "神聖魔法", "value"),  // 1-5
            create_test_spell(5, "神聖魔法", "value"),  // 1-5
            create_test_spell(8, "操霊魔法", "value"),  // 6-10
            create_test_spell(12, "妖精魔法", "value"), // 11-15
            create_test_spell(18, "古代語魔法", "value"), // 16+
        ];
        
        let stats = SpellStats::calculate(&spells);
        
        // Lv 1-5: 2件
        assert_eq!(stats.level_distribution[0].1, 2);
        // Lv 6-10: 1件
        assert_eq!(stats.level_distribution[1].1, 1);
        // Lv 11-15: 1件
        assert_eq!(stats.level_distribution[2].1, 1);
        // Lv 16+: 1件
        assert_eq!(stats.level_distribution[3].1, 1);
    }
    
    #[test]
    fn test_spell_type_counts() {
        let spells = vec![
            create_test_spell(3, "神聖魔法", "value"),
            create_test_spell(5, "操霊魔法", "value+"),
            create_test_spell(2, "妖精魔法", "rank"),
            create_test_spell(3, "妖精魔法", "rank"),
        ];
        
        let stats = SpellStats::calculate(&spells);
        
        assert_eq!(stats.level_type_count, 2); // value + value+
        assert_eq!(stats.rank_type_count, 2);  // rank x2
    }
    
    #[test]
    fn test_spell_school_top5() {
        let spells = vec![
            create_test_spell(3, "神聖魔法", "value"),
            create_test_spell(3, "神聖魔法", "value"),
            create_test_spell(3, "神聖魔法", "value"),
            create_test_spell(5, "操霊魔法", "value"),
            create_test_spell(5, "操霊魔法", "value"),
            create_test_spell(7, "妖精魔法", "value"),
        ];
        
        let stats = SpellStats::calculate(&spells);
        
        // Top 1: 神聖魔法 3件
        assert_eq!(stats.school_distribution[0].0, "神聖魔法");
        assert_eq!(stats.school_distribution[0].1, 3);
        
        // Top 2: 操霊魔法 2件
        assert_eq!(stats.school_distribution[1].0, "操霊魔法");
        assert_eq!(stats.school_distribution[1].1, 2);
    }
}
