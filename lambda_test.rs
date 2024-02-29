use lambda_runtime::{handler_fn, Context, Error};
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
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Event, _: Context) -> Result<Response, Error> {
    let value = event.key;
    let body = format!("Hello {}", value);

    let response = Response { body };
    Ok(response)
}
