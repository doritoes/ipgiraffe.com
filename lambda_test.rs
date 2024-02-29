use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    sourceIp: Option<String>, 
    xForwardedFor: Option<String>, 
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

    // Retrieve 'sourceIp' and 'xForwardedFor', handling defaults
    let src = event.sourceIp.unwrap_or_else(|| "".to_string());

    let xff = event.xForwardedFor
                    .and_then(|xffs| xffs.split(',').next().map(|s| s.to_string())) // Extract and clone
                    .unwrap_or_default(); 

    let body = format!("Hello {} and {}", src, xff);
    let response = Response { body };
    Ok(response)
}
