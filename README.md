# ipgiraffe.com
How [ipgiraffe.com](https://www.ipgiraffe.com) was built on [AWS Lambda](https://aws.amazon.com/pm/lambda/).

# Overview and Genesis
After pondering on what would make a good example use case for a Lambda instance, I decided on the simplest use case I could think of. Drawing on inspiration from [ipturtle](http://ipturtle.com) and [ipchicken](https://ipchicken.com), I decided on very simple site that returns the user's IP address. Frequently, I need a client to provide their public IP address. There are many sites loaded with ads. And some make it difficult to find the IP address! I often direct people to [ipchicken](https://ipchicken.com) or [ipdragon](http://ipdragon.com). Now I can offer a simpler alternative!

This demonstration site has the following features:
- No ads
- Single function with no external images, scripts, or stylesheets (there is one Google font that is loaded in the browser)
- Just the IP address, no extra information
- Extremely cheap to operate (there aren't any logs we have to store to a S3 bucket)
This breaks the mold of the typical Lambda use case
- Does not return JSON, but instead HTML
- Not an API with an authentication backend

# Project Goals
Here are the goals I have for this project. If you would like to encourage me to add additional goals or to complete these goals,  I'm open to [contributions](https://account.venmo.com/u/unclenuc) to pay my Cloud bills.
## In Scope
### Completed
- Lambda function returns HTML
- Returns the source IP address
- Configured API Gateway
- Configured domain name
- Configured CloudFront including SSL certificate
- When proxied by CloudFront or other proxies, leveral X-Forwarded-For headers to show the client's IP address
- Apply giraffe-themed styling to the page
- Added HTML comments to the source code
### Doing
- document step-by-step how I did this so you can learn from my experience
### Will Do
- text other runtimes
### Might Do
- Base-64 in-page image of a giraffe
- Create another site based on GCP serverless computing
- Create another site based on Azure serverless computing
## Out of Scope
- The original goal of deploying the Lambda function, distributed API Gateways, and Cloudfront configurations using CloudFormation is now out of score due to the limitations of AWS to meet my needs

# Step-by-Step
1. [Pre-Requisites](1_Prerequisites.md)
2. [Create Lambda Function](2_Lambda_Function.md)
3. [Create API Gateway](3_API_Gateway.md)
4. [Create CloudFront](4_CloudFront.md)
5. [Update DNS](5_Route_53.md)
6. [Testing and Monitoring](6_Testing_and_Monitoring.md)
7. [Adding Regions](7_Regions.md)
8. [Next Steps](8_Next_Steps.md)

# References
My favorite IP address checking web sites:
- https://ipgoat.com
- https://ipchicken.com
- http://icanhazip.com (just the IP address, great from command line: `curl http://icanhazip.com`)
- http://www.ipdragon.com
- http://ipturtle.com
- https://ip.me

My IP address checking web sites:
- https://ipgiraffe.com
- https://ipdice.com (under construction)

My other web sites:
- https://unclenuc.com
- https://systems-monitor.com/
- https://www.cottagewifi.com/ (I have a lot of content that I want to share here)
