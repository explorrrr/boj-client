mod config;
mod error;
mod mapping;
mod retry;
mod tools;

use clap::Parser;
use config::Config;
use rmcp::{
    ErrorData, Json, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde_json::json;
use tools::{
    CodeToolOutput, GetDataCodeInput, GetDataLayerInput, GetMetadataInput, LayerToolOutput,
    MetadataToolOutput,
};

#[derive(Debug, Clone)]
struct BojMcpServer {
    config: Config,
    tool_router: ToolRouter<Self>,
}

impl BojMcpServer {
    fn new(config: Config) -> Self {
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
        description = "Call BOJ getDataCode and return one page of structured data"
    )]
    async fn get_data_code_tool(
        &self,
        params: Parameters<GetDataCodeInput>,
    ) -> Result<Json<CodeToolOutput>, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        run_blocking(move || run_code_tool(config, input))
            .await
            .map(Json)
    }

    #[tool(
        name = "boj_get_data_layer",
        description = "Call BOJ getDataLayer and return one page of structured data"
    )]
    async fn get_data_layer_tool(
        &self,
        params: Parameters<GetDataLayerInput>,
    ) -> Result<Json<LayerToolOutput>, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        run_blocking(move || run_layer_tool(config, input))
            .await
            .map(Json)
    }

    #[tool(
        name = "boj_get_metadata",
        description = "Call BOJ getMetadata and return structured metadata entries"
    )]
    async fn get_metadata_tool(
        &self,
        params: Parameters<GetMetadataInput>,
    ) -> Result<Json<MetadataToolOutput>, ErrorData> {
        let config = self.config.clone();
        let input = params.0;
        run_blocking(move || run_metadata_tool(config, input))
            .await
            .map(Json)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BojMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Use these tools to fetch BOJ time-series data. Responses are single-page and include next_position when available. Set include_raw=true only when raw payload is needed."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let service = BojMcpServer::new(config).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

async fn run_blocking<T, F>(task: F) -> Result<T, ErrorData>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, ErrorData> + Send + 'static,
{
    tokio::task::spawn_blocking(task).await.map_err(|error| {
        ErrorData::internal_error(
            "blocking task join failure",
            Some(json!({
                "error_type": "INTERNAL_ERROR",
                "message": error.to_string(),
            })),
        )
    })?
}

fn run_code_tool(config: Config, input: GetDataCodeInput) -> Result<CodeToolOutput, ErrorData> {
    let include_raw = input.include_raw;
    let query = mapping::build_code_query(&input).map_err(error::to_mcp_error)?;
    let client = config.to_boj_client().map_err(error::to_mcp_error)?;

    let response =
        retry::execute_with_retry(config.retry_policy(), || client.get_data_code(&query))
            .map_err(error::to_mcp_error)?;

    Ok(mapping::to_code_output(response, include_raw))
}

fn run_layer_tool(config: Config, input: GetDataLayerInput) -> Result<LayerToolOutput, ErrorData> {
    let include_raw = input.include_raw;
    let query = mapping::build_layer_query(&input).map_err(error::to_mcp_error)?;
    let client = config.to_boj_client().map_err(error::to_mcp_error)?;

    let response =
        retry::execute_with_retry(config.retry_policy(), || client.get_data_layer(&query))
            .map_err(error::to_mcp_error)?;

    Ok(mapping::to_layer_output(response, include_raw))
}

fn run_metadata_tool(
    config: Config,
    input: GetMetadataInput,
) -> Result<MetadataToolOutput, ErrorData> {
    let include_raw = input.include_raw;
    let query = mapping::build_metadata_query(&input).map_err(error::to_mcp_error)?;
    let client = config.to_boj_client().map_err(error::to_mcp_error)?;

    let response = retry::execute_with_retry(config.retry_policy(), || client.get_metadata(&query))
        .map_err(error::to_mcp_error)?;

    Ok(mapping::to_metadata_output(response, include_raw))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::ErrorCode;

    #[test]
    fn invalid_input_maps_to_invalid_params_error() {
        let config = Config {
            base_url: "https://www.stat-search.boj.or.jp".to_string(),
            timeout_ms: 10,
            retry_max: 0,
            retry_backoff_ms: 1,
        };

        let input = GetDataCodeInput {
            db: "".to_string(),
            codes: vec!["TK99F1000601GCQ01000".to_string()],
            format: None,
            lang: None,
            start_date: None,
            end_date: None,
            start_position: None,
            include_raw: false,
        };

        let error = run_code_tool(config, input).expect_err("invalid input should fail");
        assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
    }

    #[test]
    fn layer_input_rejects_bad_frequency_date_shape() {
        let config = Config {
            base_url: "https://www.stat-search.boj.or.jp".to_string(),
            timeout_ms: 10,
            retry_max: 0,
            retry_backoff_ms: 1,
        };

        let input = GetDataLayerInput {
            db: "BP01".to_string(),
            frequency: tools::FrequencyParam::Q,
            layers: vec!["1".to_string()],
            format: None,
            lang: None,
            start_date: Some("2024".to_string()),
            end_date: None,
            start_position: None,
            include_raw: false,
        };

        let error = run_layer_tool(config, input).expect_err("invalid date should fail");
        assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
    }
}
