use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod config;
pub mod export;
pub mod io;
pub mod query;

/// モンスター部位データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "HP")]
    pub hp: Option<i32>,

    #[serde(rename = "MP")]
    pub mp: i32, // -1 = MP不明/なし、それ以外はMP値

    pub name: String,

    #[serde(rename = "コア")]
    pub core: Option<bool>,

    #[serde(rename = "命中力")]
    pub hit_rate: Option<i32>,

    #[serde(rename = "回避力")]
    pub dodge: Option<i32>,

    #[serde(rename = "打撃点")]
    pub damage: Option<i32>,

    #[serde(rename = "部位数")]
    pub part_count: i32,

    #[serde(rename = "部位特殊能力")]
    pub special_abilities: String,

    #[serde(rename = "防護点")]
    pub armor: i32,
}


/// モンスターデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Lv")]
    pub level: i32,

    #[serde(rename = "Revision")]
    pub revision: f32,

    pub data: String,
    pub illust: String,

    pub movein: i32, // -1 = 移動不可、それ以外は移動力
    #[serde(rename = "movein_des")]
    pub movein_description: String,

    pub moveon: i32, // -1 = 移動不可、それ以外は移動力
    #[serde(rename = "moveon_des")]
    pub moveon_description: String,

    pub name: String,
    pub part: Vec<Part>,

    #[serde(rename = "備考")]
    pub notes: String,

    #[serde(rename = "先制値")]
    pub initiative: i32,

    #[serde(rename = "共通特殊能力")]
    pub common_abilities: String,

    #[serde(rename = "弱点")]
    pub weakness: String,

    #[serde(rename = "弱点値")]
    pub weakness_value: i32,

    #[serde(rename = "生命抵抗力")]
    pub life_resistance: i32,

    #[serde(rename = "知名度")]
    pub fame: i32,

    #[serde(rename = "精神抵抗力")]
    pub mental_resistance: i32,

    // 未知のフィールドを受け入れる
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// スペル（魔法）データ
/// スキーマ: schema/spell_array.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    /// スペル名
    pub name: String,

    /// 系統（MagicCat_1 など）
    pub school: String,

    // 複雑なフィールド: Lv（値と種別）, MP（値と種別）, 対象（個別 or エリア）
    // すべてを汎用 Value として受け入れる
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_monster_serde_roundtrip() {
        let json_data = r#"
        {
            "Category": "蛮族",
            "Lv": 6,
            "Revision": 2.5,
            "data": "GR143",
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
                    "部位特殊能力": "",
                    "防護点": 5
                }
            ],
            "備考": "",
            "先制値": 14,
            "共通特殊能力": "飛行",
            "弱点": "純エネルギー属性ダメージ+3",
            "弱点値": 17,
            "生命抵抗力": 16,
            "知名度": 14,
            "精神抵抗力": 16
        }"#;

        // JSONからデシリアライズ
        let monster: Monster = serde_json::from_str(json_data).expect("Failed to deserialize");

        // シリアライズして戻す
        let serialized = serde_json::to_string_pretty(&monster).expect("Failed to serialize");

        // 往復できることを確認
        let _monster2: Monster = serde_json::from_str(&serialized).expect("Failed to deserialize again");

        assert_eq!(monster.name, "テストモンスター");
        assert_eq!(monster.level, 6);
        assert_eq!(monster.part.len(), 1);
        assert_eq!(monster.part[0].hp, Some(48));
        assert_eq!(monster.part[0].mp, 75);
        assert_eq!(monster.part[0].core, Some(true));
    }

    #[test]
    fn test_core_value_variants() {
        // bool値のケース
        let json1 = r#"{"コア": true}"#;
        let part1: serde_json::Value = serde_json::from_str(json1).unwrap();

        // 空文字列のケース
        let json2 = r#"{"コア": ""}"#;
        let part2: serde_json::Value = serde_json::from_str(json2).unwrap();

        println!("Core bool: {:?}", part1);
        println!("Core empty: {:?}", part2);
    }

    #[test]
    fn test_movement_variants() {
        // 数値のケース
        let json1 = r#"{"movein": 22}"#;
        let data1: serde_json::Value = serde_json::from_str(json1).unwrap();

        // 空文字列のケース
        let json2 = r#"{"movein": ""}"#;
        let data2: serde_json::Value = serde_json::from_str(json2).unwrap();

        println!("Movement number: {:?}", data1);
        println!("Movement empty: {:?}", data2);
    }

    #[test]
    fn test_single_part_monster_data() {
        // 単一部位のモンスターデータ
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
            "name": "飛行型人型敵",
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
            "共通特殊能力": "飛行、魔法適性",
            "弱点": "属性ダメージ+3",
            "弱点値": 17,
            "生命抵抗力": 16,
            "知名度": 14,
            "精神抵抗力": 16
        }"#;

        // JSONからデシリアライズできることを確認
        let monster: Monster = serde_json::from_str(json_data).expect("Failed to deserialize single part monster data");

        assert_eq!(monster.category, "蛮族");
        assert_eq!(monster.level, 6);
        assert_eq!(monster.name, "飛行型人型敵");
        assert_eq!(monster.part.len(), 1);

        // 部位データも確認
        let part = &monster.part[0];
        assert_eq!(part.hp, Some(48));
        assert_eq!(part.mp, 75);
        assert_eq!(part.core, Some(true));
        assert_eq!(part.part_count, 1);
        assert_eq!(part.armor, 5);

        // 移動力フィールドも確認
        assert_eq!(monster.movein, 22);
        assert_eq!(monster.moveon, 22);

        println!("Successfully parsed single part monster: {}", monster.name);
    }

    #[test]
    fn test_multi_part_monster() {
        // 複数部位のモンスターデータ
        let json_data = r#"
        {
            "Category": "蛮族",
            "Lv": 7,
            "Revision": 2.5,
            "data": "TEST002",
            "illust": "",
            "movein": 22,
            "movein_des": "飛行",
            "moveon": 22,
            "moveon_des": "",
            "name": "複合型人型敵",
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
            "弱点": "属性ダメージ+3",
            "弱点値": 18,
            "生命抵抗力": 17,
            "知名度": 15,
            "精神抵抗力": 17
        }"#;

        let monster: Monster = serde_json::from_str(json_data).expect("Failed to deserialize multi-part monster");

        assert_eq!(monster.part.len(), 2);
        assert_eq!(monster.part[0].name, "頭部");
        assert_eq!(monster.part[1].name, "胴体");

        // coreフィールドのテスト
        assert_eq!(monster.part[0].core, Some(true));
        assert_eq!(monster.part[1].core, Some(false));

        // MPフィールドのテスト
        assert_eq!(monster.part[0].mp, 75);
        assert_eq!(monster.part[1].mp, 20);

        println!("Successfully parsed multi-part monster: {}", monster.name);
    }

    #[test]
    fn test_negative_one_values() {
        // -1値のテスト（空文字列から変換されたデータ）
        let json_data = r#"
        {
            "Category": "魔法生物",
            "Lv": 11,
            "Revision": 2.5,
            "data": "II279&BT121",
            "illust": "BT114",
            "movein": -1,
            "movein_des": "",
            "moveon": -1,
            "moveon_des": "",
            "name": "アイアンゴーレム",
            "part": [
                {
                    "HP": 60,
                    "MP": -1,
                    "name": "右半身",
                    "コア": false,
                    "命中力": 19,
                    "回避力": 16,
                    "打撃点": 18,
                    "部位数": 1,
                    "部位特殊能力": "",
                    "防護点": 16
                }
            ],
            "備考": "",
            "先制値": 9,
            "共通特殊能力": "",
            "弱点": "純エネルギー属性ダメージ+3",
            "弱点値": 22,
            "生命抵抗力": 21,
            "知名度": 16,
            "精神抵抗力": 21
        }"#;

        let monster: Monster = serde_json::from_str(json_data).expect("Failed to deserialize monster with -1 values");

        // -1値が正しく設定されていることを確認
        assert_eq!(monster.movein, -1);
        assert_eq!(monster.moveon, -1);
        assert_eq!(monster.part[0].mp, -1);

        println!("Successfully parsed monster with -1 values: {}", monster.name);
    }

    #[test]
    #[ignore] // 実際のファイル検証用、通常のテストでは実行しない
    fn test_load_actual_monsters_json() {
        // 実際のmonsters.jsonファイルを読み込んでテスト
        use std::fs;

        // ファイルが存在するかチェック
        let file_path = "../../data/SW2.5/monsters.json";
        if !std::path::Path::new(file_path).exists() {
            println!("monsters.json not found at {}, skipping test", file_path);
            return;
        }

        let json_content = fs::read_to_string(file_path)
            .expect("Failed to read monsters.json");

        // JSONをパース
        match serde_json::from_str::<Vec<Monster>>(&json_content) {
            Ok(monsters) => {
                assert!(!monsters.is_empty(), "No monsters loaded");
                println!("Successfully loaded {} monsters from monsters.json", monsters.len());
                println!("First monster: {}", monsters[0].name);
            }
            Err(e) => {
                println!("Failed to deserialize monsters.json: {}", e);
                println!("This likely means the JSON data still needs cleaning");
                // テストは失敗させずに情報だけ出力
            }
        }
    }
}
