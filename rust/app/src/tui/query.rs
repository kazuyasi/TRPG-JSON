//! 検索バーの検索構文パーサ（TUI_DESIGN.md §5、段階2）
//!
//! 検索バーの1行を空白区切りのトークンに分け、`キー:値` は条件、それ以外はフリーワード
//! （名前の部分一致）として扱う。複数のフリーワードはすべてを名前に含む項目に絞り込む（AND）。
//!
//! | タブ | トークン | 対応する CLI |
//! |------|----------|-------------|
//! | モンスター | `lv:<n>` | `-l` |
//! | モンスター | `cat:<s>` / `c:<s>` | `-c` |
//! | スペル | `lv:<n>` | `-l` |
//! | スペル | `school:<s>` / `s:<s>` | `-s` |
//!
//! キーは英字のみで大文字小文字を区別せず、区切りは半角 `:` と全角 `：` の両方を受け付ける。
//! 解釈できないトークン（不明なキー、空の値、整数でない Lv、同じキーの重複）はエラーにする。

use thiserror::Error;
use trpg_json_core::{Monster, Spell, query};

/// 検索構文の解釈エラー（`Display` は検索バーにそのまま表示する文言）
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QueryParseError {
    #[error("{key}: に値がありません")]
    EmptyValue { key: String },

    #[error("{key}: の値は整数で指定してください（{value}）")]
    InvalidNumber { key: String, value: String },

    #[error("{key}: が2回以上指定されています")]
    Duplicate { key: String },

    #[error("{key}: は使えません（使えるキー: {available}）")]
    UnknownKey { key: String, available: &'static str },
}

/// モンスタータブの検索条件
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MonsterQuery {
    /// フリーワード（名前の部分一致。すべてを含むものに絞る）
    pub names: Vec<String>,
    /// `lv:`（完全一致）
    pub level: Option<i32>,
    /// `cat:` / `c:`（完全一致）
    pub category: Option<String>,
}

/// スペルタブの検索条件
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpellQuery {
    /// フリーワード（名前の部分一致。すべてを含むものに絞る）
    pub names: Vec<String>,
    /// `lv:`（完全一致。ランク表記のスペルはヒットしない）
    pub level: Option<i32>,
    /// `school:` / `s:`（完全一致）
    pub school: Option<String>,
}

/// ヒット 0 件のときに条件一覧へ出す名前（複数のフリーワードは空白でつなぐ）
fn joined_names(names: &[String]) -> Option<String> {
    (!names.is_empty()).then(|| names.join(" "))
}

/// フリーワードのうち、core の検索に渡す1つ目と、追加で確かめる残り
fn split_names(names: &[String]) -> (Option<&str>, &[String]) {
    match names.split_first() {
        Some((first, rest)) => (Some(first.as_str()), rest),
        None => (None, &[]),
    }
}

impl MonsterQuery {
    /// 条件に合うか（CLI と同じ判定にするため core の `find_multi` を使う）
    pub fn matches(&self, monster: &Monster) -> bool {
        let (name, rest) = split_names(&self.names);
        !query::find_multi(std::slice::from_ref(monster), name, self.level, self.category.as_deref()).is_empty()
            && rest.iter().all(|n| monster.name.contains(n.as_str()))
    }

    /// 条件一覧の表示用の名前
    pub fn name_label(&self) -> Option<String> {
        joined_names(&self.names)
    }
}

impl SpellQuery {
    /// 条件に合うか（CLI と同じ判定にするため core の `spell_find_multi` を使う）
    pub fn matches(&self, spell: &Spell) -> bool {
        let (name, rest) = split_names(&self.names);
        !query::spell_find_multi(
            std::slice::from_ref(spell),
            name,
            self.school.as_deref(),
            self.level,
            None,
            None,
            None,
        )
        .is_empty()
            && rest.iter().all(|n| spell.name.contains(n.as_str()))
    }

    /// 条件一覧の表示用の名前
    pub fn name_label(&self) -> Option<String> {
        joined_names(&self.names)
    }
}

/// 1トークン: フリーワードか `キー:値` か
enum Token<'a> {
    Word(&'a str),
    /// キーは小文字にそろえたもの
    Pair { key: String, value: &'a str },
}

fn tokenize(input: &str) -> impl Iterator<Item = Token<'_>> {
    input.split_whitespace().map(|token| {
        let sep = token.char_indices().find(|&(_, c)| c == ':' || c == '：');
        match sep {
            // キーが英字だけのときに限り条件とみなす（名前に記号が含まれていても困らないように）
            Some((i, c)) if i > 0 && token[..i].chars().all(|ch| ch.is_ascii_alphabetic()) => Token::Pair {
                key: token[..i].to_ascii_lowercase(),
                value: &token[i + c.len_utf8()..],
            },
            _ => Token::Word(token),
        }
    })
}

/// 値が空でないことを確かめる
fn non_empty<'a>(key: &str, value: &'a str) -> Result<&'a str, QueryParseError> {
    if value.is_empty() {
        Err(QueryParseError::EmptyValue { key: key.to_string() })
    } else {
        Ok(value)
    }
}

/// 整数の値を読む
fn parse_number(key: &str, value: &str) -> Result<i32, QueryParseError> {
    non_empty(key, value)?.parse().map_err(|_| QueryParseError::InvalidNumber {
        key: key.to_string(),
        value: value.to_string(),
    })
}

/// まだ設定されていない条件に値を入れる（2回目はエラー）
fn set_once<T>(slot: &mut Option<T>, key: &str, value: T) -> Result<(), QueryParseError> {
    if slot.is_some() {
        return Err(QueryParseError::Duplicate { key: key.to_string() });
    }
    *slot = Some(value);
    Ok(())
}

