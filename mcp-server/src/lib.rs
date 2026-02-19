pub mod config;
pub mod discovery;
pub mod error;
pub mod mapping;
pub mod protocol;
pub mod retry;
pub mod tools;

use boj_client::error::BojError;
use config::Config;
use rmcp::{
    ErrorData, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::schema_for_type, wrapper::Parameters},
    model::{
        CallToolResult, CompleteRequestParams, CompleteResult, GetPromptRequestParams,
        ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult,
        PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult, ServerCapabilities,
        ServerInfo,
    },
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::Serialize;
use serde_json::json;
use tools::{
    CodeToolOutput, GetDataCodeInput, GetDataLayerInput, GetMessageCatalogInput, GetMetadataInput,
    GetParameterCatalogInput, LayerToolOutput, ListDatabasesInput, ListDatabasesOutput,
    MessageCatalogOutput, MetadataToolOutput, ParameterCatalogOutput,
};

#[derive(Debug, Clone)]
pub struct BojMcpServer {
    config: Config,
    tool_router: ToolRouter<Self>,
}

impl BojMcpServer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router(router = tool_router)]
impl BojMcpServer {
    #[tool(
        name = "boj_get_data_code",
        description = "Call BOJ getDataCode and return one page of structured data",
        annotations(
            title = "BOJ getDataCode",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<CodeToolOutput>()
    )]
    async fn get_data_code_tool(
        &self,
        params: Parameters<GetDataCodeInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        let result = run_blocking(move || run_code_tool(config, input)).await?;
        into_tool_result(result)
    }

    #[tool(
        name = "boj_get_data_layer",
        description = "Call BOJ getDataLayer and return one page of structured data",
        annotations(
            title = "BOJ getDataLayer",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<LayerToolOutput>()
    )]
    async fn get_data_layer_tool(
        &self,
        params: Parameters<GetDataLayerInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        let result = run_blocking(move || run_layer_tool(config, input)).await?;
        into_tool_result(result)
    }

    #[tool(
        name = "boj_get_metadata",
        description = "Call BOJ getMetadata and return structured metadata entries",
        annotations(
            title = "BOJ getMetadata",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<MetadataToolOutput>()
    )]
    async fn get_metadata_tool(
        &self,
        params: Parameters<GetMetadataInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        let result = run_blocking(move || run_metadata_tool(config, input)).await?;
        into_tool_result(result)
    }

    #[tool(
        name = "boj_list_databases",
        description = "List known BOJ DB codes from the embedded catalog",
        annotations(
            title = "BOJ DB Catalog",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<ListDatabasesOutput>()
    )]
    async fn list_databases_tool(
        &self,
        params: Parameters<ListDatabasesInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let _input = params.0;
        let output = run_blocking(run_list_databases_tool).await;
        match output {
            Ok(data) => to_structured_success(data),
            Err(error) => Err(error),
        }
    }

    #[tool(
        name = "boj_get_parameter_catalog",
        description = "Get BOJ parameter/limit catalog for one endpoint or all endpoints",
        annotations(
            title = "BOJ Parameter Catalog",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<ParameterCatalogOutput>()
    )]
    async fn get_parameter_catalog_tool(
        &self,
        params: Parameters<GetParameterCatalogInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let input = params.0;
        let output = run_blocking(move || run_parameter_catalog_tool(input)).await;
        match output {
            Ok(data) => to_structured_success(data),
            Err(error) => Err(error),
        }
    }

    #[tool(
        name = "boj_get_message_catalog",
        description = "Get BOJ STATUS/MESSAGEID catalog with optional status filter",
        annotations(
            title = "BOJ Message Catalog",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        ),
        execution(task_support = "forbidden"),
        output_schema = schema_for_type::<MessageCatalogOutput>()
    )]
    async fn get_message_catalog_tool(
        &self,
        params: Parameters<GetMessageCatalogInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let input = params.0;
        let output = run_blocking(move || run_message_catalog_tool(input)).await;
        match output {
            Ok(data) => to_structured_success(data),
            Err(error) => Err(error),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BojMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: protocol::protocol_version_2025_11_25(),
            instructions: Some(
                "Use BOJ tools in this order: 1) boj_list_databases 2) boj_get_parameter_catalog 3) data tools 4) boj_get_message_catalog for MESSAGEID lookup.\nTool failures are returned as CallToolResult with isError=true and structured error payload.\nformat/lang accept uppercase input but are normalized to lowercase (json/csv, jp/en)."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .enable_completions()
                .build(),
            ..Default::default()
        }
    }

    fn complete(
        &self,
        request: CompleteRequestParams,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<CompleteResult, ErrorData>> + Send + '_ {
        std::future::ready(
            discovery::complete(&request.r#ref, &request.argument, request.context.as_ref())
                .map_err(|message| ErrorData::invalid_params(message, None)),
        )
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ListPromptsResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(ListPromptsResult::with_all_items(
            discovery::list_prompts(),
        )))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::GetPromptResult, ErrorData>> + Send + '_ {
        std::future::ready(
            discovery::get_prompt(&request.name, request.arguments.as_ref())
                .ok_or_else(|| ErrorData::invalid_params("prompt not found", None)),
        )
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult::with_all_items(
            discovery::list_resources(),
        )))
    }

    fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ListResourceTemplatesResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(ListResourceTemplatesResult::with_all_items(
            discovery::list_resource_templates(),
        )))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, ErrorData>> + Send + '_ {
        std::future::ready(
            discovery::read_resource(&request.uri)
                .map_err(|message| ErrorData::invalid_params(message, None)),
        )
    }
}

