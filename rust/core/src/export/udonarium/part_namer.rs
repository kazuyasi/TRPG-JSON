use crate::Part;
use std::collections::HashMap;

/// Part命名結果（各Partに対して生成されるファイル名）
#[derive(Debug, Clone)]
pub struct PartName {
    /// ファイル名（拡張子なし）例: "ゴブリン"、"トレント_幹"、"トレント_根0"
    pub filename: String,
    /// 表示名（XMLの名前フィールド）例: "ゴブリン"、"ゴブリン\n(幹)"
    pub display_name: String,
}

/// Part命名アルゴリズムを実装
pub struct PartNamer {
    /// 部位名ごとの出現回数（重複判定用）
    name_counts: HashMap<String, usize>,
}

impl PartNamer {
    /// 新しいPartNamerを生成
    pub fn new(parts: &[Part]) -> Self {
        let mut name_counts = HashMap::new();

        // 各部位名の出現回数を数える
        for part in parts {
            let name = &part.name;
            *name_counts.entry(name.clone()).or_insert(0) += 1;
        }

        PartNamer { name_counts }
    }

    /// すべてのPartに対してPart名を生成
    pub fn generate_names(&mut self, parts: &[Part], monster_name: &str) -> Vec<PartName> {
        let mut result = Vec::new();
        let mut current_indices: HashMap<String, usize> = HashMap::new();

        for part in parts {
            let part_name_key = &part.name;
            let index = current_indices
                .entry(part_name_key.clone())
                .and_modify(|e| *e += 1)
                .or_insert(0)
                .clone();

            let part_names = self.generate_name(monster_name, part, index);
            result.push(part_names);
        }

        result
    }

    /// 単一のPartに対してPart名を生成
    fn generate_name(&self, monster_name: &str, part: &Part, index: usize) -> PartName {
        let part_name = &part.name;
        let is_core = part.core.unwrap_or(false);
        let count = self.name_counts.get(part_name).unwrap_or(&1);

        let (filename, display_name) = if part_name.is_empty() {
            // part.nameが空の場合
            if is_core {
                // コア部位で名前なし → monster_name.xml
                (
                    monster_name.to_string(),
                    monster_name.to_string(),
                )
            } else {
                // 非コア部位で名前なし → monster_name_{index}.xml
                (
                    format!("{}_{}", monster_name, index),
                    format!("{}_{}", monster_name, index),
                )
            }
        } else if *count > 1 {
            // 同じ名前の部位が複数存在
            (
                format!("{}_{}_{}", monster_name, part_name, index),
                format!("{}\n({})", monster_name, part_name),
            )
        } else {
            // 部位名が一意
            (
                format!("{}_{}", monster_name, part_name),
                format!("{}\n({})", monster_name, part_name),
            )
        };

        PartName { filename, display_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_part(name: &str, core: Option<bool>) -> Part {
        Part {
            hp: Some(50),
            mp: 50,
            name: name.to_string(),
            core,
            hit_rate: Some(15),
            dodge: Some(15),
            damage: Some(6),
            part_count: 1,
            special_abilities: "".to_string(),
            armor: 5,
        }
    }

    #[test]
    fn test_single_part_with_empty_name_core() {
        let parts = vec![create_test_part("", Some(true))];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "ゴブリン");

        assert_eq!(names.len(), 1);
        assert_eq!(names[0].filename, "ゴブリン");
        assert_eq!(names[0].display_name, "ゴブリン");
    }

    #[test]
    fn test_single_part_with_name() {
        let parts = vec![create_test_part("頭部", Some(true))];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "ゴブリン");

        assert_eq!(names.len(), 1);
        assert_eq!(names[0].filename, "ゴブリン_頭部");
        assert_eq!(names[0].display_name, "ゴブリン\n(頭部)");
    }

    #[test]
    fn test_multi_part_with_unique_names() {
        let parts = vec![
            create_test_part("幹", Some(true)),
            create_test_part("根", Some(false)),
        ];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "トレント");

        assert_eq!(names.len(), 2);
        assert_eq!(names[0].filename, "トレント_幹");
        assert_eq!(names[0].display_name, "トレント\n(幹)");
        // 一意な名前の場合は連番なし
        assert_eq!(names[1].filename, "トレント_根");
        assert_eq!(names[1].display_name, "トレント\n(根)");
    }

    #[test]
    fn test_multi_part_with_duplicate_names() {
        let parts = vec![
            create_test_part("根", Some(true)),
            create_test_part("根", Some(false)),
            create_test_part("根", Some(false)),
        ];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "トレント");

        assert_eq!(names.len(), 3);
        assert_eq!(names[0].filename, "トレント_根_0");
        assert_eq!(names[1].filename, "トレント_根_1");
        assert_eq!(names[2].filename, "トレント_根_2");
        // display_nameは全て同じ
        assert_eq!(names[0].display_name, "トレント\n(根)");
        assert_eq!(names[1].display_name, "トレント\n(根)");
        assert_eq!(names[2].display_name, "トレント\n(根)");
    }

    #[test]
    fn test_non_core_with_empty_name() {
        let parts = vec![
            create_test_part("", Some(true)),
            create_test_part("", Some(false)),
            create_test_part("", Some(false)),
        ];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "ゴブリン");

        assert_eq!(names.len(), 3);
        assert_eq!(names[0].filename, "ゴブリン");
        assert_eq!(names[1].filename, "ゴブリン_1");
        assert_eq!(names[2].filename, "ゴブリン_2");
    }

    #[test]
    fn test_multiple_core_parts() {
        let parts = vec![
            create_test_part("頭0", Some(true)),
            create_test_part("頭0", Some(true)),
            create_test_part("防護膜", Some(false)),
        ];
        let mut namer = PartNamer::new(&parts);
        let names = namer.generate_names(&parts, "アンシェント・ドラゴン");

        assert_eq!(names.len(), 3);
        assert_eq!(names[0].filename, "アンシェント・ドラゴン_頭0_0");
        assert_eq!(names[1].filename, "アンシェント・ドラゴン_頭0_1");
        assert_eq!(names[2].filename, "アンシェント・ドラゴン_防護膜");
    }
}
