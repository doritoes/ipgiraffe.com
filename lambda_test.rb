require 'json'

def lambda_handler(event:, context:)
  if event.key?('xForwardedFor')
    value = event['xForwardedFor'].split(',')[0]
  elsif event.key?('sourceIp')
    value = event['sourceIp']
  else
    value = 'IP Address Not Found'
  end

  html_response = <<~HTML
    <html>
    <body>
        <h1>Your IP Address</h1>
        <p>#{value}</p>
    </body>
    </html>
  HTML
  
  {
    statusCode: 200,
    headers: { 'Content-Type': 'text/html' },
    body: html_response
  }
end
