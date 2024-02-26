import AWS from 'aws-sdk';

exports.handler = async function (event, context) {
    if (!event) {
        return {
            statusCode: 400, // HTTP Bad Request
            body: "Error: Received nil event",
            headers: { "Content-Type": "text/plain" } 
        };
    }

    let clientIp;
    if (event.headers['X-Forwarded-For']) {
        clientIp = event.headers['X-Forwarded-For'].split(",")[0];
    } else if (event.requestContext.identity.sourceIp) {
        clientIp = event.requestContext.identity.sourceIp;
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

    console.log("IP Address:", event.requestContext.identity.sourceIp);
    console.log("X-Forwarded-For:", event.headers['X-Forwarded-For']);

    return {
        statusCode: 200, // HTTP OK
        body: htmlResponse,
        headers: { "Content-Type": "text/html" } 
    };
};
