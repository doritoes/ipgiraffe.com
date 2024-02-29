use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    key: String,
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
    // Access the event data directly (if needed)
    let (event, _context) = event.into_parts();
    let value = event.key;

    let body = format!("Hello {}", value);
    let response = Response { body };
    Ok(response)
}
