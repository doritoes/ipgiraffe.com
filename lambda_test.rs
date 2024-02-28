use lambda_runtime::{service_fn, Error};
use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct MyEvent {
    source_ip: Option<String>,
    x_forwarded_for: Option<String>,
}

#[derive(Serialize)]
struct MyResponse {
    status_code: u16,
    body: String,
    headers: std::collections::HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run::<ApiGatewayProxyRequest, MyResponse, _>(func).await?; // No LambdaEvent here 
    Ok(())
}

async fn handler(event: ApiGatewayProxyRequest) -> Result<MyResponse, Error> { // No LambdaEvent here 
    // TODO: Implement event extraction, IP logic, and response building
    unimplemented!()
}
