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
    
    if event.Body != "" {
        var eventData map[string]string
        err := json.Unmarshal([]byte(event.Body), &eventData) // Unmarshal JSON event body
        if err == nil {
            if sourceIp, ok := eventData["sourceIp"]; ok { // Check if key exists
                clientIp = sourceIp
            }
            if xForwardedFor, ok := eventData["xForwardedFor"]; ok { // Check if key exists
                clientIp = strings.Split(xForwardedFor, ",")[0]
            }

        }
    }
    // Prioritize X-Forwarded-For
    if xForwardedFor := event.Headers["x-Forwarded-For"]; xForwardedFor != "" {
        clientIp = strings.Split(xForwardedFor, ",")[0]
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
