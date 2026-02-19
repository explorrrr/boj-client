use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Shared response metadata returned by BOJ endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseMeta {
    /// BOJ status code (`200`, `400`, `500`, `503`, ...).
    pub status: u16,
    /// BOJ message identifier.
    pub message_id: String,
    /// Human-readable BOJ message.
    pub message: String,
    /// Response date value provided by BOJ, when available.
    pub date: Option<String>,
}

/// A single survey data point in a time series.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataPoint {
    /// Survey date key (for example, year or period token from API payload).
    pub survey_date: String,
    /// Observed value at `survey_date`; `None` when BOJ omits the value.
    pub value: Option<String>,
}

/// Echoed request parameters for `getDataCode`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeParameterEcho {
    /// Echoed `format` parameter, if present in the response payload.
    pub format: Option<String>,
    /// Echoed `lang` parameter, if present in the response payload.
    pub lang: Option<String>,
    /// Echoed `db` parameter, if present in the response payload.
    pub db: Option<String>,
    /// Echoed `startDate` parameter, if present in the response payload.
    pub start_date: Option<String>,
    /// Echoed `endDate` parameter, if present in the response payload.
    pub end_date: Option<String>,
    /// Echoed `startPosition` parameter, if present in the response payload.
    pub start_position: Option<u32>,
    /// Additional parameter echoes not normalized into dedicated fields.
    #[serde(default)]
    pub extras: BTreeMap<String, String>,
}

/// Echoed request parameters for `getDataLayer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LayerParameterEcho {
    /// Echoed `format` parameter, if present in the response payload.
    pub format: Option<String>,
    /// Echoed `lang` parameter, if present in the response payload.
    pub lang: Option<String>,
    /// Echoed `db` parameter, if present in the response payload.
    pub db: Option<String>,
    /// Echoed `frequency` parameter, if present in the response payload.
    pub frequency: Option<String>,
    /// Echoed first layer index, if present in the response payload.
    pub layer1: Option<u32>,
    /// Echoed second layer index, if present in the response payload.
    pub layer2: Option<u32>,
    /// Echoed third layer index, if present in the response payload.
    pub layer3: Option<u32>,
    /// Echoed fourth layer index, if present in the response payload.
    pub layer4: Option<u32>,
    /// Echoed fifth layer index, if present in the response payload.
    pub layer5: Option<u32>,
    /// Echoed `startDate` parameter, if present in the response payload.
    pub start_date: Option<String>,
    /// Echoed `endDate` parameter, if present in the response payload.
    pub end_date: Option<String>,
    /// Echoed `startPosition` parameter, if present in the response payload.
    pub start_position: Option<u32>,
    /// Additional parameter echoes not normalized into dedicated fields.
    #[serde(default)]
    pub extras: BTreeMap<String, String>,
}

/// One time-series entry from `getDataCode`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeSeries {
    /// Time-series code returned by BOJ.
    pub series_code: String,
    /// Japanese time-series name, when available for response language/format.
    pub name_of_time_series_j: Option<String>,
    /// English time-series name, when available for response language/format.
    pub name_of_time_series: Option<String>,
    /// Japanese unit label, when available for response language/format.
    pub unit_j: Option<String>,
    /// English unit label, when available for response language/format.
    pub unit: Option<String>,
    /// Frequency label reported by BOJ, when available.
    pub frequency: Option<String>,
    /// Japanese category label, when available for response language/format.
    pub category_j: Option<String>,
    /// English category label, when available for response language/format.
    pub category: Option<String>,
    /// Last update timestamp/text from BOJ, when available.
    pub last_update: Option<String>,
    /// Ordered data points in the series.
    #[serde(default)]
    pub points: Vec<DataPoint>,
    /// Additional series fields not normalized into dedicated fields.
    #[serde(default)]
    pub extras: BTreeMap<String, Option<String>>,
}

