mod code;
mod layer;
mod metadata;
mod payload;

use csv::ReaderBuilder;
use encoding_rs::SHIFT_JIS;

use crate::error::BojError;
use crate::model::{CodeResponse, LayerResponse, MetadataResponse};
use crate::query::CsvEncoding;

use super::common::{normalize_optional, parse_meta_from_csv_map, parse_next_position_from_text};
use code::{parse_code_parameter_from_csv, parse_code_series_from_csv_row};
use layer::{parse_layer_parameter_from_csv, parse_layer_series_from_csv_row};
use metadata::parse_metadata_entry_from_csv_row;
use payload::{csv_header_index_map, parse_csv_payload, parse_series_from_csv_rows};

pub(crate) fn decode_code_csv(
    bytes: &[u8],
    encoding: CsvEncoding,
) -> Result<CodeResponse, BojError> {
    let text = decode_csv_text(bytes, encoding)?;
    let rows = parse_csv_records(&text)?;
    let payload = parse_csv_payload(&rows);

    let meta = parse_meta_from_csv_map(&payload.meta)?;
    let parameter = parse_code_parameter_from_csv(&payload.parameter)?;
    let next_position = parse_next_position_from_text(payload.next_position.as_deref())?;
    let series = parse_series_from_csv_rows(&payload, parse_code_series_from_csv_row)?;

    Ok(CodeResponse {
        meta,
        parameter,
        next_position,
        series,
        raw: text,
    })
}

pub(crate) fn decode_layer_csv(
    bytes: &[u8],
    encoding: CsvEncoding,
) -> Result<LayerResponse, BojError> {
    let text = decode_csv_text(bytes, encoding)?;
    let rows = parse_csv_records(&text)?;
    let payload = parse_csv_payload(&rows);

    let meta = parse_meta_from_csv_map(&payload.meta)?;
    let parameter = parse_layer_parameter_from_csv(&payload.parameter)?;
    let next_position = parse_next_position_from_text(payload.next_position.as_deref())?;
    let series = parse_series_from_csv_rows(&payload, parse_layer_series_from_csv_row)?;

    Ok(LayerResponse {
        meta,
        parameter,
        next_position,
        series,
        raw: text,
    })
}

pub(crate) fn decode_metadata_csv(
    bytes: &[u8],
    encoding: CsvEncoding,
) -> Result<MetadataResponse, BojError> {
    let text = decode_csv_text(bytes, encoding)?;
    let rows = parse_csv_records(&text)?;
    let payload = parse_csv_payload(&rows);

    let meta = parse_meta_from_csv_map(&payload.meta)?;
    let db = payload
        .db
        .as_deref()
        .and_then(normalize_optional)
        .or_else(|| {
            payload
                .parameter
                .get("DB")
                .and_then(|value| normalize_optional(value))
        })
        .unwrap_or_default();

    let index_map = csv_header_index_map(&payload.data_header);
    let entries = payload
        .data_rows
        .iter()
        .map(|row| parse_metadata_entry_from_csv_row(&index_map, &payload.data_header, row))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MetadataResponse {
        meta,
        db,
        entries,
        raw: text,
    })
}

fn decode_csv_text(bytes: &[u8], encoding: CsvEncoding) -> Result<String, BojError> {
    match encoding {
        CsvEncoding::Utf8 => std::str::from_utf8(bytes)
            .map(|value| value.to_string())
            .map_err(|error| BojError::decode(format!("invalid UTF-8 CSV payload: {error}"))),
        CsvEncoding::ShiftJis => {
            let (decoded, _, had_errors) = SHIFT_JIS.decode(bytes);
            if had_errors {
                return Err(BojError::decode(
                    "Shift-JIS CSV payload contains invalid byte sequence",
                ));
            }
            Ok(decoded.into_owned())
        }
    }
}

fn parse_csv_records(text: &str) -> Result<Vec<Vec<String>>, BojError> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(text.as_bytes());

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|error| BojError::decode(error.to_string()))?;
        let mut row = Vec::new();
        for (index, value) in record.iter().enumerate() {
            if index == 0 {
                row.push(value.trim().trim_start_matches('\u{feff}').to_string());
            } else {
                row.push(value.trim().to_string());
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::decode_code_csv;
    use crate::query::CsvEncoding;

    #[test]
    fn csv_utf8_fails_on_shift_jis_bytes() {
        let shift_jis = vec![0x82, 0xb1, 0x82, 0xf1];
        let result = decode_code_csv(&shift_jis, CsvEncoding::Utf8);
        assert!(result.is_err());
    }
}
