use std::collections::{BTreeMap, HashMap};

use serde_json::{Map, Value};

use crate::error::BojError;
use crate::model::{CodeParameterEcho, LayerParameterEcho, ResponseMeta};

pub(crate) fn parse_json_text(bytes: &[u8]) -> Result<String, BojError> {
    std::str::from_utf8(bytes)
        .map(|value| value.to_string())
        .map_err(|error| BojError::decode(format!("invalid UTF-8 JSON payload: {error}")))
}

pub(crate) fn parse_meta_from_json_object(
    root: &Map<String, Value>,
) -> Result<ResponseMeta, BojError> {
    let status = parse_status_value(
        get_ci_value(root, "STATUS").ok_or_else(|| BojError::decode("STATUS not found"))?,
    )?;

    let message_id = get_ci_string(root, "MESSAGEID").unwrap_or_default();
    let message = get_ci_string(root, "MESSAGE").unwrap_or_default();
    let date = get_ci_string(root, "DATE").and_then(|value| normalize_optional(&value));

    Ok(ResponseMeta {
        status,
        message_id,
        message,
        date,
    })
}

pub(crate) fn parse_next_position_from_json(
    root: &Map<String, Value>,
) -> Result<Option<u32>, BojError> {
    let value = get_ci_value(root, "NEXTPOSITION");
    parse_optional_u32_from_value(value, "NEXTPOSITION")
}

pub(crate) fn value_to_string_map(
    value: Option<&Value>,
    field: &str,
) -> Result<BTreeMap<String, String>, BojError> {
    let Some(value) = value else {
        return Ok(BTreeMap::new());
    };

    let object = value
        .as_object()
        .ok_or_else(|| BojError::decode(format!("{field} must be an object")))?;

    let mut map = BTreeMap::new();
    for (key, value) in object {
        if let Some(text) = value_to_scalar_string(value)? {
            map.insert(key.to_ascii_uppercase(), text);
        }
    }

    Ok(map)
}

pub(crate) fn collect_json_extras(
    row: &Map<String, Value>,
    known_keys: &[&str],
) -> Result<BTreeMap<String, Option<String>>, BojError> {
    let mut extras = BTreeMap::new();
    for (key, value) in row {
        if known_keys
            .iter()
            .any(|known| key.eq_ignore_ascii_case(known))
        {
            continue;
        }
        let value = value_to_scalar_string(value)?;
        extras.insert(key.clone(), value);
    }
    Ok(extras)
}

pub(crate) fn parse_code_parameter_map(
    map: &BTreeMap<String, String>,
) -> Result<CodeParameterEcho, BojError> {
    let mut extras = BTreeMap::new();
    for (key, value) in map {
        if matches!(
            key.as_str(),
            "FORMAT" | "LANG" | "DB" | "STARTDATE" | "ENDDATE" | "STARTPOSITION"
        ) {
            continue;
        }
        if let Some(value) = normalize_optional(value) {
            extras.insert(key.clone(), value);
        }
    }

    Ok(CodeParameterEcho {
        format: map
            .get("FORMAT")
            .and_then(|value| normalize_optional(value)),
        lang: map.get("LANG").and_then(|value| normalize_optional(value)),
        db: map.get("DB").and_then(|value| normalize_optional(value)),
        start_date: map
            .get("STARTDATE")
            .and_then(|value| normalize_optional(value)),
        end_date: map
            .get("ENDDATE")
            .and_then(|value| normalize_optional(value)),
        start_position: parse_optional_u32_from_text(
            map.get("STARTPOSITION").map(String::as_str),
            "STARTPOSITION",
        )?,
        extras,
    })
}

pub(crate) fn parse_layer_parameter_map(
    map: &BTreeMap<String, String>,
) -> Result<LayerParameterEcho, BojError> {
    let mut extras = BTreeMap::new();
    for (key, value) in map {
        if matches!(
            key.as_str(),
            "FORMAT"
                | "LANG"
                | "DB"
                | "FREQUENCY"
                | "LAYER1"
                | "LAYER2"
                | "LAYER3"
                | "LAYER4"
                | "LAYER5"
                | "STARTDATE"
                | "ENDDATE"
                | "STARTPOSITION"
        ) {
            continue;
        }
        if let Some(value) = normalize_optional(value) {
            extras.insert(key.clone(), value);
        }
    }

    Ok(LayerParameterEcho {
        format: map
            .get("FORMAT")
            .and_then(|value| normalize_optional(value)),
        lang: map.get("LANG").and_then(|value| normalize_optional(value)),
        db: map.get("DB").and_then(|value| normalize_optional(value)),
        frequency: map
            .get("FREQUENCY")
            .and_then(|value| normalize_optional(value)),
        layer1: parse_optional_u32_from_text(map.get("LAYER1").map(String::as_str), "LAYER1")?,
        layer2: parse_optional_u32_from_text(map.get("LAYER2").map(String::as_str), "LAYER2")?,
        layer3: parse_optional_u32_from_text(map.get("LAYER3").map(String::as_str), "LAYER3")?,
        layer4: parse_optional_u32_from_text(map.get("LAYER4").map(String::as_str), "LAYER4")?,
        layer5: parse_optional_u32_from_text(map.get("LAYER5").map(String::as_str), "LAYER5")?,
        start_date: map
            .get("STARTDATE")
            .and_then(|value| normalize_optional(value)),
        end_date: map
            .get("ENDDATE")
            .and_then(|value| normalize_optional(value)),
        start_position: parse_optional_u32_from_text(
            map.get("STARTPOSITION").map(String::as_str),
            "STARTPOSITION",
        )?,
        extras,
    })
}

