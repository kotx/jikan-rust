use jikan::{JikanClient, JikanResult};

#[tokio::main]
async fn main() -> JikanResult<()> {
    let client = JikanClient::default()
        .with_api_url(std::env::var("JIKAN_API_URL").unwrap_or(jikan::DEFAULT_API_URL.into()));
    println!("{:#?}", client.get_anime_by_id(1).await?);

    Ok(())
}
