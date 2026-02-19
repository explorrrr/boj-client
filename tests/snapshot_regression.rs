mod common;

use boj_client::client::BojClient;
use boj_client::query::{CodeQuery, Format, Language};
use common::{StubResponse, StubServer, fixture_bytes};
use insta::assert_yaml_snapshot;
use serde_json::json;

#[test]
fn request_url_snapshot() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = CodeQuery::new(
        "CO",
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
    .unwrap();

    let _ = client.get_data_code(&query).unwrap();
    let request = server.finish().unwrap();

    assert_yaml_snapshot!(
        json!({"target": request.target}),
        @r###"target: /api/v1/getDataCode?format=json&lang=jp&db=CO&startDate=202401&endDate=202504&code=TK99F1000601GCQ01000%2CTK99F2000601GCQ01000"###
    );
}

#[test]
fn response_shape_snapshot() {
    let server = StubServer::serve_once(StubResponse::with_content_type(
        200,
        fixture_bytes("tests/fixtures/json_success_code_api.json"),
        "application/json",
    ));
    let client = BojClient::new()
        .expect("default client should build")
        .with_base_url(server.base_url().to_string());

    let query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()]).unwrap();

    let response = client.get_data_code(&query).unwrap();
    let _ = server.finish();

    assert_yaml_snapshot!(
        json!({
            "status": response.meta.status,
            "message_id": response.meta.message_id,
            "parameter_db": response.parameter.db,
            "next_position": response.next_position,
            "series_code": response.series.first().map(|series| &series.series_code),
            "points_len": response.series.first().map(|series| series.points.len()),
            "first_point": response.series.first().and_then(|series| series.points.first()),
            "second_point_value": response
                .series
                .first()
                .and_then(|series| series.points.get(1))
                .and_then(|point| point.value.clone()),
        }),
        @r###"
        first_point:
          survey_date: "202401"
          value: "11"
        message_id: M181000I
        next_position: ~
        parameter_db: CO
        points_len: 2
        second_point_value: ~
        series_code: TK99F1000601GCQ01000
        status: 200
        "###
    );
}
