//! ココフォリア（CCFolia）クリップボード形式エクスポートモジュール
//!
//! ココフォリアはルーム画面にクリップボードの内容を貼り付けると、
//! 以下の形式の JSON をキャラクター駒として取り込める。
//!
//! ```json
//! {"kind":"character","data":{"name":"...","status":[...],"params":[...],"commands":"..."}}
//! ```
//!
//! 1回の貼り付けで取り込めるのは1キャラクターのため、モンスターの部位ごとに
//! 1キャラクターを生成する。数値の変換規則は Udonarium エクスポートと共通
//! （命中力・回避力・生命抵抗力・精神抵抗力は -7 して基本値に変換）。

use super::udonarium::data_transformer::DataTransformer;
use super::udonarium::part_namer::PartNamer;
use crate::Monster;
use serde::Serialize;

/// ココフォリアのクリップボード貼り付け用トップレベル構造
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CcfoliaClipboard {
    /// 常に "character"
    pub kind: String,
    /// キャラクターデータ
    pub data: CcfoliaCharacter,
}

/// ココフォリアのキャラクターデータ
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CcfoliaCharacter {
    /// キャラクター名
    pub name: String,
    /// イニシアティブ（先制値）
    pub initiative: i32,
    /// 参照URL
    pub external_url: String,
    /// キャラクターメモ
    pub memo: String,
    /// ステータス（HP/MP/防護点など、最大値付きのリソース）
    pub status: Vec<CcfoliaStatus>,
    /// パラメータ（チャットパレットから {名前} で参照できる値）
    pub params: Vec<CcfoliaParam>,
    /// チャットパレット
    pub commands: String,
    /// ステータスを非公開にするか
    pub secret: bool,
    /// 盤面一覧に表示しないか
    pub invisible: bool,
}

/// ココフォリアのステータス
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CcfoliaStatus {
    pub label: String,
    pub value: i32,
    pub max: i32,
}

/// ココフォリアのパラメータ（値は文字列）
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CcfoliaParam {
    pub label: String,
    pub value: String,
}

/// ココフォリア用チャットパレット（Udonarium 出力と同等のコマンド構成）
const CHAT_PALETTE: &str = "\
//-----計算
C({HP}+{防護点}+{ダメージ軽減}-()) 　【残HP（物理ダメージ）】
C({HP}+{ダメージ軽減}-())　【残HP（魔法ダメージ）】
C({MP}-())　【MP消費】
C{HP}　【現在HP】
C{MP}　【現在MP】

//-----固定値判定
C({命中力}+7) 命中判定（固定値）
C({回避力}+7) 回避判定（固定値）
C({生命抵抗力}+7) 生命抵抗判定（固定値）
C({精神抵抗力}+7) 精神抵抗判定（固定値）

//-----ダイス判定
2d+{命中力}　命中判定
2d+{打撃点}　ダメージロール
2d+{回避力}　回避判定
2d+{生命抵抗力}　生命抵抗判定
2d+{精神抵抗力}　精神抵抗判定";

