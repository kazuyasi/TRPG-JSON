use crate::Monster;
use std::fs;
use std::path::Path;

/// I/O操作時のエラー型
#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("ファイル読み込みエラー: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("JSON解析エラー: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("無効なデータ形式: {0}")]
    InvalidFormat(String),
}

/// JSONファイルから Monster 配列をロード
/// 
/// # 引数
/// * `path` - 読み込むファイルのパス（JSON配列のみ受け入れ）
///
/// # 戻り値
/// * `Ok(Vec<Monster>)` - ロードされたモンスターのリスト
/// * `Err(IoError)` - 読み込みまたはパースに失敗した場合
///
/// # エラー
/// - ファイルが読み込めない場合は `IoError::FileRead`
/// - JSONが無効な場合は `IoError::JsonParse`
/// - ルート要素が配列でない場合は `IoError::InvalidFormat`
pub fn load_json_array<P: AsRef<Path>>(path: P) -> Result<Vec<Monster>, IoError> {
    // ファイルを読み込む
    let content = fs::read_to_string(path)?;

    // JSONをパース
    let value: serde_json::Value = serde_json::from_str(&content)?;

    // ルート要素が配列であることを確認
    match value.as_array() {
        Some(arr) => {
            // 配列の各要素をMonsterにデシリアライズ
            let mut monsters = Vec::new();
            for (idx, item) in arr.iter().enumerate() {
                let monster: Monster = serde_json::from_value(item.clone())
                    .map_err(|e| {
                        // エラーメッセージを含む新しいエラーを作成
                        let msg = format!("インデックス{}のデータが無効: {}", idx, e);
                        IoError::InvalidFormat(msg)
                    })?;
                monsters.push(monster);
            }
            Ok(monsters)
        }
        None => Err(IoError::InvalidFormat(
            "JSONのルート要素が配列ではありません".to_string(),
        )),
    }
}

/// Monster 配列を JSON として標準出力に出力
///
/// # 引数
/// * `monsters` - 出力するモンスターのスライス
///
/// # 戻り値
/// * `Ok(())` - 出力に成功した場合
/// * `Err(IoError)` - 出力に失敗した場合
pub fn save_json_array_stdout(monsters: &[Monster]) -> Result<(), IoError> {
    let json = serde_json::to_string_pretty(monsters)?;
    println!("{}", json);
    Ok(())
}

/// Monster 配列を JSON ファイルに保存
///
/// # 引数
/// * `path` - 保存先ファイルのパス
/// * `monsters` - 保存するモンスターのスライス
///
/// # 戻り値
/// * `Ok(())` - 保存に成功した場合
/// * `Err(IoError)` - 保存に失敗した場合
pub fn save_json_array_file<P: AsRef<Path>>(path: P, monsters: &[Monster]) -> Result<(), IoError> {
    let json = serde_json::to_string_pretty(monsters)?;
    fs::write(path, json)?;
    Ok(())
}

