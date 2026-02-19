# III. レスポンスについて

## 1. 出力ファイル形式
> 出典: api_manual.pdf p.12

各APIは `JSON` 形式または `CSV` 形式を出力します。URL設定内容に不備がある場合は、CSV指定時でもJSON形式でエラー出力されます。

| 出力形式 | レスポンス形式 |
|---|---|
| JSON | `Content-Type: application/json` |
| CSV | `Content-Type: text/csv` |

## 2. 出力ファイルの構成
> 出典: api_manual.pdf p.12

各APIの出力構成は以下です（タグ名はAPI毎に異なる）。

- コードAPI/階層API
1. APIの処理結果情報
2. APIのパラメータ情報
3. 次回検索開始位置（`NEXTPOSITION`）
4. APIの出力部

- メタデータAPI
1. APIの処理結果情報
2. APIのパラメータ情報
3. APIの出力部

## 3. 出力データの表示ルール
> 出典: api_manual.pdf p.12-p.13

### (1) 正常終了の場合

- JSON形式の文字コードは UTF-8。
- CSV形式は、日本語出力時 Shift-JIS、英語出力時 UTF-8。
- コードAPI/階層APIでは、`NEXTPOSITION` に数値がある場合は上限超過により一部データ未取得。

### (2) エラーの場合

- パラメータ誤りやアクセス時エラー発生時は、エラー内容のみ出力。
- 指定したパラメータ情報およびデータは出力されない。
- システム上で言語特定前にエラーとなった場合は英語メッセージのみ表示。
- 詳細メッセージは [付録B: メッセージコード一覧](./appendix-message-codes.md) を参照。

## 4. 出力データの解説
> 出典: api_manual.pdf p.14-p.17

### (1) APIの処理結果情報（全API共通）

| タグ名 | 出力内容 | 補足 |
|---|---|---|
| STATUS | API処理結果をステータスコードで出力。`200` 正常、`200` 以外エラー。`400`: パラメータ誤り、`500`: 予期せぬエラー、`503`: DBアクセスエラー。 | `400`/`500`/`503` は指定形式に関わらずJSONで出力。 |
| MESSAGEID | API処理結果のメッセージID。 | メッセージ詳細は本章「5. メッセージの解説」および [付録B](./appendix-message-codes.md) を参照。 |
| MESSAGE | MESSAGEIDに対応するメッセージ。 | - |
| DATE | コードAPI/階層API: 出力ファイル作成日時。メタデータAPI: システム内部のデータ作成日時（出力作成日時ではない）。 | 日付・時刻は日本時間。 |

### (2) APIのパラメータ情報（コードAPI、階層API）

| タグ名 | 出力内容 | 補足 |
|---|---|---|
| FORMAT | 結果ファイル形式（JSON, CSV） | 未指定時はJSONで作成 |
| LANG | 言語（JP:日本語、EN:英語） | 未指定時は日本語で作成 |
| DB | DB名 | DB名一覧は [付録A](./appendix-db-list.md) |
| LAYER1, LAYER2, LAYER3, LAYER4, LAYER5 | 階層情報 | 階層APIのみ |
| FREQUENCY | 期種（`CY` `CH` `FY` `FH` `Q` `M` `W` `D`） | 階層APIのみ |
| STARTDATE | 収録開始期 | 形式: `YYYY`/`YYYYHH`/`YYYYQQ`/`YYYYMM` |
| ENDDATE | 収録終了期 | 形式はSTARTDATEと同様 |
| STARTPOSITION | 検索開始位置 | 「II.4.(2) 検索開始位置」を参照 |

STARTDATE/ENDDATE指定例:

- 2025年度上期: `202501`
- 2025年第2四半期: `202502`
- 2025年12月: `202512`

### (3) 次回検索開始位置（コードAPI、階層API）

| タグ名 | 出力内容 | 補足 |
|---|---|---|
| NEXTPOSITION | 次回検索開始位置 | 上限超過検索時に出力。未出力時は JSON:`null`、CSV:ブランク。 |

### (4) DB名（メタデータAPI）

