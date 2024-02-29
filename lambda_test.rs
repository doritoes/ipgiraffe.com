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

  let body = if let Some(xff) = event.xForwardedFor {
    xff.split(',').next().unwrap_or_default().to_string() 
  } else if let Some(sip) = event.sourceIp { 
    sip 
  } else {
    "IP Address Not Found".to_string() 
  };
  let html_response = format!(r#"
  <html>
  <body>
    <h1>Your IP Address</h1>
    <p>{}</p>
  </body>
  </html>
  "#, body);
  Ok(Response { body: html_response })
}
