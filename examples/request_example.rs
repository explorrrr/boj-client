use boj_client::client::BojClient;
use boj_client::error::BojError;
use boj_client::query::{CodeQuery, Format, Frequency, Language, LayerQuery, MetadataQuery};

fn main() -> Result<(), BojError> {
    let client = BojClient::new()?;
    run_all_requests(&client)
}

fn run_all_requests(client: &BojClient) -> Result<(), BojError> {
    println!("=== Code API ===");
    let code_query = CodeQuery::new("CO", vec!["TK99F1000601GCQ01000".to_string()])?
        .with_format(Format::Json)
        .with_lang(Language::En)
        .with_start_date("202401")?
        .with_end_date("202401")?;
    let code_response = client.get_data_code(&code_query)?;
    println!("status: {}", code_response.meta.status);
    println!("message_id: {}", code_response.meta.message_id);
    println!("series_len: {}", code_response.series.len());
    println!(
        "representative: {}",
        code_response
            .series
            .first()
            .map(|series| series.series_code.as_str())
            .unwrap_or("<none>")
    );
    println!();

    println!("=== Layer API ===");
    let layer_query = LayerQuery::new(
        "BP01",
        Frequency::M,
        vec!["1".to_string(), "1".to_string(), "1".to_string()],
    )?
    .with_format(Format::Csv)
    .with_lang(Language::En)
    .with_start_date("202504")?
    .with_end_date("202509")?;
    let layer_response = client.get_data_layer(&layer_query)?;
    println!("status: {}", layer_response.meta.status);
    println!("message_id: {}", layer_response.meta.message_id);
    println!("series_len: {}", layer_response.series.len());
    println!(
        "representative: {}",
        layer_response
            .series
            .first()
            .map(|series| series.series_code.as_str())
            .unwrap_or("<none>")
    );
    println!();

    println!("=== Metadata API ===");
    let metadata_query = MetadataQuery::new("FM08")?
        .with_format(Format::Csv)
        .with_lang(Language::En);
    let metadata_response = client.get_metadata(&metadata_query)?;
    println!("status: {}", metadata_response.meta.status);
    println!("message_id: {}", metadata_response.meta.message_id);
    println!("entries_len: {}", metadata_response.entries.len());
    println!("representative: {}", metadata_response.db);

    Ok(())
}
