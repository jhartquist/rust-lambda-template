use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false) // disable printing the name of the module in every log line.
        .without_time() // disabling time is handy because CloudWatch will add the ingestion time.
        .init();

    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_params = event.query_string_parameters();

    let a = parse_number(&query_params, "a", 1);
    let b = parse_number(&query_params, "b", 2);
    let c = common::add(a, b);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("{a} + {b} = {c}").into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn parse_number(query_params: &QueryMap, param: &str, default: usize) -> usize {
    query_params
        .first(param)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
