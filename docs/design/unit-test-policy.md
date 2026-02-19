# boj-client 単体テスト方針（v0.x）

- 作成日: 2026-02-19
- ステータス: Adopted

## 1. 目的

BOJ API クライアントの単体テストは、実データの正しさではなく「クライアント実装が公式仕様どおりに振る舞うこと」を保証対象とする。

## 2. 真実源（Source of Truth）

- `docs/api-manual/02-request.md`
- `docs/api-manual/03-response.md`
- `docs/api-manual/appendix-message-codes.md`

## 3. 対象範囲

### 3.1 単体テストに含める

- パラメータバリデーション
- URL構築
- JSON/CSVレスポンスデコード
- エラーマッピング
- `next_position: Option<u32>` 正規化
- 再試行判定

### 3.2 単体テストに含めない

- 実API疎通の常時保証
- BOJ実データの正確性
- 外部レート制限挙動の保証

## 4. レイヤー配分

- 単体テスト: 75%
- コンポーネントテスト（モックTransport）: 20%
- 実API契約テスト（夜間）: 5%

## 5. CI運用

- PR/Push: `.github/workflows/ci.yml` でオフライン `cargo test`
- Nightly: `.github/workflows/nightly-contract.yml` で `--ignored` 契約テスト実行
- 契約テスト失敗時は、仕様変更か一時障害かを先に判定する

## 6. フィクスチャ運用

- `tests/fixtures/` を最小代表ケースで維持する
- 仕様の参照節を `tests/fixtures/README.md` に明記する
- 仕様変更時のみフィクスチャとスナップショットを更新する
