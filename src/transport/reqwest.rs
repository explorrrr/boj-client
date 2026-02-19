use std::collections::HashMap;
use std::time::Duration;

use crate::error::BojError;

use super::types::{HttpRequest, HttpResponse, Transport};
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

/// [`Transport`] implementation backed by `reqwest::blocking::Client`.
pub(crate) struct ReqwestTransport {
    client: reqwest::blocking::Client,
}

impl ReqwestTransport {
    pub(crate) fn build_default_client() -> Result<reqwest::blocking::Client, BojError> {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            .user_agent(format!("boj-client/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|error| {
                BojError::transport(format!("failed to build default reqwest client: {error}"))
            })
    }

    /// Creates a transport from an existing `reqwest::blocking::Client`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use boj_client::transport::ReqwestTransport;
    ///
    /// let client = reqwest::blocking::Client::builder().build()?;
    /// let _transport = ReqwestTransport::new(client);
    /// # Ok::<(), reqwest::Error>(())
    /// ```
    pub(crate) fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }
}

impl Transport for ReqwestTransport {
    fn send(&self, request: HttpRequest) -> Result<HttpResponse, BojError> {
        let method = request.method.parse::<reqwest::Method>().map_err(|error| {
            BojError::transport(format!("invalid HTTP method '{}': {error}", request.method))
        })?;

        let mut builder = self.client.request(method, &request.url);
        for (header, value) in request.headers {
            builder = builder.header(header, value);
        }

        let response = builder
            .send()
            .map_err(|error| BojError::transport(error.to_string()))?;

        let status_code = response.status().as_u16();
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            let value = value
                .to_str()
                .map_err(|error| BojError::transport(error.to_string()))?;
            headers.insert(name.as_str().to_ascii_lowercase(), value.to_string());
        }

        let body = response
            .bytes()
            .map_err(|error| BojError::transport(error.to_string()))?
            .to_vec();

        Ok(HttpResponse {
            status_code,
            headers,
            body,
        })
    }
}
