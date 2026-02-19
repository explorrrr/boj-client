mod common;

use boj_client::client::BojClient;
use boj_client::query::{CodeQuery, Format, Frequency, Language, LayerQuery, MetadataQuery};
use common::{StubResponse, StubServer, fixture_bytes};

#[test]
fn code_query_builds_stable_url_and_normalizes_case() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = CodeQuery::new(
        "co",
        vec![
            "TK99F1000601GCQ01000".to_string(),
            "TK99F2000601GCQ01000".to_string(),
        ],
    )
    .unwrap()
    .with_format(Format::Json)
    .with_lang(Language::Jp)
    .with_start_date("202401")
    .unwrap()
    .with_end_date("202504")
    .unwrap()
    .with_start_position(250)
    .unwrap();

    let _ = client.get_data_code(&query).unwrap();

    let request = server.finish().unwrap();
    assert_eq!(
        request.target,
        "/api/v1/getDataCode?format=json&lang=jp&db=CO&startDate=202401&endDate=202504&code=TK99F1000601GCQ01000%2CTK99F2000601GCQ01000&startPosition=250"
    );
}

#[test]
fn layer_query_builds_expected_url() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = LayerQuery::new(
        "bp01",
        Frequency::M,
        vec!["1".to_string(), "1".to_string(), "1".to_string()],
    )
    .unwrap()
    .with_format(Format::Csv)
    .with_lang(Language::En)
    .with_start_date("202504")
    .unwrap()
    .with_end_date("202509")
    .unwrap();

    let _ = client.get_data_layer(&query).unwrap();

    let request = server.finish().unwrap();
    assert_eq!(
        request.target,
        "/api/v1/getDataLayer?format=csv&lang=en&db=BP01&frequency=M&layer=1%2C1%2C1&startDate=202504&endDate=202509"
    );
}

#[test]
fn metadata_query_builds_expected_url() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_metadata_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = MetadataQuery::new("pr01")
        .unwrap()
        .with_format(Format::Csv)
        .with_lang(Language::Jp);

    let _ = client.get_metadata(&query).unwrap();

    let request = server.finish().unwrap();
    assert_eq!(
        request.target,
        "/api/v1/getMetadata?format=csv&lang=jp&db=PR01"
    );
}

#[test]
fn unknown_db_is_allowed_for_backward_compatibility() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = CodeQuery::new("unknown_db", vec!["TK99F1000601GCQ01000".to_string()]).unwrap();
    let _ = client.get_data_code(&query).unwrap();

    let request = server.finish().unwrap();
    assert_eq!(
        request.target,
        "/api/v1/getDataCode?db=UNKNOWN_DB&code=TK99F1000601GCQ01000"
    );
}