| タグ名 | 出力内容 | 補足 |
|---|---|---|
| DB | DB名 | DB名一覧は [付録A](./appendix-db-list.md) |

### (5) APIの出力部（コードAPI、階層API）

| タグ名 | 出力内容 | 日本語 | 英語 |
|---|---|---|---|
| SERIES_CODE | 系列コード（先頭にDB名は付かない） | ✓ | ✓ |
| NAME_OF_TIME_SERIES_J | 系列名称（日本語） | ✓ |  |
| NAME_OF_TIME_SERIES | 系列名称（英語） |  | ✓ |
| UNIT_J | 単位（日本語） | ✓ |  |
| UNIT | 単位（英語） |  | ✓ |
| FREQUENCY | 期種（`ANNUAL` `ANNUAL(MAR)` `SEMIANNUAL` `SEMIANNUAL(SEP)` `QUARTERLY` `MONTHLY` `WEEKLY(MONDAY)` `DAILY`） | ✓ | ✓ |
| CATEGORY_J | 統計種別・カテゴリ（日本語） | ✓ |  |
| CATEGORY | 統計種別・カテゴリ（英語） |  | ✓ |
| LAST_UPDATE | 最終更新日（`YYYYMMDD`）。系列メタ情報またはデータ更新時に更新。CSVはデータ系列毎に同値。 | ✓ | ✓ |
| SURVEY_DATES | 時期（暦年/年度:`YYYY`、暦年半期/年度半期:`YYYYHH`、四半期:`YYYYQQ`、月次:`YYYYMM`、週次/日次:`YYYYMMDD`） | ✓ | ✓ |
| VALUES | 値。欠損値は `null`（検索画面では `NA` または `ND`） | ✓ | ✓ |

### (6) APIの出力部（メタデータAPI）

| タグ名 | 出力内容 | 日本語 | 英語 |
|---|---|---|---|
| SERIES_CODE | 系列コード（先頭にDB名は付かない。階層情報のみ出力時はブランク） | ✓ | ✓ |
| NAME_OF_TIME_SERIES_J | 系列名称（日本語） | ✓ |  |
| NAME_OF_TIME_SERIES | 系列名称（英語） | ✓ | ✓ |
| UNIT_J | 単位（日本語） | ✓ |  |
| UNIT | 単位（英語） | ✓ | ✓ |
| FREQUENCY | 期種（正式名称）。パラメータ指定時は略称文字を使うこと。正式名称を指定するとエラー。 | ✓ | ✓ |
| CATEGORY_J | 統計種別・カテゴリ（日本語） | ✓ |  |
| CATEGORY | 統計種別・カテゴリ（英語） | ✓ | ✓ |
| LAYER1 | 階層1 | ✓ | ✓ |
| LAYER2 | 階層2 | ✓ | ✓ |
| LAYER3 | 階層3 | ✓ | ✓ |
| LAYER4 | 階層4 | ✓ | ✓ |
| LAYER5 | 階層5 | ✓ | ✓ |
| START_OF_THE_TIME_SERIES | 収録開始期（暦年/年度:`YYYY`、暦年半期/年度半期:`YYYYHH`、四半期:`YYYYQQ`、月次:`YYYYMM`、週次/日次:`YYYYMMDD`） | ✓ | ✓ |
| END_OF_THE_TIME_SERIES | 収録終了期 | ✓ | ✓ |
| LAST_UPDATE | 最終更新日（`YYYYMMDD`） | ✓ | ✓ |
| NOTES_J | 備考（日本語） | ✓ |  |
| NOTES | 備考（英語） | ✓ | ✓ |

## 5. メッセージの解説
> 出典: api_manual.pdf p.17-p.18

`STATUS=200` は正常終了、`STATUS!=200` はエラーです。`STATUS=400` はパラメータ設定誤りを示します。

- メッセージ一覧（`M181000I` 〜 `M181091S`）は [付録B: メッセージコード一覧](./appendix-message-codes.md) を参照してください。
- `STATUS=400` の場合はパラメータ修正後に再実行してください（設定方法は `02-request.md` を参照）。
