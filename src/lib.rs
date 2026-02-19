//! `boj-client` is a Rust client for the Bank of Japan time-series statistics API.
//!
//! This crate provides:
//! - request builders for each BOJ endpoint under [`query`],
//! - a synchronous API client under [`client`],
//! - strongly-typed response models under [`model`],
//! - shared error definitions under [`error`].
//!
//! Internal transport/decoder details are intentionally hidden from the
//! external API surface. The recommended starting point is [`client::BojClient`].
//!
//! ```compile_fail
//! use boj_client::decode::decode_code;
//! use boj_client::retry::should_retry;
//! use boj_client::transport::ReqwestTransport;
//! ```

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::bare_urls)]

/// BOJ API client entry point.
pub mod client;
/// Error definitions shared across query, transport, and decode layers.
pub mod error;
/// Public response model types.
pub mod model;
/// Query builders and option enums for BOJ API requests.
pub mod query;

mod decode;
mod transport;
