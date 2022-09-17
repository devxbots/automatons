use std::net::{SocketAddr, TcpListener};

use reqwest::Client;

use automatons_aws_ingress::{app, AppState, GitHubWebhookSecret};
use aws_config::SdkConfig;

#[tokio::test]
async fn health_returns_ok() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(app(
        AppState {
            aws_configuration: SdkConfig::builder().build(),
            aws_event_queue_url: "aws_event_queue".into(),
            github_webhook_secret: GitHubWebhookSecret::from("secret"),
        },
        listener,
    ));

    let response = Client::new()
        .get(format!("http://{}/_health", addr))
        .send()
        .await
        .expect("failed to execute GET /_health request");

    assert!(response.status().is_success());
}
