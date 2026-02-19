use rmcp::model::ProtocolVersion;

pub fn protocol_version_2025_11_25() -> ProtocolVersion {
    // rmcp 0.16.0 does not expose a constant for 2025-11-25 yet.
    serde_json::from_str("\"2025-11-25\"").expect("valid MCP protocol version literal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_target_protocol_version() {
        assert_eq!(protocol_version_2025_11_25().to_string(), "2025-11-25");
    }
}
