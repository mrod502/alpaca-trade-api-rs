use std::{collections::HashMap, time::Duration};

use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Method, Request, Response, Uri};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::net::TcpStream;

struct HttpClient {
    base_url: Uri,
    default_headers: HeaderMap,
}

#[derive(Debug, Default)]
struct HttpClientError {
    message: String,
}

impl HttpClientError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

type ClientResult = Result<Response<Bytes>, HttpClientError>;

impl HttpClient {
    async fn get(
        &self,
        path: String,
        params: Option<HashMap<String, Vec<String>>>,
    ) -> ClientResult {
        Err(HttpClientError::default())
    }

    async fn perform(
        &self,
        method: Method,
        path: String,
        body: Option<Bytes>,
        headers: Option<HeaderMap>,
    ) -> ClientResult {
        let b = match body {
            Some(bb) => bb,
            None => Bytes::new(),
        };

        let uri = self.uri(path)?;
        let req = match Request::builder()
            .method(method)
            .uri(uri.clone())
            .body(Full::from(b))
        {
            Ok(r) => r,
            Err(e) => return Err(HttpClientError::new(format!("{}", e))),
        };

        let host = uri.host().unwrap();
        let port = uri.port_u16().unwrap_or(80);
        let address = format!("{}:{}", host, port);
        let client: Client<_, Full<Bytes>> =
            hyper_util::client::legacy::Client::builder(TokioExecutor::new())
                .pool_idle_timeout(Duration::from_secs(30))
                .http2_only(true)
                .build_http();
        let res = match client.request(req).await {
            Ok(o) => o,
            Err(e) => return Err(HttpClientError::new(format!("{}", e))),
        };

        Err(Default::default())
    }

    fn uri(&self, path: String) -> Result<Uri, HttpClientError> {
        let uri = Uri::builder()
            .authority(self.base_url.authority().unwrap().clone())
            .path_and_query(path)
            .scheme(self.base_url.scheme().unwrap().clone())
            .build();
        match uri {
            Ok(u) => Ok(u),
            Err(e) => Err(HttpClientError::new(format!("{}", e))),
        }
    }
}
