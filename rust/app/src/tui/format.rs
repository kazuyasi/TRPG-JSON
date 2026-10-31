//! Monster / Spell → 詳細ペイン表示用の値への整形（純粋関数）
//!
//! 値はデータに入っている数値をそのまま表示する（ココフォリア/Udonarium 出力時の -7 補正などはかけない）。
//! 描画（ratatui ウィジェットの組み立て）は `ui` 側で行う。

use serde_json::Value;
use trpg_json_core::export::palette;
use trpg_json_core::{Monster, Part, Spell};

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

/// スペルの習得レベル（`Lv` の3種類）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellLevel {
    /// `Lv.value`: 固定レベル
    Value(i32),
    /// `Lv.value+`: 最低レベル
    ValuePlus(i32),
    /// `Lv.rank`: ランク
    Rank(i32),
    /// `Lv` がない・読めない
    Unknown,
}

impl SpellLevel {
    /// 並び順のキー（レベル系 → ランク → 不明。それぞれ数値の昇順）
    pub fn sort_key(self) -> (u8, i32) {
        match self {
            SpellLevel::Value(n) | SpellLevel::ValuePlus(n) => (0, n),
            SpellLevel::Rank(n) => (1, n),
            SpellLevel::Unknown => (2, 0),
        }
    }

    /// 結果リストの Lv 列（3 桁に収まる短い表記）
    pub fn short(self) -> String {
        match self {
            SpellLevel::Value(n) => n.to_string(),
            SpellLevel::ValuePlus(n) => format!("{}+", n),
            SpellLevel::Rank(n) => format!("R{}", n),
            SpellLevel::Unknown => NONE.to_string(),
        }
    }

    /// 詳細ペインの見出しに出す表記
    pub fn label(self) -> String {
        match self {
            SpellLevel::Value(n) => format!("Lv.{}", n),
            SpellLevel::ValuePlus(n) => format!("Lv.{}+", n),
            SpellLevel::Rank(n) => format!("ランク{}", n),
            SpellLevel::Unknown => format!("Lv.{}", NONE),
        }
    }
}

/// `Lv` を読み取る（core の検索と同じく value → value+ → rank の順にキーを見る）
pub fn spell_level(s: &Spell) -> SpellLevel {
    let Some(lv) = s.extra.get("Lv").and_then(Value::as_object) else {
        return SpellLevel::Unknown;
    };
    let int = |key: &str| lv.get(key).and_then(Value::as_i64).map(|n| n as i32);
    if let Some(n) = int("value") {
        SpellLevel::Value(n)
    } else if let Some(n) = int("value+") {
        SpellLevel::ValuePlus(n)
    } else if let Some(n) = int("rank") {
        SpellLevel::Rank(n)
    } else {
        SpellLevel::Unknown
    }
}

/// 見出し行: 名前と「Lv（またはランク）・系統（schoolVariant / god があれば併記）」
pub fn spell_title(s: &Spell) -> (String, String) {
    let mut right = format!("{}  {}", spell_level(s).label(), text_or_none(&s.school));
    if let Some(variant) = extra_text(s, "schoolVariant") {
        right.push_str(&format!(" [{}]", variant));
    }
    if let Some(god) = extra_text(s, "god") {
        right.push_str(&format!(" {}", god));
    }
    (s.name.clone(), right)
}

/// 基本値の行（ラベルと値の組の並び）
///
/// MP・対象・射程・時間・抵抗は値がなければ `-`、形状・属性は値がなければ省く。
pub fn spell_summary(s: &Spell) -> Vec<Vec<(&'static str, String)>> {
    let or_none = |r: Result<String, String>| r.unwrap_or_else(|_| NONE.to_string());
    let mut rows = vec![
        vec![("MP", or_none(palette::format_mp(s))), ("時間", or_none(palette::format_duration(s)))],
        vec![("対象", or_none(palette::format_target(s)))],
    ];

    let mut range_row = vec![("射程", spell_range(s))];
    if let Some(shape) = extra_text(s, "形状") {
        range_row.push(("形状", shape));
    }
    rows.push(range_row);

    let mut resist_row = vec![("抵抗", extra_text(s, "抵抗").unwrap_or_else(|| NONE.to_string()))];
    if let Some(attr) = extra_text(s, "属性") {
        resist_row.push(("属性", attr));
    }
    rows.push(resist_row);
    rows
}

/// 射程（`射程` と `射程(m)` の両方があれば「術者(30m)」のように併記）
pub fn spell_range(s: &Spell) -> String {
    let range = extra_text(s, "射程");
    let meters = extra_text(s, "射程(m)").map(|m| format!("{}m", m));
    match (range, meters) {
        (Some(r), Some(m)) => format!("{}({})", r, m),
        (Some(r), None) => r,
        (None, Some(m)) => m,
        (None, None) => NONE.to_string(),
    }
}

/// 詳細ペイン下部のテキスト節（効果）。空なら含めない
pub fn spell_sections(s: &Spell) -> Vec<(String, String)> {
    extra_text(s, "効果")
        .map(|effect| vec![("効果".to_string(), effect)])
        .unwrap_or_default()
}

