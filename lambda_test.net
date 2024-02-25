using Amazon.Lambda.Core;
using Amazon.Lambda.APIGatewayEvents;

[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]

namespace MyIPLambda
{
    public class Function
    {
        public APIGatewayProxyResponse FunctionHandler(APIGatewayProxyRequest request, ILambdaContext context)
        {
            string clientIp = "IP Address Not Found";

            // Check X-Forwarded-For with potential multiple IPs
            if (request.Headers != null && request.Headers.TryGetValue("X-Forwarded-For", out string xForwardedFor))
            {
                clientIp = xForwardedFor.Split(',')[0];
            }

            // Check API Gateway's sourceIp
            if ("IP Address Not Found".Equals(clientIp) && request.RequestContext?.Identity?.SourceIp != null)
            {
                clientIp = request.RequestContext.Identity.SourceIp;
            }

            var htmlResponse = $@"
                <html>
                <body>
                    <h1>Your IP Address</h1>
                    <p>{clientIp}</p>
                </body>
                </html>
            ";

            return new APIGatewayProxyResponse
            {
                StatusCode = 200,
                Body = htmlResponse,
                Headers = new Dictionary<string, string> { { "Content-Type", "text/html" } }
            };
        }
    }
}
