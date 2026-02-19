# Contributing to boj-client

このドキュメントは、`boj-client` への寄稿手順を定義します。

## 1. 目的と対象範囲

- 対象: 外部Contributorによる機能追加、バグ修正、テスト追加、ドキュメント更新
- 本書の範囲外: maintainer権限でのリリース公開実行（workflow_dispatch 実行権限が必要）

## 2. セットアップ

- Rust stable を利用してください
- 依存取得後、以下のコマンドが実行できることを確認してください

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
cargo test --doc --workspace
```

## 3. リポジトリ構成

- `src/`: ライブラリ実装
- `tests/`: 統合テスト
- `tests/fixtures/`: デコーダー・挙動確認用フィクスチャ
- `docs/api-manual/`: 公式仕様の一次参照（原本準拠Markdown）
- `docs/design/`: クライアント設計方針

ライブラリcrateとして `Cargo.lock` は追跡しません。crates.io に同梱するファイルは `.gitignore` ではなく `Cargo.toml` の `[package].include` で管理し、公開物は `src/` とメタデータ（`Cargo.toml`, `README*`, `LICENSE`）を最小構成として維持してください。

## 4. ブランチ戦略

`master` への直接コミット、直接プッシュは禁止です。すべてPR経由で取り込んでください。

- 機能追加・仕様拡張: `feature/<topic>`
- バグ修正: `hotfix/<topic>`
- 運用改善・開発フロー改善・docs-only・CI/ツール整備・依存更新（外部挙動非変更）: `chore/<topic>`

ブランチ種別の判定は、変更の「主目的」で行います。

- 複合変更は主目的で判定する
- バグ修正を含む場合は `hotfix/*` を優先する
- 利用者向け機能追加・仕様拡張が主なら `feature/*`
- それ以外（運用・開発体験・文書のみ）は `chore/*`
- docs-only は常に `chore/*`

ブランチ名の例:

- `feature/add-layer-query-validation`
- `hotfix/fix-csv-json-error-fallback`
- `chore/improve-contributing-workflow`

## 5. 変更フロー

1. 最新 `master` を取得する
2. 目的に応じたブランチを作成する（`feature/*` / `hotfix/*` / `chore/*`）
3. 実装・テスト・ドキュメントを更新する
4. 必須チェックを通す
5. PRを作成し、レビューを受ける

## 6. 仕様・実装ルール

- API仕様解釈は `docs/api-manual/` を優先してください
- クライアントの設計判断は `docs/design/` と整合させてください
- 仕様変更を取り込む場合は、必要に応じて実装とテストとドキュメントを同時更新してください

### 6.1 rustdoc 記述方針（公開API）

- 公開API向けの `rustdoc` は英語を基本としてください
- 記述は簡潔かつ利用者視点で、利用方法と挙動が読み取りやすい内容にしてください
- 固有名詞や公式API用語は、必要に応じて日本語/原語の併記を許容します
- 公開APIのドキュメント欠落はビルドエラーとして扱います（`#![deny(missing_docs)]`）
- リンク不整合と裸URLはビルドエラーとして扱います（`rustdoc::broken_intra_doc_links`, `rustdoc::bare_urls`）

推奨テンプレート（公開型・公開関数）:

- 1行要約（何を提供する型/関数か）
- 必要に応じて制約条件（例: date format や value range）
- `Result` を返す公開関数は `# Errors` を必須記載
- 利用例が有効な公開APIは `# Examples` を記載
- ネットワークI/Oを伴う例は `no_run` を付けて doctest のコンパイル検証のみ行う

## 7. テスト追加ルール

- クエリ構築やURL変更: `tests/url_building.rs` を更新
- デコーダー変更: `tests/json_decoder.rs` または `tests/csv_decoder.rs` を更新
- クライアントの挙動変更: `tests/client_behavior.rs` を更新
- スナップショット対象の変更: `tests/snapshot_regression.rs` を更新
- フィクスチャを追加・更新した場合: `tests/fixtures/README.md` の参照節を更新

## 8. 提出前の必須チェック

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps
cargo test --doc --workspace
```

## 9. 任意チェック

- Nightly契約テスト（ライブAPI疎通を確認したい場合）

```bash
BOJ_CONTRACT_TEST=1 cargo test --test contract_nightly -- --ignored
```

- Instaスナップショットのフォーマット更新が必要な場合

```bash
cargo insta test --force-update-snapshots
```

## 10. ドキュメント同期ルール

- `README.md` または `README.en.md` の片方だけ更新する場合は、PRで deferred 理由を明記してください
- README翻訳は緩め同期ポリシーで運用します
- 同期完了時は deferred 状態を解消してください

## 11. PRルール

- `.github/pull_request_template.md` の項目を埋めてください
- `Summary / 概要` と `What Changed / 変更内容` に、目的と変更点を記載してください
- `Compatibility & Release Impact / 互換性とリリース影響` で API影響区分・SemVer想定を明示し、破壊的変更時は移行ノートを必ず記載してください
- `Test Evidence / テスト実施内容` に、実行したチェックと追加・更新したテストを記載してください
- `Spec & Docs Sync / 仕様・ドキュメント同期` と `README Synchronization / README同期` で、ドキュメント更新有無を明示してください
- `Reviewer Focus / レビュアー注目点` と `Risks & Rollback / リスクとロールバック` に、重点確認ポイントとリスク対処方針を記載してください
- `Related issue` は任意です（Issueがない場合は空欄で構いません）
- 変更が大きい場合のIssue起票は任意ですが、背景共有のため推奨します

## 12. Definition of Done

以下をすべて満たした状態を完了とします。

- 適切なブランチ種別（`feature/*` / `hotfix/*` / `chore/*`）で作業している
- 必須チェックを通過している
- 必要なテストが追加または更新されている
- 仕様参照と実装が整合している
- PRテンプレートが記入されている

## 13. メンテナー向けリリースフロー（参照）

保護ブランチ運用（PR必須）を前提とした公開手順です。版上げを workflow が直接 `master` に push する運用は行いません。

1. 版上げはPRで実施し、`master` にマージする
2. `release-publish.yml` を `workflow_dispatch` で実行する
   - 入力 `version`: ルート `Cargo.toml` と一致必須
   - 入力 `dry_run=true`: release gate のみ実行
   - 入力 `dry_run=false`: gate 通過後に crates.io publish
3. MCP公開が必要な場合は `mcp-release.yml` を実行する
   - 入力 `version`: `mcp-server/Cargo.toml` と `npm/boj-mcp-server/package.json` の両方と一致必須
   - `publish_npm=false`: GitHub Release assets のみ公開
   - `publish_npm=true`: assets公開後に npm 公開（失敗時も summary で assets成否を確認可能）
