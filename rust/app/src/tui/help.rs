//! ヘルプモーダルの内容（全キーバインドと検索構文）
//!
//! キーの割り当て（`action.rs`）を変えたらここも合わせて更新する。

/// ヘルプの1節: 見出しと（キー, 説明）の並び
pub struct HelpSection {
    pub title: &'static str,
    pub entries: &'static [(&'static str, &'static str)],
}

/// ヘルプモーダルに表示する節（表示順）
pub const SECTIONS: &[HelpSection] = &[
    HelpSection {
        title: "全体",
        entries: &[
            ("1 / 2", "モンスター / スペルのタブへ切り替え"),
            ("Tab / Shift-Tab", "ペイン移動（検索 → 結果 → 詳細）"),
            ("/", "検索バーへ"),
            ("?", "このヘルプ"),
            ("J", "選択中の項目の JSON を表示"),
            ("q / Ctrl-c", "終了"),
        ],
    },
    HelpSection {
        title: "検索バー",
        entries: &[
            ("文字入力", "名前の部分一致で即時に絞り込み"),
            ("Backspace", "1文字削除"),
            ("Ctrl-u", "入力をクリア"),
            ("Enter / Esc", "結果リストへ（入力は保持）"),
            ("↑↓", "結果リストの選択を移動"),
        ],
    },
    HelpSection {
        title: "結果リスト",
        entries: &[
            ("↑↓ / j k", "選択を移動"),
            ("PgUp / PgDn", "1ページ移動"),
            ("g / G", "先頭 / 末尾へ"),
            ("Enter", "詳細ペインへ"),
        ],
    },
    HelpSection {
        title: "詳細ペイン",
        entries: &[
            ("↑↓ / j k", "モンスター: 部位の選択 / スペル: 1行スクロール"),
            ("PgUp / PgDn", "本文のスクロール"),
            ("g / G", "本文の先頭 / 末尾へ"),
            ("Esc", "結果リストへ"),
        ],
    },
    HelpSection {
        title: "コピー",
        entries: &[
            ("c", "モンスター: 選択中の部位をココフォリア形式でコピー"),
            ("y", "スペル: 選択中のチャットパレットをコピー"),
            ("Y", "スペル: ヒット中の全件のパレットを改行区切りでコピー"),
        ],
    },
    HelpSection {
        title: "ヘルプ・JSON ビュー",
        entries: &[
            ("↑↓ / j k", "1行スクロール"),
            ("PgUp / PgDn / Space", "1ページスクロール"),
            ("g / G", "先頭 / 末尾へ"),
            ("y", "JSON ビュー: 表示中の JSON をコピー"),
            ("Esc / q / Enter", "閉じる"),
        ],
    },
    HelpSection {
        title: "検索構文",
        entries: &[
            ("（入力全体）", "名前の部分一致。空なら全件表示"),
            ("例: テスト", "名前に「テスト」を含む項目"),
        ],
    },
    HelpSection {
        title: "画面",
        entries: &[
            ("幅 100 桁未満", "結果 / 詳細を1ペインで表示（Enter / Esc で切り替え）"),
            ("高さ 15 行未満", "「端末が小さすぎます」と表示"),
        ],
    },
];