pub async fn run_stdio(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let service = BojMcpServer::new(config).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

async fn run_blocking<T, F>(task: F) -> Result<T, ErrorData>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    tokio::task::spawn_blocking(task).await.map_err(|error| {
        ErrorData::internal_error(
            "blocking task join failure",
            Some(json!({
                "error_type": "INTERNAL_ERROR",
                "message": error.to_string(),
            })),
        )
    })
}

fn run_code_tool(config: Config, input: GetDataCodeInput) -> Result<CodeToolOutput, BojError> {
    let include_raw = input.include_raw;
    let query = mapping::build_code_query(&input)?;
    let client = config.to_boj_client()?;

    let response =
        retry::execute_with_retry(config.retry_policy(), || client.get_data_code(&query))?;

    Ok(mapping::to_code_output(response, include_raw))
}

fn run_layer_tool(config: Config, input: GetDataLayerInput) -> Result<LayerToolOutput, BojError> {
    let include_raw = input.include_raw;
    let query = mapping::build_layer_query(&input)?;
    let client = config.to_boj_client()?;

    let response =
        retry::execute_with_retry(config.retry_policy(), || client.get_data_layer(&query))?;

    Ok(mapping::to_layer_output(response, include_raw))
}

fn run_metadata_tool(
    config: Config,
    input: GetMetadataInput,
) -> Result<MetadataToolOutput, BojError> {
    let include_raw = input.include_raw;
    let query = mapping::build_metadata_query(&input)?;
    let client = config.to_boj_client()?;

    let response =
        retry::execute_with_retry(config.retry_policy(), || client.get_metadata(&query))?;

    Ok(mapping::to_metadata_output(response, include_raw))
}

fn run_list_databases_tool() -> ListDatabasesOutput {
    mapping::to_list_databases_output()
}

fn run_parameter_catalog_tool(input: GetParameterCatalogInput) -> ParameterCatalogOutput {
    mapping::to_parameter_catalog_output(input.endpoint)
}

fn run_message_catalog_tool(input: GetMessageCatalogInput) -> MessageCatalogOutput {
    mapping::to_message_catalog_output(input.status)
}

fn into_tool_result<T>(result: Result<T, BojError>) -> Result<CallToolResult, ErrorData>
where
    T: Serialize,
{
    match result {
        Ok(output) => to_structured_success(output),
        Err(error) => {
            let payload = serde_json::to_value(error::to_tool_error(error)).unwrap_or_else(|_| {
                json!({
                    "error_type": "INTERNAL_ERROR",
                    "message": "failed to serialize tool error",
                    "retryable": false,
                })
            });
            Ok(CallToolResult::structured_error(payload))
        }
    }
}

