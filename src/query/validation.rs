use crate::error::BojError;

use super::layer::LayerValue;
use super::options::Frequency;

const FORBIDDEN_ASCII_CHARS: [char; 8] = ['<', '>', '!', '|', '\\', ';', '\'', '"'];

pub(super) fn validate_db(value: &str) -> Result<(), BojError> {
    validate_ascii_parameter("DB", value)?;
    if value.contains(',') {
        return Err(BojError::validation("DB must not include comma"));
    }
    Ok(())
}

pub(super) fn validate_code(value: &str) -> Result<(), BojError> {
    validate_ascii_parameter("CODE", value)?;
    if value.contains(',') {
        return Err(BojError::validation(
            "CODE must be passed as separate items, not comma-containing strings",
        ));
    }
    Ok(())
}

pub(super) fn parse_layer_value(value: &str) -> Result<LayerValue, BojError> {
    validate_ascii_parameter("LAYER", value)?;
    if value == "*" {
        return Ok(LayerValue::Wildcard);
    }
    let number = value
        .parse::<u32>()
        .map_err(|_| BojError::validation("LAYER value must be '*' or a positive integer"))?;
    if number == 0 {
        return Err(BojError::validation(
            "LAYER value must be '*' or a positive integer",
        ));
    }
    Ok(LayerValue::Index(number))
}

fn validate_ascii_parameter(name: &str, value: &str) -> Result<(), BojError> {
    if value.trim().is_empty() {
        return Err(BojError::validation(format!("{name} is required")));
    }

    if !value.is_ascii() {
        return Err(BojError::validation(format!(
            "{name} must use ASCII characters only",
        )));
    }

    if value.chars().any(|ch| FORBIDDEN_ASCII_CHARS.contains(&ch)) {
        return Err(BojError::validation(format!(
            "{name} contains forbidden character",
        )));
    }

    Ok(())
}

pub(super) fn validate_date_generic(value: &str) -> Result<(), BojError> {
    if !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(BojError::validation("date must be numeric"));
    }

    match value.len() {
        4 => {
            let year = parse_year(value)?;
            validate_year_range(year)?;
            Ok(())
        }
        6 => {
            let year = parse_year(&value[0..4])?;
            validate_year_range(year)?;
            let suffix = parse_suffix(&value[4..6])?;
            if !(1..=12).contains(&suffix) {
                return Err(BojError::validation(
                    "date suffix must be between 01 and 12",
                ));
            }
            Ok(())
        }
        _ => Err(BojError::validation(
            "date format must be YYYY or YYYYXX (XX=01..12)",
        )),
    }
}

pub(super) fn validate_date_for_frequency(
    value: &str,
    frequency: Frequency,
) -> Result<(), BojError> {
    if !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(BojError::validation("date must be numeric"));
    }

    match frequency {
        Frequency::Cy | Frequency::Fy => {
            if value.len() != 4 {
                return Err(BojError::validation("date format for CY/FY must be YYYY"));
            }
            let year = parse_year(value)?;
            validate_year_range(year)?;
            Ok(())
        }
        Frequency::Ch | Frequency::Fh => validate_yyyyxx(value, 1, 2, "CH/FH"),
        Frequency::Q => validate_yyyyxx(value, 1, 4, "Q"),
        Frequency::M | Frequency::W | Frequency::D => validate_yyyyxx(value, 1, 12, "M/W/D"),
    }
}

fn validate_yyyyxx(value: &str, min: u32, max: u32, label: &str) -> Result<(), BojError> {
    if value.len() != 6 {
        return Err(BojError::validation(format!(
            "date format for {label} must be YYYYXX",
        )));
    }
    let year = parse_year(&value[0..4])?;
    validate_year_range(year)?;
    let suffix = parse_suffix(&value[4..6])?;
    if !(min..=max).contains(&suffix) {
        return Err(BojError::validation(format!(
            "date suffix for {label} must be between {min:02} and {max:02}",
        )));
    }
    Ok(())
}

pub(super) fn validate_date_order(start: &str, end: &str) -> Result<(), BojError> {
    if start.len() != end.len() {
        return Err(BojError::validation(
            "STARTDATE and ENDDATE formats must match",
        ));
    }
    if start > end {
        return Err(BojError::validation(
            "STARTDATE must be earlier than or equal to ENDDATE",
        ));
    }
    Ok(())
}

fn parse_year(value: &str) -> Result<u32, BojError> {
    value
        .parse::<u32>()
        .map_err(|_| BojError::validation("year must be numeric"))
}

fn parse_suffix(value: &str) -> Result<u32, BojError> {
    value
        .parse::<u32>()
        .map_err(|_| BojError::validation("date suffix must be numeric"))
}

fn validate_year_range(year: u32) -> Result<(), BojError> {
    if !(1850..=2050).contains(&year) {
        return Err(BojError::validation("year must be between 1850 and 2050"));
    }
    Ok(())
}
