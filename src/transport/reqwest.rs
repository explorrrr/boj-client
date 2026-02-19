use std::collections::HashMap;

use crate::error::BojError;

use super::types::{HttpRequest, HttpResponse, Transport};

/// [`Transport`] implementation backed by `reqwest::blocking::Client`.
///
/// Use [`Default`] to get a client configured with
/// `User-Agent: boj-client/<crate-version>`, or [`ReqwestTransport::new`] to
/// inject a custom `reqwest` client.
///
/// # Examples
///
/// ```ignore
/// use boj_client::transport::ReqwestTransport;
///
/// let _default_transport = ReqwestTransport::default();
///
/// let client = reqwest::blocking::Client::builder().build()?;
/// let _custom_transport = ReqwestTransport::new(client);
/// # Ok::<(), reqwest::Error>(())
/// ```
pub(crate) struct ReqwestTransport {
    client: reqwest::blocking::Client,
}

impl ReqwestTransport {
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

impl Default for ReqwestTransport {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent(format!("boj-client/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build reqwest client");
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
