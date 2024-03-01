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
        headers.put("Content-Type", "application/json");
        response.setHeaders(headers);

        try {
            // Cast the input to a Map (since it's direct JSON input in a test)
            Map inputMap = objectMapper.convertValue(input, Map.class);

            String value = (String) inputMap.get("foo"); 

            response.setBody("{\"body\": \"" + value + "\"}");
        } catch (Exception e) {
            // Handle error with more descriptive message
            response.setBody("{\"error\": \"Error processing input: " + e.getMessage() + "\"}");
            response.setStatusCode(500);
        }

        return response;
    }
}
