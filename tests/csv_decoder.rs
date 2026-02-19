mod common;

use boj_client::client::BojClient;
use boj_client::error::BojError;
use boj_client::query::{CodeQuery, Format, Frequency, Language, LayerQuery, MetadataQuery};
use common::{StubResponse, StubServer, fixture_bytes};

#[test]
fn decodes_utf8_csv_code_response() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_en_utf8.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::En);

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.parameter.format.as_deref(), Some("CSV"));
    assert_eq!(response.parameter.lang.as_deref(), Some("EN"));
    assert_eq!(response.series.len(), 1);
    assert_eq!(response.series[0].points.len(), 2);
    assert_eq!(response.series[0].points[1].value, None);
}

#[test]
fn decodes_utf8_csv_layer_response() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_en_utf8.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = LayerQuery::new("BP01", Frequency::M, vec!["1".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::En);

    let response = client.get_data_layer(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.series.len(), 1);
    assert_eq!(response.series[0].series_code, "TK99F1000601GCQ01000");
}

#[test]
fn decodes_shift_jis_csv() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_jp_shiftjis.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::Jp);

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(
        response.series[0].name_of_time_series_j.as_deref(),
        Some("D.I./業況/大企業/製造業/実績")
    );
}

#[test]
fn handles_utf8_bom_in_csv_header() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_jp_utf8_bom.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::En);

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_eq!(response.meta.status, 200);
    assert_eq!(response.meta.message_id, "M181000I");
}

#[test]
fn utf8_decode_fails_for_shift_jis_payload() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_jp_shiftjis.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::En);

    let result = client.get_data_code(&query);
    let _ = server.finish();

    assert!(matches!(result, Err(BojError::DecodeError(_))));
}

#[test]
fn decodes_metadata_csv() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_success_metadata_en_utf8.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = MetadataQuery::new("FM08")
        .unwrap()
        .with_format(Format::Csv)
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
fn rejects_legacy_csv_shape() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/csv_legacy_code_api.csv"),
        "text/csv",
    ));
    let client = BojClient::new().with_base_url(server.base_url().to_string());
    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::En);

    let result = client.get_data_code(&query);
    let _ = server.finish();

    assert!(matches!(result, Err(BojError::DecodeError(_))));
}
