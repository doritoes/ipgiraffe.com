import json

def lambda_handler(event, context):
    if 'xForwardedFor' in event and event['xForwardedFor']:
        client_ip = event['xForwardedFor'].split(',')[0]
    elif 'sourceIp' in event:
        client_ip = event['sourceIp']
    else:
        client_ip = "IP Address Not Found"
    html_response = f"""
    <html>
    <body>
       <h1>Your IP Address</h1>
       <p>{client_ip}</p>
    </body>
    </html>
    """
    return {
        'statusCode': 200,
        'body': html_response,
        'headers': {
            'Content-Type': 'text/html'
        }
    }
