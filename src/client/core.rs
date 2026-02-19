use crate::decode::{decode_code, decode_layer, decode_metadata};
use crate::error::BojError;
use crate::model::{CodeResponse, LayerResponse, MetadataResponse};
use crate::query::{CodeQuery, LayerQuery, MetadataQuery};
use crate::transport::ReqwestTransport;

use super::http::{execute_request, header_value};
use super::response::{ensure_success_status, normalize_response_body};

const DEFAULT_BASE_URL: &str = "https://www.stat-search.boj.or.jp";

/// Synchronous BOJ API client.
///
/// `BojClient` uses an internal reqwest-based transport. Use [`BojClient::new`]
/// for defaults or [`BojClient::with_reqwest_client`] to inject a customized
/// reqwest client.
///
/// # Examples
///
/// ```no_run
/// use boj_client::client::BojClient;
/// use boj_client::query::{CodeQuery, Format, Language};
///
/// let client = BojClient::new();
/// let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
///     .with_format(Format::Json)
///     .with_lang(Language::En)
///     .with_start_date("202401")?
///     .with_end_date("202401")?;
/// let _response = client.get_data_code(&query)?;
/// # Ok::<(), boj_client::error::BojError>(())
/// ```
pub struct BojClient {
    transport: ReqwestTransport,
    base_url: String,
}

impl Default for BojClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BojClient {
    /// Creates a client with the default reqwest transport and BOJ base URL.
    pub fn new() -> Self {
        Self {
            transport: ReqwestTransport::default(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Creates a client from an existing `reqwest::blocking::Client`.
    ///
    /// This can be used to customize timeout, proxy, TLS, and other reqwest
    /// settings while keeping the BOJ client API surface stable.
    pub fn with_reqwest_client(client: reqwest::blocking::Client) -> Self {
        Self {
            transport: ReqwestTransport::new(client),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Replaces the base URL used for endpoint calls.
    ///
    /// This is mainly intended for tests and non-production environments.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Calls `getDataCode` and decodes the response into [`CodeResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] when request sending fails, response decoding fails,
    /// or when BOJ returns `STATUS != 200`.
    pub fn get_data_code(&self, query: &CodeQuery) -> Result<CodeResponse, BojError> {
        let response = execute_request(
            &self.transport,
            &self.base_url,
            query.endpoint(),
            query.query_pairs(),
        )?;

        let content_type = header_value(&response, "content-type");
        let body = normalize_response_body(&response)?;
        let decoded = decode_code(&body, content_type.as_deref(), query.csv_encoding_hint())?;

        ensure_success_status(
            decoded.meta.status,
            &decoded.meta.message_id,
            &decoded.meta.message,
        )?;

        Ok(decoded)
    }

    /// Calls `getDataLayer` and decodes the response into [`LayerResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] when request sending fails, response decoding fails,
    /// or when BOJ returns `STATUS != 200`.
    pub fn get_data_layer(&self, query: &LayerQuery) -> Result<LayerResponse, BojError> {
        let response = execute_request(
            &self.transport,
            &self.base_url,
            query.endpoint(),
            query.query_pairs(),
        )?;

        let content_type = header_value(&response, "content-type");
        let body = normalize_response_body(&response)?;
        let decoded = decode_layer(&body, content_type.as_deref(), query.csv_encoding_hint())?;

        ensure_success_status(
            decoded.meta.status,
            &decoded.meta.message_id,
            &decoded.meta.message,
        )?;

        Ok(decoded)
    }

    /// Calls `getMetadata` and decodes the response into [`MetadataResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`BojError`] when request sending fails, response decoding fails,
    /// or when BOJ returns `STATUS != 200`.
    pub fn get_metadata(&self, query: &MetadataQuery) -> Result<MetadataResponse, BojError> {
        let response = execute_request(
            &self.transport,
            &self.base_url,
            query.endpoint(),
            query.query_pairs(),
        )?;

        let content_type = header_value(&response, "content-type");
        let body = normalize_response_body(&response)?;
        let decoded = decode_metadata(&body, content_type.as_deref(), query.csv_encoding_hint())?;

        ensure_success_status(
            decoded.meta.status,
            &decoded.meta.message_id,
            &decoded.meta.message,
        )?;

        Ok(decoded)
    }
}
