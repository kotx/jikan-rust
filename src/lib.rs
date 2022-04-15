#![forbid(unsafe_code)]

pub use hyper;
#[cfg(feature = "tls")]
pub use hyper_tls;

pub mod models;

use hyper::{
    client::{HttpConnector, ResponseFuture},
    header::HeaderValue,
    Body, Method, Response,
};
use models::{
    anime::{Anime, Characters},
    JikanAPIError, JikanResponse,
};
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

    #[error("jikan api error")]
    API(JikanAPIError),

    #[error("unknown error")]
    Unknown,
}

#[cfg(feature = "tls")]
pub type DefaultConnector = HttpsConnector<HttpConnector>;

#[cfg(not(feature = "tls"))]
pub type DefaultConnector = HttpConnector;

#[derive(Clone)]
pub struct JikanClient<C = DefaultConnector> {
    api_url: String,
    http_client: hyper::Client<C>,
}

impl<C> JikanClient<C> {
    pub fn new<S: Into<String>>(api_url: S, http_client: hyper::Client<C>) -> Self {
        JikanClient {
            api_url: api_url.into(),
            http_client,
        }
    }

    pub fn api_url(self) -> String {
        self.api_url
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
    fn try_request(&self, path: String) -> Result<ResponseFuture, JikanError> {
        let mut builder = hyper::Request::builder()
            .method(Method::GET) // Jikan only supports (readonly) GET requests
            .uri(format!("{}/{}", self.api_url, path));

        if let Some(headers) = builder.headers_mut() {
            headers.insert(
                hyper::header::USER_AGENT,
                HeaderValue::from_static(concat!(
                    "jikan-rust (",
                    env!("CARGO_PKG_HOMEPAGE"),
                    ", ",
                    env!("CARGO_PKG_VERSION"),
                    ")"
                )),
            );
        }

        let req_final = builder.body(Body::empty()).unwrap();
        Ok(self.http_client.request(req_final))
    }

    async fn parse_json_response<T: for<'de> serde::Deserialize<'de>>(
        mut res: Response<Body>,
    ) -> JikanResult<T> {
        let body = hyper::body::to_bytes(res.body_mut()).await?;
        let body_str = std::str::from_utf8(&body).unwrap();

        let status = res.status();

        if status.is_success() {
            #[cfg(feature = "tracing")]
            tracing::trace!("Successful response body: {body_str:?}");
            Ok(serde_json::from_str(body_str)?)
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Unsuccessful response ({status}): {body_str:#?}");
            Err(JikanError::API(serde_json::from_str(body_str)?))
        }
    }

    pub async fn get_anime_by_id(&self, id: u32) -> JikanResult<Anime> {
        let res = self.try_request(format!("anime/{}", id))?.await?;

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
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn cowboy_bebop() {
        let client = JikanClient::default();
        let response = client.get_anime_by_id(1).await;
        assert_ok!(&response);

        if let Ok(anime) = response {
            assert_eq!(anime.title, "Cowboy Bebop");
            assert_eq!(anime.year, Some(1998));
        }
    }
}
