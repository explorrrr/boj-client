mod code;
mod layer;
mod metadata;
mod options;
mod validation;

pub use code::CodeQuery;
pub use layer::LayerQuery;
pub use metadata::MetadataQuery;
pub(crate) use options::CsvEncoding;
pub use options::{Format, Frequency, Language};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BojError;

    #[test]
    fn code_query_rejects_forbidden_chars_and_full_width() {
        let query = CodeQuery::new("CO", vec!["IR01'MADR1Z@D".to_string()]);
        assert!(query.is_err());

        let query = CodeQuery::new("ＣＯ", vec!["MADR1Z@D".to_string()]);
        assert!(query.is_err());
    }

    #[test]
    fn code_query_rejects_invalid_date_range() {
        let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
            .unwrap()
            .with_start_date("202502")
            .unwrap()
            .with_end_date("202401");

        assert!(query.is_err());
    }

    #[test]
    fn code_query_accepts_yyyyxx_and_rejects_invalid_suffix() {
        let valid = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
            .unwrap()
            .with_start_date("202412")
            .unwrap()
            .with_end_date("202501");
        assert!(valid.is_ok());

        let invalid = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
            .unwrap()
            .with_start_date("202413");
        assert!(matches!(invalid, Err(BojError::ValidationError(_))));
    }

    #[test]
    fn layer_query_enforces_layer_length_and_frequency_dates() {
        let too_many = LayerQuery::new(
            "BP01",
            Frequency::M,
            vec![
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
            ],
        );
        assert!(too_many.is_err());

        let invalid_date = LayerQuery::new("BP01", Frequency::Q, vec!["1".to_string()])
            .unwrap()
            .with_start_date("202513");
        assert!(invalid_date.is_err());
    }

    #[test]
    fn layer_query_rejects_start_position_zero() {
        let query = LayerQuery::new("BP01", Frequency::M, vec!["*".to_string()])
            .unwrap()
            .with_start_position(0);
        assert!(query.is_err());
    }

    #[test]
    fn metadata_query_requires_db() {
        let query = MetadataQuery::new(" ");
        assert!(query.is_err());
    }

    #[test]
    fn layer_query_accepts_wildcard_first_layer() {
        let query = LayerQuery::new("MD10", Frequency::Q, vec!["*".to_string()]);
        assert!(query.is_ok());
    }

    #[test]
    fn code_query_rejects_too_many_codes() {
        let mut codes = Vec::new();
        for _ in 0..1251 {
            codes.push("AAA".to_string());
        }
        let query = CodeQuery::new("CO", codes);
        assert!(query.is_err());
    }

    #[test]
    fn frequency_date_shape_is_strict() {
        let ch = LayerQuery::new("FF", Frequency::Ch, vec!["1".to_string()])
            .unwrap()
            .with_start_date("202503")
            .unwrap_err();
        assert!(matches!(ch, BojError::ValidationError(_)));

        let cy = LayerQuery::new("FF", Frequency::Cy, vec!["1".to_string()])
            .unwrap()
            .with_start_date("202501")
            .unwrap_err();
        assert!(matches!(cy, BojError::ValidationError(_)));
    }
}
