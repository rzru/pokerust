use hyper::{
    body::{to_bytes, Bytes},
    client::HttpConnector,
    Client, StatusCode,
};
use hyper_tls::HttpsConnector;

pub struct Http {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Http {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self { client }
    }

    pub async fn get(&self, uri: &str) -> Option<Bytes> {
        if let Ok(uri) = uri.parse() {
            let resp = self.client.get(uri).await.unwrap();

            if resp.status() == StatusCode::OK {
                if let Ok(result) = to_bytes(resp.into_body()).await {
                    return Some(result);
                }
            }
        }

        None
    }
}
