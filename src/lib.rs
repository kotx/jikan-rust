#![forbid(unsafe_code)]

pub use hyper;
pub use hyper_tls;

pub mod models;

use hyper::{body::Buf, client::HttpConnector, Body, Response};
use hyper_tls::HttpsConnector;
use models::{anime::Anime, JikanResponse};
use thiserror::Error;

pub const DEFAULT_API_URL: &str = "https://api.jikan.moe/v4";

pub type JikanResult<T> = Result<T, JikanError>;

#[derive(Error, Debug)]
pub enum JikanError {
    #[error("invalid json data")]
    Json(#[from] serde_json::Error),

    #[error("http error")]
    Http(#[from] hyper::Error),

    #[error("unknown jikan error")]
    Unknown,
}

pub struct JikanClient<C> {
    pub api_url: String,
    pub http_client: hyper::Client<C>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> JikanClient<C> {
    pub fn new(http_client: hyper::Client<C>, api_url: String) -> Self {
        JikanClient {
            http_client,
            api_url,
        }
    }

    async fn parse_json_response<T: for<'de> serde::Deserialize<'de>>(
        mut res: Response<Body>,
    ) -> JikanResult<T> {
        Ok(serde_json::from_reader::<_, T>(
            hyper::body::aggregate(res.body_mut()).await?.reader(),
        )?)
    }

    pub async fn get_anime_by_id(self, id: u32) -> JikanResult<Anime> {
        let res = self
            .http_client
            .get((self.api_url + &format!("/anime/{id}")).parse().unwrap())
            .await?;

        return Ok(
            JikanClient::<C>::parse_json_response::<JikanResponse<Anime>>(res)
                .await?
                .data,
        );
    }
}

impl Default for JikanClient<HttpConnector> {
    fn default() -> Self {
        JikanClient::new(hyper::Client::new(), String::from(DEFAULT_API_URL))
    }
}

impl Default for JikanClient<HttpsConnector<HttpConnector>> {
    fn default() -> Self {
        let https = HttpsConnector::new();
        JikanClient::new(
            hyper::Client::builder().build::<_, hyper::Body>(https),
            String::from(DEFAULT_API_URL),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{hyper::client::HttpConnector, hyper_tls::HttpsConnector, JikanClient};

    #[tokio::test]
    async fn cowboy_bebop() {
        let client = JikanClient::<HttpsConnector<HttpConnector>>::default();
        let anime = client.get_anime_by_id(1).await.unwrap();
        assert_eq!(anime.title, "Cowboy Bebop");
        assert_eq!(anime.year, 1998);
    }
}
