package com.doritoes;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.google.gson.Gson;
import com.google.gson.JsonObject;

import java.util.HashMap;
import java.util.Map;

public class MyRequestHandler implements RequestHandler<Map<String, Object>, Map<String, Object>> { 
    @Override
    public Map<String, Object> handleRequest(Map<String, Object> event, Context context) {
        String inputString = event.containsKey("xForwardedFor") ? ((String) event.get("xForwardedFor")).split(",")[0] :
                             event.containsKey("sourceIp") ? (String) event.get("sourceIp") : "IP Address Not Found";
        Map<String, Object> responseBody = new HashMap<>();
        String outputString = 
            "<html>" +
            "<body>" + 
            "<h1>Your IP Address</h1>" + 
            "<p>" + inputString + "</p>" + 
            "</body>" +
            "</html>";
        responseBody.put("body", outputString);
        return responseBody; 
    }
}
