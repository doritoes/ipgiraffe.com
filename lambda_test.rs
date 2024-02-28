use lambda_runtime::{service_fn, Error};
use serde::{Deserialize, Serialize};
use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;

#[derive(Deserialize, Serialize)]
struct MyEvent {
    source_ip: Option<String>,     // Make fields optional 
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
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(e: ApiGatewayProxyRequest) -> Result<MyResponse, Error> {
    let event: MyEvent = serde_json::from_str(&e.body.unwrap_or_default())?; // Handle potential parse errors

    let client_ip = match (event.x_forwarded_for, event.source_ip) {
        (Some(x_forwarded), _) => x_forwarded.split(',').next().unwrap_or("IP Address Not Found").to_string(),
        (_, Some(source_ip)) => source_ip,
        _ => "IP Address Not Found".to_string(), 
    }; 

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

    Ok(MyResponse {
        status_code: 200,
        body: html_response,
        headers: [("Content-Type".to_string(), "text/html".to_string())].into_iter().collect(),
    }) 
}
