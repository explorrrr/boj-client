use serde::{Deserialize, Serialize};

/// Response format parameter used by BOJ API endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Format {
    /// JSON response (`format=json`).
    Json,
    /// CSV response (`format=csv`).
    Csv,
}

impl Format {
    /// Returns the BOJ query parameter value for this format.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::Format;
    ///
    /// assert_eq!(Format::Json.as_query_value(), "json");
    /// assert_eq!(Format::Csv.as_query_value(), "csv");
    /// ```
    pub fn as_query_value(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
        }
    }
}

/// Language option for localized labels in API responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    /// Japanese labels (`lang=jp`).
    #[default]
    Jp,
    /// English labels (`lang=en`).
    En,
}

impl Language {
    /// Returns the BOJ query parameter value for this language.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::Language;
    ///
    /// assert_eq!(Language::Jp.as_query_value(), "jp");
    /// assert_eq!(Language::En.as_query_value(), "en");
    /// ```
    pub fn as_query_value(self) -> &'static str {
        match self {
            Self::Jp => "jp",
            Self::En => "en",
        }
    }
}

/// Frequency selector used by the `getDataLayer` endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    /// Calendar year frequency (`CY`).
    Cy,
    /// Fiscal year frequency (`FY`).
    Fy,
    /// Half-year frequency in calendar-year basis (`CH`).
    Ch,
    /// Half-year frequency in fiscal-year basis (`FH`).
    Fh,
    /// Quarterly frequency (`Q`).
    Q,
    /// Monthly frequency (`M`).
    M,
    /// Weekly frequency (`W`).
    W,
    /// Daily frequency (`D`).
    D,
}

impl Frequency {
    /// Returns the BOJ query parameter value for this frequency.
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::query::Frequency;
    ///
    /// assert_eq!(Frequency::Q.as_query_value(), "Q");
    /// assert_eq!(Frequency::M.as_query_value(), "M");
    /// ```
    pub fn as_query_value(self) -> &'static str {
        match self {
            Self::Cy => "CY",
            Self::Fy => "FY",
            Self::Ch => "CH",
            Self::Fh => "FH",
            Self::Q => "Q",
            Self::M => "M",
            Self::W => "W",
            Self::D => "D",
        }
    }
}

/// Expected CSV character encoding used by decoders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum CsvEncoding {
    /// Shift_JIS-encoded CSV.
    ShiftJis,
    /// UTF-8 encoded CSV.
    Utf8,
}
