use boj_client::error::BojError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolErrorOutput {
    pub error_type: String,
    pub message: String,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

pub fn to_tool_error(error: BojError) -> ToolErrorOutput {
    match error {
        BojError::ValidationError(message) => ToolErrorOutput {
            error_type: "VALIDATION_ERROR".to_string(),
            message,
            retryable: false,
            status: None,
            message_id: None,
        },
        BojError::TransportError(message) => ToolErrorOutput {
            error_type: "TRANSPORT_ERROR".to_string(),
            message,
            retryable: true,
            status: None,
            message_id: None,
        },
        BojError::DecodeError(message) => ToolErrorOutput {
            error_type: "DECODE_ERROR".to_string(),
            message,
            retryable: false,
            status: None,
            message_id: None,
        },
        BojError::ApiError {
            status,
            message_id,
            message,
        } => ToolErrorOutput {
            error_type: "API_ERROR".to_string(),
            message,
            retryable: status == 500 || status == 503,
            status: Some(status),
            message_id: Some(message_id),
        },
    }
}
