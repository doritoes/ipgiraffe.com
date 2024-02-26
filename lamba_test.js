export const handler = async (event) => {
  const sourceIp = event.sourceIp;
  let clientIp;
  if (event.xForwardedFor && event.xForwardedFor !== "") {
      clientIp = event.xForwardedFor.split(",")[0];
  } else if(event.sourceIp && event.sourceIp !== "") {
      clientIp = event.sourceIp;
  } else {
      clientIp = "IP Address Not Found";
  }
const htmlResponse = `
    <html>
    <body>
        <h1>Your IP Address</h1>
        <p>${clientIp}</p>
    </body>
    </html>
    `;
  const response = {
    statusCode: 200,
    body: htmlResponse,
    headers: { "Content-Type": "text/html" } 
  };
  return response;
};
