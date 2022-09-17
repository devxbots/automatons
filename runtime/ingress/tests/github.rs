use std::fs::read;
use std::net::{SocketAddr, TcpListener};

use reqwest::Client;
use tokio::task::JoinHandle;

use automatons_aws_ingress::{app, AppState, GitHubWebhookSecret};
use aws_config::SdkConfig;
use aws_smithy_http::endpoint::Endpoint;
use aws_types::{credentials::SharedCredentialsProvider, region::Region, Credentials};
use http::Uri;

const QUEUE_URL: &str = "http://localhost:4566/000000000000/automatons-event-queue";

fn aws_configuration() -> SdkConfig {
    SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::from_keys(
            "aws_access_key_id",
            "aws_secret_access_key",
            None,
        )))
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "http://localhost:4566/",
        )))
        .region(Region::new("eu-central-1"))
        .build()
}

fn spawn_app() -> (JoinHandle<anyhow::Result<()>>, SocketAddr) {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(app(
        AppState {
            aws_configuration: aws_configuration(),
            aws_event_queue_url: QUEUE_URL.into(),
            github_webhook_secret: GitHubWebhookSecret::from("secret"),
        },
        listener,
    ));

    (handle, addr)
}

#[tokio::test]
async fn webhook_queues_event() {
    let (_handle, addr) = spawn_app();

    let fixture = format!(
        "{}/tests/fixtures/check_run.created.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let body = read(fixture).unwrap();

    let response = Client::new()
        .post(format!("http://{}/github", addr))
        .header("X-GitHub-Event", "not_a_real_event")
        .header(
            "X-Hub-Signature-256",
            "sha256=ba9f77aa6bc9740e9be7f68e4e21a64821cc5b59fd286d409d605a0b8affe7ff",
        )
        .body(body)
        .send()
        .await
        .expect("failed to send request to test server");

    assert_eq!(response.status(), 201);

    let messages = aws_sdk_sqs::Client::new(&aws_configuration())
        .receive_message()
        .queue_url(QUEUE_URL)
        .send()
        .await
        .expect("failed to receive messages")
        .messages
        .expect("failed to find a message in response");

    assert_eq!(1, messages.len());
}

#[tokio::test]
async fn webhook_rejects_missing_signature() {
    let (_handle, addr) = spawn_app();

    let fixture = format!(
        "{}/tests/fixtures/check_run.created.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let body = read(fixture).unwrap();

    let response = Client::new()
        .post(format!("http://{}/github", addr))
        .body(body)
        .send()
        .await
        .expect("failed to send request to test server");

    assert_eq!(400, response.status().as_u16());

    assert!(response
        .text()
        .await
        .unwrap()
        .contains("missing X-Hub-Signature-256 header"));
}

#[tokio::test]
async fn webhook_rejects_invalid_signature() {
    let (_handle, addr) = spawn_app();

    let fixture = format!(
        "{}/tests/fixtures/check_run.created.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let body = read(fixture).unwrap();

    let response = Client::new()
        .post(format!("http://{}/github", addr))
        .header(
            "X-Hub-Signature-256",
            "sha256=21fc0cdd18aa13806dec49fa657a57571704de8690eaeda53c103493d55d6a37",
        )
        .body(body)
        .send()
        .await
        .expect("failed to send request to test server");

    assert_eq!(401, response.status().as_u16());

    assert!(response
        .text()
        .await
        .unwrap()
        .contains("X-Hub-Signature-256 header is invalid"));
}
