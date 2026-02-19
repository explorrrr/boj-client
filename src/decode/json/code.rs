use serde_json::{Map, Value};

use crate::error::BojError;
use crate::model::{CodeParameterEcho, CodeSeries, DataPoint};

use super::super::common::{
    collect_json_extras, get_ci_string, get_ci_value, normalize_optional, parse_code_parameter_map,
    required_non_empty_string, value_to_scalar_string, value_to_string_map,
};

pub(crate) fn parse_code_parameter_from_json(
    root: &Map<String, Value>,
) -> Result<CodeParameterEcho, BojError> {
    let parameter = get_ci_value(root, "PARAMETER");
    parse_code_parameter_from_json_value(parameter)
}

fn parse_code_parameter_from_json_value(
    parameter: Option<&Value>,
) -> Result<CodeParameterEcho, BojError> {
    let map = value_to_string_map(parameter, "PARAMETER")?;
    parse_code_parameter_map(&map)
}

pub(crate) fn parse_code_series_from_json(
    root: &Map<String, Value>,
) -> Result<Vec<CodeSeries>, BojError> {
    let rows = match get_ci_value(root, "RESULTSET") {
        Some(Value::Array(rows)) => rows,
        Some(_) => return Err(BojError::decode("RESULTSET must be an array")),
        None => return Ok(Vec::new()),
    };

    rows.iter().map(parse_code_series_from_json_row).collect()
}

fn parse_code_series_from_json_row(value: &Value) -> Result<CodeSeries, BojError> {
    let row = value
        .as_object()
        .ok_or_else(|| BojError::decode("each RESULTSET element must be an object"))?;

    let series_code = required_non_empty_string(row, "SERIES_CODE")?;
    let points = parse_points_from_json_row(row)?;

    Ok(CodeSeries {
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

pub(crate) fn parse_points_from_json_row(
    row: &Map<String, Value>,
) -> Result<Vec<DataPoint>, BojError> {
    let values = get_ci_value(row, "VALUES").ok_or_else(|| {
        BojError::decode("VALUES object is required in RESULTSET rows for code/layer API")
    })?;

    let values = values
        .as_object()
        .ok_or_else(|| BojError::decode("VALUES must be an object"))?;

    let survey_dates = get_ci_value(values, "SURVEY_DATES")
        .ok_or_else(|| BojError::decode("VALUES.SURVEY_DATES is required"))?;
    let survey_dates = survey_dates
        .as_array()
        .ok_or_else(|| BojError::decode("VALUES.SURVEY_DATES must be an array"))?;

    let data_values = get_ci_value(values, "VALUES")
        .ok_or_else(|| BojError::decode("VALUES.VALUES is required"))?;
    let data_values = data_values
        .as_array()
        .ok_or_else(|| BojError::decode("VALUES.VALUES must be an array"))?;

    if survey_dates.len() != data_values.len() {
        return Err(BojError::decode(
            "VALUES.SURVEY_DATES and VALUES.VALUES length mismatch",
        ));
    }

    let mut points = Vec::with_capacity(survey_dates.len());
    for index in 0..survey_dates.len() {
        let survey_date = value_to_scalar_string(&survey_dates[index])?
            .ok_or_else(|| BojError::decode("survey date must be string/number and not null"))?;

        let value = value_to_scalar_string(&data_values[index])?;
        points.push(DataPoint { survey_date, value });
    }

    Ok(points)
}
