use std::collections::{BTreeMap, HashMap};

use crate::error::BojError;
use crate::model::DataPoint;

use super::super::common::{normalize_optional, parse_optional_u32_from_text};

pub(crate) struct CsvPayload {
    pub(crate) meta: HashMap<String, String>,
    pub(crate) parameter: BTreeMap<String, String>,
    pub(crate) next_position: Option<String>,
    pub(crate) db: Option<String>,
    pub(crate) data_header: Vec<String>,
    pub(crate) data_rows: Vec<Vec<String>>,
}

pub(crate) type CsvSeriesRowParser<T> =
    fn(&HashMap<String, usize>, &[String], &[String], &[DataPoint]) -> Result<T, BojError>;

pub(crate) fn parse_csv_payload(rows: &[Vec<String>]) -> CsvPayload {
    let mut meta = HashMap::new();
    let mut parameter = BTreeMap::new();
    let mut next_position = None;
    let mut db = None;
    let mut data_header = Vec::new();
    let mut data_rows = Vec::new();

    let mut index = 0usize;
    while index < rows.len() {
        let row = &rows[index];
        if row.is_empty() || row.iter().all(|cell| cell.trim().is_empty()) {
            index += 1;
            continue;
        }

        let key = row[0].trim().to_ascii_uppercase();
        match key.as_str() {
            "STATUS" | "MESSAGEID" | "MESSAGE" | "DATE" => {
                meta.insert(key, row.get(1).cloned().unwrap_or_default());
            }
            "PARAMETER" => {
                let param_name = row
                    .get(1)
                    .map(|value| value.trim().to_ascii_uppercase())
                    .unwrap_or_default();
                let param_value = row.get(2).cloned().unwrap_or_default();
                if !param_name.is_empty() {
                    parameter.insert(param_name, param_value);
                }
            }
            "NEXTPOSITION" => {
                next_position = Some(row.get(1).cloned().unwrap_or_default());
            }
            "DB" => {
                db = row.get(1).cloned();
            }
            "SERIES_CODE" => {
                data_header = row.clone();
                for candidate in rows.iter().skip(index + 1) {
                    if candidate.is_empty() || candidate.iter().all(|cell| cell.trim().is_empty()) {
                        continue;
                    }
                    data_rows.push(candidate.clone());
                }
                break;
            }
            _ => {}
        }

        index += 1;
    }

    CsvPayload {
        meta,
        parameter,
        next_position,
        db,
        data_header,
        data_rows,
    }
}

pub(crate) fn parse_series_from_csv_rows<T>(
    payload: &CsvPayload,
    row_parser: CsvSeriesRowParser<T>,
) -> Result<Vec<T>, BojError> {
    if payload.data_header.is_empty() {
        return Ok(Vec::new());
    }

    let index_map = csv_header_index_map(&payload.data_header);
    let series_index = find_header_index(&index_map, "SERIES_CODE")
        .ok_or_else(|| BojError::decode("SERIES_CODE column is required in CSV data"))?;
    let survey_index = find_header_index(&index_map, "SURVEY_DATES")
        .ok_or_else(|| BojError::decode("SURVEY_DATES column is required in CSV data"))?;
    let value_index = find_header_index(&index_map, "VALUES")
        .ok_or_else(|| BojError::decode("VALUES column is required in CSV data"))?;

    struct GroupedRow {
        first_row: Vec<String>,
        points: Vec<DataPoint>,
    }

    let mut groups = Vec::<GroupedRow>::new();
    let mut positions = HashMap::<String, usize>::new();

    for row in &payload.data_rows {
        let series_code = csv_row_cell(row, series_index);
        if series_code.trim().is_empty() {
            return Err(BojError::decode("SERIES_CODE must not be empty"));
        }

        let survey_date = csv_row_cell(row, survey_index);
        let survey_date = normalize_optional(&survey_date)
            .ok_or_else(|| BojError::decode("SURVEY_DATES must not be empty"))?;

        let value = normalize_optional(&csv_row_cell(row, value_index));
        let key = csv_grouping_key(row, payload.data_header.len(), survey_index, value_index);

        if let Some(position) = positions.get(&key) {
            groups[*position]
                .points
                .push(DataPoint { survey_date, value });
        } else {
            let position = groups.len();
            groups.push(GroupedRow {
                first_row: row.clone(),
                points: vec![DataPoint { survey_date, value }],
            });
            positions.insert(key, position);
        }
    }

    groups
        .iter()
        .map(|group| {
            row_parser(
                &index_map,
                &payload.data_header,
                &group.first_row,
                &group.points,
            )
        })
        .collect()
}

pub(crate) fn csv_header_index_map(header: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (index, name) in header.iter().enumerate() {
        map.insert(name.trim().to_ascii_uppercase(), index);
    }
    map
}

fn find_header_index(index_map: &HashMap<String, usize>, name: &str) -> Option<usize> {
    index_map.get(&name.to_ascii_uppercase()).copied()
}

fn csv_row_cell(row: &[String], index: usize) -> String {
    row.get(index).cloned().unwrap_or_default()
}

fn csv_grouping_key(
    row: &[String],
    column_count: usize,
    survey_index: usize,
    value_index: usize,
) -> String {
    let mut parts = Vec::with_capacity(column_count.saturating_sub(2));
    for index in 0..column_count {
        if index == survey_index || index == value_index {
            continue;
        }
        parts.push(csv_row_cell(row, index));
    }
    parts.join("\u{1f}")
}

pub(crate) fn csv_optional_cell(
    index_map: &HashMap<String, usize>,
    row: &[String],
    column: &str,
) -> Option<String> {
    let index = find_header_index(index_map, column)?;

    normalize_optional(&csv_row_cell(row, index))
}

pub(crate) fn csv_required_cell(
    index_map: &HashMap<String, usize>,
    row: &[String],
    column: &str,
) -> Result<String, BojError> {
    csv_optional_cell(index_map, row, column)
        .ok_or_else(|| BojError::decode(format!("{column} must not be empty")))
}

pub(crate) fn csv_optional_u32_cell(
    index_map: &HashMap<String, usize>,
    row: &[String],
    column: &str,
) -> Result<Option<u32>, BojError> {
    let value = csv_optional_cell(index_map, row, column);
    parse_optional_u32_from_text(value.as_deref(), column)
}

pub(crate) fn csv_collect_extras(
    header: &[String],
    row: &[String],
    known_columns: &[&str],
) -> BTreeMap<String, Option<String>> {
    let mut extras = BTreeMap::new();

    for (index, name) in header.iter().enumerate() {
        if known_columns
            .iter()
            .any(|known| name.eq_ignore_ascii_case(known))
        {
            continue;
        }
        extras.insert(name.clone(), normalize_optional(&csv_row_cell(row, index)));
    }

    extras
}
