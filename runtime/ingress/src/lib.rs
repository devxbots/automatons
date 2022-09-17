use std::net::TcpListener;

use axum::routing::get;
use axum::Router;

mod health;

pub async fn app(listener: TcpListener) -> hyper::Result<()> {
    axum::Server::from_tcp(listener)
        .expect("failed to create server")
        .serve(router().into_make_service())
        .await
}

fn router() -> Router {
    Router::new().route("/_health", get(health::health_check_handler))
}
