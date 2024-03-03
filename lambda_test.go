package main

import (
	"context"
	"fmt"
	"net/http"
	"strings"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

type MyEvent struct {
	SourceIP   string `json:"sourceIp"`
	XForwarded string `json:"xForwardedFor"`
}

func handler(ctx context.Context, event *MyEvent) (events.APIGatewayProxyResponse, error) {
	var clientIp string
	if event == nil {
		return events.APIGatewayProxyResponse{
			StatusCode: http.StatusBadRequest, // Or another appropriate error code
			Body:       "Error: Received nil event",
			Headers:    map[string]string{"Content-Type": "text/plain"}, // Adjust if needed
		}, fmt.Errorf("received nil event")
	}
	if event.XForwarded != "" {
		clientIp = strings.Split(event.XForwarded, ",")[0]
	} else if event.SourceIP != "" {
		clientIp = event.SourceIP
	} else {
		clientIp = "IP Address Not Found"
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