/// 文字列または数値のフィールドを表示用の文字列にする（空・その他の型は `None`）
fn extra_text(s: &Spell, key: &str) -> Option<String> {
    match s.extra.get(key)? {
        Value::String(v) if !v.trim().is_empty() => Some(v.trim().to_string()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
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

    pub fn spell(name: &str, school: &str, extra: Value) -> Spell {
        let Value::Object(map) = extra else { panic!("extra はオブジェクトで指定する") };
        Spell {
            name: name.to_string(),
            school: school.to_string(),
            extra: map.into_iter().collect(),
        }
    }

    /// パレットを生成できる最小限のスペル
    pub fn full_spell(name: &str, level: Value) -> Spell {
        spell(
            name,
            "MagicCat_1",
            serde_json::json!({
                "Lv": level,
                "MP": {"kind": "value", "value": 5},
                "対象": {"kind": "個別", "個別": "1体"},
                "時間": {"value": "一瞬"},
                "射程": "術者",
                "射程(m)": 30,
                "抵抗": "半減",
                "効果": "テスト効果",
            }),
        )
    }

    #[test]
    fn test_spell_level_variants() {
        let lv = |v: Value| spell_level(&spell("テスト", "", serde_json::json!({ "Lv": v })));
        assert_eq!(lv(serde_json::json!({"kind": "value", "value": 3})), SpellLevel::Value(3));
        assert_eq!(lv(serde_json::json!({"kind": "value+", "value+": 5})), SpellLevel::ValuePlus(5));
        assert_eq!(lv(serde_json::json!({"kind": "rank", "rank": 2})), SpellLevel::Rank(2));
        assert_eq!(lv(serde_json::json!("不正")), SpellLevel::Unknown);
        assert_eq!(spell_level(&spell("テスト", "", serde_json::json!({}))), SpellLevel::Unknown);

        assert_eq!(SpellLevel::Value(3).short(), "3");
        assert_eq!(SpellLevel::ValuePlus(5).short(), "5+");
        assert_eq!(SpellLevel::Rank(2).short(), "R2");
        assert_eq!(SpellLevel::Rank(2).label(), "ランク2");
        assert_eq!(SpellLevel::ValuePlus(5).label(), "Lv.5+");
        // レベル系 → ランク → 不明 の順
        assert!(SpellLevel::Value(15).sort_key() < SpellLevel::Rank(1).sort_key());
        assert!(SpellLevel::ValuePlus(2).sort_key() < SpellLevel::Value(3).sort_key());
        assert!(SpellLevel::Rank(9).sort_key() < SpellLevel::Unknown.sort_key());
    }

    #[test]
    fn test_spell_title_with_variant_and_god() {
        let s = full_spell("テストスペル", serde_json::json!({"kind": "value", "value": 9}));
        assert_eq!(spell_title(&s), ("テストスペル".into(), "Lv.9  MagicCat_1".into()));

        let s = spell(
            "テストスペル",
            "MagicCat_2",
            serde_json::json!({"Lv": {"kind": "rank", "rank": 2}, "schoolVariant": "特殊", "god": "神名"}),
        );
        assert_eq!(spell_title(&s).1, "ランク2  MagicCat_2 [特殊] 神名");
    }

    #[test]
    fn test_spell_summary_values_and_omissions() {
        let s = full_spell("テスト", serde_json::json!({"kind": "value", "value": 1}));
        let rows = spell_summary(&s);
        assert_eq!(rows[0], vec![("MP", "5".to_string()), ("時間", "一瞬".to_string())]);
        assert_eq!(rows[1], vec![("対象", "1体".to_string())]);
        // 形状・属性がないので省略
        assert_eq!(rows[2], vec![("射程", "術者(30m)".to_string())]);
        assert_eq!(rows[3], vec![("抵抗", "半減".to_string())]);

        let s = spell(
            "テスト",
            "",
            serde_json::json!({
                "MP": {"kind": "value+", "value+": 3},
                "対象": {"kind": "エリア", "エリア": {"value": "1エリア", "半径(m)": 5, "末尾": "すべて"}},
                "時間": {"value": 3, "unit": "R"},
                "形状": "射撃",
                "属性": "炎",
            }),
        );
        let rows = spell_summary(&s);
        assert_eq!(rows[0], vec![("MP", "3～".to_string()), ("時間", "3R".to_string())]);
        assert_eq!(rows[1][0].1, "1エリア(半径5mすべて)");
        assert_eq!(rows[2], vec![("射程", "-".to_string()), ("形状", "射撃".to_string())]);
        assert_eq!(rows[3], vec![("抵抗", "-".to_string()), ("属性", "炎".to_string())]);
    }

    #[test]
    fn test_spell_range_variants() {
        let r = |v: Value| spell_range(&spell("テスト", "", v));
        assert_eq!(r(serde_json::json!({"射程": "接触"})), "接触");
        assert_eq!(r(serde_json::json!({"射程(m)": 10})), "10m");
        assert_eq!(r(serde_json::json!({"射程": "術者", "射程(m)": "30"})), "術者(30m)");
        assert_eq!(r(serde_json::json!({"射程": ""})), "-");
    }

    #[test]
    fn test_spell_sections() {
        let s = full_spell("テスト", serde_json::json!({"kind": "value", "value": 1}));
        assert_eq!(spell_sections(&s), vec![("効果".to_string(), "テスト効果".to_string())]);
        let s = spell("テスト", "", serde_json::json!({"効果": "  "}));
        assert!(spell_sections(&s).is_empty());
    }
}
