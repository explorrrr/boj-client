use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum FormatParam {
    #[serde(rename = "json", alias = "JSON", alias = "Json")]
    Json,
    #[serde(rename = "csv", alias = "CSV", alias = "Csv")]
    Csv,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub enum LanguageParam {
    #[serde(rename = "jp", alias = "JP", alias = "Jp")]
    Jp,
    #[serde(rename = "en", alias = "EN", alias = "En")]
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

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EndpointScopeParam {
    #[default]
    All,
    GetDataCode,
    GetDataLayer,
    GetMetadata,
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

#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub struct ListDatabasesInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct GetParameterCatalogInput {
    #[serde(default)]
    pub endpoint: EndpointScopeParam,
}

#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub struct GetMessageCatalogInput {
    #[serde(default)]
    pub status: Option<u16>,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatabaseCatalogEntryOutput {
    pub category_ja: String,
    pub code: String,
    pub name_ja: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListDatabasesOutput {
    pub source_document: String,
    pub source_date: String,
    pub count: usize,
    pub databases: Vec<DatabaseCatalogEntryOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequirementMatrixOutput {
    pub code_api: String,
    pub layer_api: String,
    pub metadata_api: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParameterCatalogEntryOutput {
    pub name: String,
    pub description_ja: String,
    pub allowed_values: String,
    pub requirements: RequirementMatrixOutput,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestLimitOutput {
    pub api_scope: String,
    pub target: String,
    pub max_value: u32,
    pub overflow_behavior: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParameterCatalogOutput {
    pub source_document: String,
    pub source_date: String,
    pub endpoint_scope: String,
    pub general_notes: Vec<String>,
    pub format_codes: Vec<String>,
    pub language_codes: Vec<String>,
    pub mcp_format_codes: Vec<String>,
    pub mcp_language_codes: Vec<String>,
    pub frequency_codes: Vec<String>,
    pub parameters: Vec<ParameterCatalogEntryOutput>,
    pub limits: Vec<RequestLimitOutput>,
    pub layer_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MessageCatalogEntryOutput {
    pub status: u16,
    pub message_id: String,
    pub message: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MessageCatalogOutput {
    pub source_document: String,
    pub source_date: String,
    pub status_filter: Option<u16>,
    pub count: usize,
    pub messages: Vec<MessageCatalogEntryOutput>,
}
