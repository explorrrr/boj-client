use std::collections::HashMap;

use crate::transport::{HttpRequest, HttpResponse, Transport};

pub(super) fn execute_request<T: Transport>(
    transport: &T,
    base_url: &str,
    endpoint: &str,
    query_pairs: Vec<(String, String)>,
) -> Result<HttpResponse, crate::error::BojError> {
    let url = build_url(base_url, endpoint, &query_pairs);
    let mut headers = HashMap::new();
    headers.insert("Accept-Encoding".to_string(), "gzip".to_string());

    transport.send(HttpRequest {
        method: "GET".to_string(),
        url,
        headers,
    })
}

pub(super) fn build_url(
    base_url: &str,
    endpoint: &str,
    query_pairs: &[(String, String)],
) -> String {
    let base = base_url.trim_end_matches('/');
    let mut url = format!("{base}{endpoint}");

    if !query_pairs.is_empty() {
        let query = query_pairs
            .iter()
            .map(|(key, value)| {
                let key = urlencoding::encode(key);
                let value = urlencoding::encode(value);
                format!("{key}={value}")
            })
            .collect::<Vec<_>>()
            .join("&");
        url.push('?');
        url.push_str(&query);
    }

    url
}

pub(super) fn header_value(response: &HttpResponse, header_name: &str) -> Option<String> {
    for (key, value) in &response.headers {
        if key.eq_ignore_ascii_case(header_name) {
            return Some(value.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::build_url;

    #[test]
    fn build_url_encodes_query_pairs() {
        let url = build_url(
            "https://example.com/",
            "/api/v1/getDataCode",
            &[("code".to_string(), "A,B".to_string())],
        );
        assert_eq!(
            url,
            "https://example.com/api/v1/getDataCode?code=A%2CB".to_string()
        );
    }
}
