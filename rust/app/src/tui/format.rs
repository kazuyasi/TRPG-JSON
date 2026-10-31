//! Monster → 詳細ペイン表示用の値への整形（純粋関数）
//!
//! 値はデータに入っている数値をそのまま表示する（ココフォリア/Udonarium 出力時の -7 補正などはかけない）。
//! 描画（ratatui ウィジェットの組み立て）は `ui` 側で行う。

use trpg_json_core::{Monster, Part};

/// 値がないことを表す表示
pub const NONE: &str = "-";

/// 部位表の1行
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartRow {
    /// 1 始まりの部位番号
    pub number: usize,
    pub name: String,
    pub count: String,
    pub hp: String,
    pub mp: String,
    pub hit: String,
    pub dodge: String,
    pub damage: String,
    pub armor: String,
    pub core: bool,
}

/// 見出し行: 名前、レベル、カテゴリ
pub fn monster_title(m: &Monster) -> (String, String) {
    let right = if m.category.is_empty() {
        format!("Lv.{}", m.level)
    } else {
        format!("Lv.{} [{}]", m.level, m.category)
    };
    (m.name.clone(), right)
}

/// 基本値・抵抗・移動・弱点の行（ラベルと値の組の並び）
pub fn monster_summary(m: &Monster) -> Vec<Vec<(&'static str, String)>> {
    let mut rows = vec![
        vec![
            ("知名度/弱点値", format!("{}/{}", num(m.fame), num(m.weakness_value))),
            ("先制値", num(m.initiative)),
        ],
        vec![
            ("生命抵抗力", num(m.life_resistance)),
            ("精神抵抗力", num(m.mental_resistance)),
        ],
        vec![(
            "移動速度",
            format!(
                "地上 {}  /  水中 {}",
                movement(m.moveon, &m.moveon_description),
                movement(m.movein, &m.movein_description)
            ),
        )],
    ];
    if !m.weakness.trim().is_empty() {
        rows.push(vec![("弱点", m.weakness.trim().to_string())]);
    }
    rows
}

/// 部位表の行
pub fn part_rows(m: &Monster) -> Vec<PartRow> {
    m.part.iter().enumerate().map(|(i, p)| part_row(i + 1, p)).collect()
}

fn part_row(number: usize, p: &Part) -> PartRow {
    PartRow {
        number,
        name: text_or_none(&p.name),
        count: p.part_count.to_string(),
        hp: opt(p.hp),
        mp: if p.mp == -1 { "--".to_string() } else { p.mp.to_string() },
        hit: opt(p.hit_rate),
        dodge: opt(p.dodge),
        damage: match p.damage {
            Some(d) if d >= 0 => format!("+{}", d),
            Some(d) => d.to_string(),
            None => NONE.to_string(),
        },
        armor: p.armor.to_string(),
        core: p.core.unwrap_or(false),
    }
}

/// 詳細ペイン下部のテキスト節（見出しと本文）。空の節は含めない
///
/// `selected_part` は 0 始まりの部位 index。範囲外なら部位特殊能力の節を出さない。
pub fn monster_sections(m: &Monster, selected_part: usize) -> Vec<(String, String)> {
    let mut sections = Vec::new();

    if let Some(p) = m.part.get(selected_part)
        && !p.special_abilities.trim().is_empty()
    {
        let label = if m.part.len() > 1 {
            format!("部位特殊能力（選択中: {}）", text_or_none(&p.name))
        } else {
            "部位特殊能力".to_string()
        };
        sections.push((label, p.special_abilities.trim().to_string()));
    }
    if !m.common_abilities.trim().is_empty() {
        sections.push(("共通特殊能力".to_string(), m.common_abilities.trim().to_string()));
    }
    if !m.notes.trim().is_empty() {
        sections.push(("備考".to_string(), m.notes.trim().to_string()));
    }
    sections
}

/// 既定で選択する部位（最初のコア部位、なければ先頭）。ココフォリア出力の既定と同じ
pub fn default_part_index(m: &Monster) -> usize {
    trpg_json_core::export::ccfolia::default_part_index(m)
}

