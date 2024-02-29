use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    key: Option<String>, 
    color: Option<String>, 
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

    // Retrieve 'key' and 'color', handling defaults
    let value = event.key.unwrap_or_else(|| "".to_string());
    let color = event.color.unwrap_or_else(|| "".to_string());

    let body = format!("Hello {} and {}", value, color);
    let response = Response { body };
    Ok(response)
}
