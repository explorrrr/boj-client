use boj_client::catalog;
use boj_client::error::BojError;
use boj_client::model::{CodeResponse, LayerResponse, MetadataResponse, ResponseMeta};
use boj_client::query::{CodeQuery, Format, Frequency, Language, LayerQuery, MetadataQuery};
use serde_json::Value;

use crate::tools::{
    CodeToolOutput, DatabaseCatalogEntryOutput, EndpointScopeParam, FormatParam, FrequencyParam,
    GetDataCodeInput, GetDataLayerInput, GetMetadataInput, LanguageParam, LayerToolOutput,
    ListDatabasesOutput, MessageCatalogEntryOutput, MessageCatalogOutput, MetaOutput,
    MetadataToolOutput, ParameterCatalogEntryOutput, ParameterCatalogOutput, RequestLimitOutput,
    RequirementMatrixOutput,
};

impl From<FormatParam> for Format {
    fn from(value: FormatParam) -> Self {
        match value {
            FormatParam::Json => Format::Json,
            FormatParam::Csv => Format::Csv,
        }
    }
}

impl From<LanguageParam> for Language {
    fn from(value: LanguageParam) -> Self {
        match value {
            LanguageParam::Jp => Language::Jp,
            LanguageParam::En => Language::En,
        }
    }
}

impl From<FrequencyParam> for Frequency {
    fn from(value: FrequencyParam) -> Self {
        match value {
            FrequencyParam::Cy => Frequency::Cy,
            FrequencyParam::Fy => Frequency::Fy,
            FrequencyParam::Ch => Frequency::Ch,
            FrequencyParam::Fh => Frequency::Fh,
            FrequencyParam::Q => Frequency::Q,
            FrequencyParam::M => Frequency::M,
            FrequencyParam::W => Frequency::W,
            FrequencyParam::D => Frequency::D,
        }
    }
}

pub fn build_code_query(input: &GetDataCodeInput) -> Result<CodeQuery, BojError> {
    let mut query = CodeQuery::new(input.db.clone(), input.codes.clone())?;

    if let Some(format) = input.format {
        query = query.with_format(format.into());
    }
    if let Some(lang) = input.lang {
        query = query.with_lang(lang.into());
    }
    if let Some(start_date) = &input.start_date {
        query = query.with_start_date(start_date)?;
    }
    if let Some(end_date) = &input.end_date {
        query = query.with_end_date(end_date)?;
    }
    if let Some(start_position) = input.start_position {
        query = query.with_start_position(start_position)?;
    }

    Ok(query)
}

pub fn build_layer_query(input: &GetDataLayerInput) -> Result<LayerQuery, BojError> {
    let mut query = LayerQuery::new(
        input.db.clone(),
        input.frequency.into(),
        input.layers.clone(),
    )?;

    if let Some(format) = input.format {
        query = query.with_format(format.into());
    }
    if let Some(lang) = input.lang {
        query = query.with_lang(lang.into());
    }
    if let Some(start_date) = &input.start_date {
        query = query.with_start_date(start_date)?;
    }
    if let Some(end_date) = &input.end_date {
        query = query.with_end_date(end_date)?;
    }
    if let Some(start_position) = input.start_position {
        query = query.with_start_position(start_position)?;
    }

    Ok(query)
}

pub fn build_metadata_query(input: &GetMetadataInput) -> Result<MetadataQuery, BojError> {
    let mut query = MetadataQuery::new(input.db.clone())?;

    if let Some(format) = input.format {
        query = query.with_format(format.into());
    }
    if let Some(lang) = input.lang {
        query = query.with_lang(lang.into());
    }

    Ok(query)
}

pub fn to_code_output(response: CodeResponse, include_raw: bool) -> CodeToolOutput {
    let CodeResponse {
        meta,
        parameter,
        next_position,
        series,
        raw,
    } = response;

    let series = series
        .into_iter()
        .map(to_json_value)
        .collect::<Vec<serde_json::Value>>();

    CodeToolOutput {
        meta: to_meta_output(meta),
        parameter: to_json_value(parameter),
        next_position,
        series_count: series.len(),
        series,
        raw: include_raw.then_some(raw),
    }
}

