require 'json'

def lambda_handler(event:, context:)
  client_ip = "IP Address Not Found"

  # Check 'X-Forwarded-For' for potential multiple IPs
  if event['headers'] && x_forwarded_for = event['headers']['X-Forwarded-For']
    client_ip = x_forwarded_for.split(',')[0]
  end

  # Try API Gateway's sourceIp if not found
  if client_ip == "IP Address Not Found" && event['requestContext']['identity']['sourceIp']
    client_ip = event['requestContext']['identity']['sourceIp']
  end

  html_response = <<~HTML
    <html>
    <body>
        <h1>Your IP Address</h1>
        <p>#{client_ip}</p>
    </body>
    </html>
  HTML

  {
    statusCode: 200,
    body: html_response,
    headers: {
      'Content-Type' => 'text/html'
    }
  }
end
