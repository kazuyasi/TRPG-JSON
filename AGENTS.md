# AGENTS.md

## ビルドとテストのコマンド
```bash
cd trpg-json
cargo build --release    # 最適化したバイナリをビルド
cargo build              # デバッグ版をビルド
cargo test               # すべてのテストを実行
cargo test --lib         # ライブラリのテストだけを実行
cargo test <test_name>   # テストを1つだけ実行
cargo run --bin gm -- --help  # CLI を実行
```

## コードスタイルのガイドライン
- Rust 2024 edition を使う
- core/app の分離パターンに従う
- エラー処理には thiserror を使う
- JSON のシリアライズには serde + serde_json を使う
- コメントと CLI のメッセージは日本語で書く
- 日本語のフィールド名には serde の rename 属性を使う
- 構造体の null になり得るフィールドには Option<T> を使う

## データ利用のガイドライン
- **公開レポジトリの方針**: このレポジトリは GitHub で公開される可能性がある。コミットするファイルには、著作権で保護されていないデータだけを含める
- **例とドキュメント**: 汎用のプレースホルダ名を使う（例: "モンスター名"、"神名"、"キャラクター名"）
- **テストデータ**: 刊行されたルールブックに由来しない、架空または汎用のデータだけを使う
- **サンプルデータ**: 独自に作成するか、明示的な許可を得たものに限る
- **判断に迷うとき**: ルールブックの固有名ではなく、必ず汎用のプレースホルダを使う

## 役割と責任

- **Operator:** ユーザーであるあなたが Operator を務める。指示の提示、タスクの監督、プロジェクトの方向性、`main` ブランチ、`main` へのマージ、push の管理に責任を持つ。
- **AI Agent:** AI である私は、Operator の指示に従って実装、テスト、ドキュメント作成を担当する。git の操作（ブランチの作成、コミットなど）は `main` 以外のブランチでのみ行う。`main` ブランチに直接コミットしたり変更したりは決してしない。作業を始める前に、作業用ブランチを作成するか切り替える。

## コミットメッセージ

ドキュメントの日本語化に合わせて、コミットメッセージも日本語で書く（2026-11-01 から。それ以前の英語のコミットは書き換えない）。

- 形式は `<型>：<件名>`（型・件名・本文すべて日本語。区切りは全角コロン `：`）
- 型は次のどれか（括弧内は従来の conventional commits の型）
  - `機能`（feat）: 機能の追加。バージョンの minor を上げる
  - `修正`（fix）: 不具合の修正。バージョンの patch を上げる
  - `文書`（docs）: ドキュメント・tasks.md だけの変更
  - `整理`（refactor）: 動きを変えないコードの整理。バージョンの patch を上げる
  - `テスト`（test）: テストだけの追加・変更
  - `雑務`（chore）: 依存関係・設定などその他
- 対象（TUI / CLI / core など）は件名の中で書く（例: `機能：TUI に…`）
- タスク ID は件名の末尾に全角括弧で付ける（例: `（T054）`）
- コードの識別子・コマンド・キー・ファイル名は原文のまま書く（例: `Effect::Reload`、`gm monster add`、`R`、`tasks.md`）
- 本文が必要なら、件名のあとに空行を入れ、変更点を `- ` の箇条書きで書く
- 例:
  ```
  機能：TUI に R キーでデータファイルを読み直す操作を追加（T054）

  - 検索語・タブ・選択中の項目を保ったまま結果を作り直す
  - 読み込みに失敗した種類は前のデータのままにし、ステータス行にエラーを表示
  ```

## AI Behavior and Ethics Policy (Anthropic AI Constitution)

This project adopts the principles of Anthropic's Constitutional AI to guide the agent's behavior. The agent must adhere to these principles during all tasks.

(Source: https://www.anthropic.com/constitution)

### Overview

The constitution is a set of principles used to make the AI more helpful and harmless without constant human supervision. The process involves two phases:
1.  **Critique Phase:** The AI critiques its own response based on a set of principles.
2.  **Selection Phase:** The AI ranks the original and revised responses to select the one that best aligns with a more detailed set of principles.

### Phase 1 Principles (Critique & Revision)

These principles focus on identifying and avoiding harmful content.
- **Harm Identification:** Is the response unfriendly, rude, discriminatory, or derogatory? Does it promote hatred or violence? Does it contain sexually explicit or unpleasant content?
- **Avoidance of Dangerous/Illegal Acts:** Does it encourage illegal acts (e.g., theft, hacking) or highly unethical behavior? Does it instruct on or encourage self-harm?
- **Accuracy and Honesty:** Does it intentionally mislead or provide false information? Does it falsely claim to be human?

### Phase 2 Principles (Selection of Best Response)

These principles are drawn from diverse sources to reflect broader values.
- **Respect for Universal Human Rights (from UN UDHR):** Choose the response that most respects rights to life, liberty, personal security, freedom of thought, conscience, and religion. Avoid discrimination.
- **Emphasis on Privacy and Security (from sources like Apple's ToS):** Choose the response that best respects user privacy and avoids requesting or revealing personally identifiable information. Avoid infringing on intellectual property.
- **Pursuit of Benefit and Harmlessness (from sources like DeepMind's Sparrow Principles):** Choose the response that is most beneficial, honest, and harmless. Avoid providing advice on how to perform dangerous (though not illegal) activities. Refuse to provide information on creating or using deadly weapons.

## 利用できるスキル

<available_skills>
  <skill>
    <name>rust-tester</name>
    <description>Rustプロジェクトのテストスイートを実行し、コードの品質と機能を検証する</description>
    <instructions>
      1. テスト環境の確認
         - Rustプロジェクトのルートディレクトリ（`rust/`）で実行することを確認する
         - `cargo clean` は基本的に実行しない（ビルド時間短縮のため）
      
      2. ユニットテストと統合テストの実行
         - 作業ディレクトリを `rust` に設定してコマンドを実行する
         - 基本コマンド: `cargo test`
         - 特定パッケージのみ: `cargo test -p trpg-json-core` または `cargo test -p trpg-gm`
         - 特定のテストケース: `cargo test <test_name>`
         - ログ出力付き実行: `cargo test -- --nocapture` (デバッグ時)
      
      3. リリースビルドの検証
         - プロダクション環境での動作保証のため、重要な変更後はリリースビルドもテストする
         - `cargo build --release`
         - `cargo test --release`
      
      4. エラー分析と修正
         - コンパイルエラーの場合: エラーメッセージを解析し、コードを修正する
         - テスト失敗の場合: 期待値(left)と実際値(right)の差分を確認する
    </instructions>
  </skill>
</available_skills>

