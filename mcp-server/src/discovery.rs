use std::collections::BTreeSet;

use boj_client::catalog;
use rmcp::model::{
    AnnotateAble, ArgumentInfo, CompleteResult, CompletionContext, CompletionInfo, GetPromptResult,
    JsonObject, Prompt, PromptArgument, PromptMessage, PromptMessageRole, RawResource,
    RawResourceTemplate, ReadResourceResult, Reference, Resource, ResourceContents,
    ResourceTemplate,
};

use crate::mapping;

pub const PROMPT_DISCOVERY_FLOW: &str = "boj_discovery_flow";
pub const PROMPT_FETCH_DATA_CODE_FLOW: &str = "boj_fetch_data_code_flow";
pub const PROMPT_LATEST_VALUE_FLOW: &str = "boj_latest_value_flow";

pub const RESOURCE_CALL_ORDER_URI: &str = "boj://guide/call-order";
pub const RESOURCE_INPUT_NORMALIZATION_URI: &str = "boj://guide/input-normalization";
pub const RESOURCE_DATABASES_URI: &str = "boj://catalog/databases";

pub const RESOURCE_TEMPLATE_PARAMETERS_URI: &str = "boj://catalog/parameters/{endpoint}";
pub const RESOURCE_TEMPLATE_MESSAGES_URI: &str = "boj://catalog/messages/{status}";

pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt {
            name: PROMPT_DISCOVERY_FLOW.to_string(),
            title: Some("BOJ Discovery Flow".to_string()),
            description: Some(
                "Discover DBs, validate constraints, and prepare safe BOJ tool inputs.".to_string(),
            ),
            arguments: Some(vec![prompt_arg(
                "endpoint",
                "Endpoint",
                "Optional endpoint scope: all/get_data_code/get_data_layer/get_metadata.",
                false,
            )]),
            icons: None,
            meta: None,
        },
        Prompt {
            name: PROMPT_FETCH_DATA_CODE_FLOW.to_string(),
            title: Some("Fetch getDataCode".to_string()),
            description: Some(
                "Build a valid getDataCode request and execute it safely.".to_string(),
            ),
            arguments: Some(vec![
                prompt_arg("db", "DB", "BOJ DB code (e.g., FM01).", true),
                prompt_arg(
                    "codes",
                    "Codes",
                    "Comma-separated series codes (e.g., STRDCLUCON).",
                    true,
                ),
                prompt_arg(
                    "start_date",
                    "Start date",
                    "Optional YYYY or YYYYMM.",
                    false,
                ),
                prompt_arg("end_date", "End date", "Optional YYYY or YYYYMM.", false),
                prompt_arg("format", "Format", "Optional json/csv.", false),
                prompt_arg("lang", "Language", "Optional jp/en.", false),
            ]),
            icons: None,
            meta: None,
        },
        Prompt {
            name: PROMPT_LATEST_VALUE_FLOW.to_string(),
            title: Some("Fetch Latest Value".to_string()),
            description: Some(
                "Resolve metadata for one series and fetch the latest available points."
                    .to_string(),
            ),
            arguments: Some(vec![
                prompt_arg("db", "DB", "BOJ DB code.", true),
                prompt_arg("series_code", "Series code", "Target series code.", true),
                prompt_arg("format", "Format", "Optional json/csv.", false),
                prompt_arg("lang", "Language", "Optional jp/en.", false),
            ]),
            icons: None,
            meta: None,
        },
    ]
}

pub fn get_prompt(name: &str, arguments: Option<&JsonObject>) -> Option<GetPromptResult> {
    match name {
        PROMPT_DISCOVERY_FLOW => {
            let endpoint = arg_text(arguments, "endpoint").unwrap_or_else(|| "all".to_string());
            Some(GetPromptResult {
                description: Some("Recommended discovery and validation sequence.".to_string()),
                messages: vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Discover BOJ inputs for endpoint scope '{endpoint}' and avoid invalid parameter calls."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Run in order: 1) boj_list_databases 2) boj_get_parameter_catalog 3) data tool(s) 4) boj_get_message_catalog if STATUS/MESSAGEID needs lookup. Normalize format/lang to lowercase before calls."
                            .to_string(),
                    ),
                ],
            })
        }
        PROMPT_FETCH_DATA_CODE_FLOW => {
            let db = arg_text(arguments, "db").unwrap_or_else(|| "FM01".to_string());
            let codes = arg_text(arguments, "codes").unwrap_or_else(|| "STRDCLUCON".to_string());
            let start_date = arg_text(arguments, "start_date");
            let end_date = arg_text(arguments, "end_date");
            let format = arg_text(arguments, "format").unwrap_or_else(|| "json".to_string());
            let lang = arg_text(arguments, "lang").unwrap_or_else(|| "jp".to_string());

            Some(GetPromptResult {
                description: Some("Safe getDataCode invocation template.".to_string()),
                messages: vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Fetch BOJ getDataCode for db={db}, codes={codes}, start_date={start_date:?}, end_date={end_date:?}, format={format}, lang={lang}."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Before execution, call boj_get_parameter_catalog and validate date formats. Then call boj_get_data_code with lowercase format/lang and inspect next_position for pagination."
                            .to_string(),
                    ),
                ],
            })
        }
        PROMPT_LATEST_VALUE_FLOW => {
            let db = arg_text(arguments, "db").unwrap_or_else(|| "FM01".to_string());
            let series_code =
                arg_text(arguments, "series_code").unwrap_or_else(|| "STRDCLUCON".to_string());
            let format = arg_text(arguments, "format").unwrap_or_else(|| "json".to_string());
            let lang = arg_text(arguments, "lang").unwrap_or_else(|| "jp".to_string());

            Some(GetPromptResult {
                description: Some("Latest-value retrieval workflow.".to_string()),
                messages: vec![
                    PromptMessage::new_text(
                        PromptMessageRole::User,
                        format!(
                            "Get latest values for db={db}, series={series_code}, format={format}, lang={lang}."
                        ),
                    ),
                    PromptMessage::new_text(
                        PromptMessageRole::Assistant,
                        "Run boj_get_metadata to confirm series validity, then call boj_get_data_code with a narrow date range and choose the latest non-null point."
                            .to_string(),
                    ),
                ],
            })
        }
        _ => None,
    }
}

