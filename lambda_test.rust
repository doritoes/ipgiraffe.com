use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use hyper::{Body, Request, Response, StatusCode};

#[derive(Deserialize)]
struct Event {
    #[serde(rename = "requestContext")]
    request_context: RequestContext,
    headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Deserialize)]
struct RequestContext {
    identity: Identity,
}

#[derive(Deserialize)]
struct Identity {
    #[serde(rename = "sourceIp")]
    source_ip: Option<String>,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: Event, _: Context) -> Result<Response<Body>, Error> {
    let client_ip = get_client_ip(&event);

    let html_response = format!(
        r#"
        <html>
        <body>
            <h1>Your IP Address</h1>
            <p>{}</p>
        </body>
        </html>
        "#,
        client_ip
    );

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(html_response))?;

    Ok(response)
}

fn get_client_ip(event: &Event) -> String {
    if let Some(headers) = &event.headers {
        if let Some(x_forwarded_for) = headers.get("X-Forwarded-For") {
            return x_forwarded_for.split(',').next().unwrap_or("").to_string();
        }
    }
    if let Some(source_ip) = &event.request_context.identity.source_ip {
        return source_ip.to_string();
    }

    "IP Address Not Found".to_string()
} 
