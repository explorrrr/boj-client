mod code;
mod layer;
mod metadata;

use serde_json::Value;

use crate::error::BojError;
use crate::model::{CodeResponse, LayerResponse, MetadataResponse};

use super::common::{
    get_ci_string, get_ci_value, parse_json_text, parse_meta_from_json_object,
    parse_next_position_from_json,
};
use code::{parse_code_parameter_from_json, parse_code_series_from_json};
use layer::{parse_layer_parameter_from_json, parse_layer_series_from_json};
use metadata::parse_metadata_entry_from_json_row;

pub(crate) fn decode_code_json(bytes: &[u8]) -> Result<CodeResponse, BojError> {
    let text = parse_json_text(bytes)?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| BojError::decode(format!("invalid JSON payload: {error}")))?;

    let root = value
        .as_object()
        .ok_or_else(|| BojError::decode("top-level JSON object is required"))?;

    let meta = parse_meta_from_json_object(root)?;
    let parameter = parse_code_parameter_from_json(root)?;
    let next_position = parse_next_position_from_json(root)?;
    let series = parse_code_series_from_json(root)?;

    Ok(CodeResponse {
        meta,
        parameter,
        next_position,
        series,
        raw: text,
    })
}

pub(crate) fn decode_layer_json(bytes: &[u8]) -> Result<LayerResponse, BojError> {
    let text = parse_json_text(bytes)?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| BojError::decode(format!("invalid JSON payload: {error}")))?;

    let root = value
        .as_object()
        .ok_or_else(|| BojError::decode("top-level JSON object is required"))?;

    let meta = parse_meta_from_json_object(root)?;
    let parameter = parse_layer_parameter_from_json(root)?;
    let next_position = parse_next_position_from_json(root)?;
    let series = parse_layer_series_from_json(root)?;

    Ok(LayerResponse {
        meta,
        parameter,
        next_position,
        series,
        raw: text,
    })
}

pub(crate) fn decode_metadata_json(bytes: &[u8]) -> Result<MetadataResponse, BojError> {
    let text = parse_json_text(bytes)?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| BojError::decode(format!("invalid JSON payload: {error}")))?;

    let root = value
        .as_object()
        .ok_or_else(|| BojError::decode("top-level JSON object is required"))?;

    let meta = parse_meta_from_json_object(root)?;
    let db = get_ci_string(root, "DB").unwrap_or_default();

    let entries = match get_ci_value(root, "RESULTSET") {
        Some(Value::Array(values)) => values
            .iter()
            .map(parse_metadata_entry_from_json_row)
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(BojError::decode("RESULTSET must be an array")),
        None => Vec::new(),
    };

    Ok(MetadataResponse {
        meta,
        db,
        entries,
        raw: text,
    })
}
