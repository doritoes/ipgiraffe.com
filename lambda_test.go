package main

import (
    "fmt"
    "net/http"
    "strings"
    "github.com/aws/aws-lambda-go/events"
    "github.com/aws/aws-lambda-go/lambda"
)

func handler(event events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
    clientIp := "IP Address Not Found"

    // Prioritize X-Forwarded-For
    if xForwardedFor := event.Headers["x-Forwarded-For"]; xForwardedFor != "" {
        clientIp = strings.Split(xForwardedFor, ",")[0]
    }

    // Fallback to API Gateway's sourceIp
    if clientIp == "IP Address Not Found" && event["SourceIp"] != "" {
        clientIp = event.RequestContext.Identity.SourceIP
    }

    htmlResponse := fmt.Sprintf(`
        <html>
        <body>
            <h1>Your IP Address</h1>
            <p>%s</p>
        </body>
        </html>
    `, clientIp)

    return events.APIGatewayProxyResponse{
        StatusCode: http.StatusOK,
        Body:       htmlResponse,
        Headers:    map[string]string{"Content-Type": "text/html"},
    }, nil
}

func main() {
    lambda.Start(handler)
}
