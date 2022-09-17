use std::net::{SocketAddr, TcpListener};

use reqwest::Client;

use automatons_aws_ingress::app;

#[tokio::test]
async fn health_returns_ok() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(app(listener));

    let response = Client::new()
        .get(format!("http://{}/_health", addr))
        .send()
        .await
        .expect("failed to execute GET /_health request");

    assert!(response.status().is_success());
}
