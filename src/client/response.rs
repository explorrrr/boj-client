use std::io::Read;

use flate2::read::GzDecoder;

use crate::error::BojError;
use crate::transport::HttpResponse;

use super::http::header_value;

pub(super) fn ensure_success_status(
    status: u16,
    message_id: &str,
    message: &str,
) -> Result<(), BojError> {
    if status != 200 {
        return Err(BojError::api(
            status,
            message_id.to_string(),
            message.to_string(),
        ));
    }
    Ok(())
}

pub(super) fn normalize_response_body(response: &HttpResponse) -> Result<Vec<u8>, BojError> {
    let content_encoding = header_value(response, "content-encoding");
    if let Some(value) = content_encoding
        && value.to_ascii_lowercase().contains("gzip")
    {
        let mut decoder = GzDecoder::new(response.body.as_slice());
        let mut decoded = Vec::new();
        decoder
            .read_to_end(&mut decoded)
            .map_err(|error| BojError::decode(format!("failed to decode gzip body: {error}")))?;
        return Ok(decoded);
    }

    Ok(response.body.clone())
}
