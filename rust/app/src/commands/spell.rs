use std::process;
use trpg_json_core::{export, query, stats};
use crate::utils;

/// スペル検索コマンドのハンドラ
pub fn handle_find(
    data_paths: &[String],
    name: &str,
    level: Option<i32>,
    rank: Option<i32>,
    school: Option<&str>,
    school_variant: Option<&str>,
    god: Option<&str>,
) {
    // level と rank の同時指定チェック
    if level.is_some() && rank.is_some() {
        eprintln!("エラー: -l (level) と -r (rank) は同時に指定できません");
        process::exit(1);
    }

    let spells = utils::load_spells_or_exit(data_paths);

    // 検索を実行
    let results = query::spell_find_multi(&spells, Some(name), school, level, rank, school_variant, god);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_spell_filter_conditions(Some(name), school, level, rank, school_variant, god);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(&results.iter().map(|&s| s.clone()).collect::<Vec<_>>());
        }
        n => {
            // 複数件の場合は件数を出力
            println!("{} 件のスペルが見つかりました", n);
            
            // 完全一致するスペルがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|s| s.name == name) {
                let exact_spell = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_spell]);
            }
        }
    }
}

/// スペル一覧コマンドのハンドラ
pub fn handle_list(data_paths: &[String], pattern: &str) {
    let spells = utils::load_spells_or_exit(data_paths);

    // 検索を実行
    let results = query::spell_find_by_name(&spells, pattern);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_spell_filter_conditions(Some(pattern), None, None, None, None, None);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(&results.iter().map(|&s| s.clone()).collect::<Vec<_>>());
        }
         n => {
             // 複数件の場合は名前一覧を出力
             println!("{}個のスペルが見つかりました:", n);
             for spell in &results {
                 println!("  - {} ({})", spell.name, spell.school);
             }
            
            // 完全一致するスペルがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|s| s.name == pattern) {
                let exact_spell = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_spell]);
            }
        }
    }
}

/// スペル統計コマンドのハンドラ
pub fn handle_stats(data_paths: &[String]) {
    let spells = utils::load_spells_or_exit(data_paths);

    // 統計情報を計算
    let stats = stats::SpellStats::calculate(&spells);
    
    // 整形して出力
    print!("{}", stats.format());
}

/// スペルパレットコマンドのハンドラ
pub fn handle_palette(
    data_paths: &[String],
    name: Option<&str>,
    level: Option<i32>,
    rank: Option<i32>,
    school: Option<&str>,
    school_variant: Option<&str>,
    god: Option<&str>,
    copy: bool,
) {
    // level と rank の同時指定チェック
    if level.is_some() && rank.is_some() {
        eprintln!("エラー: -l (level) と -r (rank) は同時に指定できません");
        process::exit(1);
    }

    // 最低1つのフィルタが必須
    if name.is_none() && level.is_none() && rank.is_none() && school.is_none() && school_variant.is_none() && god.is_none() {
        eprintln!("エラー: 最低1つのフィルタ（-n, -l, -r, -s, -v, -g）を指定してください");
        process::exit(1);
    }

    let spells = utils::load_spells_or_exit(data_paths);

    // マルチフィルタで検索
    let results = query::spell_find_multi(&spells, name, school, level, rank, school_variant, god);

    match results.len() {
        0 => {
            let error_msg = utils::format_spell_filter_conditions(name, school, level, rank, school_variant, god);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        _ => {
            // 全マッチしたスペルのパレットを複数行で出力
            let mut first_palette: Option<String> = None;

            for spell in &results {
                match export::palette::generate_spell_palette(&spell) {
                    Ok(palette) => {
                        println!("{}", palette);
                        
                        // 先頭のパレットを保存（copy用）
                        if first_palette.is_none() {
                            first_palette = Some(palette);
                        }
                    }
                    Err(e) => {
                        eprintln!("警告: スペル '{}' のパレット生成に失敗しました: {}", spell.name, e);
                    }
                }
            }

            // --copy フラグが指定されている場合、先頭のパレットをクリップボードにコピー
            if copy {
                if let Some(palette) = first_palette {
                    match utils::copy_to_clipboard(&palette) {
                        Ok(_) => {
                            eprintln!("✓ チャットパレット（先頭行）をクリップボードにコピーしました");
                        }
                        Err(e) => {
                            eprintln!("警告: クリップボードへのコピーに失敗しました: {}", e);
                        }
                    }
                }
            }
        }
    }
}
