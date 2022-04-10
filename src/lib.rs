#![forbid(unsafe_code)]

pub use hyper;
#[cfg(feature = "tls")]
pub use hyper_tls;

pub mod models;

use hyper::{body::Buf, client::HttpConnector, Body, Response};
use models::{anime::Anime, JikanResponse};
use thiserror::Error;

#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;

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

#[cfg(feature = "tls")]
pub type DefaultConnector = HttpsConnector<HttpConnector>;

#[cfg(not(feature = "tls"))]
pub type DefaultConnector = HttpConnector;

#[derive(Clone)]
pub struct JikanClient<C = DefaultConnector> {
    pub api_url: String,
    pub http_client: hyper::Client<C>,
}

impl<C> JikanClient<C> {
    pub fn new<S: Into<String>>(api_url: S, http_client: hyper::Client<C>) -> Self {
        JikanClient {
            api_url: api_url.into(),
            http_client,
        }
    }

    pub fn with_http_client(mut self, http_client: hyper::Client<C>) -> Self {
        self.http_client = http_client;
        self
    }

    pub fn with_api_url<S: Into<String>>(mut self, api_url: S) -> Self {
        self.api_url = api_url.into();
        self
    }
}

#[cfg(feature = "tls")]
impl Default for JikanClient<HttpsConnector<HttpConnector>> {
    fn default() -> Self {
        let https = HttpsConnector::new();
        JikanClient::new(
            String::from(DEFAULT_API_URL),
            hyper::Client::builder().build::<_, hyper::Body>(https),
        )
    }
}

#[cfg(not(feature = "tls"))]
impl Default for JikanClient<HttpConnector> {
    fn default() -> Self {
        JikanClient::new(String::from(DEFAULT_API_URL), hyper::Client::new())
    }
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> JikanClient<C> {
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

#[cfg(test)]
mod tests {
    use crate::JikanClient;

    #[tokio::test]
    async fn cowboy_bebop() {
        let client = JikanClient::default();
        let anime = client.get_anime_by_id(1).await.unwrap();
        assert_eq!(anime.title, "Cowboy Bebop");
        assert_eq!(anime.year, 1998);
    }
}
