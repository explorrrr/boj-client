use std::time::Duration;

use boj_client::client::BojClient;
use boj_client::error::BojError;
use clap::Parser;

use crate::retry::RetryPolicy;

const DEFAULT_BASE_URL: &str = "https://www.stat-search.boj.or.jp";

#[derive(Debug, Clone, Parser)]
#[command(name = "boj-mcp-server")]
#[command(about = "MCP server for BOJ time-series statistics API")]
pub struct Config {
    #[arg(long, env = "BOJ_BASE_URL", default_value = DEFAULT_BASE_URL)]
    pub base_url: String,

    #[arg(long, env = "BOJ_TIMEOUT_MS", default_value_t = 10_000)]
    pub timeout_ms: u64,

    #[arg(long, env = "BOJ_RETRY_MAX", default_value_t = 2)]
    pub retry_max: u32,

    #[arg(long, env = "BOJ_RETRY_BACKOFF_MS", default_value_t = 200)]
    pub retry_backoff_ms: u64,
}

impl Config {
    pub fn to_boj_client(&self) -> Result<BojClient, BojError> {
        let reqwest_client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .build()
            .map_err(|error| {
                BojError::transport(format!(
                    "failed to build reqwest client for MCP server: {error}"
                ))
            })?;

        Ok(BojClient::with_reqwest_client(reqwest_client).with_base_url(self.base_url.clone()))
    }

    pub fn retry_policy(&self) -> RetryPolicy {
        RetryPolicy {
            max_retries: self.retry_max,
            initial_backoff_ms: self.retry_backoff_ms,
        }
    }
}
