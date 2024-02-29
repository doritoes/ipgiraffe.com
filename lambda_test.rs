use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {
    key: String,
    color: Option<String>, // Make 'color' optional
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

    // Handle the color if it exists
    let color_part = if let Some(color) = event.color {
        format!(" and {}", color)
    } else {
        String::new() // Empty string if color is absent
    };

    let body = format!("Hello {}{}", value, color_part);
    let response = Response { body };
    Ok(response)
}
