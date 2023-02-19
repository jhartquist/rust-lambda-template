use lambda_http::{run, service_fn, Body, Error, Request, /*RequestExt,*/ Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false) // disable printing the name of the module in every log line.
        .without_time() // disabling time is handy because CloudWatch will add the ingestion time.
        .init();

    run(service_fn(function_handler)).await
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let a = 1;
    let b = 2;
    let c = common::add(1, 2);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("{a} + {b} = {c}").into())
        .map_err(Box::new)?;
    Ok(resp)
}
