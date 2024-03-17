# Testing and Monitoring
Your site should be up and running (allow time for DNS propagation)

## Browser Testing
Point your Browser to your site to test each way the site can be accessed
- HTTP to the domain name (i.e., http://ipgiraffe.com which should redirect to https)
- HTTPS to the domain name (i.e., https://ipgiraffe.com)
- HTTP to the alias (i.e., http://www.ipgiraffe.com which should redirect to https)
- HTTPS to the alias (i.e., https://www.ipgiraffe.com)

## nslookup
From a command line test both DNS lookups. For example:
- ```nslookup ipgiraffe.com```
- ```nslookup www.ipgiraffe.com```

## DNS global propagation
- Visit https://dnschecker.org/
- Enter your domain name and click Search
  - Examples:
    - https://dnschecker.org/#A/ipgiraffe.com
    - https://dnschecker.org/#CNAME/www.ipgiraffe.com
   
## Monitoring
Feel free to experiment with CloudWatch (search for "CloudWatch" in the AWS console). Be aware that configuring monitoring will generate additional traffic (load) to your Lamba function, API gateway, and to CloudFront.

Start with *view automatic dashboards* and work from there.

CloudWatch > Metrics > Lambda
- Invocations
- Average duration (you are billed by the duration times the amount of memory allocated)
- Errors
- Throttles (when the volume of requests to your function exceeds its available concurrency limit (how many instances of the Lambda function can run simultaneously)

CloudWatch > Metrics > API Gateway
- HTTP errors
- Latency

CloudWatch > Metrics > CloudFront
- Request count
- Error rates

# Learning More
How many instances can your application scale to?

## Lambda
1. Navigate to Lambda in the AWS Console
2. Click on your Lamba function
3. Click Configuration > Concurrency
    - Functional concurrency: Unreserved amount concurrency
    - Use unreserved amount concurrency: 1000

**IMPORTANT notes**
- Account vs. Region: Lambda concurrency limits exist both at the <ins>account level</ins> (across all regions) and at a <ins>per-region level</ins>.
- Reserved vs. Unreserved:
  - Reserved concurrency: A portion of concurrency you've specifically allocated to a function.
  - Unreserved concurrency: The remaining concurrency available for use by any of your Lambda functions in the account/region.
- Provisioned concurrency
  - Pre-Warmed Execution Environments: pre-allocate a specified number of Lambda execution environments to be always initialized and ready to process requests
  - Eliminates cold starts (the delay when Lambda needs to initialize a new execution environment) for lowest latency
  - You pay for provisioned concurrency even when your function isn't actively processing requests

What is a Lambda Throttle Event?
- Execution Limit: A Lambda throttle event occurs when the volume of requests to your Lambda function exceeds its available concurrency limit.
- Concurrency: This means how many instances of your Lambda function can run simultaneously.
- Protective Mechanism: Throttling is a way for AWS to protect the overall health of its Lambda service and ensure resources are shared fairly.

What Happens When Throttling Occurs?
- Requests Rejected: New requests triggering your Lambda function will receive an error (usually a 429 Too Many Requests response).
- Impact on Users: This leads to timeouts or errors from the user's perspective.

Why Throttling Might Happen:
- Sudden Traffic Spike: An unexpected surge of requests to your function.
- Default Concurrency Limits: New AWS accounts have default limits that might be too low for your application's workload.
- Burst Concurrency Limit: Each AWS region has limits on how quickly you can scale Lambda concurrency.

## API Gateway
API Gateway also has limits and they are different than Lambda concurrency limits.

Types of API Gateway Limits
- Requests per Second (RPS) Quota
  - This is the maximum number of requests your API Gateway can process per second.
  - The default is 10,000 RPS across all APIs in an account/region, but you can request increases.
- Burst Limit
  - API Gateway can handle short, temporary spikes in traffic above your sustained RPS quota
  - The default burst limit is 5,000 requests across all APIs
  
If you exceed your API Gateway RPS quota or burst limit, subsequent requests will receive a 429 Too Many Requests error.
