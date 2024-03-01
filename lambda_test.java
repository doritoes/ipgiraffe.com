package com.doritoes;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyRequestEvent;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyResponseEvent;
import com.fasterxml.jackson.databind.ObjectMapper; 

import java.util.HashMap;
import java.util.Map;

public class MyRequestHandler implements RequestHandler<Object, APIGatewayProxyResponseEvent> {

    @Override
    public APIGatewayProxyResponseEvent handleRequest(Object input, Context context) {
        ObjectMapper objectMapper = new ObjectMapper();
        APIGatewayProxyResponseEvent response = new APIGatewayProxyResponseEvent();
        response.setStatusCode(200);

        Map<String, String> headers = new HashMap<>();
        headers.put("Content-Type", "text/html"); // Set Content-Type to 'text/html'
        response.setHeaders(headers);

        try {
            Map inputMap = objectMapper.convertValue(input, Map.class);

            String value = null; 

            if (inputMap.containsKey("xForwardedFor")) {
                String values = (String) inputMap.get("xForwardedFor");
                value = values.split(",")[0]; 
            } else if (inputMap.containsKey("sourceIp")) {
                value = (String) inputMap.get("sourceIp");
            } else {
                value = "No IP found"; 
            }

            // Build HTML response
            String htmlResponse = 
                "<html>" +
                "<body>" +
                "<h1>Your IP Address</h1>" +
                "<p>" + value + "</p>" +
                "</body>" +
                "</html>";

            response.setBody(htmlResponse);

        } catch (Exception e) {
            response.setBody("{\"error\": \"Error processing input: " + e.getMessage() + "\"}");
            response.setStatusCode(500);
        }

        return response;
    }
}