pub fn to_layer_output(response: LayerResponse, include_raw: bool) -> LayerToolOutput {
    let LayerResponse {
        meta,
        parameter,
        next_position,
        series,
        raw,
    } = response;

    let series = series
        .into_iter()
        .map(to_json_value)
        .collect::<Vec<serde_json::Value>>();

    LayerToolOutput {
        meta: to_meta_output(meta),
        parameter: to_json_value(parameter),
        next_position,
        series_count: series.len(),
        series,
        raw: include_raw.then_some(raw),
    }
}

pub fn to_metadata_output(response: MetadataResponse, include_raw: bool) -> MetadataToolOutput {
    let MetadataResponse {
        meta,
        db,
        entries,
        raw,
    } = response;

    let entries = entries
        .into_iter()
        .map(to_json_value)
        .collect::<Vec<serde_json::Value>>();

    MetadataToolOutput {
        meta: to_meta_output(meta),
        db,
        entries_count: entries.len(),
        entries,
        raw: include_raw.then_some(raw),
    }
}

pub fn to_list_databases_output() -> ListDatabasesOutput {
    let snapshot = catalog::snapshot();
    let databases = catalog::databases()
        .iter()
        .map(|entry| DatabaseCatalogEntryOutput {
            category_ja: entry.category_ja.to_string(),
            code: entry.code.to_string(),
            name_ja: entry.name_ja.to_string(),
        })
        .collect::<Vec<_>>();

    ListDatabasesOutput {
        source_document: snapshot.source_document.to_string(),
        source_date: snapshot.source_date.to_string(),
        count: databases.len(),
        databases,
    }
}

pub fn to_parameter_catalog_output(endpoint: EndpointScopeParam) -> ParameterCatalogOutput {
    let snapshot = catalog::snapshot();

    let parameters = catalog::parameter_specs()
        .iter()
        .filter(|spec| parameter_applies_to_scope(spec, endpoint))
        .map(|spec| ParameterCatalogEntryOutput {
            name: spec.name.to_string(),
            description_ja: spec.description_ja.to_string(),
            allowed_values: spec.allowed_values.to_string(),
            requirements: RequirementMatrixOutput {
                code_api: spec.code_api.as_str().to_string(),
                layer_api: spec.layer_api.as_str().to_string(),
                metadata_api: spec.metadata_api.as_str().to_string(),
            },
            notes: spec.notes.iter().map(|note| (*note).to_string()).collect(),
        })
        .collect::<Vec<_>>();

    let limits = catalog::request_limits()
        .iter()
        .filter(|limit| limit_applies_to_scope(limit.api_scope, endpoint))
        .map(|limit| RequestLimitOutput {
            api_scope: limit.api_scope.to_string(),
            target: limit.target.to_string(),
            max_value: limit.max_value,
            overflow_behavior: limit.overflow_behavior.to_string(),
        })
        .collect::<Vec<_>>();

    ParameterCatalogOutput {
        source_document: snapshot.source_document.to_string(),
        source_date: snapshot.source_date.to_string(),
        endpoint_scope: endpoint_scope_name(endpoint).to_string(),
        general_notes: snapshot
            .general_notes
            .iter()
            .map(|note| (*note).to_string())
            .collect(),
        format_codes: catalog::format_codes()
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        language_codes: catalog::language_codes()
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        frequency_codes: frequency_codes_for_scope(endpoint),
        parameters,
        limits,
        layer_rules: layer_rules_for_scope(endpoint),
    }
}

pub fn to_message_catalog_output(status_filter: Option<u16>) -> MessageCatalogOutput {
    let snapshot = catalog::snapshot();
    let messages = catalog::message_codes()
        .iter()
        .filter(|entry| status_filter.is_none_or(|status| entry.status == status))
        .map(|entry| MessageCatalogEntryOutput {
            status: entry.status,
            message_id: entry.message_id.to_string(),
            message: entry.message.to_string(),
            note: entry.note.to_string(),
        })
        .collect::<Vec<_>>();

    MessageCatalogOutput {
        source_document: snapshot.source_document.to_string(),
        source_date: snapshot.source_date.to_string(),
        status_filter,
        count: messages.len(),
        messages,
    }
}

