use std::collections::HashMap;

use crate::error::BojError;
use crate::model::MetadataEntry;

use super::payload::{csv_collect_extras, csv_optional_cell, csv_optional_u32_cell};

pub(crate) fn parse_metadata_entry_from_csv_row(
    index_map: &HashMap<String, usize>,
    header: &[String],
    row: &[String],
) -> Result<MetadataEntry, BojError> {
    Ok(MetadataEntry {
        series_code: csv_optional_cell(index_map, row, "SERIES_CODE"),
        name_of_time_series_j: csv_optional_cell(index_map, row, "NAME_OF_TIME_SERIES_J"),
        name_of_time_series: csv_optional_cell(index_map, row, "NAME_OF_TIME_SERIES"),
        unit_j: csv_optional_cell(index_map, row, "UNIT_J"),
        unit: csv_optional_cell(index_map, row, "UNIT"),
        frequency: csv_optional_cell(index_map, row, "FREQUENCY"),
        category_j: csv_optional_cell(index_map, row, "CATEGORY_J"),
        category: csv_optional_cell(index_map, row, "CATEGORY"),
        layer1: csv_optional_u32_cell(index_map, row, "LAYER1")?,
        layer2: csv_optional_u32_cell(index_map, row, "LAYER2")?,
        layer3: csv_optional_u32_cell(index_map, row, "LAYER3")?,
        layer4: csv_optional_u32_cell(index_map, row, "LAYER4")?,
        layer5: csv_optional_u32_cell(index_map, row, "LAYER5")?,
        start_of_the_time_series: csv_optional_cell(index_map, row, "START_OF_THE_TIME_SERIES"),
        end_of_the_time_series: csv_optional_cell(index_map, row, "END_OF_THE_TIME_SERIES"),
        last_update: csv_optional_cell(index_map, row, "LAST_UPDATE"),
        notes_j: csv_optional_cell(index_map, row, "NOTES_J"),
        notes: csv_optional_cell(index_map, row, "NOTES"),
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
        ),
    })
}