pub fn list_resources() -> Vec<Resource> {
    vec![
        resource(
            RESOURCE_CALL_ORDER_URI,
            "BOJ Call Order Guide",
            "Recommended BOJ MCP call order for reliable execution.",
            "text/plain",
        ),
        resource(
            RESOURCE_INPUT_NORMALIZATION_URI,
            "BOJ Input Normalization Guide",
            "Normalization rules (case, date shape, endpoint scope).",
            "text/plain",
        ),
        resource(
            RESOURCE_DATABASES_URI,
            "BOJ Database Catalog",
            "Static DB catalog snapshot bundled with boj-client.",
            "application/json",
        ),
    ]
}

pub fn list_resource_templates() -> Vec<ResourceTemplate> {
    vec![
        resource_template(
            RESOURCE_TEMPLATE_PARAMETERS_URI,
            "BOJ Parameter Catalog Template",
            "Read parameter and limits for one endpoint.",
            "application/json",
        ),
        resource_template(
            RESOURCE_TEMPLATE_MESSAGES_URI,
            "BOJ Message Catalog Template",
            "Read BOJ STATUS/MESSAGEID rows for one status code.",
            "application/json",
        ),
    ]
}

pub fn read_resource(uri: &str) -> Result<ReadResourceResult, String> {
    if uri == RESOURCE_CALL_ORDER_URI {
        let body = "Recommended order:\n1. boj_list_databases\n2. boj_get_parameter_catalog\n3. data tools (boj_get_data_code / boj_get_data_layer / boj_get_metadata)\n4. boj_get_message_catalog for MESSAGEID lookup.";
        return Ok(text_resource(uri, "text/plain", body.to_string()));
    }

    if uri == RESOURCE_INPUT_NORMALIZATION_URI {
        let body = "Normalization rules:\n- format: json|csv (uppercase accepted, normalized internally)\n- lang: jp|en (uppercase accepted, normalized internally)\n- endpoint scope: all|get_data_code|get_data_layer|get_metadata\n- date formats must match BOJ API requirements by endpoint/frequency.";
        return Ok(text_resource(uri, "text/plain", body.to_string()));
    }

    if uri == RESOURCE_DATABASES_URI {
        let payload = mapping::to_list_databases_output();
        let body = serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("failed to serialize databases payload: {error}"))?;
        return Ok(text_resource(uri, "application/json", body));
    }

    if let Some(endpoint) = uri.strip_prefix("boj://catalog/parameters/") {
        let endpoint_scope = mapping::parse_endpoint_scope(endpoint).ok_or_else(|| {
            "unsupported endpoint. expected all/get_data_code/get_data_layer/get_metadata"
                .to_string()
        })?;
        let payload = mapping::to_parameter_catalog_output(endpoint_scope);
        let body = serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("failed to serialize parameter payload: {error}"))?;
        return Ok(text_resource(uri, "application/json", body));
    }

    if let Some(status_raw) = uri.strip_prefix("boj://catalog/messages/") {
        let status = status_raw
            .parse::<u16>()
            .map_err(|_| "status must be a valid u16 integer".to_string())?;
        let payload = mapping::to_message_catalog_output(Some(status));
        let body = serde_json::to_string_pretty(&payload)
            .map_err(|error| format!("failed to serialize message payload: {error}"))?;
        return Ok(text_resource(uri, "application/json", body));
    }

    Err("resource not found".to_string())
}