/// 移動速度（-1 は移動不可で `-`、補足があれば括弧で併記）
pub fn movement(value: i32, description: &str) -> String {
    if value == -1 {
        NONE.to_string()
    } else if description.trim().is_empty() {
        value.to_string()
    } else {
        format!("{}({})", value, description.trim())
    }
}

/// -1（なし）を `-` にする
fn num(v: i32) -> String {
    if v == -1 { NONE.to_string() } else { v.to_string() }
}

fn opt(v: Option<i32>) -> String {
    v.map(num).unwrap_or_else(|| NONE.to_string())
}

fn text_or_none(s: &str) -> String {
    if s.trim().is_empty() { NONE.to_string() } else { s.trim().to_string() }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::HashMap;

    pub fn part(name: &str, core: bool) -> Part {
        Part {
            hp: Some(30),
            mp: 10,
            name: name.to_string(),
            core: Some(core),
            hit_rate: Some(12),
            dodge: Some(11),
            damage: Some(5),
            part_count: 1,
            special_abilities: String::new(),
            armor: 4,
        }
    }

    pub fn monster(name: &str, level: i32, category: &str, parts: Vec<Part>) -> Monster {
        Monster {
            category: category.to_string(),
            level,
            revision: 2.5,
            data: String::new(),
            illust: String::new(),
            movein: -1,
            movein_description: String::new(),
            moveon: 15,
            moveon_description: String::new(),
            name: name.to_string(),
            part: parts,
            notes: String::new(),
            initiative: 10,
            common_abilities: String::new(),
            weakness: String::new(),
            weakness_value: 14,
            life_resistance: 8,
            fame: 9,
            mental_resistance: 7,
            extra: HashMap::new(),
        }
    }

    #[test]
    fn test_title_with_and_without_category() {
        let m = monster("テストモンスター", 6, "蛮族", vec![]);
        assert_eq!(monster_title(&m), ("テストモンスター".into(), "Lv.6 [蛮族]".into()));
        let m = monster("テストモンスター", 6, "", vec![]);
        assert_eq!(monster_title(&m).1, "Lv.6");
    }

    #[test]
    fn test_movement() {
        assert_eq!(movement(-1, ""), "-");
        assert_eq!(movement(-1, "飛行"), "-");
        assert_eq!(movement(20, ""), "20");
        assert_eq!(movement(20, "飛行"), "20(飛行)");
    }

    #[test]
    fn test_summary_omits_empty_weakness() {
        let mut m = monster("テストモンスター", 3, "動物", vec![]);
        let rows = monster_summary(&m);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0].1, "9/14");
        assert_eq!(rows[2][0].1, "地上 15  /  水中 -");

        m.weakness = "テスト属性ダメージ+3".into();
        let rows = monster_summary(&m);
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[3][0], ("弱点", "テスト属性ダメージ+3".to_string()));
    }

    #[test]
    fn test_part_row_mp_none_and_damage_sign() {
        let mut p = part("", true);
        p.mp = -1;
        p.damage = Some(0);
        p.hp = None;
        let row = part_row(1, &p);
        assert_eq!(row.mp, "--");
        assert_eq!(row.damage, "+0");
        assert_eq!(row.hp, "-");
        assert_eq!(row.name, "-");
        assert!(row.core);
    }

    #[test]
    fn test_part_rows_numbering() {
        let m = monster("テスト", 1, "", vec![part("頭部", false), part("胴体", true)]);
        let rows = part_rows(&m);
        assert_eq!(rows.iter().map(|r| r.number).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(default_part_index(&m), 1);
    }

    #[test]
    fn test_sections_follow_selected_part() {
        let mut head = part("頭部", true);
        head.special_abilities = "テスト能力A".into();
        let body = part("胴体", false);
        let mut m = monster("テスト", 1, "", vec![head, body]);
        m.common_abilities = "共通テスト能力".into();

        let s = monster_sections(&m, 0);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].0, "部位特殊能力（選択中: 頭部）");
        assert_eq!(s[1].0, "共通特殊能力");

        // 胴体は部位特殊能力が空なので節ごと省略。備考も空なので省略
        let s = monster_sections(&m, 1);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].0, "共通特殊能力");
    }
}