/// One time-series entry from `getDataLayer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerSeries {
    /// Time-series code returned by BOJ.
    pub series_code: String,
    /// Japanese time-series name, when available for response language/format.
    pub name_of_time_series_j: Option<String>,
    /// English time-series name, when available for response language/format.
    pub name_of_time_series: Option<String>,
    /// Japanese unit label, when available for response language/format.
    pub unit_j: Option<String>,
    /// English unit label, when available for response language/format.
    pub unit: Option<String>,
    /// Frequency label reported by BOJ, when available.
    pub frequency: Option<String>,
    /// Japanese category label, when available for response language/format.
    pub category_j: Option<String>,
    /// English category label, when available for response language/format.
    pub category: Option<String>,
    /// Last update timestamp/text from BOJ, when available.
    pub last_update: Option<String>,
    /// Ordered data points in the series.
    #[serde(default)]
    pub points: Vec<DataPoint>,
    /// Additional series fields not normalized into dedicated fields.
    #[serde(default)]
    pub extras: BTreeMap<String, Option<String>>,
}

/// One metadata entry from `getMetadata`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataEntry {
    /// Time-series code, when present for the metadata row.
    pub series_code: Option<String>,
    /// Japanese time-series name, when available for response language/format.
    pub name_of_time_series_j: Option<String>,
    /// English time-series name, when available for response language/format.
    pub name_of_time_series: Option<String>,
    /// Japanese unit label, when available for response language/format.
    pub unit_j: Option<String>,
    /// English unit label, when available for response language/format.
    pub unit: Option<String>,
    /// Frequency label reported by BOJ, when available.
    pub frequency: Option<String>,
    /// Japanese category label, when available for response language/format.
    pub category_j: Option<String>,
    /// English category label, when available for response language/format.
    pub category: Option<String>,
    /// First layer index, when present.
    pub layer1: Option<u32>,
    /// Second layer index, when present.
    pub layer2: Option<u32>,
    /// Third layer index, when present.
    pub layer3: Option<u32>,
    /// Fourth layer index, when present.
    pub layer4: Option<u32>,
    /// Fifth layer index, when present.
    pub layer5: Option<u32>,
    /// Start of the time series, when present.
    pub start_of_the_time_series: Option<String>,
    /// End of the time series, when present.
    pub end_of_the_time_series: Option<String>,
    /// Last update timestamp/text from BOJ, when available.
    pub last_update: Option<String>,
    /// Japanese notes, when available for response language/format.
    pub notes_j: Option<String>,
    /// English notes, when available for response language/format.
    pub notes: Option<String>,
    /// Additional metadata fields not normalized into dedicated fields.
    #[serde(default)]
    pub extras: BTreeMap<String, Option<String>>,
}

/// Decoded response model for `getDataCode`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeResponse {
    /// Response metadata.
    pub meta: ResponseMeta,
    /// Echoed request parameters.
    pub parameter: CodeParameterEcho,
    /// Pagination cursor for subsequent calls, when available.
    pub next_position: Option<u32>,
    /// Returned series entries.
    pub series: Vec<CodeSeries>,
    /// Raw decoded text body preserved for diagnostics.
    pub raw: String,
}

/// Decoded response model for `getDataLayer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerResponse {
    /// Response metadata.
    pub meta: ResponseMeta,
    /// Echoed request parameters.
    pub parameter: LayerParameterEcho,
    /// Pagination cursor for subsequent calls, when available.
    pub next_position: Option<u32>,
    /// Returned series entries.
    pub series: Vec<LayerSeries>,
    /// Raw decoded text body preserved for diagnostics.
    pub raw: String,
}

/// Decoded response model for `getMetadata`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataResponse {
    /// Response metadata.
    pub meta: ResponseMeta,
    /// Database code corresponding to the metadata response.
    pub db: String,
    /// Returned metadata entries.
    pub entries: Vec<MetadataEntry>,
    /// Raw decoded text body preserved for diagnostics.
    pub raw: String,
}
