use std::process;
use trpg_json_core::{export, io, query, stats, Monster};
use crate::utils;

/// 検索コマンドのハンドラ
pub fn handle_find(data_paths: &[String], name: &str, level: Option<i32>, category: Option<&str>) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 検索を実行
    let results = query::find_multi(&monsters, Some(name), level, category);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(Some(name), level, category);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(&results.iter().map(|&m| m.clone()).collect::<Vec<_>>());
        }
        n => {
            // 複数件の場合は件数を出力
            println!("{} 件のモンスターが見つかりました", n);
            
            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == name) {
                let exact_monster = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_monster]);
            }
        }
    }
}

/// 一覧コマンドのハンドラ
pub fn handle_list(data_paths: &[String], pattern: &str) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // パターンマッチで検索
    let results = query::find_by_name(&monsters, pattern);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(Some(pattern), None, None);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        1 => {
            // 1件の場合は JSON で出力
            utils::save_json_stdout_or_exit(&results.iter().map(|&m| m.clone()).collect::<Vec<_>>());
        }
        n => {
            // 複数件の場合は名前の一覧を出力
            for (i, monster) in results.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                println!("Lv.{:2} {} [{}]", monster.level, monster.name, monster.category);
            }
            println!("\n計 {} 件のモンスターが見つかりました", n);
            
            // 完全一致するモンスターがあればそのデータを出力
            if let Some(exact_match) = results.iter().find(|m| m.name == pattern) {
                let exact_monster = (*exact_match).clone();
                utils::save_json_stdout_or_exit(&[exact_monster]);
            }
        }
    }
}

/// 選択・エクスポートコマンドのハンドラ
pub fn handle_select(
    data_paths: &[String],
    name: Option<&str>,
    level: Option<i32>,
    category: Option<&str>,
    export_format: Option<&str>,
    output_dest: Option<&str>,
) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 複合検索を実行
    let results = query::find_multi(&monsters, name, level, category);

    // 結果を処理
    match results.len() {
        0 => {
            let error_msg = utils::format_monster_filter_conditions(name, level, category);
            eprintln!("エラー: {}", error_msg);
            process::exit(1);
        }
        _ => {
            let json_results: Vec<_> = results.iter().map(|&m| m.clone()).collect();

            // エクスポート機能が指定されている場合
            if let Some(fmt) = export_format {
                if let Some(output) = output_dest {
                    export_results(&json_results, fmt, output);
                } else {
                    eprintln!("エラー: --export を使用する場合は --output で出力先を指定してください");
                    process::exit(1);
                }
            } else {
                // 通常の stdout 出力
                utils::save_json_stdout_or_exit(&json_results);
            }
        }
    }
}

/// エクスポート処理を実行
fn export_results(monsters: &[Monster], format: &str, output: &str) {
    // エクスポート形式をパース
    let export_format = match format.parse::<export::ExportFormat>() {
        Ok(fmt) => fmt,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // エクスポーターを生成
    let exporter = match export::ExporterFactory::create_exporter(export_format) {
        Ok(exp) => exp,
        Err(e) => {
            eprintln!("エラー: {}", e);
            process::exit(1);
        }
    };

    // エクスポート設定を作成
    let config = export::ExportConfig {
        destination: output.to_string(),
        format: export_format,
    };

    // エクスポート実行
    match exporter.export(monsters, &config) {
        Ok(()) => {
            println!("成功: {} 件のモンスターを {} にエクスポートしました", monsters.len(), output);
        }
        Err(e) => {
            eprintln!("エラー: エクスポートに失敗しました: {}", e);
            process::exit(1);
        }
    }
}

/// 追加コマンドのハンドラ
pub fn handle_add(data_paths: &[String], file: &str) {
    // JSON ファイルから新規モンスターを読み込む
    let new_monster_json = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("エラー: ファイルを読み込めません: {}", e);
            process::exit(1);
        }
    };

    let new_monster: Monster = match serde_json::from_str(&new_monster_json) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("エラー: JSON を解析できません: {}", e);
            process::exit(1);
        }
    };

    // 現在のデータを読み込む
    let mut monsters = utils::load_monsters_or_exit(data_paths);

    // 重複チェック
    if let Some(_duplicate) = query::find_by_exact_name(&monsters, &new_monster.name) {
        // 確認ダイアログを表示
        if !utils::confirm_action(&format!("\"{}\" という名前のモンスターは既に存在します。上書きしますか？", new_monster.name)) {
            eprintln!("キャンセルされました");
            process::exit(1);
        }

        // 既存のモンスターを削除
        monsters.retain(|m| m.name != new_monster.name);
    }

    // モンスターを追加
    monsters.push(new_monster.clone());

    // ファイルに保存（最初のファイルに保存）
    if let Err(e) = io::save_json_array_file(&data_paths[0], &monsters) {
        eprintln!("エラー: ファイルに保存できません: {}", e);
        process::exit(1);
    }

    println!("成功: \"{}\" を追加しました", new_monster.name);
}

/// 削除コマンドのハンドラ
pub fn handle_delete(data_paths: &[String], name: &str) {
    // 現在のデータを読み込む
    let mut monsters = utils::load_monsters_or_exit(data_paths);

    // 完全一致で検索
    if query::find_by_exact_name(&monsters, name).is_none() {
        eprintln!("エラー: \"{}\" という名前のモンスターが見つかりません", name);
        process::exit(1);
    }

    // 確認ダイアログを表示
    if !utils::confirm_action(&format!("\"{}\" を削除しますか？", name)) {
        eprintln!("キャンセルされました");
        process::exit(1);
    }

    // モンスターを削除
    monsters.retain(|m| m.name != name);

    // ファイルに保存（最初のファイルに保存）
    if let Err(e) = io::save_json_array_file(&data_paths[0], &monsters) {
        eprintln!("エラー: ファイルに保存できません: {}", e);
        process::exit(1);
    }

    println!("成功: \"{}\" を削除しました", name);
}

/// 統計コマンドのハンドラ
pub fn handle_stats(data_paths: &[String]) {
    let monsters = utils::load_monsters_or_exit(data_paths);

    // 統計情報を計算
    let stats = stats::MonsterStats::calculate(&monsters);
    
    // 整形して出力
    print!("{}", stats.format());
}