pub(crate) fn parse_meta_from_csv_map(
    meta: &HashMap<String, String>,
) -> Result<ResponseMeta, BojError> {
    let status = meta
        .get("STATUS")
        .ok_or_else(|| BojError::decode("STATUS not found in CSV response"))
        .and_then(|value| {
            value.parse::<u16>().map_err(|error| {
                BojError::decode(format!("STATUS is not a valid integer: {error}"))
            })
        })?;

    let message_id = meta.get("MESSAGEID").cloned().unwrap_or_default();
    let message = meta.get("MESSAGE").cloned().unwrap_or_default();
    let date = meta.get("DATE").and_then(|value| normalize_optional(value));

    Ok(ResponseMeta {
        status,
        message_id,
        message,
        date,
    })
}

pub(crate) fn parse_status_value(value: &Value) -> Result<u16, BojError> {
    let text = value_to_scalar_string(value)?
        .ok_or_else(|| BojError::decode("STATUS must be a string or number"))?;

    text.parse::<u16>()
        .map_err(|error| BojError::decode(format!("STATUS is not a valid integer: {error}")))
}

pub(crate) fn parse_optional_u32_from_value(
    value: Option<&Value>,
    field: &str,
) -> Result<Option<u32>, BojError> {
    let Some(value) = value else {
        return Ok(None);
    };

    let value = value_to_scalar_string(value)?;
    parse_optional_u32_from_text(value.as_deref(), field)
}

pub(crate) fn parse_optional_u32_from_text(
    value: Option<&str>,
    field: &str,
) -> Result<Option<u32>, BojError> {
    let Some(value) = value else {
        return Ok(None);
    };

    let Some(value) = normalize_optional(value) else {
        return Ok(None);
    };
    if value.eq_ignore_ascii_case("null") {
        return Ok(None);
    }

    value
        .parse::<u32>()
        .map(Some)
        .map_err(|error| BojError::decode(format!("{field} is not a valid integer: {error}")))
}

pub(crate) fn parse_next_position_from_text(value: Option<&str>) -> Result<Option<u32>, BojError> {
    parse_optional_u32_from_text(value, "NEXTPOSITION")
}

pub(crate) fn required_non_empty_string(
    row: &Map<String, Value>,
    key: &str,
) -> Result<String, BojError> {
    let Some(value) = get_ci_value(row, key) else {
        return Err(BojError::decode(format!("{key} is required")));
    };

    let value = value_to_scalar_string(value)?
        .and_then(|value| normalize_optional(&value))
        .ok_or_else(|| BojError::decode(format!("{key} must not be empty")))?;

    Ok(value)
}

pub(crate) fn get_ci_value<'a>(map: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
    map.iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
        .map(|(_, value)| value)
}

pub(crate) fn get_ci_string(map: &Map<String, Value>, key: &str) -> Option<String> {
    let value = get_ci_value(map, key)?;
    value_to_scalar_string(value).ok().flatten()
}

pub(crate) fn value_to_scalar_string(value: &Value) -> Result<Option<String>, BojError> {
    match value {
        Value::Null => Ok(None),
        Value::String(value) => Ok(Some(value.clone())),
        Value::Number(value) => Ok(Some(value.to_string())),
        Value::Bool(value) => Ok(Some(value.to_string())),
        Value::Array(_) | Value::Object(_) => Err(BojError::decode(
            "nested array/object is not allowed in scalar fields",
        )),
    }
}

pub(crate) fn normalize_optional(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub(crate) fn looks_like_json(body: &[u8]) -> bool {
    for byte in body {
        match *byte {
            b' ' | b'\n' | b'\t' | b'\r' => continue,
            b'{' | b'[' => return true,
            _ => return false,
        }
    }
    false
}