/// 複数の JSON ファイルから Monster 配列をロードして統合
/// 
/// # 引数
/// * `paths` - 読み込むファイルのパスのリスト
///
/// # 戻り値
/// * `Ok(Vec<Monster>)` - ロードされたすべてのモンスターのリスト
/// * `Err(IoError)` - 読み込みまたはパースに失敗した場合（最初のエラーで中止）
///
/// # エラー
/// - ファイルが読み込めない場合は `IoError::FileRead`
/// - JSONが無効な場合は `IoError::JsonParse`
/// - ルート要素が配列でない場合は `IoError::InvalidFormat`
pub fn load_multiple_json_arrays<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<Monster>, IoError> {
    let mut all_monsters = Vec::new();
    
    for path in paths {
        let monsters = load_json_array(path)?;
        all_monsters.extend(monsters);
    }
    
    Ok(all_monsters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_json_array() {
        // テスト用の一時ファイルを作成
        let mut file = NamedTempFile::new().expect("Failed to create temp file");

        let json_data = r#"[
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
                "name": "テストモンスター1",
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
            },
            {
                "Category": "蛮族",
                "Lv": 7,
                "Revision": 2.5,
                "data": "GR144",
                "illust": "",
                "movein": 20,
                "movein_des": "飛行",
                "moveon": 20,
                "moveon_des": "",
                "name": "テストモンスター2",
                "part": [
                    {
                        "HP": 50,
                        "MP": 80,
                        "name": "",
                        "コア": true,
                        "命中力": 16,
                        "回避力": 16,
                        "打撃点": 7,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 6
                    }
                ],
                "備考": "",
                "先制値": 15,
                "共通特殊能力": "飛行",
                "弱点": "純エネルギー属性ダメージ+3",
                "弱点値": 18,
                "生命抵抗力": 17,
                "知名度": 15,
                "精神抵抗力": 17
            }
        ]"#;

        writeln!(file, "{}", json_data).expect("Failed to write to temp file");

        // ロード
        let result = load_json_array(file.path());
        assert!(result.is_ok(), "ロードが失敗しました");

        let monsters = result.unwrap();
        assert_eq!(monsters.len(), 2, "2つのモンスターがロードされるべき");
        assert_eq!(monsters[0].name, "テストモンスター1");
        assert_eq!(monsters[1].name, "テストモンスター2");
        assert_eq!(monsters[0].level, 6);
        assert_eq!(monsters[1].level, 7);
    }

    #[test]
    fn test_load_single_monster_array() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");

        let json_data = r#"[
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
                "name": "単体テスト",
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
            }
        ]"#;

        writeln!(file, "{}", json_data).expect("Failed to write to temp file");

        let result = load_json_array(file.path());
        assert!(result.is_ok());

        let monsters = result.unwrap();
        assert_eq!(monsters.len(), 1);
        assert_eq!(monsters[0].name, "単体テスト");
    }

    #[test]
    fn test_load_empty_array() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "[]").expect("Failed to write to temp file");

        let result = load_json_array(file.path());
        assert!(result.is_ok());

        let monsters = result.unwrap();
        assert_eq!(monsters.len(), 0);
    }

    #[test]
    fn test_load_invalid_json() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "{{invalid json}}").expect("Failed to write to temp file");

        let result = load_json_array(file.path());
        assert!(result.is_err());

        match result {
            Err(IoError::JsonParse(_)) => {
                // 期待通り
            }
            _ => panic!("JsonParseエラーが期待される"),
        }
    }

    #[test]
    fn test_load_non_array_root() {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");

        let json_data = r#"{
            "Category": "蛮族",
            "Lv": 6,
            "name": "テストモンスター"
        }"#;

        writeln!(file, "{}", json_data).expect("Failed to write to temp file");

        let result = load_json_array(file.path());
        assert!(result.is_err());

        match result {
            Err(IoError::InvalidFormat(_)) => {
                // 期待通り
            }
            _ => panic!("InvalidFormatエラーが期待される"),
        }
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_json_array("/nonexistent/path/to/file.json");
        assert!(result.is_err());

        match result {
            Err(IoError::FileRead(_)) => {
                // 期待通り
            }
            _ => panic!("FileReadエラーが期待される"),
        }
    }

    #[test]
    fn test_save_json_array_stdout() {
        // save_json_array_stdoutの動作確認
        // 標準出力への出力を直接テストするのは難しいため、
        // エラーが発生しないことを確認

        let monsters = vec![];
        let result = save_json_array_stdout(&monsters);
        assert!(result.is_ok(), "空配列の出力は成功するべき");
    }

    #[test]
    fn test_load_multiple_json_arrays() {
        // 複数の一時ファイルを作成
        let mut file1 = NamedTempFile::new().expect("Failed to create temp file 1");
        let mut file2 = NamedTempFile::new().expect("Failed to create temp file 2");

        let json_data1 = r#"[
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
                "name": "ファイル1のモンスター",
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
            }
        ]"#;

        let json_data2 = r#"[
            {
                "Category": "蛮族",
                "Lv": 7,
                "Revision": 2.5,
                "data": "GR144",
                "illust": "",
                "movein": 20,
                "movein_des": "飛行",
                "moveon": 20,
                "moveon_des": "",
                "name": "ファイル2のモンスター1",
                "part": [
                    {
                        "HP": 50,
                        "MP": 80,
                        "name": "",
                        "コア": true,
                        "命中力": 16,
                        "回避力": 16,
                        "打撃点": 7,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 6
                    }
                ],
                "備考": "",
                "先制値": 15,
                "共通特殊能力": "飛行",
                "弱点": "純エネルギー属性ダメージ+3",
                "弱点値": 18,
                "生命抵抗力": 17,
                "知名度": 15,
                "精神抵抗力": 17
            },
            {
                "Category": "魔法生物",
                "Lv": 8,
                "Revision": 2.5,
                "data": "GR145",
                "illust": "",
                "movein": 18,
                "movein_des": "",
                "moveon": 18,
                "moveon_des": "",
                "name": "ファイル2のモンスター2",
                "part": [
                    {
                        "HP": 52,
                        "MP": 85,
                        "name": "",
                        "コア": true,
                        "命中力": 17,
                        "回避力": 17,
                        "打撃点": 8,
                        "部位数": 1,
                        "部位特殊能力": "",
                        "防護点": 7
                    }
                ],
                "備考": "",
                "先制値": 16,
                "共通特殊能力": "",
                "弱点": "純エネルギー属性ダメージ+3",
                "弱点値": 19,
                "生命抵抗力": 18,
                "知名度": 16,
                "精神抵抗力": 18
            }
        ]"#;

        writeln!(file1, "{}", json_data1).expect("Failed to write to file 1");
        writeln!(file2, "{}", json_data2).expect("Failed to write to file 2");

        // 複数ファイルをロード
        let paths = [file1.path(), file2.path()];
        let result = load_multiple_json_arrays(&paths);
        assert!(result.is_ok(), "複数ファイルのロードが失敗しました");

        let monsters = result.unwrap();
        assert_eq!(monsters.len(), 3, "3つのモンスターがロードされるべき");
        assert_eq!(monsters[0].name, "ファイル1のモンスター");
        assert_eq!(monsters[1].name, "ファイル2のモンスター1");
        assert_eq!(monsters[2].name, "ファイル2のモンスター2");
    }

    #[test]
    fn test_load_multiple_json_arrays_with_empty_file() {
        // 1つの空のファイルと1つのデータを持つファイルを作成
        let mut file1 = NamedTempFile::new().expect("Failed to create temp file 1");
        let mut file2 = NamedTempFile::new().expect("Failed to create temp file 2");

        writeln!(file1, "[]").expect("Failed to write to file 1");

        let json_data2 = r#"[
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
                "name": "単一モンスター",
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
            }
        ]"#;

        writeln!(file2, "{}", json_data2).expect("Failed to write to file 2");

        let paths = [file1.path(), file2.path()];
        let result = load_multiple_json_arrays(&paths);
        assert!(result.is_ok());

        let monsters = result.unwrap();
        assert_eq!(monsters.len(), 1, "1つのモンスターがロードされるべき");
        assert_eq!(monsters[0].name, "単一モンスター");
    }

    #[test]
    fn test_load_multiple_json_arrays_missing_file() {
        let mut file1 = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file1, "[]").expect("Failed to write to file");

        let paths = [file1.path(), std::path::Path::new("/nonexistent/file.json")];
        let result = load_multiple_json_arrays(&paths);
        assert!(result.is_err(), "存在しないファイルがあるため、エラーが発生するべき");

        match result {
            Err(IoError::FileRead(_)) => {
                // 期待通り
            }
            _ => panic!("FileReadエラーが期待される"),
        }
    }
}