/// モンスタータブの検索バーを解釈する
pub fn parse_monster_query(input: &str) -> Result<MonsterQuery, QueryParseError> {
    let mut q = MonsterQuery::default();
    for token in tokenize(input) {
        match token {
            Token::Word(w) => q.names.push(w.to_string()),
            Token::Pair { key, value } => match key.as_str() {
                "lv" => set_once(&mut q.level, &key, parse_number(&key, value)?)?,
                "cat" | "c" => set_once(&mut q.category, "cat", non_empty(&key, value)?.to_string())?,
                _ => {
                    return Err(QueryParseError::UnknownKey { key, available: "lv: cat:" });
                }
            },
        }
    }
    Ok(q)
}

/// スペルタブの検索バーを解釈する
pub fn parse_spell_query(input: &str) -> Result<SpellQuery, QueryParseError> {
    let mut q = SpellQuery::default();
    for token in tokenize(input) {
        match token {
            Token::Word(w) => q.names.push(w.to_string()),
            Token::Pair { key, value } => match key.as_str() {
                "lv" => set_once(&mut q.level, &key, parse_number(&key, value)?)?,
                "school" | "s" => set_once(&mut q.school, "school", non_empty(&key, value)?.to_string())?,
                _ => {
                    return Err(QueryParseError::UnknownKey { key, available: "lv: s:" });
                }
            },
        }
    }
    Ok(q)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_empty_query_has_no_conditions() {
        assert_eq!(parse_monster_query(""), Ok(MonsterQuery::default()));
        assert_eq!(parse_monster_query("   "), Ok(MonsterQuery::default()));
        assert_eq!(parse_spell_query(""), Ok(SpellQuery::default()));
    }

    #[test]
    fn test_monster_query_mixes_words_and_conditions() {
        let q = parse_monster_query(" テスト lv:6  cat:蛮族 戦士").unwrap();
        assert_eq!(q.names, names(&["テスト", "戦士"]));
        assert_eq!(q.level, Some(6));
        assert_eq!(q.category.as_deref(), Some("蛮族"));
    }

    #[test]
    fn test_monster_query_aliases_case_and_fullwidth_colon() {
        let q = parse_monster_query("c:動物 LV：-1").unwrap();
        assert_eq!(q.category.as_deref(), Some("動物"));
        assert_eq!(q.level, Some(-1));
        assert!(q.names.is_empty());
    }

    #[test]
    fn test_spell_query_conditions() {
        let q = parse_spell_query("テスト s:MagicCat_1 lv:3").unwrap();
        assert_eq!(q.names, names(&["テスト"]));
        assert_eq!(q.school.as_deref(), Some("MagicCat_1"));
        assert_eq!(q.level, Some(3));
        assert_eq!(parse_spell_query("school:MagicCat_2").unwrap().school.as_deref(), Some("MagicCat_2"));
    }

    #[test]
    fn test_non_alphabetic_prefix_is_a_word() {
        // キーが英字だけでなければ名前の一部として扱う
        for input in ["テスト:改", ":テスト", "Lv1:テスト", "x_y:z"] {
            let q = parse_monster_query(input).unwrap();
            assert_eq!(q.names, names(&[input]), "{input}");
        }
    }

    #[test]
    fn test_invalid_tokens_are_errors() {
        assert_eq!(
            parse_monster_query("lv:abc"),
            Err(QueryParseError::InvalidNumber { key: "lv".into(), value: "abc".into() })
        );
        // 入力途中の `lv:` も解釈できない
        assert_eq!(parse_monster_query("テスト lv:"), Err(QueryParseError::EmptyValue { key: "lv".into() }));
        assert_eq!(parse_monster_query("cat:"), Err(QueryParseError::EmptyValue { key: "cat".into() }));
        assert_eq!(
            parse_monster_query("lv:1 lv:2"),
            Err(QueryParseError::Duplicate { key: "lv".into() })
        );
        // 別名で重ねても重複
        assert_eq!(
            parse_monster_query("c:蛮族 cat:動物"),
            Err(QueryParseError::Duplicate { key: "cat".into() })
        );
    }

    #[test]
    fn test_keys_are_tab_specific() {
        assert!(matches!(
            parse_monster_query("s:MagicCat_1"),
            Err(QueryParseError::UnknownKey { ref key, .. }) if key == "s"
        ));
        assert!(matches!(
            parse_spell_query("cat:蛮族"),
            Err(QueryParseError::UnknownKey { ref key, .. }) if key == "cat"
        ));
        // 段階3のキーはまだ使えない
        assert!(parse_spell_query("rank:2").is_err());
    }

    #[test]
    fn test_error_messages() {
        assert_eq!(
            parse_monster_query("lv:abc").unwrap_err().to_string(),
            "lv: の値は整数で指定してください（abc）"
        );
        assert_eq!(
            parse_monster_query("foo:1").unwrap_err().to_string(),
            "foo: は使えません（使えるキー: lv: cat:）"
        );
        assert_eq!(
            parse_spell_query("cat:x").unwrap_err().to_string(),
            "cat: は使えません（使えるキー: lv: s:）"
        );
        assert_eq!(parse_spell_query("s:").unwrap_err().to_string(), "s: に値がありません");
    }

    #[test]
    fn test_name_label_joins_words() {
        assert_eq!(parse_monster_query("lv:3").unwrap().name_label(), None);
        assert_eq!(
            parse_monster_query("テスト 戦士").unwrap().name_label().as_deref(),
            Some("テスト 戦士")
        );
    }
}
