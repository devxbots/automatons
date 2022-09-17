use std::net::{SocketAddr, TcpListener};

use automatons_aws_ingress::app;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).expect("failed to bind socket");

    app(listener).await.expect("failed to start server");
}
