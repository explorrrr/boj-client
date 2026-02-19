mod common;
mod csv;
mod json;

use crate::error::BojError;
use crate::model::{CodeResponse, LayerResponse, MetadataResponse};
use crate::query::CsvEncoding;

use common::looks_like_json;
use csv::{decode_code_csv, decode_layer_csv, decode_metadata_csv};
use json::{decode_code_json, decode_layer_json, decode_metadata_json};

/// Decodes a `getDataCode` response payload.
///
/// Decoder selection order:
/// 1. If payload body looks like JSON, decode as JSON.
/// 2. Otherwise, if `content_type` contains `json` or `csv`, follow it.
/// 3. Otherwise, fall back between JSON and CSV decoders.
///
/// `csv_encoding_hint` is used only when CSV decoding is attempted.
///
/// # Examples
///
/// ```ignore
/// use boj_client::decode::decode_code;
/// use boj_client::query::CsvEncoding;
///
/// let body = br#"{
///   "STATUS": 200,
///   "MESSAGEID": "M181000I",
///   "MESSAGE": "OK",
///   "PARAMETER": { "DB": "CO" },
///   "RESULTSET": [
///     {
///       "SERIES_CODE": "TK99F1000601GCQ01000",
///       "VALUES": {
///         "SURVEY_DATES": ["202401"],
///         "VALUES": ["1.23"]
///       }
///     }
///   ]
/// }"#;
///
/// let decoded = decode_code(body, Some("application/json"), CsvEncoding::Utf8)?;
/// assert_eq!(decoded.meta.status, 200);
/// assert_eq!(decoded.series.len(), 1);
/// # Ok::<(), boj_client::error::BojError>(())
/// ```ignore
///
/// # Errors
///
/// Returns [`BojError`] when no decoding path can parse the payload.
pub(crate) fn decode_code(
    body: &[u8],
    content_type: Option<&str>,
    csv_encoding_hint: CsvEncoding,
) -> Result<CodeResponse, BojError> {
    if looks_like_json(body) {
        return decode_code_json(body);
    }

    if let Some(content_type) = content_type {
        let content_type = content_type.to_ascii_lowercase();
        if content_type.contains("json") {
            return decode_code_json(body);
        }
        if content_type.contains("csv") {
            return decode_code_csv(body, csv_encoding_hint).or_else(|_| decode_code_json(body));
        }
    }

    decode_code_json(body).or_else(|_| decode_code_csv(body, csv_encoding_hint))
}

/// Decodes a `getDataLayer` response payload.
///
/// Decoder selection order:
/// 1. If payload body looks like JSON, decode as JSON.
/// 2. Otherwise, if `content_type` contains `json` or `csv`, follow it.
/// 3. Otherwise, fall back between JSON and CSV decoders.
///
/// `csv_encoding_hint` is used only when CSV decoding is attempted.
///
/// # Examples
///
/// ```ignore
/// use boj_client::decode::decode_layer;
/// use boj_client::query::CsvEncoding;
///
/// let body = br#"{
///   "STATUS": 200,
///   "MESSAGEID": "M181000I",
///   "MESSAGE": "OK",
///   "PARAMETER": { "DB": "BP01", "FREQUENCY": "Q" },
///   "RESULTSET": [
///     {
///       "SERIES_CODE": "TK99F1000601GCQ01000",
///       "VALUES": {
///         "SURVEY_DATES": ["202401"],
///         "VALUES": ["2.34"]
///       }
///     }
///   ]
/// }"#;
///
/// let decoded = decode_layer(body, Some("application/json"), CsvEncoding::Utf8)?;
/// assert_eq!(decoded.meta.status, 200);
/// assert_eq!(decoded.series.len(), 1);
/// # Ok::<(), boj_client::error::BojError>(())
/// ```ignore
///
/// # Errors
///
/// Returns [`BojError`] when no decoding path can parse the payload.
pub(crate) fn decode_layer(
    body: &[u8],
    content_type: Option<&str>,
    csv_encoding_hint: CsvEncoding,
) -> Result<LayerResponse, BojError> {
    if looks_like_json(body) {
        return decode_layer_json(body);
    }

    if let Some(content_type) = content_type {
        let content_type = content_type.to_ascii_lowercase();
        if content_type.contains("json") {
            return decode_layer_json(body);
        }
        if content_type.contains("csv") {
            return decode_layer_csv(body, csv_encoding_hint).or_else(|_| decode_layer_json(body));
        }
    }

    decode_layer_json(body).or_else(|_| decode_layer_csv(body, csv_encoding_hint))
}

/// Decodes a `getMetadata` response payload.
///
/// Decoder selection order:
/// 1. If payload body looks like JSON, decode as JSON.
/// 2. Otherwise, if `content_type` contains `json` or `csv`, follow it.
/// 3. Otherwise, fall back between JSON and CSV decoders.
///
/// `csv_encoding_hint` is used only when CSV decoding is attempted.
///
/// # Examples
///
/// ```ignore
/// use boj_client::decode::decode_metadata;
/// use boj_client::query::CsvEncoding;
///
/// let body = br#"{
///   "STATUS": 200,
///   "MESSAGEID": "M181000I",
///   "MESSAGE": "OK",
///   "DB": "ME",
///   "RESULTSET": []
/// }"#;
///
/// let decoded = decode_metadata(body, Some("application/json"), CsvEncoding::Utf8)?;
/// assert_eq!(decoded.meta.status, 200);
/// assert_eq!(decoded.db, "ME");
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
///
/// # Errors
///
/// Returns [`BojError`] when no decoding path can parse the payload.
pub(crate) fn decode_metadata(
    body: &[u8],
    content_type: Option<&str>,
    csv_encoding_hint: CsvEncoding,
) -> Result<MetadataResponse, BojError> {
    if looks_like_json(body) {
        return decode_metadata_json(body);
    }

    if let Some(content_type) = content_type {
        let content_type = content_type.to_ascii_lowercase();
        if content_type.contains("json") {
            return decode_metadata_json(body);
        }
        if content_type.contains("csv") {
            return decode_metadata_csv(body, csv_encoding_hint)
                .or_else(|_| decode_metadata_json(body));
        }
    }

    decode_metadata_json(body).or_else(|_| decode_metadata_csv(body, csv_encoding_hint))
}

#[cfg(test)]
mod tests {
    use super::common::parse_next_position_from_text;

    #[test]
    fn parse_next_position_handles_blank_and_null() {
        assert_eq!(parse_next_position_from_text(Some("")).unwrap(), None);
        assert_eq!(parse_next_position_from_text(None).unwrap(), None);
        assert_eq!(
            parse_next_position_from_text(Some("250")).unwrap(),
            Some(250)
        );
    }
}
