use std::collections::HashMap;

use crate::error::BojError;

/// HTTP request payload passed to [`Transport::send`].
///
/// # Examples
///
/// ```ignore
/// use std::collections::HashMap;
///
/// use boj_client::transport::HttpRequest;
///
/// let mut headers = HashMap::new();
/// headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
///
/// let request = HttpRequest {
///     method: "GET".to_string(),
///     url: "https://example.invalid/api".to_string(),
///     headers,
/// };
/// assert_eq!(request.method, "GET");
/// ```ignore
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HttpRequest {
    /// HTTP method name (for example, `GET`).
    pub method: String,
    /// Fully-qualified request URL.
    pub url: String,
    /// Request headers keyed by header name.
    pub headers: HashMap<String, String>,
}

/// HTTP response payload returned by [`Transport::send`].
///
/// # Examples
///
/// ```ignore
/// use std::collections::HashMap;
///
/// use boj_client::transport::HttpResponse;
///
/// let response = HttpResponse {
///     status_code: 200,
///     headers: HashMap::new(),
///     body: br#"{"STATUS":200}"#.to_vec(),
/// };
/// assert_eq!(response.status_code, 200);
/// ```ignore
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HttpResponse {
    /// Numeric HTTP status code.
    pub status_code: u16,
    /// Response headers keyed by header name.
    pub headers: HashMap<String, String>,
    /// Raw response body bytes.
    pub body: Vec<u8>,
}

/// Abstraction over the synchronous transport layer used by [`crate::client::BojClient`].
///
/// # Examples
///
/// ```ignore
/// use std::collections::HashMap;
///
/// use boj_client::error::BojError;
/// use boj_client::transport::{HttpRequest, HttpResponse, Transport};
///
/// struct MockTransport;
///
/// impl Transport for MockTransport {
///     fn send(&self, _request: HttpRequest) -> Result<HttpResponse, BojError> {
///         Ok(HttpResponse {
///             status_code: 200,
///             headers: HashMap::new(),
///             body: br#"{"STATUS":200}"#.to_vec(),
///         })
///     }
/// }
///
/// let transport = MockTransport;
/// let response = transport.send(HttpRequest {
///     method: "GET".to_string(),
///     url: "https://example.invalid".to_string(),
///     headers: HashMap::new(),
/// })?;
/// assert_eq!(response.status_code, 200);
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
pub(crate) trait Transport: Send + Sync {
    /// Sends an HTTP request and returns the raw response.
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] if the underlying transport fails.
    fn send(&self, request: HttpRequest) -> Result<HttpResponse, BojError>;
}
