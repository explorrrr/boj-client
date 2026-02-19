use std::thread;
use std::time::Duration;

use boj_client::error::BojError;

#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
}

impl RetryPolicy {
    fn delay_for_attempt(self, retry_index: u32) -> Duration {
        let multiplier = 2_u64.saturating_pow(retry_index);
        Duration::from_millis(self.initial_backoff_ms.saturating_mul(multiplier))
    }
}

pub fn execute_with_retry<T, F>(policy: RetryPolicy, mut action: F) -> Result<T, BojError>
where
    F: FnMut() -> Result<T, BojError>,
{
    let mut retry_index = 0_u32;

    loop {
        match action() {
            Ok(value) => return Ok(value),
            Err(error) => {
                if retry_index >= policy.max_retries || !should_retry(&error) {
                    return Err(error);
                }

                let delay = policy.delay_for_attempt(retry_index);
                thread::sleep(delay);
                retry_index = retry_index.saturating_add(1);
            }
        }
    }
}

pub fn should_retry(error: &BojError) -> bool {
    match error {
        BojError::TransportError(_) => true,
        BojError::ApiError { status, .. } => *status == 500 || *status == 503,
        BojError::ValidationError(_) | BojError::DecodeError(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retries_transport_error_until_success() {
        let mut attempts = 0_u8;
        let result = execute_with_retry(
            RetryPolicy {
                max_retries: 2,
                initial_backoff_ms: 1,
            },
            || {
                attempts += 1;
                if attempts < 3 {
                    return Err(BojError::transport("temporary network failure"));
                }
                Ok("ok")
            },
        )
        .expect("expected eventual success");

        assert_eq!(result, "ok");
        assert_eq!(attempts, 3);
    }

    #[test]
    fn does_not_retry_validation_error() {
        let mut attempts = 0_u8;
        let error: Result<(), BojError> = execute_with_retry(
            RetryPolicy {
                max_retries: 3,
                initial_backoff_ms: 1,
            },
            || {
                attempts += 1;
                Err(BojError::validation("invalid db"))
            },
        );
        let error = error.expect_err("expected immediate failure");

        assert!(matches!(error, BojError::ValidationError(_)));
        assert_eq!(attempts, 1);
    }
}
