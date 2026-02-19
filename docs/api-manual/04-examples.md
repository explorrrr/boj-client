# IV. 利用例

## 1. パラメータと出力ファイル
> 出典: api_manual.pdf p.19-p.20

パラメータ設定例と出力ファイル例は以下の通りです。

原文注記:

- 下記出力ファイルは開発段階のもので、予告なく変更される場合があります。
- 本マニュアルの目的に即して示したものであり、データの正確性・完全性を保証するものではありません。
- 同様のパラメータ設定でも、操作時点により出力データが異なる場合があります。

### (1) コードAPI

- 例URL:

`https://www.stat-search.boj.or.jp/api/v1/getDataCode?format=json&lang=jp&db=CO&startDate=202401&endDate=202504&code=TK99F1000601GCQ01000,TK99F2000601GCQ01000`

- 解説:
- 結果ファイル形式: JSON
- 言語: 日本語
- DB名: CO（短観）
- 開始期: 2024年1Q（2024年1月〜3月）
- 終了期: 2025年4Q（2025年10月〜12月）
- 系列コード: `TK99F1000601GCQ01000`, `TK99F2000601GCQ01000`

注記（原本外）: 原本の出力ファイル画像は値を推測せず、URLと説明のみ記載しています。

### (2) 階層API

- 例URL:

`https://www.stat-search.boj.or.jp/api/v1/getDataLayer?format=csv&db=BP01&frequency=M&startDate=202504&endDate=202509&layer=1,1,1`

- 解説:
- 結果ファイル形式: CSV
- DB名: BP01（国際収支統計）
- 期種: M（月次）
- 開始期: 2025年4月
- 終了期: 2025年9月
- 階層情報: 階層1=1、階層2=1、階層3=1

注記（原本外）: 原本の出力ファイル画像は値を推測せず、URLと説明のみ記載しています。

### (3) メタデータAPI

- 例URL:

`https://www.stat-search.boj.or.jp/api/v1/getMetadata?format=csv&lang=jp&db=fm08`

- 解説:
- 結果ファイル形式: CSV
- 言語: 日本語
- DB名: FM08（外国為替市況）

注記（原本外）: 原本の出力ファイル画像は値を推測せず、URLと説明のみ記載しています。

## 2. パラメータ指定例
> 出典: api_manual.pdf p.21-p.22

### (1) コードAPI 例A

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | JSON |
| 言語 | 日本語 |
| DB名 | CO（短観） |
| 系列コード | `TK99F1000601GCQ01000`, `TK99F2000601GCQ01000` |
| 開始期 | 2024年1Q（2024年1月〜3月） |
| 終了期 | 2025年4Q（2025年10月〜12月） |
| 検索開始位置 | 指定しない |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getDataCode?format=json&lang=jp&db=CO&startDate=202401&endDate=202504&code=TK99F1000601GCQ01000,TK99F2000601GCQ01000` |

### (2) コードAPI 例B

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | 指定しない |
| 言語 | 指定しない |
| DB名 | FM01（無担保コールＯ／Ｎ物レート（毎営業日）） |
| 系列コード | `STRDCLUCON`, `STRDCLUCONH`, `STRDCLUCONL` |
| 開始期 | 2025年1月 |
| 終了期 | 指定しない |
| 検索開始位置 | 指定しない |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getDataCode?db=FM01&code=STRDCLUCON,STRDCLUCONH,STRDCLUCONL&startDate=202501` |

### (3) 階層API 例A

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | CSV |
| 言語 | 指定しない |
| DB名 | BP01（国際収支統計） |
| 期種 | M（月次） |
| 階層情報 | 階層1=1、階層2=1、階層3=1 |
| 開始期 | 2025年4月 |
| 終了期 | 2025年9月 |
| 検索開始位置 | 指定しない |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getDataLayer?format=csv&db=BP01&frequency=M&startDate=202504&endDate=202509&layer=1,1,1` |

### (4) 階層API 例B

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | 指定しない |
| 言語 | 英語 |
| DB名 | MD10（預金者別預金） |
| 期種 | Q（四半期） |
| 階層情報 | `*`（全てのデータ） |
| 開始期 | 指定しない |
| 終了期 | 指定しない |
| 検索開始位置 | 255番目 |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getDataLayer?lang=en&db=md10&frequency=q&layer=*&startPosition=255` |

### (5) メタデータAPI 例A

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | CSV |
| 言語 | 日本語 |
| DB名 | FM08（外国為替市況） |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getMetadata?format=csv&lang=jp&db=fm08` |

### (6) メタデータAPI 例B

| 設定項目 | 設定値 |
|---|---|
| 結果ファイル形式 | 指定しない |
| 言語 | 指定しない |
| DB名 | PR01（企業物価指数） |
| URL | `https://www.stat-search.boj.or.jp/api/v1/getMetadata?db=pr01` |
