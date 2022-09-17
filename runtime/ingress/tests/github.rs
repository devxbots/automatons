use std::fs::read;
use std::net::{SocketAddr, TcpListener};

use reqwest::Client;

use automatons_aws_ingress::{app, AppState, GitHubWebhookSecret};

#[tokio::test]
async fn webhook_accepts_valid_signature() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(app(
        AppState {
            github_webhook_secret: GitHubWebhookSecret::from("secret"),
        },
        listener,
    ));

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
}

#[tokio::test]
async fn webhook_requires_signature() {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(app(
        AppState {
            github_webhook_secret: GitHubWebhookSecret::from("secret"),
        },
        listener,
    ));

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
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(app(
        AppState {
            github_webhook_secret: GitHubWebhookSecret::from("secret"),
        },
        listener,
    ));

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
