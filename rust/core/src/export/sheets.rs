use crate::Monster;
use serde::{Deserialize, Serialize};

/// Google Sheets の行データ（0インデックスの列）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetRow {
    /// 列ごとのセル値。キーは列文字（A, B, C...）
    pub values: Vec<Option<String>>,
}

/// Google Sheets への出力行とメタデータ
#[derive(Debug, Clone)]
pub struct SheetOutput {
    /// 行番号（1-indexed）
    pub row_number: usize,

    /// 行のセル値（0-indexed columns）
    pub values: Vec<Option<String>>,

    /// この行がマージ対象かどうか
    pub is_merged_row: bool,
}

/// データ変換エンジン
pub struct DataTransformer;

impl DataTransformer {
    /// Monster をスプレッドシートの行に変換
    /// 各パートについて2行を生成（奇数行と偶数行）
    pub fn transform_monster(monster: &Monster, start_row: usize) -> Vec<SheetOutput> {
        let mut rows = Vec::new();

        for (part_index, part) in monster.part.iter().enumerate() {
            let is_first_part = part_index == 0;

            // 奇数行（first line of part）
            let odd_row = Self::create_odd_row(monster, part, is_first_part);
            rows.push(SheetOutput {
                row_number: start_row + part_index * 2,
                values: odd_row,
                is_merged_row: false,
            });

            // 偶数行（second line of part）
            let even_row = Self::create_even_row(monster, part, is_first_part);
            rows.push(SheetOutput {
                row_number: start_row + part_index * 2 + 1,
                values: even_row,
                is_merged_row: false,
            });
        }

        rows
    }

    /// 奇数行を生成
    fn create_odd_row(monster: &Monster, part: &crate::Part, is_first_part: bool) -> Vec<Option<String>> {
        let mut row = vec![None; 54]; // A-AZ列（26） + AA-AZ列（26） = 52列程度

        // A列: name with part.name
        let name_value = if part.name.is_empty() {
            monster.name.clone()
        } else {
            format!("{}\n({})", monster.name, part.name)
        };
        let name_value = if part.core.unwrap_or(false) {
            format!("★{}", name_value)
        } else {
            name_value
        };
        row[0] = Some(name_value);

        // L列（11）: part.HP
        if let Some(hp) = part.hp {
            row[11] = Some(hp.to_string());
        }

        // P列（15）: part.MP
        if part.mp >= 0 {
            row[15] = Some(part.mp.to_string());
        } else {
            row[15] = Some("-".to_string());
        }

        // R列（17）: part.防護点
        row[17] = Some(part.armor.to_string());

        // T列（19）: 先制値
        if is_first_part {
            row[19] = Some(monster.initiative.to_string());
        } else {
            row[19] = Some("-".to_string());
        }

        // V列（21）: 生命抵抗力
        if is_first_part {
            row[21] = Some(monster.life_resistance.to_string());
        } else {
            row[21] = Some("-".to_string());
        }

        // X列（23）: 精神抵抗力
        if is_first_part {
            row[23] = Some(monster.mental_resistance.to_string());
        } else {
            row[23] = Some("-".to_string());
        }

        // Z列（25）: Fixed "3"
        row[25] = Some("3".to_string());

        // AB列（27）: moveon\n(moveon_des)
        if monster.moveon == -1 {
            row[27] = Some("-".to_string());
        } else {
            let moveon_str = if monster.moveon_description.is_empty() {
                monster.moveon.to_string()
            } else {
                format!("{}\n({})", monster.moveon, monster.moveon_description)
            };
            row[27] = Some(moveon_str);
        }

        // AD列（29）: movein\n(movein_des)
        if monster.movein == -1 {
            row[29] = Some("-".to_string());
        } else {
            let movein_str = if monster.movein_description.is_empty() {
                monster.movein.to_string()
            } else {
                format!("{}\n({})", monster.movein, monster.movein_description)
            };
            row[29] = Some(movein_str);
        }

        // AF列（31）: part.命中力
        if let Some(hit_rate) = part.hit_rate {
            row[31] = Some(hit_rate.to_string());
        }

        // AH列（33）: part.回避力
        if let Some(dodge) = part.dodge {
            row[33] = Some(dodge.to_string());
        }

        // AJ列（35）: data
        row[35] = Some(monster.data.clone());

         // AM列（38）: 共通特殊能力（奇数行）
         if !monster.common_abilities.is_empty() {
             row[38] = Some(monster.common_abilities.clone());
         }

        // AW列（48）: 知名度 / 弱点値
        if is_first_part {
            row[48] = Some(format!("{}/{}", monster.fame, monster.weakness_value));
        } else {
            row[48] = Some("-/-".to_string());
        }

        row
    }

