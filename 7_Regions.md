# Adding Regions
Our test application https://www.ipgiraffe.com is running just fine. How can we make it more globally accessible and performant?

*See https://aws.amazon.com/blogs/networking-and-content-delivery/latency-based-routing-leveraging-amazon-cloudfront-for-a-multi-region-active-active-architecture/*

## Primary and Failover Regions
In this example we are doing to use:
- us-east-1 (N. Virginia)
- us-west-2 (Oregon)
- eu-central-1 (Frankfurt)

Overview:
- add the Lambda function to each region
- create the API gateway in reach region
- add new API gateway function URL to CloudFront

### Steps
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lambda" and click on **Lambda**
3. Locate the Region dropdown at the top of the AWS Console next to your account name
4. Click the dropdown and select the region you want to add
5. Create the Lambba function (refer to [Create Lambda Function](2_Lambda_Function.md))
6. Create the API gateway (refer to [Create API Gateway](3_API_Gateway.md))
7. Navigate the CloudFront in the AWS console
    - note the region is *Global* for CloudFront
8. Select your CloudFront distribution
9. Click on the *Origins* tab (refer to [Create CloudFront](4_CloudFront.md)
10. Click **Create origin**
    - Origin name: *paste your invoke URL here*
    - You can find this in your API Gateway console; one way is open the API, click **Stages** and find *Invoke URL*
      - Example: (t2efulpgok.execute-api.us-west-2.amazonaws.com/Prod)
    - Origin path - optional: *leave blank*
    - Name: **myIP-backend-prod-us-west-2** (this name must be unique)
    - Leave the remaining Origin settings at their default values
    - Click **Create origin**
11. Click **Create origin group**
    - Select each origin and click Add
    - Name: myIP-origin-group
    - Failover criteria: (required)
      - 502
      - 503
      - 504
    - Click **Create origin group**
12. Edit Default behavior
    - Click **Behaviors** tab
    - Select the *Default* behavior and click **Edit**
    - Change *Origin and origin groups* to **myIP-origin-group**
    - Click **Save changes**

## Active-Active Regions
We are going to add our configuration to multiple regions and allow CloudFront to direct users to the closest region.

Overview
- create a matching SSL certificate in ACM <u>in the additional region</u>
- configure Route 53 to have two records for the same domain name and set the routing policy to *latency*

### Steps

In progress
