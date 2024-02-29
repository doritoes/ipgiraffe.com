use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    key: String,
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
    let value = event.key;

    let first_color = if let Some(colors) = event.color {
        colors.split(',').next().unwrap_or_default() // Extract first color
    } else {
        "None".to_string() // Placeholder if no color is provided
    };

    let body = format!("Hello {} and {}", value, first_color);
    let response = Response { body };
    Ok(response)
}
