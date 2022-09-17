use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
