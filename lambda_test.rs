use lambda_runtime::{service_fn, Error};
use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct MyEvent {
    source_ip: Option<String>,
    x_forwarded_for: Option<String>,
}

#[derive(Serialize)]
struct MyResponse {
    status_code: u16,
    body: String,
    headers: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: ApiGatewayProxyRequest) -> Result<MyResponse, Error> {
    // TODO: 1. Extract MyEvent data (if present)
    //       2. Implement IP logic
    //       3. Build MyResponse
    unimplemented!()
}
