use std::io::{self as std_io, Write};
use std::process;
use trpg_json_core::{io, Monster, Spell};
use serde::Serialize;

/// モンスターデータを読み込む（失敗時はエラーを表示して終了）
pub fn load_monsters_or_exit(data_paths: &[String]) -> Vec<Monster> {
    match io::load_multiple_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    }
}

/// スペルデータを読み込む（失敗時はエラーを表示して終了）
pub fn load_spells_or_exit(data_paths: &[String]) -> Vec<Spell> {
    match io::load_multiple_spells_json_arrays(
        &data_paths.iter().map(|p| p.as_str()).collect::<Vec<_>>()
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    }
}

/// データをJSONとして標準出力に出力する（失敗時はエラーを表示して終了）
pub fn save_json_stdout_or_exit<T: Serialize>(data: &[T]) {
    let json = match serde_json::to_string_pretty(data) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("エラー: JSON 変換に失敗しました: {}", e);
            process::exit(1);
        }
    };
    println!("{}", json);
}

/// 確認ダイアログを表示する
pub fn confirm_action(message: &str) -> bool {
    eprint!("警告: {} (y/n) ", message);
    let _ = std_io::stdout().flush();

    let mut response = String::new();
    if std_io::stdin().read_line(&mut response).is_err() {
        eprintln!("\nエラー: 入力の読み込みに失敗しました");
        return false;
    }

    let trimmed = response.trim();
    trimmed.eq_ignore_ascii_case("y") || trimmed.eq_ignore_ascii_case("yes")
}

/// クリップボードに文字列をコピーする
pub fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
    use std::process::Command;
    
    #[cfg(target_os = "macos")]
    {
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    {
        let mut child = Command::new("cmd")
            .args(&["/C", "clip"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        child.wait()?;
        Ok(())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Clipboard functionality is not supported on this platform"
        ))
    }
}

/// モンスターのフィルター条件を整形して文字列で返す
pub fn format_monster_filter_conditions(
    name: Option<&str>,
    level: Option<i32>,
    category: Option<&str>,
) -> String {
    let mut conditions = Vec::new();
    
    if let Some(n) = name {
        conditions.push(format!("  - name: \"{}\"", n));
    }
    if let Some(l) = level {
        conditions.push(format!("  - level: {}", l));
    }
    if let Some(c) = category {
        conditions.push(format!("  - category: \"{}\"", c));
    }
    
    if conditions.is_empty() {
        String::new()
    } else {
        format!("以下の条件でマッチするモンスターが見つかりません\n{}", conditions.join("\n"))
    }
}

/// スペルのフィルター条件を整形して文字列で返す
pub fn format_spell_filter_conditions(
    name: Option<&str>,
    school: Option<&str>,
    level: Option<i32>,
    rank: Option<i32>,
    school_variant: Option<&str>,
    god: Option<&str>,
) -> String {
    let mut conditions = Vec::new();
    
    if let Some(n) = name {
        conditions.push(format!("  - name: \"{}\"", n));
    }
    if let Some(s) = school {
        conditions.push(format!("  - school: \"{}\"", s));
    }
    if let Some(l) = level {
        conditions.push(format!("  - level: {}", l));
    }
    if let Some(r) = rank {
        conditions.push(format!("  - rank: {}", r));
    }
    if let Some(v) = school_variant {
        conditions.push(format!("  - schoolVariant: \"{}\"", v));
    }
    if let Some(g) = god {
        conditions.push(format!("  - god: \"{}\"", g));
    }
    
    if conditions.is_empty() {
        String::new()
    } else {
        format!("以下の条件でマッチするスペルが見つかりません\n{}", conditions.join("\n"))
    }
}
