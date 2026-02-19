use std::collections::HashMap;

use crate::error::BojError;
use crate::model::{CodeParameterEcho, CodeSeries, DataPoint};

use super::super::common::parse_code_parameter_map;
use super::payload::{csv_collect_extras, csv_optional_cell, csv_required_cell};

pub(crate) fn parse_code_parameter_from_csv(
    parameter: &std::collections::BTreeMap<String, String>,
) -> Result<CodeParameterEcho, BojError> {
    parse_code_parameter_map(parameter)
}

pub(crate) fn parse_code_series_from_csv_row(
    index_map: &HashMap<String, usize>,
    header: &[String],
    row: &[String],
    points: &[DataPoint],
) -> Result<CodeSeries, BojError> {
    let series_code = csv_required_cell(index_map, row, "SERIES_CODE")?;

    Ok(CodeSeries {
        series_code,
        name_of_time_series_j: csv_optional_cell(index_map, row, "NAME_OF_TIME_SERIES_J"),
        name_of_time_series: csv_optional_cell(index_map, row, "NAME_OF_TIME_SERIES"),
        unit_j: csv_optional_cell(index_map, row, "UNIT_J"),
        unit: csv_optional_cell(index_map, row, "UNIT"),
        frequency: csv_optional_cell(index_map, row, "FREQUENCY"),
        category_j: csv_optional_cell(index_map, row, "CATEGORY_J"),
        category: csv_optional_cell(index_map, row, "CATEGORY"),
        last_update: csv_optional_cell(index_map, row, "LAST_UPDATE"),
        points: points.to_vec(),
        extras: csv_collect_extras(
            header,
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
                "SURVEY_DATES",
                "VALUES",
            ],
        ),
    })
}
