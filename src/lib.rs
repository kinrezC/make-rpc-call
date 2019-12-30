use hyper::{Client, Request, Uri};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    #[serde(default = "default_json")]
    json: String,
    id: usize,
    method: String,
    params: Vec<String>,
}

impl RpcRequest {
    pub fn new(id: usize, method: &str, params: Vec<String>) -> Self {
        RpcRequest {
            json: "2.0".to_string(),
            id: id,
            method: method.to_string(),
            params: params,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    #[serde(default = "default_json")]
    json: String,
    id: usize,
    result: String,
}

fn default_json() -> String {
    "2.0".to_string()
}

#[derive(Debug, Clone)]
pub struct HttpRpc {
    pub id: Arc<AtomicUsize>,
    url: Uri,
}

impl HttpRpc {
    pub fn new(url: &str) -> Result<Self> {
        let url = url.parse::<Uri>()?;
        Ok(HttpRpc {
            id: Arc::new(AtomicUsize::new(0)),
            url: url,
        })
    }

    pub async fn send(&self, request: RpcRequest) -> Result<()> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let request = serde_json::to_string(&request)?;
        let req = Request::builder()
            .method("POST")
            .uri(&self.url)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(request))?;
        let res = client.request(req).await?;
        let res_body = hyper::body::to_bytes(res).await?;
        println!("{:?}", res_body);
        Ok(())
    }
}

