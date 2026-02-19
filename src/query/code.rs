use crate::error::BojError;

use super::options::{CsvEncoding, Format, Language};
use super::validation::{validate_code, validate_date_generic, validate_date_order, validate_db};

/// Query builder for the `getDataCode` endpoint.
///
/// Constraints enforced at build time:
/// - `DB` must be non-empty ASCII and must not contain commas.
/// - `CODE` entries must be non-empty ASCII, must not contain commas, and
///   the number of entries must be between `1` and `1250`.
/// - `startDate` and `endDate` must be `YYYY` or `YYYYXX` (`XX=01..12`), and
///   if both are set
///   they must have the same format and `startDate <= endDate`.
///
/// # Examples
///
/// ```
/// use boj_client::query::{CodeQuery, Format, Language};
///
/// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
///     .with_format(Format::Json)
///     .with_lang(Language::En)
///     .with_start_date("202401")?
///     .with_end_date("202402")?;
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeQuery {
    db: String,
    codes: Vec<String>,
    format: Option<Format>,
    lang: Option<Language>,
    start_date: Option<String>,
    end_date: Option<String>,
    start_position: Option<u32>,
}

impl CodeQuery {
    /// Creates a `getDataCode` query.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::CodeQuery;
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if `db` or `codes` violate API constraints.
    pub fn new(db: impl Into<String>, codes: Vec<String>) -> Result<Self, BojError> {
        let db = db.into();
        validate_db(&db)?;

        if codes.is_empty() {
            return Err(BojError::validation("CODE is required"));
        }
        if codes.len() > 1250 {
            return Err(BojError::validation(
                "CODE must contain 1250 or fewer series codes",
            ));
        }
        for code in &codes {
            validate_code(code)?;
        }

        Ok(Self {
            db: db.to_ascii_uppercase(),
            codes,
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
    /// use boj_client::query::{CodeQuery, Format};
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
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
    /// use boj_client::query::{CodeQuery, Language};
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
    ///     .with_lang(Language::Jp);
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    /// Sets `startDate`.
    ///
    /// Accepted format is `YYYY` or `YYYYXX` (`XX=01..12`).
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::CodeQuery;
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
    ///     .with_start_date("2024")?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if the value format is invalid or if `endDate`
    /// is already set and the date order becomes invalid.
    pub fn with_start_date(mut self, value: impl Into<String>) -> Result<Self, BojError> {
        let value = value.into();
        validate_date_generic(&value)?;
        if let Some(end_date) = &self.end_date {
            validate_date_order(&value, end_date)?;
        }
        self.start_date = Some(value);
        Ok(self)
    }

    /// Sets `endDate`.
    ///
    /// Accepted format is `YYYY` or `YYYYXX` (`XX=01..12`).
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::CodeQuery;
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
    ///     .with_end_date("202402")?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if the value format is invalid or if `startDate`
    /// is already set and the date order becomes invalid.
    pub fn with_end_date(mut self, value: impl Into<String>) -> Result<Self, BojError> {
        let value = value.into();
        validate_date_generic(&value)?;
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
    /// use boj_client::query::CodeQuery;
    ///
    /// let _query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
    ///     .with_start_position(1)?;
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
        "/api/v1/getDataCode"
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
        if let Some(start_date) = &self.start_date {
            pairs.push(("startDate".to_string(), start_date.clone()));
        }
        if let Some(end_date) = &self.end_date {
            pairs.push(("endDate".to_string(), end_date.clone()));
        }
        pairs.push(("code".to_string(), self.codes.join(",")));
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
