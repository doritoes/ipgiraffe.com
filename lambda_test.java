import com.amazonaws.services.lambda.runtime.Context; 
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyRequestEvent;
import com.amazonaws.services.lambda.runtime.events.APIGatewayProxyResponseEvent;

public class IpLambdaHandler implements RequestHandler<APIGatewayProxyRequestEvent, APIGatewayProxyResponseEvent> {

    @Override
    public APIGatewayProxyResponseEvent handleRequest(APIGatewayProxyRequestEvent event, Context context) {
        String clientIp = "IP Address Not Found";

        // Prioritize X-Forwarded-For with potential comma-separated IPs
        if (event.getHeaders() != null) {
            String xForwardedFor = event.getHeaders().get("X-Forwarded-For");
            if (xForwardedFor != null) {
                clientIp = xForwardedFor.split(",")[0];
            }
        }

        // Fallback to API Gateway's sourceIp 
        if ("IP Address Not Found".equals(clientIp) && event.getRequestContext() != null) {
            clientIp = event.getRequestContext().getIdentity().getSourceIp();
        }

        String htmlResponse = String.format("""
                <html>
                <body>
                    <h1>Your IP Address</h1>
                    <p>%s</p>
                </body>
                </html>
                """, clientIp);

        APIGatewayProxyResponseEvent response = new APIGatewayProxyResponseEvent();
        response.setStatusCode(200);
        response.setBody(htmlResponse);
        response.getHeaders().put("Content-Type", "text/html");
        return response;
    }
}
