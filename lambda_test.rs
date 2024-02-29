use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    sourcIp: Option<String>,
    xForwardedFor: Option<String>
}

#[derive(Serialize)]
struct Response {
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Event>) -> Result<Response, Error> {
    let (event, _context) = event.into_parts();
    let value = event.key;

    // Handle the xff if it exists
    let xff = if let Some(xff) = event.xForwardedFor {
        xff.split(',').next().unwrap_or_default()
    } else {
        String::new()
    };
    // Handle the sourceip if it exists
    let srcip = if let Some(srcips) = event.sourceIp {
        srcip.split(',').next().unwrap_or_default()
    } else {
        String::new()
    };

    
    let body = format!("{},{}", xff, srcip);
    let response = Response { body };
    Ok(response)
}
