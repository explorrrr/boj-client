use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FormatParam {
    Json,
    Csv,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LanguageParam {
    Jp,
    En,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum FrequencyParam {
    #[serde(rename = "CY", alias = "cy")]
    Cy,
    #[serde(rename = "FY", alias = "fy")]
    Fy,
    #[serde(rename = "CH", alias = "ch")]
    Ch,
    #[serde(rename = "FH", alias = "fh")]
    Fh,
    #[serde(rename = "Q", alias = "q")]
    Q,
    #[serde(rename = "M", alias = "m")]
    M,
    #[serde(rename = "W", alias = "w")]
    W,
    #[serde(rename = "D", alias = "d")]
    D,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetDataCodeInput {
    pub db: String,
    pub codes: Vec<String>,
    #[serde(default)]
    pub format: Option<FormatParam>,
    #[serde(default)]
    pub lang: Option<LanguageParam>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub start_position: Option<u32>,
    #[serde(default)]
    pub include_raw: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetDataLayerInput {
    pub db: String,
    pub frequency: FrequencyParam,
    pub layers: Vec<String>,
    #[serde(default)]
    pub format: Option<FormatParam>,
    #[serde(default)]
    pub lang: Option<LanguageParam>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub start_position: Option<u32>,
    #[serde(default)]
    pub include_raw: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetMetadataInput {
    pub db: String,
    #[serde(default)]
    pub format: Option<FormatParam>,
    #[serde(default)]
    pub lang: Option<LanguageParam>,
    #[serde(default)]
    pub include_raw: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MetaOutput {
    pub status: u16,
    pub message_id: String,
    pub message: String,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeToolOutput {
    pub meta: MetaOutput,
    pub parameter: Value,
    pub next_position: Option<u32>,
    pub series_count: usize,
    pub series: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerToolOutput {
    pub meta: MetaOutput,
    pub parameter: Value,
    pub next_position: Option<u32>,
    pub series_count: usize,
    pub series: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MetadataToolOutput {
    pub meta: MetaOutput,
    pub db: String,
    pub entries_count: usize,
    pub entries: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}
