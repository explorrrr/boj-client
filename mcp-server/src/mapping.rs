use boj_client::error::BojError;
use boj_client::model::{CodeResponse, LayerResponse, MetadataResponse, ResponseMeta};
use boj_client::query::{CodeQuery, Format, Frequency, Language, LayerQuery, MetadataQuery};
use serde_json::Value;

use crate::tools::{
    CodeToolOutput, FormatParam, FrequencyParam, GetDataCodeInput, GetDataLayerInput,
    GetMetadataInput, LanguageParam, LayerToolOutput, MetaOutput, MetadataToolOutput,
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
}
