use thiserror::Error;

/// Error type returned by the BOJ client.
///
/// # Examples
///
/// ```
/// use boj_client::error::BojError;
///
/// let error = BojError::validation("invalid parameter");
/// assert!(matches!(error, BojError::ValidationError(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BojError {
    /// Input validation failed before sending a request.
    #[error("validation error: {0}")]
    ValidationError(String),

    /// Response decoding failed for JSON/CSV payloads.
    #[error("decode error: {0}")]
    DecodeError(String),

    /// Transport-level failure (network, HTTP client, invalid request shape).
    #[error("transport error: {0}")]
    TransportError(String),

    /// API-level error represented by BOJ status and message fields.
    #[error("api error: status={status}, message_id={message_id}, message={message}")]
    ApiError {
        /// BOJ `STATUS` value.
        status: u16,
        /// BOJ `MESSAGE-ID` value.
        message_id: String,
        /// BOJ `MESSAGE` value.
        message: String,
    },
}

impl BojError {
    /// Creates [`BojError::ValidationError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::error::BojError;
    ///
    /// let error = BojError::validation("invalid db");
    /// assert!(matches!(error, BojError::ValidationError(_)));
    /// ```
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// Creates [`BojError::DecodeError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::error::BojError;
    ///
    /// let error = BojError::decode("invalid JSON");
    /// assert!(matches!(error, BojError::DecodeError(_)));
    /// ```
    pub fn decode(message: impl Into<String>) -> Self {
        Self::DecodeError(message.into())
    }

    /// Creates [`BojError::TransportError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::error::BojError;
    ///
    /// let error = BojError::transport("timeout");
    /// assert!(matches!(error, BojError::TransportError(_)));
    /// ```
    pub fn transport(message: impl Into<String>) -> Self {
        Self::TransportError(message.into())
    }

    /// Creates [`BojError::ApiError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use boj_client::error::BojError;
    ///
    /// let error = BojError::api(500, "M181090S", "internal");
    /// assert!(matches!(error, BojError::ApiError { status: 500, .. }));
    /// ```
    pub fn api(status: u16, message_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ApiError {
            status,
            message_id: message_id.into(),
            message: message.into(),
        }
    }
}
