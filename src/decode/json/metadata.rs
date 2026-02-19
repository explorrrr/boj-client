use serde_json::Value;

use crate::error::BojError;
use crate::model::MetadataEntry;

use super::super::common::{
    collect_json_extras, get_ci_string, get_ci_value, normalize_optional,
    parse_optional_u32_from_value,
};

pub(crate) fn parse_metadata_entry_from_json_row(value: &Value) -> Result<MetadataEntry, BojError> {
    let row = value
        .as_object()
        .ok_or_else(|| BojError::decode("each RESULTSET element must be an object"))?;

    Ok(MetadataEntry {
        series_code: get_ci_string(row, "SERIES_CODE").and_then(|value| normalize_optional(&value)),
        name_of_time_series_j: get_ci_string(row, "NAME_OF_TIME_SERIES_J")
            .and_then(|value| normalize_optional(&value)),
        name_of_time_series: get_ci_string(row, "NAME_OF_TIME_SERIES")
            .and_then(|value| normalize_optional(&value)),
        unit_j: get_ci_string(row, "UNIT_J").and_then(|value| normalize_optional(&value)),
        unit: get_ci_string(row, "UNIT").and_then(|value| normalize_optional(&value)),
        frequency: get_ci_string(row, "FREQUENCY").and_then(|value| normalize_optional(&value)),
        category_j: get_ci_string(row, "CATEGORY_J").and_then(|value| normalize_optional(&value)),
        category: get_ci_string(row, "CATEGORY").and_then(|value| normalize_optional(&value)),
        layer1: parse_optional_u32_from_value(get_ci_value(row, "LAYER1"), "LAYER1")?,
        layer2: parse_optional_u32_from_value(get_ci_value(row, "LAYER2"), "LAYER2")?,
        layer3: parse_optional_u32_from_value(get_ci_value(row, "LAYER3"), "LAYER3")?,
        layer4: parse_optional_u32_from_value(get_ci_value(row, "LAYER4"), "LAYER4")?,
        layer5: parse_optional_u32_from_value(get_ci_value(row, "LAYER5"), "LAYER5")?,
        start_of_the_time_series: get_ci_string(row, "START_OF_THE_TIME_SERIES")
            .and_then(|value| normalize_optional(&value)),
        end_of_the_time_series: get_ci_string(row, "END_OF_THE_TIME_SERIES")
            .and_then(|value| normalize_optional(&value)),
        last_update: get_ci_string(row, "LAST_UPDATE").and_then(|value| normalize_optional(&value)),
        notes_j: get_ci_string(row, "NOTES_J").and_then(|value| normalize_optional(&value)),
        notes: get_ci_string(row, "NOTES").and_then(|value| normalize_optional(&value)),
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
                "LAYER1",
                "LAYER2",
                "LAYER3",
                "LAYER4",
                "LAYER5",
                "START_OF_THE_TIME_SERIES",
                "END_OF_THE_TIME_SERIES",
                "LAST_UPDATE",
                "NOTES_J",
                "NOTES",
            ],
        )?,
    })
}