/// モンスターをココフォリアのキャラクター（部位ごと）に変換する
pub fn monster_to_characters(monster: &Monster) -> Vec<CcfoliaClipboard> {
    let mut namer = PartNamer::new(&monster.part);
    let part_names = namer.generate_names(&monster.part, &monster.name);
    let display_names: Vec<String> = part_names
        .iter()
        // Udonarium 用の改行入り表示名 "名前\n(部位)" を1行に整形
        .map(|pn| pn.display_name.replace('\n', ""))
        .collect();

    let transformed = DataTransformer::transform(monster, display_names);

    transformed
        .parts
        .iter()
        .map(|part| {
            let status = vec![
                CcfoliaStatus {
                    label: "HP".to_string(),
                    value: part.hp,
                    max: part.hp,
                },
                CcfoliaStatus {
                    label: "MP".to_string(),
                    value: part.mp,
                    max: part.mp,
                },
                CcfoliaStatus {
                    label: "防護点".to_string(),
                    value: part.armor,
                    max: part.armor,
                },
            ];

            let params = vec![
                param("命中力", DataTransformer::adjust_value(part.hit_rate)),
                param("打撃点", part.damage),
                param("回避力", DataTransformer::adjust_value(part.dodge)),
                param(
                    "生命抵抗力",
                    DataTransformer::adjust_value(part.life_resistance),
                ),
                param(
                    "精神抵抗力",
                    DataTransformer::adjust_value(part.mental_resistance),
                ),
                // チャットパレットの残HP計算で参照するため 0 で用意しておく
                param("ダメージ軽減", 0),
            ];

            CcfoliaClipboard {
                kind: "character".to_string(),
                data: CcfoliaCharacter {
                    name: part.display_name.clone(),
                    initiative: monster.initiative,
                    external_url: String::new(),
                    memo: build_memo(monster, part.is_core, &part.special_abilities),
                    status,
                    params,
                    commands: CHAT_PALETTE.to_string(),
                    secret: false,
                    invisible: false,
                },
            }
        })
        .collect()
}

/// クリップボードに貼り付ける JSON 文字列を生成する
pub fn to_clipboard_json(character: &CcfoliaClipboard) -> Result<String, serde_json::Error> {
    serde_json::to_string(character)
}

/// 部位を選択する際の既定インデックス（最初のコア部位、なければ先頭）
pub fn default_part_index(monster: &Monster) -> usize {
    monster
        .part
        .iter()
        .position(|p| p.core.unwrap_or(false))
        .unwrap_or(0)
}

fn param(label: &str, value: i32) -> CcfoliaParam {
    CcfoliaParam {
        label: label.to_string(),
        value: value.to_string(),
    }
}

