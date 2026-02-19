use boj_client::error::BojError;
use rmcp::ErrorData;
use serde_json::json;

pub fn to_mcp_error(error: BojError) -> ErrorData {
    match error {
        BojError::ValidationError(message) => ErrorData::invalid_params(
            message,
            Some(json!({
                "error_type": "VALIDATION_ERROR"
            })),
        ),
        BojError::TransportError(message) => ErrorData::internal_error(
            "transport error while calling BOJ API",
            Some(json!({
                "error_type": "TRANSPORT_ERROR",
                "message": message,
            })),
        ),
        BojError::DecodeError(message) => ErrorData::internal_error(
            "failed to decode BOJ API response",
            Some(json!({
                "error_type": "DECODE_ERROR",
                "message": message,
            })),
        ),
        BojError::ApiError {
            status,
            message_id,
            message,
        } => {
            let data = Some(json!({
                "error_type": "API_ERROR",
                "status": status,
                "message_id": message_id,
                "message": message,
            }));

            if status == 400 {
                ErrorData::invalid_params("BOJ API returned an invalid-parameter response", data)
            } else {
                ErrorData::internal_error("BOJ API returned an error response", data)
            }
        }
    }
}