fn parameter_applies_to_scope(spec: &catalog::ParameterSpec, endpoint: EndpointScopeParam) -> bool {
    match endpoint {
        EndpointScopeParam::All => true,
        EndpointScopeParam::GetDataCode => {
            spec.code_api != catalog::EndpointRequirement::Unsupported
        }
        EndpointScopeParam::GetDataLayer => {
            spec.layer_api != catalog::EndpointRequirement::Unsupported
        }
        EndpointScopeParam::GetMetadata => {
            spec.metadata_api != catalog::EndpointRequirement::Unsupported
        }
    }
}

fn limit_applies_to_scope(api_scope: &str, endpoint: EndpointScopeParam) -> bool {
    match endpoint {
        EndpointScopeParam::All => true,
        EndpointScopeParam::GetDataCode => api_scope.split(',').any(|value| value == "getDataCode"),
        EndpointScopeParam::GetDataLayer => {
            api_scope.split(',').any(|value| value == "getDataLayer")
        }
        EndpointScopeParam::GetMetadata => api_scope.split(',').any(|value| value == "getMetadata"),
    }
}

fn frequency_codes_for_scope(endpoint: EndpointScopeParam) -> Vec<String> {
    if matches!(
        endpoint,
        EndpointScopeParam::All | EndpointScopeParam::GetDataLayer
    ) {
        catalog::frequency_codes()
            .iter()
            .map(|value| (*value).to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn layer_rules_for_scope(endpoint: EndpointScopeParam) -> Vec<String> {
    if matches!(
        endpoint,
        EndpointScopeParam::All | EndpointScopeParam::GetDataLayer
    ) {
        catalog::layer_rules()
            .iter()
            .map(|value| (*value).to_string())
            .collect()
    } else {
        Vec::new()
    }
}

fn endpoint_scope_name(endpoint: EndpointScopeParam) -> &'static str {
    match endpoint {
        EndpointScopeParam::All => "all",
        EndpointScopeParam::GetDataCode => "getDataCode",
        EndpointScopeParam::GetDataLayer => "getDataLayer",
        EndpointScopeParam::GetMetadata => "getMetadata",
    }
}

fn to_meta_output(meta: ResponseMeta) -> MetaOutput {
    MetaOutput {
        status: meta.status,
        message_id: meta.message_id,
        message: meta.message,
        date: meta.date,
    }
}

fn to_json_value<T>(value: T) -> Value
where
    T: serde::Serialize,
{
    serde_json::to_value(value).unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::{CodeToolOutput, FrequencyParam, GetDataLayerInput};
    use boj_client::model::{CodeParameterEcho, CodeResponse};

    #[test]
    fn layer_frequency_enforces_date_shape() {
        let input = GetDataLayerInput {
            db: "BP01".to_string(),
            frequency: FrequencyParam::Q,
            layers: vec!["1".to_string()],
            format: None,
            lang: None,
            start_date: Some("2024".to_string()),
            end_date: None,
            start_position: None,
            include_raw: false,
        };

        let error = build_layer_query(&input).expect_err("query should fail");
        assert!(matches!(error, BojError::ValidationError(_)));
    }

    #[test]
    fn include_raw_false_omits_raw_from_output() {
        let response = CodeResponse {
            meta: ResponseMeta {
                status: 200,
                message_id: "M181000I".to_string(),
                message: "ok".to_string(),
                date: Some("20260219".to_string()),
            },
            parameter: CodeParameterEcho::default(),
            next_position: None,
            series: Vec::new(),
            raw: "{\"STATUS\":200}".to_string(),
        };

        let output: CodeToolOutput = to_code_output(response, false);
        assert_eq!(output.raw, None);
    }

    #[test]
    fn list_databases_includes_known_code() {
        let output = to_list_databases_output();
        assert!(output.databases.iter().any(|entry| entry.code == "BP01"));
    }

    #[test]
    fn parameter_catalog_for_metadata_excludes_layer_only_rules() {
        let output = to_parameter_catalog_output(EndpointScopeParam::GetMetadata);
        assert!(output.frequency_codes.is_empty());
        assert!(output.layer_rules.is_empty());
        assert!(
            output
                .parameters
                .iter()
                .all(|item| item.requirements.metadata_api != "unsupported")
        );
    }

    #[test]
    fn message_catalog_can_filter_by_status() {
        let output = to_message_catalog_output(Some(503));
        assert_eq!(output.count, 1);
        assert_eq!(output.messages[0].message_id, "M181091S");
    }
}
