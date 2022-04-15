use jikan::{JikanClient, JikanResult};

#[tokio::main]
async fn main() -> JikanResult<()> {
    let id = if let Some(id) = std::env::args().nth(1) {
        id.parse().unwrap()
    } else {
        1
    };

    let client = JikanClient::default().with_api_url(
        std::env::var("JIKAN_API_URL").unwrap_or_else(|_| jikan::DEFAULT_API_URL.into()),
    );

    println!("{:#?}", client.get_anime_by_id(id).await?);

    Ok(())
}