pub fn complete(
    reference: &Reference,
    argument: &ArgumentInfo,
    _context: Option<&CompletionContext>,
) -> Result<CompleteResult, String> {
    if !supports_reference(reference) {
        return Ok(CompleteResult {
            completion: CompletionInfo::default(),
        });
    }

    let values = completion_values(&argument.name);
    let filtered = filter_by_prefix(values, &argument.value);

    let completion = if filtered.len() > CompletionInfo::MAX_VALUES {
        let total = filtered.len() as u32;
        CompletionInfo::with_pagination(
            filtered
                .into_iter()
                .take(CompletionInfo::MAX_VALUES)
                .collect::<Vec<_>>(),
            Some(total),
            true,
        )
        .map_err(|error| format!("failed to build paginated completion: {error}"))?
    } else {
        CompletionInfo::with_all_values(filtered)
            .map_err(|error| format!("failed to build completion: {error}"))?
    };

    Ok(CompleteResult { completion })
}

fn prompt_arg(name: &str, title: &str, description: &str, required: bool) -> PromptArgument {
    PromptArgument {
        name: name.to_string(),
        title: Some(title.to_string()),
        description: Some(description.to_string()),
        required: Some(required),
    }
}

fn arg_text(arguments: Option<&JsonObject>, key: &str) -> Option<String> {
    let value = arguments?.get(key)?;
    if let Some(text) = value.as_str() {
        return Some(text.to_string());
    }
    Some(value.to_string())
}

fn resource(uri: &str, name: &str, description: &str, mime_type: &str) -> Resource {
    let mut raw = RawResource::new(uri, name);
    raw.description = Some(description.to_string());
    raw.mime_type = Some(mime_type.to_string());
    raw.no_annotation()
}

fn resource_template(
    uri_template: &str,
    name: &str,
    description: &str,
    mime_type: &str,
) -> ResourceTemplate {
    RawResourceTemplate {
        uri_template: uri_template.to_string(),
        name: name.to_string(),
        title: None,
        description: Some(description.to_string()),
        mime_type: Some(mime_type.to_string()),
        icons: None,
    }
    .no_annotation()
}

fn text_resource(uri: &str, mime_type: &str, text: String) -> ReadResourceResult {
    ReadResourceResult {
        contents: vec![ResourceContents::TextResourceContents {
            uri: uri.to_string(),
            mime_type: Some(mime_type.to_string()),
            text,
            meta: None,
        }],
    }
}

fn supports_reference(reference: &Reference) -> bool {
    match reference {
        Reference::Prompt(prompt_ref) => {
            matches!(
                prompt_ref.name.as_str(),
                PROMPT_DISCOVERY_FLOW | PROMPT_FETCH_DATA_CODE_FLOW | PROMPT_LATEST_VALUE_FLOW
            )
        }
        Reference::Resource(resource_ref) => resource_ref.uri.starts_with("boj://"),
    }
}

fn completion_values(argument_name: &str) -> Vec<String> {
    match argument_name {
        "db" => catalog::databases()
            .iter()
            .map(|entry| entry.code.to_string())
            .collect(),
        "format" => vec!["json".to_string(), "csv".to_string()],
        "lang" => vec!["jp".to_string(), "en".to_string()],
        "endpoint" => vec![
            "all".to_string(),
            "get_data_code".to_string(),
            "get_data_layer".to_string(),
            "get_metadata".to_string(),
        ],
        "frequency" => catalog::frequency_codes()
            .iter()
            .map(|code| (*code).to_string())
            .collect(),
        "status" => {
            let mut set = BTreeSet::new();
            for entry in catalog::message_codes() {
                set.insert(entry.status.to_string());
            }
            set.into_iter().collect()
        }
        _ => Vec::new(),
    }
}

fn filter_by_prefix(values: Vec<String>, prefix: &str) -> Vec<String> {
    if prefix.is_empty() {
        return values;
    }

    let prefix_lower = prefix.to_ascii_lowercase();
    values
        .into_iter()
        .filter(|value| value.to_ascii_lowercase().starts_with(&prefix_lower))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_catalog_contains_expected_names() {
        let prompts = list_prompts();
        assert!(
            prompts
                .iter()
                .any(|prompt| prompt.name == PROMPT_DISCOVERY_FLOW)
        );
        assert!(
            prompts
                .iter()
                .any(|prompt| prompt.name == PROMPT_FETCH_DATA_CODE_FLOW)
        );
        assert!(
            prompts
                .iter()
                .any(|prompt| prompt.name == PROMPT_LATEST_VALUE_FLOW)
        );
    }

    #[test]
    fn parameters_template_resource_reads_json() {
        let result = read_resource("boj://catalog/parameters/get_data_code")
            .expect("parameters template should resolve");
        assert_eq!(result.contents.len(), 1);
        let text = match &result.contents[0] {
            ResourceContents::TextResourceContents { text, .. } => text,
            _ => panic!("unexpected content variant"),
        };
        assert!(text.contains("\"endpoint_scope\": \"getDataCode\""));
    }

    #[test]
    fn completion_filters_db_prefix_case_insensitive() {
        let result = complete(
            &Reference::for_prompt(PROMPT_FETCH_DATA_CODE_FLOW),
            &ArgumentInfo {
                name: "db".to_string(),
                value: "fm".to_string(),
            },
            None,
        )
        .expect("completion should succeed");

        assert!(
            result
                .completion
                .values
                .iter()
                .all(|value| value.starts_with("FM"))
        );
        assert!(result.completion.values.iter().any(|value| value == "FM01"));
    }
}
