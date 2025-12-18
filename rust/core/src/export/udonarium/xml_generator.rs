use super::data_transformer::{TransformedMonster, TransformedPart, DataTransformer};

/// XML生成器
pub struct XmlGenerator;

impl XmlGenerator {
    /// TransformedMonster からXML文字列を生成
    /// 各Partごとに独立したXML（character要素）を生成する
    pub fn generate_xml(
        transformed: &TransformedMonster,
        part_index: usize,
    ) -> Result<String, String> {
        if part_index >= transformed.parts.len() {
            return Err(format!("Part index {} out of bounds", part_index));
        }

        let part = &transformed.parts[part_index];

        if part.is_core {
            Self::generate_core_part_xml(transformed, part)
        } else {
            Self::generate_non_core_part_xml(transformed, part)
        }
    }

    /// コア部位用XML生成
    fn generate_core_part_xml(
        monster: &TransformedMonster,
        part: &TransformedPart,
    ) -> Result<String, String> {
        let xml = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<character location.name="table" location.x="0" location.y="0" posZ="0" rotate="0" roll="0">
  <data name="character">
    <data name="image">
      <data type="image" name="imageIdentifier"></data>
    </data>
    <data name="common">
      <data name="name">{}</data>
      <data name="size">1</data>
    </data>
    <data name="detail">
      <data name="リソース">
        <data type="numberResource" currentValue="{}" name="HP">{}</data>
        <data type="numberResource" currentValue="{}" name="MP">{}</data>
        <data type="numberResource" currentValue="{}" name="防護点">{}</data>
      </data>
      <data name="ステータス・バフ・デバフ">
        <data name="命中力" type="number">{}</data>
        <data name="打撃点" type="number">{}</data>
        <data name="回避力" type="number">{}</data>
        <data name="生命抵抗力" type="number">{}</data>
        <data name="精神抵抗力" type="number">{}</data>
      </data>
      <data name="特殊能力">
        <data name="特殊能力1" type="note">{}</data>
        <data name="特殊能力2" type="note">{}</data>
      </data>
       <data name="戦闘準備">
         <data name="魔物知識・先制判定" type="note">{}/{}
{}</data>
       </data>
      <data name="情報">
        <data name="弱点" type="note">{}</data>
      </data>
      <data name="魔物知識">
        <data name="生態" type="note">{} Lv.{}</data>
      </data>
    </data>
  </data>
  <chat-palette dicebot="SwordWorld2.5">
//-----計算
C({{HP}}+{{防護点}}+{{ダメージ軽減}}-()) 　【残HP（物理ダメージ）】
C({{HP}}+{{ダメージ軽減}}-())　【残HP（魔法ダメージ）】
C({{MP}}-())　【MP消費】
C{{HP}}　【現在HP】
C{{MP}}　【現在MP】

//-----固定値判定
C({{命中力}}+7) 命中判定（固定値）
C({{回避力}}+7) 回避判定（固定値）
C({{生命抵抗力}}+7) 生命抵抗判定（固定値）
C({{精神抵抗力}}+7) 精神抵抗判定（固定値）

//-----ダイス判定
2d+{{命中力}}　命中判定
2d+{{打撃点}}　ダメージロール
2d+{{回避力}}　回避判定
2d+{{生命抵抗力}}　生命抵抗判定
2d+{{精神抵抗力}}　精神抵抗判定
  </chat-palette>
</character>"#,
            part.display_name,
            part.hp,
            part.hp,
            part.mp,
            part.mp,
             part.armor,
             part.armor,
             DataTransformer::adjust_value(part.hit_rate),
             part.damage,
             DataTransformer::adjust_value(part.dodge),
             DataTransformer::adjust_value(part.life_resistance),
             DataTransformer::adjust_value(part.mental_resistance),
             monster.common_abilities,
            part.special_abilities,
            monster.fame,
            part.weakness_value,
            monster.initiative,
            Self::transform_weakness(&part.weakness),
            monster.category,
            monster.level,
        );

