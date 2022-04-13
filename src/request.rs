use hyper::{header::HeaderValue, HeaderMap, Method};

pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Default)]
pub(crate) struct RequestBuilder {
    path: Option<String>,
    method: Option<Method>,
    headers: Option<HeaderMap<HeaderValue>>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn method(mut self, method: Method) -> RequestBuilder {
        self.method = Some(method);
        self
    }

    pub fn path(mut self, path: String) -> RequestBuilder {
        self.path = Some(path);
        self
    }

    pub fn headers(mut self, headers: HeaderMap<HeaderValue>) -> RequestBuilder {
        self.headers = Some(headers);
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> RequestBuilder {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Result<Request, String> {
        if let None = self.path {
            Err("Request path is required".to_string())
        } else {
            Ok(Request {
                path: self.path.unwrap(),
                method: self.method.unwrap_or(Method::GET),
                headers: self.headers,
                body: self.body,
            })
        }
    }
}
