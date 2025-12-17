use crate::Monster;

/// Udonarium用に変換されたモンスター表現
#[derive(Debug, Clone)]
pub struct TransformedMonster {
    /// モンスター名
    pub name: String,
    /// カテゴリ
    pub category: String,
    /// レベル
    pub level: i32,
    /// 知名度（コア部位のみ）
    pub fame: i32,
    /// 先制値（コア部位のみ）
    pub initiative: i32,
    /// 共通特殊能力
    pub common_abilities: String,
    /// 各部位の変換データ
    pub parts: Vec<TransformedPart>,
}

/// Udonarium用に変換された部位表現
#[derive(Debug, Clone)]
pub struct TransformedPart {
    /// 部位名（XML用表示名）
    pub display_name: String,
    /// HP
    pub hp: i32,
    /// MP（-1の場合は0に変換）
    pub mp: i32,
    /// 防護点
    pub armor: i32,
    /// 命中力（元値、XML生成時に-7調整）
    pub hit_rate: i32,
    /// 回避力（元値、XML生成時に-7調整）
    pub dodge: i32,
    /// 打撃点（既に基本値、調整不要）
    pub damage: i32,
    /// 生命抵抗力（元値、コア部位のみ、XML生成時に-7調整）
    pub life_resistance: i32,
    /// 精神抵抗力（元値、コア部位のみ、XML生成時に-7調整）
    pub mental_resistance: i32,
    /// 部位特殊能力
    pub special_abilities: String,
    /// コア判定
    pub is_core: bool,
    /// 弱点（コア部位のみ）
    pub weakness: String,
    /// 弱点値（コア部位のみ）
    pub weakness_value: i32,
}

/// データ変換器
pub struct DataTransformer;

impl DataTransformer {
    /// Monster を TransformedMonster に変換
    pub fn transform(monster: &Monster, display_names: Vec<String>) -> TransformedMonster {
        let mut parts = Vec::new();

        for (i, part) in monster.part.iter().enumerate() {
            let display_name = display_names.get(i).cloned().unwrap_or_default();
            let is_core = part.core.unwrap_or(false);

            let transformed_part = TransformedPart {
                display_name,
                hp: part.hp.unwrap_or(0),
                mp: if part.mp < 0 { 0 } else { part.mp },
                armor: part.armor,
                hit_rate: part.hit_rate.unwrap_or(0),
                dodge: part.dodge.unwrap_or(0),
                damage: part.damage.unwrap_or(0),
                life_resistance: if is_core {
                    monster.life_resistance
                } else {
                    monster.life_resistance
                },
                mental_resistance: if is_core {
                    monster.mental_resistance
                } else {
                    monster.mental_resistance
                },
                special_abilities: part.special_abilities.clone(),
                is_core,
                weakness: if is_core {
                    monster.weakness.clone()
                } else {
                    String::new()
                },
                weakness_value: if is_core {
                    monster.weakness_value
                } else {
                    0
                },
            };

            parts.push(transformed_part);
        }

        TransformedMonster {
            name: monster.name.clone(),
            category: monster.category.clone(),
            level: monster.level,
            fame: monster.fame,
            initiative: monster.initiative,
            common_abilities: monster.common_abilities.clone(),
            parts,
        }
    }

    /// Weakness テキストを変換（Udonarium用）
    /// "炎属性ダメージ+3" → "炎ダメ+3"
    /// XML生成時に使用
    pub fn transform_weakness(weakness: &str) -> String {
        let mut result = weakness.to_string();

        // テキスト置換
        result = result.replace("エネルギー", "E");
        result = result.replace("ダメージ", "ダメ");
        result = result.replace("属性", "");

        result
    }

    /// 命中力、回避力、生命抵抗力、精神抵抗力から7を引く
    /// （期待値から基本値への変換）
    /// 
    /// 注意: 打撃点は調整不要。打撃点は既に基本値のため。
    /// 
    /// XML生成時とチャットパレット生成時に使用
    /// 仕様参照: DESIGN_GUIDE.md の Udonarium Export Format
    pub fn adjust_value(value: i32) -> i32 {
        (value - 7).max(0) // 負の値は0に
    }

    /// Movement情報をフォーマット
    /// moveon/moveinが-1の場合は"-"、それ以外は値を返す
    pub fn format_movement(value: i32, description: &str) -> String {
        if value == -1 {
            "-".to_string()
        } else if description.is_empty() {
            value.to_string()
        } else {
            format!("{}\n({})", value, description)
        }
    }

