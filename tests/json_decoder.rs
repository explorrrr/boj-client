mod common;

use boj_client::client::BojClient;
use boj_client::error::BojError;
use boj_client::query::{CodeQuery, Format, Language, MetadataQuery};
use common::{StubResponse, StubServer, fixture_bytes};

#[test]
fn parses_success_json_and_normalizes_points() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Json)
        .with_lang(Language::En);

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.meta.message_id, "M181000I");
    assert_eq!(response.parameter.db.as_deref(), Some("CO"));
    assert_eq!(response.next_position, None);
    assert_eq!(response.series.len(), 1);
    assert_eq!(response.series[0].series_code, "TK99F1000601GCQ01000");
    assert_eq!(response.series[0].points.len(), 2);
    assert_eq!(response.series[0].points[0].survey_date, "202401");
    assert_eq!(response.series[0].points[0].value.as_deref(), Some("11"));
    assert_eq!(response.series[0].points[1].value, None);
}

#[test]
fn parses_success_json_with_no_data() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_no_data.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Json)
        .with_lang(Language::En);

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.meta.message_id, "M181030I");
    assert!(response.series.is_empty());
}

#[test]
fn returns_api_error_for_error_json_payload() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_error_400_invalid_db.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Json)
        .with_lang(Language::En);

    let error = client.get_data_code(&query).unwrap_err();
    let _ = server.finish();

    assert_eq!(
        error,
        BojError::ApiError {
            status: 400,
            message_id: "M181005E".to_string(),
            message: "DB名が正しくありません。".to_string(),
        }
    );
}

#[test]
fn parses_metadata_json_payload() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_metadata_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());
    let query = MetadataQuery::new("FM08")
        .unwrap()
        .with_format(Format::Json)
        .with_lang(Language::En);

    let response = client.get_metadata(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.db, "FM08");
    assert_eq!(response.entries.len(), 2);
    assert_eq!(response.entries[0].layer1, Some(1));
    assert_eq!(response.entries[1].series_code.as_deref(), Some("FXERD01"));
}

#[test]
fn rejects_legacy_json_shape() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_legacy_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Json)
        .with_lang(Language::En);

    let result = client.get_data_code(&query);
    let _ = server.finish();

    assert!(matches!(result, Err(BojError::DecodeError(_))));
}
