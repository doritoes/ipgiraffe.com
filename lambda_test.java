package com.doritoes;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyRequestEvent;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyResponseEvent;
import com.fasterxml.jackson.databind.ObjectMapper; 

public class MyRequestHandler implements RequestHandler<APIGatewayProxyRequestEvent, APIGatewayProxyResponseEvent> {

    private static final int SUCCESS_STATUS_CODE = 200;
    private static final ObjectMapper objectMapper = new ObjectMapper();

    // Data classes to hold event and response details 
    public static class Event {
        public String sourceIp;
        public String xForwardedFor;
    }

    public static class Response {
        public String body;
        public int statusCode;  
        public boolean isBase64Encoded; 
    }

    @Override
    public APIGatewayProxyResponseEvent handleRequest(APIGatewayProxyRequestEvent event, Context context) {

        Event inputEvent = null;
        String ipAddress = "IP Address Not Found";

        try {
            inputEvent = objectMapper.readValue(event.getBody(), Event.class); // Deserialize JSON from the event body

            if (inputEvent.xForwardedFor != null) {
                ipAddress = inputEvent.xForwardedFor.split(",")[0];
            } else if (inputEvent.sourceIp != null) {
                ipAddress = inputEvent.sourceIp;
            }

        } catch (Exception e) {
            // Handle JSON parsing exceptions if needed
            System.err.println("Error parsing input event: " + e.getMessage());
        } 

        // Fixed HTML formatting: escape '#' characters
        String htmlResponse = "<html>\n" +
                      "<body>\n" +
                      "  <h1>Your IP Address</h1>\n" +
                      "  <p>" + ipAddress + "</p>\n" +
                      "</body>\n" +
                      "</html>\n";

        Response response = new Response();
        response.body = htmlResponse;
        response.statusCode = SUCCESS_STATUS_CODE;
        response.isBase64Encoded = false; 

        return new APIGatewayProxyResponseEvent()
                .withHeaders(null) 
                .withStatusCode(response.statusCode)
                .withBody(response.body)
                .withIsBase64Encoded(response.isBase64Encoded);
    }
}
