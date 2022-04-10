use jikan::{JikanClient, JikanResult};

#[tokio::main]
async fn main() -> JikanResult<()> {
    let client = JikanClient::default().with_api_url("");
    println!("{:#?}", client.get_anime_by_id(1).await?);

    Ok(())
}
