import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.fasterxml.jackson.databind.ObjectMapper; // For JSON deserialization

public class MyIPHandler implements RequestHandler<Event, String> {

    private static final ObjectMapper objectMapper = new ObjectMapper();

    @Override
    public String handleRequest(Event event, Context context) {
        String body;
        if (event.getXForwardedFor() != null) {
            body = event.getXForwardedFor().split(",")[0];
        } else if (event.getSourceIp() != null) {
            body = event.getSourceIp();
        } else {
            body = "IP Address Not Found";
        }

        String htmlResponse = String.format(
                "<html><body><h1>Your IP Address</h1><p>%s</p></body></html>",
                body
        );

        return htmlResponse;
    }
}
