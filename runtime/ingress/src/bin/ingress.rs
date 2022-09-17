use std::net::{SocketAddr, TcpListener};

use automatons_aws_ingress::{app, AppState, GitHubWebhookSecret};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).expect("failed to bind socket");

    let aws_configuration = aws_config::load_from_env().await;

    let aws_event_queue_url = std::env::var("AWS_EVENT_QUEUE_URL")
        .expect("environment variable AWS_EVENT_QUEUE_URL is not set");

    let github_webhook_secret = GitHubWebhookSecret::from(
        std::env::var("GITHUB_WEBHOOK_SECRET")
            .expect("environment variable GITHUB_WEBHOOK_SECRET is not set"),
    );

    let app_state = AppState {
        aws_configuration,
        aws_event_queue_url,
        github_webhook_secret,
    };

    app(app_state, listener)
        .await
        .expect("failed to start server");
}