    /// 命名が必要でない場合の省略形式（`\n()`を削除）
    pub fn format_optional_name(value: &str, name: &str) -> String {
        if name.is_empty() {
            value.to_string()
        } else {
            format!("{}\n({})", value, name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Part;

    fn create_test_monster() -> Monster {
        Monster {
            category: "蛮族".to_string(),
            level: 6,
            revision: 2.5,
            data: "TEST001".to_string(),
            illust: "".to_string(),
            movein: -1,
            movein_description: "".to_string(),
            moveon: -1,
            moveon_description: "".to_string(),
            name: "テストモンスター".to_string(),
            part: vec![Part {
                hp: Some(50),
                mp: 50,
                name: "".to_string(),
                core: Some(true),
                hit_rate: Some(15),
                dodge: Some(15),
                damage: Some(6),
                part_count: 1,
                special_abilities: "".to_string(),
                armor: 5,
            }],
            notes: "".to_string(),
            initiative: 14,
            common_abilities: "飛行".to_string(),
            weakness: "属性ダメージ+3".to_string(),
            weakness_value: 17,
            life_resistance: 16,
            fame: 14,
            mental_resistance: 16,
            extra: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_transform_basic_monster() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed = DataTransformer::transform(&monster, display_names);

        assert_eq!(transformed.name, "テストモンスター");
        assert_eq!(transformed.category, "蛮族");
        assert_eq!(transformed.level, 6);
        assert_eq!(transformed.fame, 14);
        assert_eq!(transformed.initiative, 14);
        assert_eq!(transformed.parts.len(), 1);
    }

    #[test]
    fn test_transform_part_data() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed = DataTransformer::transform(&monster, display_names);

        let part = &transformed.parts[0];
        assert_eq!(part.hp, 50);
        assert_eq!(part.mp, 50);
        assert_eq!(part.armor, 5);
        assert_eq!(part.hit_rate, 15);
        assert_eq!(part.dodge, 15);
        assert_eq!(part.damage, 6);
        assert!(part.is_core);
    }

    #[test]
    fn test_transform_negative_mp() {
        let mut monster = create_test_monster();
        monster.part[0].mp = -1;
        let display_names = vec!["テストモンスター".to_string()];
        let transformed = DataTransformer::transform(&monster, display_names);

        assert_eq!(transformed.parts[0].mp, 0);
    }

    #[test]
    fn test_transform_non_core_part() {
        let mut monster = create_test_monster();
        monster.part[0].core = Some(false);
        let display_names = vec!["テストモンスター_部位".to_string()];
        let transformed = DataTransformer::transform(&monster, display_names);

        let part = &transformed.parts[0];
        assert!(!part.is_core);
        assert_eq!(part.weakness, ""); // 非コア部位は弱点なし
    }

    #[test]
    fn test_transform_weakness() {
        let result = DataTransformer::transform_weakness("炎属性ダメージ+3");
        assert_eq!(result, "炎ダメ+3");

        let result = DataTransformer::transform_weakness("エネルギー属性ダメージ+2");
        assert_eq!(result, "Eダメ+2");

        let result = DataTransformer::transform_weakness("純エネルギー属性ダメージ+5");
        assert_eq!(result, "純Eダメ+5");
    }

    #[test]
    fn test_adjust_value() {
        assert_eq!(DataTransformer::adjust_value(22), 15); // 22 - 7 = 15
        assert_eq!(DataTransformer::adjust_value(7), 0); // 7 - 7 = 0
        assert_eq!(DataTransformer::adjust_value(5), 0); // 5 - 7 = -2 → 0
        assert_eq!(DataTransformer::adjust_value(30), 23); // 30 - 7 = 23
    }

    #[test]
    fn test_format_movement_normal() {
        let result = DataTransformer::format_movement(22, "飛行");
        assert_eq!(result, "22\n(飛行)");
    }

    #[test]
    fn test_format_movement_minus_one() {
        let result = DataTransformer::format_movement(-1, "");
        assert_eq!(result, "-");
    }

    #[test]
    fn test_format_movement_no_description() {
        let result = DataTransformer::format_movement(22, "");
        assert_eq!(result, "22");
    }

    #[test]
    fn test_format_optional_name_with_name() {
        let result = DataTransformer::format_optional_name("テストモンスター", "頭部");
        assert_eq!(result, "テストモンスター\n(頭部)");
    }

    #[test]
    fn test_format_optional_name_without_name() {
        let result = DataTransformer::format_optional_name("テストモンスター", "");
        assert_eq!(result, "テストモンスター");
    }

    #[test]
    fn test_transform_multi_part_monster() {
        let mut monster = create_test_monster();
        monster.part.push(Part {
            hp: Some(40),
            mp: 30,
            name: "足".to_string(),
            core: Some(false),
            hit_rate: Some(14),
            dodge: Some(13),
            damage: Some(5),
            part_count: 1,
            special_abilities: "移動力+3".to_string(),
            armor: 3,
        });

        let display_names = vec![
            "テストモンスター\n(本体)".to_string(),
            "テストモンスター\n(足)".to_string(),
        ];
        let transformed = DataTransformer::transform(&monster, display_names);

        assert_eq!(transformed.parts.len(), 2);
        assert!(transformed.parts[0].is_core);
        assert!(!transformed.parts[1].is_core);
        assert_eq!(transformed.parts[1].hp, 40);
    }
}