     /// 偶数行を生成
    fn create_even_row(monster: &Monster, part: &crate::Part, is_first_part: bool) -> Vec<Option<String>> {
        let mut row = vec![None; 54];

        // AM列（38）: 部位特殊能力（偶数行）
        if !part.special_abilities.is_empty() {
            row[38] = Some(part.special_abilities.clone());
        }

        // AW列（48）: 弱点（偶数行）
        if is_first_part {
            if !monster.weakness.is_empty() {
                let weakness = Self::transform_weakness(&monster.weakness);
                row[48] = Some(weakness);
            } else {
                row[48] = Some("-".to_string());
            }
        } else {
            row[48] = Some("-".to_string());
        }

        row
    }

    /// 弱点フィールドの変換
    /// "エネルギー" → "E", "ダメージ" → "ダメ", "属性" を削除
    fn transform_weakness(weakness: &str) -> String {
        let transformed = weakness
            .replace("エネルギー", "E")
            .replace("ダメージ", "ダメ")
            .replace("属性", "");

        transformed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_weakness() {
        assert_eq!(
            DataTransformer::transform_weakness("炎属性ダメージ+3"),
            "炎ダメ+3"
        );
        assert_eq!(
            DataTransformer::transform_weakness("純エネルギー属性ダメージ+2"),
            "純Eダメ+2"
        );
        assert_eq!(
            DataTransformer::transform_weakness("ダメージ+1"),
            "ダメ+1"
        );
    }

    #[test]
    fn test_transform_single_part_monster() {
        let json_data = r#"
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
            "name": "テストモンスター",
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
                    "部位特殊能力": "飛行適性",
                    "防護点": 5
                }
            ],
            "備考": "",
            "先制値": 14,
            "共通特殊能力": "飛行",
            "弱点": "属性ダメージ+3",
            "弱点値": 17,
            "生命抵抗力": 16,
            "知名度": 14,
            "精神抵抗力": 16
        }"#;

        let monster: Monster = serde_json::from_str(json_data).unwrap();
        let rows = DataTransformer::transform_monster(&monster, 3);

        assert_eq!(rows.len(), 2); // 1パート × 2行

        // 奇数行の確認
        let odd_row = &rows[0];
        assert_eq!(odd_row.row_number, 3);
        assert_eq!(odd_row.values[0], Some("★テストモンスター".to_string()));
        assert_eq!(odd_row.values[11], Some("48".to_string()));
        assert_eq!(odd_row.values[15], Some("75".to_string()));

        // 偶数行の確認
        let even_row = &rows[1];
        assert_eq!(even_row.row_number, 4);
        assert_eq!(even_row.values[38], Some("飛行適性".to_string()));
        assert_eq!(even_row.values[48], Some("ダメ+3".to_string()));
    }

    #[test]
    fn test_transform_multi_part_monster() {
        let json_data = r#"
        {
            "Category": "蛮族",
            "Lv": 7,
            "Revision": 2.5,
            "data": "TEST002",
            "illust": "",
            "movein": -1,
            "movein_des": "",
            "moveon": -1,
            "moveon_des": "",
            "name": "複合型モンスター",
            "part": [
                {
                    "HP": 48,
                    "MP": 75,
                    "name": "頭部",
                    "コア": true,
                    "命中力": 15,
                    "回避力": 15,
                    "打撃点": 6,
                    "部位数": 1,
                    "部位特殊能力": "魔法適性",
                    "防護点": 5
                },
                {
                    "HP": 43,
                    "MP": 20,
                    "name": "胴体",
                    "コア": false,
                    "命中力": 16,
                    "回避力": 14,
                    "打撃点": 8,
                    "部位数": 1,
                    "部位特殊能力": "飛翔",
                    "防護点": 7
                }
            ],
            "備考": "",
            "先制値": 15,
            "共通特殊能力": "強靭な皮膚",
            "弱点": "属性ダメージ+2",
            "弱点値": 18,
            "生命抵抗力": 17,
            "知名度": 15,
            "精神抵抗力": 17
        }"#;

        let monster: Monster = serde_json::from_str(json_data).unwrap();
        let rows = DataTransformer::transform_monster(&monster, 3);

        assert_eq!(rows.len(), 4); // 2パート × 2行

        // 最初のパート（奇数行）
        let part1_odd = &rows[0];
        assert_eq!(part1_odd.row_number, 3);
        assert_eq!(part1_odd.values[0], Some("★複合型モンスター\n(頭部)".to_string()));
        assert_eq!(part1_odd.values[19], Some("15".to_string())); // 先制値は最初のパートのみ

        // 最初のパート（偶数行）
        let part1_even = &rows[1];
        assert_eq!(part1_even.row_number, 4);
        assert_eq!(part1_even.values[38], Some("魔法適性".to_string()));
        assert_eq!(part1_even.values[48], Some("ダメ+2".to_string())); // 弱点は最初のパートのみ

        // 次のパート（奇数行）
        let part2_odd = &rows[2];
        assert_eq!(part2_odd.row_number, 5);
        assert_eq!(part2_odd.values[0], Some("複合型モンスター\n(胴体)".to_string())); // ★なし
        assert_eq!(part2_odd.values[19], Some("-".to_string())); // 先制値は2番目以降 "-"

        // 次のパート（偶数行）
        let part2_even = &rows[3];
        assert_eq!(part2_even.row_number, 6);
        assert_eq!(part2_even.values[38], Some("飛翔".to_string()));
        assert_eq!(part2_even.values[48], Some("-".to_string())); // 弱点は2番目以降 "-"
    }

    #[test]
    fn test_transform_monster_negative_mp() {
        let json_data = r#"
        {
            "Category": "蛮族",
            "Lv": 6,
            "Revision": 2.5,
            "data": "TEST003",
            "illust": "",
            "movein": -1,
            "movein_des": "",
            "moveon": -1,
            "moveon_des": "",
            "name": "MP無しモンスター",
            "part": [
                {
                    "HP": 50,
                    "MP": -1,
                    "name": "",
                    "コア": true,
                    "命中力": 14,
                    "回避力": 14,
                    "打撃点": 6,
                    "部位数": 1,
                    "部位特殊能力": "",
                    "防護点": 5
                }
            ],
            "備考": "",
            "先制値": 12,
            "共通特殊能力": "",
            "弱点": "属性ダメージ+2",
            "弱点値": 16,
            "生命抵抗力": 15,
            "知名度": 12,
            "精神抵抗力": 15
        }"#;

        let monster: Monster = serde_json::from_str(json_data).unwrap();
        let rows = DataTransformer::transform_monster(&monster, 3);

        let odd_row = &rows[0];
        // MP が -1 の場合、"-" が出力されることを確認
        assert_eq!(odd_row.values[15], Some("-".to_string()));
    }

    #[test]
    fn test_transform_monster_with_movement_description() {
        let json_data = r#"
        {
            "Category": "蛮族",
            "Lv": 6,
            "Revision": 2.5,
            "data": "TEST004",
            "illust": "",
            "movein": 20,
            "movein_des": "飛行",
            "moveon": 18,
            "moveon_des": "",
            "name": "移動型モンスター",
            "part": [
                {
                    "HP": 45,
                    "MP": 60,
                    "name": "",
                    "コア": true,
                    "命中力": 13,
                    "回避力": 13,
                    "打撃点": 6,
                    "部位数": 1,
                    "部位特殊能力": "",
                    "防護点": 4
                }
            ],
            "備考": "",
            "先制値": 11,
            "共通特殊能力": "飛行",
            "弱点": "属性ダメージ+1",
            "弱点値": 14,
            "生命抵抗力": 13,
            "知名度": 11,
            "精神抵抗力": 13
        }"#;

        let monster: Monster = serde_json::from_str(json_data).unwrap();
        let rows = DataTransformer::transform_monster(&monster, 3);

        let odd_row = &rows[0];
        // 移動力と説明を含む形式をチェック
        assert_eq!(odd_row.values[27], Some("18".to_string())); // moveon（説明なし）
        assert_eq!(odd_row.values[29], Some("20\n(飛行)".to_string())); // movein（説明あり）
    }

    #[test]
    fn test_transform_weakness_multiple_replacements() {
        assert_eq!(
            DataTransformer::transform_weakness("純エネルギー属性ダメージ+3"),
            "純Eダメ+3"
        );
        assert_eq!(
            DataTransformer::transform_weakness("エネルギーとダメージ属性"),
            "Eとダメ"
        );
    }
}
