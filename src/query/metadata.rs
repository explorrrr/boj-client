use crate::error::BojError;

use super::options::{CsvEncoding, Format, Language};
use super::validation::validate_db;

/// Query builder for the `getMetadata` endpoint.
///
/// Constraints enforced at build time:
/// - `DB` must be non-empty ASCII and must not contain commas.
///
/// # Examples
///
/// ```
/// use boj_client::query::{Format, Language, MetadataQuery};
///
/// let _query = MetadataQuery::new("ME")?
///     .with_format(Format::Json)
///     .with_lang(Language::En);
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataQuery {
    db: String,
    format: Option<Format>,
    lang: Option<Language>,
}

impl MetadataQuery {
    /// Creates a `getMetadata` query.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::MetadataQuery;
    ///
    /// let _query = MetadataQuery::new("ME")?;
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if `db` violates API constraints.
    pub fn new(db: impl Into<String>) -> Result<Self, BojError> {
        let db = db.into();
        validate_db(&db)?;

        Ok(Self {
            db: db.to_ascii_uppercase(),
            format: None,
            lang: None,
        })
    }

    /// Sets the response format (`json` or `csv`).
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::{Format, MetadataQuery};
    ///
    /// let _query = MetadataQuery::new("ME")?.with_format(Format::Csv);
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
    /// use boj_client::query::{Language, MetadataQuery};
    ///
    /// let _query = MetadataQuery::new("ME")?.with_lang(Language::Jp);
    /// # Ok::<(), boj_client::error::BojError>(())
    /// ```
    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub(crate) fn endpoint(&self) -> &'static str {
        "/api/v1/getMetadata"
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
        pairs
    }

    pub(crate) fn csv_encoding_hint(&self) -> CsvEncoding {
        match self.lang.unwrap_or_default() {
            Language::Jp => CsvEncoding::ShiftJis,
            Language::En => CsvEncoding::Utf8,
        }
    }
}
