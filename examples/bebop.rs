use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use jikan::{JikanClient, JikanResult};

#[tokio::main]
async fn main() -> JikanResult<()> {
    let client = JikanClient::<HttpsConnector<HttpConnector>>::default();
    dbg!(client.get_anime_by_id(1).await?);

    Ok(())
}
