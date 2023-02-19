use lambda_http::{run, service_fn, Body, Error, Request, /*RequestExt,*/ Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false) // disable printing the name of the module in every log line.
        .without_time() // disabling time is handy because CloudWatch will add the ingestion time.
        .init();

    run(service_fn(function_handler)).await
}
