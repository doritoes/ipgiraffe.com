using Amazon.Lambda.APIGatewayEvents;
using Amazon.Lambda.Core;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq; 

// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]

namespace myip 
{
    public class MyEventInput
    {
        public string sourceIp { get; set; }
        public string xForwardedFor { get; set; }
    }
    public class Function 
    {
        public APIGatewayProxyResponse FunctionHandler(MyEventInput input, ILambdaContext context)
        {
            string clientIp = "IP Address Not Found"; // Default

            if (input != null) 
            {
                // 1. Prioritize "X-Forwarded-For" header
                if (!string.IsNullOrEmpty(input.xForwardedFor))
                {
                    string[] forwardedIps = input.xForwardedFor.Split(',');
                    clientIp = forwardedIps[0].Trim(); // Extract the first IP
                }
                // 2. Fallback to "sourceIp" 
                if (clientIp == "IP Address Not Found" && !string.IsNullOrEmpty(input.sourceIp))
                {
                    clientIp = input.sourceIp;
                }             
            }
            // Build the HTML response using string interpolation
            string htmlResponse = $@"
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