        Ok(xml)
    }

     /// 非コア部位用XML生成
     fn generate_non_core_part_xml(
         monster: &TransformedMonster,
         part: &TransformedPart,
     ) -> Result<String, String> {
          let xml = format!(
              r#"<?xml version="1.0" encoding="utf-8"?>
  <character location.name="table" location.x="0" location.y="0" posZ="0" rotate="0" roll="0">
    <data name="character">
      <data name="image">
        <data type="image" name="imageIdentifier"></data>
      </data>
      <data name="common">
        <data name="name">{}</data>
        <data name="size">1</data>
      </data>
      <data name="detail">
       <data name="リソース">
         <data type="numberResource" currentValue="{}" name="HP">{}</data>
         <data type="numberResource" currentValue="{}" name="MP">{}</data>
         <data type="numberResource" currentValue="{}" name="防護点">{}</data>
       </data>
       <data name="ステータス・バフ・デバフ">
         <data name="命中力" type="number">{}</data>
         <data name="打撃点" type="number">{}</data>
         <data name="回避力" type="number">{}</data>
         <data name="生命抵抗力" type="number">{}</data>
         <data name="精神抵抗力" type="number">{}</data>
       </data>
       <data name="特殊能力">
         <data name="特殊能力1" type="note">{}</data>
         <data name="特殊能力2" type="note">{}</data>
       </data>
     </data>
   </data>
   <chat-palette dicebot="SwordWorld2.5">
//-----計算
C({{HP}}+{{防護点}}+{{ダメージ軽減}}-()) 　【残HP（物理ダメージ）】
C({{HP}}+{{ダメージ軽減}}-())　【残HP（魔法ダメージ）】
C({{MP}}-())　【MP消費】
C{{HP}}　【現在HP】
C{{MP}}　【現在MP】

//-----固定値判定
C({{命中力}}+7) 命中判定（固定値）
C({{回避力}}+7) 回避判定（固定値）
C({{生命抵抗力}}+7) 生命抵抗判定（固定値）
C({{精神抵抗力}}+7) 精神抵抗判定（固定値）

//-----ダイス判定
2d+{{命中力}}　命中判定
2d+{{打撃点}}　ダメージロール
2d+{{回避力}}　回避判定
2d+{{生命抵抗力}}　生命抵抗判定
2d+{{精神抵抗力}}　精神抵抗判定
   </chat-palette>
 </character>"#,
             part.display_name,
             part.hp,
             part.hp,
             part.mp,
             part.mp,
             part.armor,
             part.armor,
             DataTransformer::adjust_value(part.hit_rate),
             part.damage,
             DataTransformer::adjust_value(part.dodge),
             DataTransformer::adjust_value(part.life_resistance),
             DataTransformer::adjust_value(part.mental_resistance),
             monster.common_abilities,
             part.special_abilities,
         );

         Ok(xml)
     }

    /// Weakness テキスト変換（XML埋め込み用）
    /// パターン: "炎属性ダメージ+3" → "炎ダメ+3"
    fn transform_weakness(weakness: &str) -> String {
        DataTransformer::transform_weakness(weakness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Monster, Part};
    use std::collections::HashMap;

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
            extra: HashMap::new(),
        }
    }

    #[test]
    fn test_core_part_xml_generation() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let result = XmlGenerator::generate_xml(&transformed, 0);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
        assert!(xml.contains("テストモンスター"));
        assert!(xml.contains("HP"));
        assert!(xml.contains("MP"));
        assert!(xml.contains("防護点"));
    }

    #[test]
    fn test_xml_contains_character_element() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        assert!(xml.contains("<character location.name=\"table\""));
        assert!(xml.contains("</character>"));
    }

    #[test]
    fn test_xml_contains_status_values() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        // 命中力、打撃点、回避力が含まれていることを確認
        assert!(xml.contains("<data name=\"命中力\""));
        assert!(xml.contains("<data name=\"打撃点\""));
        assert!(xml.contains("<data name=\"回避力\""));
    }

    #[test]
    fn test_xml_contains_chat_palette() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        assert!(xml.contains("<chat-palette"));
        assert!(xml.contains("命中判定"));
        assert!(xml.contains("ダメージロール"));
        assert!(xml.contains("回避判定"));
    }

    #[test]
    fn test_xml_contains_special_abilities_section() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        assert!(xml.contains("<data name=\"特殊能力\">"));
        assert!(xml.contains("<data name=\"特殊能力1\""));
        assert!(xml.contains("<data name=\"特殊能力2\""));
        // 共通特殊能力と部位特殊能力が含まれていることを確認
        assert!(xml.contains("飛行")); // monster.common_abilities
    }

    #[test]
    fn test_xml_core_part_contains_all_sections() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        // コア部位には以下の全セクションが含まれる
        assert!(xml.contains("リソース"));
        assert!(xml.contains("ステータス・バフ・デバフ"));
        assert!(xml.contains("特殊能力"));
        assert!(xml.contains("戦闘準備"));
        assert!(xml.contains("情報"));
        assert!(xml.contains("魔物知識"));
    }

    #[test]
    fn test_xml_non_core_part_lacks_combat_sections() {
        let mut monster = create_test_monster();
        monster.part[0].core = Some(false);
        let display_names = vec!["テストモンスター_部位".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        // 非コア部位には戦闘準備、情報、魔物知識がない
        assert!(!xml.contains("戦闘準備"));
        assert!(!xml.contains("情報"));
        assert!(!xml.contains("魔物知識"));
        // ただし特殊能力セクションはある
        assert!(xml.contains("特殊能力"));
    }

    #[test]
    fn test_non_core_part_xml_generation() {
        let mut monster = create_test_monster();
        monster.part[0].core = Some(false);
        let display_names = vec!["テストモンスター_部位".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let result = XmlGenerator::generate_xml(&transformed, 0);
        assert!(result.is_ok());

        let xml = result.unwrap();
        // 非コア部位には戦闘準備や情報セクションがない
        assert!(!xml.contains("戦闘準備"));
        assert!(!xml.contains("情報"));
        assert!(!xml.contains("魔物知識"));
    }

    #[test]
    fn test_invalid_part_index() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let result = XmlGenerator::generate_xml(&transformed, 99);
        assert!(result.is_err());
    }

    #[test]
    fn test_weakness_transformation() {
        let weakness = "炎属性ダメージ+3";
        let result = XmlGenerator::transform_weakness(weakness);
        assert_eq!(result, "炎ダメ+3");
    }

    #[test]
    fn test_core_part_contains_weakness_value_in_combat_section() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        // 魔物知識・先制判定に知名度/弱点値\n先制値が含まれていることを確認
        // フォーマット: {知名度}/{弱点値}\n{先制値}
        // テストモンスターの値: fame=14, weakness_value=17, initiative=14
        assert!(xml.contains("14/17"));
        assert!(xml.contains("14</"));
    }

    // T024: Chat Palette Auto-Generation Tests
    #[test]
    fn test_chat_palette_contains_all_required_commands() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // All required chat commands must be present
        assert!(xml.contains("2d+{命中力}　命中判定"), "Missing 命中力 command");
        assert!(xml.contains("2d+{打撃点}　ダメージロール"), "Missing 打撃点 command");
        assert!(xml.contains("2d+{回避力}　回避判定"), "Missing 回避力 command");
        assert!(xml.contains("2d+{生命抵抗力}　生命抵抗判定"), "Missing 生命抵抗力 command");
        assert!(xml.contains("2d+{精神抵抗力}　精神抵抗判定"), "Missing 精神抵抗力 command");
    }

    #[test]
    fn test_chat_palette_dicebot_configuration() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Verify dicebot is set to SwordWorld2.5
        assert!(xml.contains(r#"<chat-palette dicebot="SwordWorld2.5">"#));
    }

    #[test]
    fn test_chat_palette_format_consistency() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify all commands follow format: 2d+{field}　description
        assert!(palette.contains("2d+{"), "Commands must use 2d+{{field}} format");
        
        // Count command lines (excluding dicebot attribute line)
        let command_count = palette.matches("2d+{").count();
        assert_eq!(command_count, 5, "Must have exactly 5 commands");
    }

    #[test]
    fn test_core_part_chat_palette_includes_resistance_checks() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Core parts should include both life and mental resistance checks
        assert!(xml.contains("2d+{生命抵抗力}"), "Core part missing 生命抵抗力 check");
        assert!(xml.contains("2d+{精神抵抗力}"), "Core part missing 精神抵抗力 check");
    }

    #[test]
    fn test_non_core_part_chat_palette_includes_resistance_checks() {
        let mut monster = create_test_monster();
        monster.part[0].core = Some(false);
        let display_names = vec!["テストモンスター_部位".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Non-core parts should also have chat palette with all commands
        // (resistance values are used for judgment rolls)
        assert!(xml.contains("2d+{生命抵抗力}"), "Non-core part missing 生命抵抗力 check");
        assert!(xml.contains("2d+{精神抵抗力}"), "Non-core part missing 精神抵抗力 check");
        assert!(xml.contains("<chat-palette"), "Non-core part missing chat-palette");
    }

    #[test]
    fn test_chat_palette_does_not_include_special_abilities() {
        let mut monster = create_test_monster();
        monster.common_abilities = "飛行,遠隔".to_string();
        monster.part[0].special_abilities = "再生＝5,拘束攻撃".to_string();
        
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Special abilities should NOT appear in chat palette
        assert!(!palette.contains("飛行"), "Chat palette should not contain common abilities");
        assert!(!palette.contains("再生"), "Chat palette should not contain special abilities");
        assert!(!palette.contains("拘束攻撃"), "Chat palette should not contain part abilities");
    }

    #[test]
    fn test_chat_palette_command_variable_references() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify each command references correct variable
        assert!(palette.contains("2d+{命中力}"), "Missing 命中力 variable reference");
        assert!(palette.contains("2d+{打撃点}"), "Missing 打撃点 variable reference");
        assert!(palette.contains("2d+{回避力}"), "Missing 回避力 variable reference");
        assert!(palette.contains("2d+{生命抵抗力}"), "Missing 生命抵抗力 variable reference");
        assert!(palette.contains("2d+{精神抵抗力}"), "Missing 精神抵抗力 variable reference");
    }

    #[test]
    fn test_chat_palette_command_descriptions_in_japanese() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // All descriptions should be in Japanese
        assert!(palette.contains("命中判定"), "Missing Japanese description");
        assert!(palette.contains("ダメージロール"), "Missing Japanese description");
        assert!(palette.contains("回避判定"), "Missing Japanese description");
        assert!(palette.contains("生命抵抗判定"), "Missing Japanese description");
        assert!(palette.contains("精神抵抗判定"), "Missing Japanese description");
    }

    #[test]
    fn test_chat_palette_placement_in_xml() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Chat palette should be inside character element
        let char_start = xml.find("<character").expect("character element not found");
        let char_end = xml.find("</character>").expect("character close tag not found");
        let char_content = &xml[char_start..char_end];
        
        // chat-palette should be within character element
        assert!(char_content.contains("<chat-palette"), 
                "chat-palette should be inside character element");
    }

    #[test]
    fn test_chat_palette_properly_closed() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Count opening and closing tags
        let palette_open = xml.matches("<chat-palette").count();
        let palette_close = xml.matches("</chat-palette>").count();
        
        assert_eq!(palette_open, 1, "Should have exactly one opening chat-palette tag");
        assert_eq!(palette_close, 1, "Should have exactly one closing chat-palette tag");
        
        // Verify closing tag follows opening tag
        let open_pos = xml.find("<chat-palette").expect("Opening tag not found");
        let close_pos = xml.find("</chat-palette>").expect("Closing tag not found");
        assert!(close_pos > open_pos, "Closing tag should come after opening tag");
    }

    #[test]
    fn test_chat_palette_with_different_status_values() {
        let mut monster = create_test_monster();
        monster.part[0].hit_rate = Some(20);
        monster.part[0].dodge = Some(18);
        monster.part[0].damage = Some(8);
        monster.life_resistance = 18;
        monster.mental_resistance = 17;
        
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Chat palette should remain consistent regardless of status values
        assert!(xml.contains("2d+{命中力}"), "Chat palette should work with different hit_rate");
        assert!(xml.contains("2d+{回避力}"), "Chat palette should work with different dodge");
        assert!(xml.contains("2d+{打撃点}"), "Chat palette should work with different damage");
        
         // Actual status values in XML should be correct (adjusted by -7)
         assert!(xml.contains("<data name=\"命中力\" type=\"number\">13</data>"), 
                 "Status values should be correctly adjusted (20-7=13)");
    }

    // T025: Fixed-Value Judgment Commands Tests
    #[test]
    fn test_chat_palette_contains_fixed_value_judgment_section() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Fixed-value judgment section should exist
        assert!(xml.contains("//-----固定値判定"), "Fixed-value section header missing");
    }

    #[test]
    fn test_chat_palette_fixed_value_hit_command() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify fixed-value hit judgment command
        // Note: {{ in format string becomes { in output
        assert!(palette.contains("C({命中力}+7) 命中判定（固定値）"), 
                "Fixed-value hit command format is incorrect");
    }

    #[test]
    fn test_chat_palette_fixed_value_dodge_command() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify fixed-value dodge command
        // Note: {{ in format string becomes { in output
        assert!(palette.contains("C({回避力}+7) 回避判定（固定値）"), 
                "Fixed-value dodge command format is incorrect");
    }

    #[test]
    fn test_chat_palette_fixed_value_resistance_commands() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify fixed-value resistance commands
        // Note: {{ in format string becomes { in output
        assert!(palette.contains("C({生命抵抗力}+7) 生命抵抗判定（固定値）"), 
                "Fixed-value life resistance command format is incorrect");
        assert!(palette.contains("C({精神抵抗力}+7) 精神抵抗判定（固定値）"), 
                "Fixed-value mental resistance command format is incorrect");
    }

    #[test]
    fn test_chat_palette_fixed_value_judgment_count() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Extract fixed-value section
        let fixed_start = palette.find("//-----固定値判定").expect("Fixed-value section not found");
        let dice_start = palette.find("//-----ダイス判定").expect("Dice section not found");
        let fixed_section = &palette[fixed_start..dice_start];
        
        // Count fixed-value commands (C( pattern, with { not {{)
        let fixed_command_count = fixed_section.matches("C({").count();
        assert_eq!(fixed_command_count, 4, "Must have exactly 4 fixed-value commands");
    }

    #[test]
    fn test_chat_palette_dice_judgment_section_intact() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify dice judgment section header
        assert!(palette.contains("//-----ダイス判定"), "Dice judgment section header missing");
        
        // Verify all dice commands are present ({{ becomes { in output)
        assert!(palette.contains("2d+{命中力}　命中判定"), "Dice hit command missing");
        assert!(palette.contains("2d+{打撃点}　ダメージロール"), "Dice damage command missing");
        assert!(palette.contains("2d+{回避力}　回避判定"), "Dice dodge command missing");
        assert!(palette.contains("2d+{生命抵抗力}　生命抵抗判定"), "Dice life resistance command missing");
        assert!(palette.contains("2d+{精神抵抗力}　精神抵抗判定"), "Dice mental resistance command missing");
    }

    #[test]
    fn test_chat_palette_dice_judgment_count() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Extract dice section
        let dice_start = palette.find("//-----ダイス判定").expect("Dice section not found");
        let palette_end = palette.find("</chat-palette>").expect("Palette end not found");
        let dice_section = &palette[dice_start..palette_end];
        
        // Count dice commands (2d+{ pattern, with { not {{)
        let dice_command_count = dice_section.matches("2d+{").count();
        assert_eq!(dice_command_count, 5, "Must have exactly 5 dice judgment commands");
    }

    #[test]
    fn test_chat_palette_section_order() {
        let monster = create_test_monster();
        let display_names = vec!["テストモンスター".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Verify section order: 計算 -> 固定値判定 -> ダイス判定
        let calc_pos = palette.find("//-----計算").expect("Calculation section not found");
        let fixed_pos = palette.find("//-----固定値判定").expect("Fixed-value section not found");
        let dice_pos = palette.find("//-----ダイス判定").expect("Dice section not found");
        
        assert!(calc_pos < fixed_pos, "Calculation section should come before fixed-value section");
        assert!(fixed_pos < dice_pos, "Fixed-value section should come before dice section");
    }

    #[test]
    fn test_non_core_part_fixed_value_commands() {
        let mut monster = create_test_monster();
        monster.part[0].core = Some(false);
        let display_names = vec!["テストモンスター_部位".to_string()];
        let transformed =
            super::super::data_transformer::DataTransformer::transform(&monster, display_names);

        let xml = XmlGenerator::generate_xml(&transformed, 0).unwrap();
        
        // Extract chat-palette section
        let start = xml.find("<chat-palette").expect("chat-palette not found");
        let end = xml.find("</chat-palette>").expect("chat-palette end not found");
        let palette = &xml[start..end + "</chat-palette>".len()];
        
        // Non-core parts should also have fixed-value commands ({{ becomes { in output)
        assert!(palette.contains("C({命中力}+7) 命中判定（固定値）"), 
                "Non-core part missing fixed-value hit command");
        assert!(palette.contains("C({回避力}+7) 回避判定（固定値）"), 
                "Non-core part missing fixed-value dodge command");
        assert!(palette.contains("C({生命抵抗力}+7) 生命抵抗判定（固定値）"), 
                "Non-core part missing fixed-value life resistance command");
        assert!(palette.contains("C({精神抵抗力}+7) 精神抵抗判定（固定値）"), 
                "Non-core part missing fixed-value mental resistance command");
    }

    #[test]
    fn test_core_and_non_core_chat_palette_parity() {
        let monster_core = create_test_monster();
        let display_names_core = vec!["テストモンスター".to_string()];
        let transformed_core =
            super::super::data_transformer::DataTransformer::transform(&monster_core, display_names_core);
        let xml_core = XmlGenerator::generate_xml(&transformed_core, 0).unwrap();

        let mut monster_non_core = create_test_monster();
        monster_non_core.part[0].core = Some(false);
        let display_names_non_core = vec!["テストモンスター_部位".to_string()];
        let transformed_non_core =
            super::super::data_transformer::DataTransformer::transform(&monster_non_core, display_names_non_core);
        let xml_non_core = XmlGenerator::generate_xml(&transformed_non_core, 0).unwrap();

        // Extract chat-palette sections
        let core_start = xml_core.find("<chat-palette").unwrap();
        let core_end = xml_core.find("</chat-palette>").unwrap();
        let core_palette = &xml_core[core_start..core_end + "</chat-palette>".len()];

        let non_core_start = xml_non_core.find("<chat-palette").unwrap();
        let non_core_end = xml_non_core.find("</chat-palette>").unwrap();
        let non_core_palette = &xml_non_core[non_core_start..non_core_end + "</chat-palette>".len()];

        // Count total commands (calculation + fixed-value + dice)
        // Note: {{ in format string becomes { in output
        let core_calc = core_palette.matches("C({").count() + core_palette.matches("C{").count();
        let non_core_calc = non_core_palette.matches("C({").count() + non_core_palette.matches("C{").count();
        let core_dice = core_palette.matches("2d+{").count();
        let non_core_dice = non_core_palette.matches("2d+{").count();

        // Both should have same structure (calculation and fixed-value commands)
        assert_eq!(core_calc, non_core_calc, "Core and non-core should have same number of calculation/fixed-value commands");
        assert_eq!(core_dice, non_core_dice, "Core and non-core should have same number of dice commands");
    }
}
