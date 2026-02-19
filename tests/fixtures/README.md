# テストフィクスチャ

本ディレクトリのフィクスチャは、BOJ API仕様の次節を参照して作成しています。

## 現行レスポンス形状（サポート対象）

- `json_success_code_api.json`: コードAPI JSON 成功（現行 `RESULTSET` + `VALUES.SURVEY_DATES/VALUES` 形状）
- `json_success_no_data.json`: コードAPI JSON 成功（データなし）
- `json_success_metadata_api.json`: メタデータAPI JSON 成功
- `json_error_400_invalid_db.json`: JSON エラー応答（400）
- `json_error_500_internal.json`: JSON エラー応答（500）
- `csv_success_en_utf8.csv`: コード/階層API CSV 成功（EN=UTF-8）
- `csv_success_jp_utf8_no_bom.csv`: コード/階層API CSV 成功（JP=UTF-8, BOMなし）
- `csv_success_jp_utf8_bom.csv`: コード/階層API CSV 成功（JP=UTF-8, BOMあり）
- `csv_success_jp_shiftjis.csv`: コード/階層API CSV 成功（JP=Shift-JIS）
- `csv_success_metadata_en_utf8.csv`: メタデータAPI CSV 成功（EN=UTF-8）
- `csv_error_json_payload.json`: CSV指定時に返るJSONエラー

## 旧レスポンス形状（非サポート検証用）

- `json_legacy_code_api.json`: 旧 `STATISTICS_DATA.RESULT/DATA_INF.DATA_OBJ` 形状
- `csv_legacy_code_api.csv`: 旧 `STATUS,MESSAGEID,...` ヘッダ行形状