/// キャラクターメモを生成する
fn build_memo(monster: &Monster, is_core: bool, special_abilities: &str) -> String {
    let mut lines = vec![format!("{} Lv.{}", monster.category, monster.level)];

    if is_core {
        lines.push(format!(
            "知名度/弱点値: {}/{}",
            monster.fame, monster.weakness_value
        ));
        lines.push(format!("先制値: {}", monster.initiative));
        if !monster.weakness.is_empty() {
            lines.push(format!(
                "弱点: {}",
                DataTransformer::transform_weakness(&monster.weakness)
            ));
        }
    }

    lines.push(format!(
        "移動速度: {} / {}",
        DataTransformer::format_movement(monster.moveon, &monster.moveon_description)
            .replace('\n', ""),
        DataTransformer::format_movement(monster.movein, &monster.movein_description)
            .replace('\n', ""),
    ));

    if !monster.common_abilities.is_empty() {
        lines.push(String::new());
        lines.push("【共通特殊能力】".to_string());
        lines.push(monster.common_abilities.clone());
    }
    if !special_abilities.is_empty() {
        lines.push(String::new());
        lines.push("【部位特殊能力】".to_string());
        lines.push(special_abilities.to_string());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Part;
    use std::collections::HashMap;

    fn test_part(name: &str, core: bool, hp: i32) -> Part {
        Part {
            hp: Some(hp),
            mp: -1,
            name: name.to_string(),
            core: Some(core),
            hit_rate: Some(15),
            dodge: Some(14),
            damage: Some(6),
            part_count: 1,
            special_abilities: String::new(),
            armor: 5,
        }
    }

    fn test_monster(parts: Vec<Part>) -> Monster {
        Monster {
            category: "カテゴリ".to_string(),
            level: 6,
            revision: 2.5,
            data: "TEST".to_string(),
            illust: String::new(),
            movein: -1,
            movein_description: String::new(),
            moveon: 20,
            moveon_description: String::new(),
            name: "モンスター名".to_string(),
            part: parts,
            notes: String::new(),
            initiative: 14,
            common_abilities: "共通能力テキスト".to_string(),
            weakness: "炎属性ダメージ+3".to_string(),
            weakness_value: 17,
            life_resistance: 16,
            fame: 12,
            mental_resistance: 15,
            extra: HashMap::new(),
        }
    }

    #[test]
    fn test_single_part_character() {
        let monster = test_monster(vec![test_part("", true, 50)]);
        let chars = monster_to_characters(&monster);

        assert_eq!(chars.len(), 1);
        let c = &chars[0];
        assert_eq!(c.kind, "character");
        assert_eq!(c.data.name, "モンスター名");
        assert_eq!(c.data.initiative, 14);
        assert_eq!(
            c.data.status[0],
            CcfoliaStatus {
                label: "HP".to_string(),
                value: 50,
                max: 50
            }
        );
        // MP -1 は 0 に変換
        assert_eq!(c.data.status[1].value, 0);
        assert_eq!(c.data.status[2].value, 5);
    }

    #[test]
    fn test_params_are_adjusted() {
        let monster = test_monster(vec![test_part("", true, 50)]);
        let c = &monster_to_characters(&monster)[0];
        let get = |label: &str| {
            c.data
                .params
                .iter()
                .find(|p| p.label == label)
                .map(|p| p.value.clone())
                .unwrap()
        };
        assert_eq!(get("命中力"), "8"); // 15 - 7
        assert_eq!(get("打撃点"), "6"); // 調整なし
        assert_eq!(get("回避力"), "7"); // 14 - 7
        assert_eq!(get("生命抵抗力"), "9"); // 16 - 7
        assert_eq!(get("精神抵抗力"), "8"); // 15 - 7
        assert_eq!(get("ダメージ軽減"), "0");
    }

    #[test]
    fn test_core_memo_contains_knowledge_info() {
        let monster = test_monster(vec![test_part("", true, 50)]);
        let memo = &monster_to_characters(&monster)[0].data.memo;
        assert!(memo.contains("カテゴリ Lv.6"));
        assert!(memo.contains("知名度/弱点値: 12/17"));
        assert!(memo.contains("先制値: 14"));
        assert!(memo.contains("弱点: 炎ダメ+3"));
        assert!(memo.contains("移動速度: 20 / -"));
        assert!(memo.contains("共通能力テキスト"));
    }

    #[test]
    fn test_multi_part_names_and_non_core_memo() {
        let mut leg = test_part("部位B", false, 30);
        leg.special_abilities = "部位能力テキスト".to_string();
        let monster = test_monster(vec![test_part("部位A", true, 50), leg]);
        let chars = monster_to_characters(&monster);

        assert_eq!(chars.len(), 2);
        assert_eq!(chars[0].data.name, "モンスター名(部位A)");
        assert_eq!(chars[1].data.name, "モンスター名(部位B)");
        assert_eq!(chars[1].data.status[0].value, 30);
        // 非コア部位には知名度・弱点情報を含めない
        assert!(!chars[1].data.memo.contains("知名度"));
        assert!(chars[1].data.memo.contains("部位能力テキスト"));
    }

    #[test]
    fn test_default_part_index_prefers_core() {
        let monster = test_monster(vec![test_part("部位B", false, 30), test_part("部位A", true, 50)]);
        assert_eq!(default_part_index(&monster), 1);

        let no_core = test_monster(vec![test_part("部位B", false, 30)]);
        assert_eq!(default_part_index(&no_core), 0);
    }

    #[test]
    fn test_clipboard_json_shape() {
        let monster = test_monster(vec![test_part("", true, 50)]);
        let json = to_clipboard_json(&monster_to_characters(&monster)[0]).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(v["kind"], "character");
        assert_eq!(v["data"]["name"], "モンスター名");
        assert_eq!(v["data"]["externalUrl"], "");
        assert_eq!(v["data"]["status"][0]["label"], "HP");
        assert_eq!(v["data"]["status"][0]["max"], 50);
        assert_eq!(v["data"]["params"][0]["value"], "8");
        assert!(v["data"]["commands"].as_str().unwrap().contains("2d+{命中力}"));
    }
}
