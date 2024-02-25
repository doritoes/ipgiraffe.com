exports.handler = async function (event, context) {
  let client_ip;

  // Prioritize 'X-Forwarded-For', handling potential multiple IPs
  if (event.headers['X-Forwarded-For']) {
    client_ip = event.headers['X-Forwarded-For'].split(',')[0];
  } else if (event.requestContext && event.requestContext.identity.sourceIp) { 
    // Fallback to API Gateway's 'sourceIp'
    client_ip = event.requestContext.identity.sourceIp;
  } else {
    client_ip = "IP Address Not Found";
  }

  const html_response = `
    <html>
    <body>
        <h1>Your IP Address</h1>
        <p>${client_ip}</p>
    </body>
    </html>
  `;

  return {
    statusCode: 200,
    body: html_response,
    headers: {
      'Content-Type': 'text/html'
    }
  };
};
