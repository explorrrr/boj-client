use serde_json::{Map, Value};

use crate::error::BojError;
use crate::model::{LayerParameterEcho, LayerSeries};

use super::super::common::{
    collect_json_extras, get_ci_string, get_ci_value, normalize_optional,
    parse_layer_parameter_map, required_non_empty_string, value_to_string_map,
};
use super::code::parse_points_from_json_row;

pub(crate) fn parse_layer_parameter_from_json(
    root: &Map<String, Value>,
) -> Result<LayerParameterEcho, BojError> {
    let parameter = get_ci_value(root, "PARAMETER");
    parse_layer_parameter_from_json_value(parameter)
}

fn parse_layer_parameter_from_json_value(
    parameter: Option<&Value>,
) -> Result<LayerParameterEcho, BojError> {
    let map = value_to_string_map(parameter, "PARAMETER")?;
    parse_layer_parameter_map(&map)
}

pub(crate) fn parse_layer_series_from_json(
    root: &Map<String, Value>,
) -> Result<Vec<LayerSeries>, BojError> {
    let rows = match get_ci_value(root, "RESULTSET") {
        Some(Value::Array(rows)) => rows,
        Some(_) => return Err(BojError::decode("RESULTSET must be an array")),
        None => return Ok(Vec::new()),
    };

    rows.iter().map(parse_layer_series_from_json_row).collect()
}

fn parse_layer_series_from_json_row(value: &Value) -> Result<LayerSeries, BojError> {
    let row = value
        .as_object()
        .ok_or_else(|| BojError::decode("each RESULTSET element must be an object"))?;

    let series_code = required_non_empty_string(row, "SERIES_CODE")?;
    let points = parse_points_from_json_row(row)?;

    Ok(LayerSeries {
        series_code,
        name_of_time_series_j: get_ci_string(row, "NAME_OF_TIME_SERIES_J")
            .and_then(|value| normalize_optional(&value)),
        name_of_time_series: get_ci_string(row, "NAME_OF_TIME_SERIES")
            .and_then(|value| normalize_optional(&value)),
        unit_j: get_ci_string(row, "UNIT_J").and_then(|value| normalize_optional(&value)),
        unit: get_ci_string(row, "UNIT").and_then(|value| normalize_optional(&value)),
        frequency: get_ci_string(row, "FREQUENCY").and_then(|value| normalize_optional(&value)),
        category_j: get_ci_string(row, "CATEGORY_J").and_then(|value| normalize_optional(&value)),
        category: get_ci_string(row, "CATEGORY").and_then(|value| normalize_optional(&value)),
        last_update: get_ci_string(row, "LAST_UPDATE").and_then(|value| normalize_optional(&value)),
        points,
        extras: collect_json_extras(
            row,
            &[
                "SERIES_CODE",
                "NAME_OF_TIME_SERIES_J",
                "NAME_OF_TIME_SERIES",
                "UNIT_J",
                "UNIT",
                "FREQUENCY",
                "CATEGORY_J",
                "CATEGORY",
                "LAST_UPDATE",
                "VALUES",
            ],
        )?,
    })
}
