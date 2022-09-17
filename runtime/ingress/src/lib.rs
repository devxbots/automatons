use std::net::TcpListener;

use anyhow::Context;
use axum::routing::{get, post};
use axum::Router;

use automatons_github::secret;

mod error;
mod github;
mod health;

secret!(GitHubWebhookSecret);

#[derive(Clone, Debug)]
pub struct AppState {
    pub github_webhook_secret: GitHubWebhookSecret,
}

pub async fn app(app_state: AppState, listener: TcpListener) -> anyhow::Result<()> {
    let router = Router::with_state(app_state)
        .route("/_health", get(health::health_check_handler))
        .route("/github", post(github::github_webhook_handler));

    axum::Server::from_tcp(listener)
        .context("failed to create server")?
        .serve(router.into_make_service())
        .await
        .context("failed to run server")?;

    Ok(())
}