fn to_structured_success<T>(output: T) -> Result<CallToolResult, ErrorData>
where
    T: Serialize,
{
    let value = serde_json::to_value(output).map_err(|error| {
        ErrorData::internal_error(
            "failed to serialize tool output",
            Some(json!({
                "error_type": "INTERNAL_ERROR",
                "message": error.to_string(),
            })),
        )
    })?;
    Ok(CallToolResult::structured(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::{CallToolRequestParams, ClientInfo, ReadResourceRequestParams, TaskSupport};
    use rmcp::{ClientHandler, ServiceExt};

    #[derive(Debug, Clone, Default)]
    struct DummyClientHandler;

    impl ClientHandler for DummyClientHandler {
        fn get_info(&self) -> ClientInfo {
            ClientInfo {
                protocol_version: protocol::protocol_version_2025_11_25(),
                ..ClientInfo::default()
            }
        }
    }

    fn test_config() -> Config {
        Config {
            base_url: "https://www.stat-search.boj.or.jp".to_string(),
            timeout_ms: 10,
            retry_max: 0,
            retry_backoff_ms: 1,
        }
    }

    async fn spawn_client_server()
    -> rmcp::service::RunningService<rmcp::RoleClient, DummyClientHandler> {
        let (server_transport, client_transport) = tokio::io::duplex(4096);

        let server = BojMcpServer::new(test_config());
        tokio::spawn(async move {
            let running = server.serve(server_transport).await.expect("server start");
            running.waiting().await.expect("server wait");
        });

        DummyClientHandler
            .serve(client_transport)
            .await
            .expect("client connect")
    }

    #[tokio::test]
    async fn initialize_returns_target_protocol_version() {
        let client = spawn_client_server().await;
        let server_info = client.peer_info().expect("server info must exist");
        assert_eq!(server_info.protocol_version.to_string(), "2025-11-25");
        client.cancel().await.expect("client cancel");
    }

    #[tokio::test]
    async fn tools_list_exposes_output_schema_annotations_and_execution() {
        let client = spawn_client_server().await;
        let tools = client.list_all_tools().await.expect("list tools");

        assert!(!tools.is_empty());
        for tool in &tools {
            assert!(
                tool.output_schema.is_some(),
                "{} missing output_schema",
                tool.name
            );
            let annotations = tool
                .annotations
                .as_ref()
                .expect("annotations must be set for every tool");
            assert_eq!(annotations.read_only_hint, Some(true));
            assert_eq!(annotations.destructive_hint, Some(false));
            assert_eq!(annotations.idempotent_hint, Some(true));
            assert_eq!(
                tool.execution
                    .as_ref()
                    .and_then(|execution| execution.task_support),
                Some(TaskSupport::Forbidden),
            );
        }

        client.cancel().await.expect("client cancel");
    }

    #[tokio::test]
    async fn invalid_params_return_tool_error_payload() {
        let client = spawn_client_server().await;

        let arguments = serde_json::json!({
            "db": "",
            "codes": ["INVALID_CODE"]
        })
        .as_object()
        .expect("object")
        .clone();

        let result = client
            .call_tool(CallToolRequestParams {
                meta: None,
                name: "boj_get_data_code".to_string().into(),
                arguments: Some(arguments),
                task: None,
            })
            .await
            .expect("tool call should return result");

        assert_eq!(result.is_error, Some(true));
        let structured = result
            .structured_content
            .expect("structured error payload should exist");
        assert_eq!(structured["error_type"], "VALIDATION_ERROR");
        assert_eq!(structured["retryable"], false);

        client.cancel().await.expect("client cancel");
    }

    #[test]
    fn uppercase_format_lang_are_accepted() {
        let input: GetMetadataInput = serde_json::from_value(serde_json::json!({
            "db": "FM01",
            "format": "JSON",
            "lang": "JP"
        }))
        .expect("uppercase format/lang should deserialize");

        assert!(mapping::build_metadata_query(&input).is_ok());
    }

    #[tokio::test]
    async fn prompts_resources_and_completion_are_available() {
        let client = spawn_client_server().await;

        let prompts = client.list_all_prompts().await.expect("list prompts");
        assert!(
            prompts
                .iter()
                .any(|prompt| prompt.name == discovery::PROMPT_DISCOVERY_FLOW)
        );

        let resources = client.list_all_resources().await.expect("list resources");
        assert!(
            resources
                .iter()
                .any(|resource| resource.uri == discovery::RESOURCE_CALL_ORDER_URI)
        );

        let templates = client
            .list_all_resource_templates()
            .await
            .expect("list templates");
        assert!(
            templates.iter().any(
                |template| template.uri_template == discovery::RESOURCE_TEMPLATE_PARAMETERS_URI
            )
        );

        let resource = client
            .read_resource(ReadResourceRequestParams {
                meta: None,
                uri: discovery::RESOURCE_CALL_ORDER_URI.to_string(),
            })
            .await
            .expect("read resource");
        assert_eq!(resource.contents.len(), 1);

        let completion_values = client
            .complete_prompt_simple(discovery::PROMPT_FETCH_DATA_CODE_FLOW, "db", "fm")
            .await
            .expect("completion");
        assert!(completion_values.iter().any(|value| value == "FM01"));

        client.cancel().await.expect("client cancel");
    }
}
