use crate::error::BojError;

use super::options::{CsvEncoding, Format, Frequency, Language};
use super::validation::{
    parse_layer_value, validate_date_for_frequency, validate_date_order, validate_db,
};

/// A layer selector value for `getDataLayer`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LayerValue {
    /// Select all values in the layer (`*`).
    Wildcard,
    /// Select a specific positive layer index.
    Index(u32),
}

impl LayerValue {
    fn as_query_value(&self) -> String {
        match self {
            Self::Wildcard => "*".to_string(),
            Self::Index(value) => value.to_string(),
        }
    }
}

/// Query builder for the `getDataLayer` endpoint.
///
/// Constraints enforced at build time:
/// - `DB` must be non-empty ASCII and must not contain commas.
/// - `LAYER` must contain between `1` and `5` entries.
/// - Each layer entry must be either `*` or a positive integer.
/// - Date format depends on `frequency`:
///   - `CY` / `FY`: `YYYY`
///   - `CH` / `FH`: `YYYYXX` where `XX` is `01..02`
///   - `Q`: `YYYYXX` where `XX` is `01..04`
///   - `M` / `W` / `D`: `YYYYXX` where `XX` is `01..12`
/// - If both dates are set they must share the same format and satisfy
///   `startDate <= endDate`.
///
/// # Examples
///
/// ```
/// use boj_client::query::{Format, Frequency, Language, LayerQuery};
///
/// let _query = LayerQuery::new("BP01", Frequency::Q, vec!["1".to_string(), "*".to_string()])?
///     .with_format(Format::Json)
///     .with_lang(Language::En)
///     .with_start_date("202401")?
///     .with_end_date("202404")?;
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerQuery {
    db: String,
    frequency: Frequency,
    layer: Vec<LayerValue>,
    format: Option<Format>,
    lang: Option<Language>,
    start_date: Option<String>,
    end_date: Option<String>,
    start_position: Option<u32>,
}

impl LayerQuery {
    /// Creates a `getDataLayer` query.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Frequency, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::M, vec!["1".to_string()])?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if `db`, `layers`, or parsed layer values violate
    /// API constraints.
    pub fn new(
        db: impl Into<String>,
        frequency: Frequency,
        layers: Vec<String>,
    ) -> Result<Self, BojError> {
        let db = db.into();
        validate_db(&db)?;

        if layers.is_empty() {
            return Err(BojError::validation("LAYER is required"));
        }
        if layers.len() > 5 {
            return Err(BojError::validation("LAYER accepts 1 to 5 levels only"));
        }

        let mut parsed_layers = Vec::with_capacity(layers.len());
        for layer in &layers {
            parsed_layers.push(parse_layer_value(layer)?);
        }

        Ok(Self {
            db: db.to_ascii_uppercase(),
            frequency,
            layer: parsed_layers,
            format: None,
            lang: None,
            start_date: None,
            end_date: None,
            start_position: None,
        })
    }

    /// Sets the response format (`json` or `csv`).
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Format, Frequency, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::M, vec!["1".to_string()])?
    ///     .with_format(Format::Csv);
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets response language (`jp` or `en`).
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Frequency, Language, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::M, vec!["1".to_string()])?
    ///     .with_lang(Language::Jp);
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets `startDate` using the date format implied by `frequency`.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Frequency, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::Q, vec!["1".to_string()])?
    ///     .with_start_date("202401")?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if the date format is invalid for the selected
    /// `frequency`, or if `endDate` is already set and the date order is
    /// invalid.
    pub fn with_start_date(mut self, value: impl Into<String>) -> Result<Self, BojError> {
        let value = value.into();
        validate_date_for_frequency(&value, self.frequency)?;
        if let Some(end_date) = &self.end_date {
            validate_date_order(&value, end_date)?;
        }
        self.start_date = Some(value);
        Ok(self)
    }

    /// Sets `endDate` using the date format implied by `frequency`.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Frequency, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::Q, vec!["1".to_string()])?
    ///     .with_end_date("202404")?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if the date format is invalid for the selected
    /// `frequency`, or if `startDate` is already set and the date order is
    /// invalid.
    pub fn with_end_date(mut self, value: impl Into<String>) -> Result<Self, BojError> {
        let value = value.into();
        validate_date_for_frequency(&value, self.frequency)?;
        if let Some(start_date) = &self.start_date {
            validate_date_order(start_date, &value)?;
        }
        self.end_date = Some(value);
        Ok(self)
    }

    /// Sets `startPosition`.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Frequency, LayerQuery};
    ///
    /// let _query = LayerQuery::new("BP01", Frequency::M, vec!["1".to_string()])?
    ///     .with_start_position(10)?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] when `start_position` is `0`.
    pub fn with_start_position(mut self, start_position: u32) -> Result<Self, BojError> {
        if start_position == 0 {
            return Err(BojError::validation("STARTPOSITION must be >= 1"));
        }
        self.start_position = Some(start_position);
        Ok(self)
    }

    pub(crate) fn endpoint(&self) -> &'static str {
        "/api/v1/getDataLayer"
    }

    pub(crate) fn query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(format) = self.format {
            pairs.push(("format".to_string(), format.as_query_value().to_string()));
        }
        if let Some(lang) = self.lang {
            pairs.push(("lang".to_string(), lang.as_query_value().to_string()));
        }
        pairs.push(("db".to_string(), self.db.clone()));
        pairs.push((
            "frequency".to_string(),
            self.frequency.as_query_value().to_string(),
        ));
        pairs.push((
            "layer".to_string(),
            self.layer
                .iter()
                .map(LayerValue::as_query_value)
                .collect::<Vec<_>>()
                .join(","),
        ));
        if let Some(start_date) = &self.start_date {
            pairs.push(("startDate".to_string(), start_date.clone()));
        }
        if let Some(end_date) = &self.end_date {
            pairs.push(("endDate".to_string(), end_date.clone()));
        }
        if let Some(start_position) = self.start_position {
            pairs.push(("startPosition".to_string(), start_position.to_string()));
        }
        pairs
    }

    pub(crate) fn csv_encoding_hint(&self) -> CsvEncoding {
        match self.lang.unwrap_or_default() {
            Language::Jp => CsvEncoding::ShiftJis,
            Language::En => CsvEncoding::Utf8,
        }
    }
}
