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
#### Re-Create SSL Certificate
(www.ipgiraffe.com) was initially created in us-east-1. The SSL certificate was therefore created in us-east-1. We need to add a new subdomain to the certificate.
1. Browse to (https://console.aws.amazon.com) and log in
2. Make sure you have the correction region selected in the region dropdown
3. In the search bar enter "certificate" and click on **Certificate Manager**
4. Click **Request a certificate**
    - Certificate type *Public*, click **Next**
    - Enter the domain names you will use using the button *Add another name to this certificate*. See examples below.
      - ipgiraffe.com
      - www.ipgiraffe.com
      - app.ipgiraffe.com
    - Validation method: **DNS validation**
    - Click **Request**
      - Click **List certificates**, click view your certificate
      - The status will be *Pending validation*
      - Click on the certificate
      - Click **Create Records in Route 53**
      - Click **Create records** (This allows the certificate to successfully validate; it shouldn't take long)
5. Modify your CloudFront distribution to use the new certificate
    - Navigate to CloudFront > Distributions
    - Click on your distribution
    - Click **Edit**
    - Select the new SSL certificate
    - Click **Save changes**
    - At this point you can navigate to web site, click refresh and examine the certificate to find the new *Subject Alternative Names* (SAN) listed
6. Remove the old certificate
    - Navigate to Certificate Manager > List certificate
    - Note that the new certificate has *In use* as *Yes*
    - Select the unused certificate, click **Delete**, and confirm
#### Create SSL Certificate in the New Region
(www.ipgiraffe.com) was initially created in us-east-1. The SSL certificate was therefore created in us-east-1. Next we will re-create the same SSL certificate in the new region (in my case, us-west-2)
1. Browse to (https://console.aws.amazon.com) and log in
2. Make sure you have the correction region selected in the region dropdown
3. In the search bar enter "certificate" and click on **Certificate Manager**
4. Click **Request a certificate**
    - Certificate type *Public*, click **Next**
    - Enter the domain names you will use using the button *Add another name to this certificate*. See examples below.
      - ipgiraffe.com
      - www.ipgiraffe.com
      - app.ipgiraffe.com
    - Validation method: **DNS validation**
    - Click **Request**
      - Click **List certificates**, click view your certificate
      - The status will be *Pending validation*
      - Click on the certificate
      - Click **Create Records in Route 53**
      - Click **Create records** (This allows the certificate to successfully validate; it shouldn't take long)

#### Route 53 Setup
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "route" and click on **Route 53**
3. Note that Route 53 is in the *Global* region
4. Click **Hosted zones**
5. Click on your domain name (in my example, ipgiraffe.com)
6. Click **Create record**
    - Record Name: **app** (or your designed subdomain)
    - Record Type: Leave as **A - IPv4 Address**
    - Alias: **YES**
    - Route traffic to:
      - Select **API Gateway REST APIs** from the dropdown menu
      - Region: Chooose the region
    - Value: Enter the domain name of your API gateway in this region (my example is 59o1ou36kb.execute-api.us-east-1.amazonaws.com)
    - Routing Policy: **Latency**
    - Region: Select your first region (in my case us-east-1)
    - Healtch check ID - options: **Leave blank**
    - Evaluate target health: **NO** (default) to save cost, or enable to allow Route 53 to monitor your API Gateway endpoint
    - Record ID: **app-us-east-1**
7. Click **Add another record** and repeat for your second region (in my case us-west-2)
    - As this is an advanced section of the lab, I am allowing you to determine the settings
8. Click **Create records**

#### CloudFront Setup
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "CloudFront" and click on **CloudFront**
3. Note that CloudFront is in the *Global* region
4. Find and click on your distribution
5. Click **Edit**
6. Alternate Domain Names - add the *app* subdomain record (see my example below)
    - add **app.ipgiraffe.com**
7. We recreated the Custom SSL certificate. Just make sure it is selected.
8. Click **Save changes**

### Testing
Generate traffic source from different global regions, followed by confirming traffic is reaching all regions.

Test the different results of using Cloudfront at `ipgiraffe.com` vs DNS latencey at `app.ipgiraffe.com`

#### Generate traffic with Online Speed Test Tools
These have limited locations, but can quickly renerate traffic to your different regions.
- Pingdom: https://tools.pingdom.com/ (select from *Test from*)
- WebPageTest: https://www.webpagetest.org/ (advanced options let you select more locations)

####  Generate traffic with VPN Services
Use a VPN service (some have free options) that lets you connect through servers in various countries. In this way you can access the site from, say, Seattle.

#### Validating Lambda funtion usage
1. In the search bar enter "CloudWatch" and click on **CloudWatch**
2. From the left Menu, expand **Metrics** and click **All metrics**
3. Find the *Lambda* chicklet and click **View automatic dashboard**
4. Use region dropdown to confirm you have invocations in each region

# Learning More
- Think about pricing model of CloudFront affects your ability to add your application to regious outside US and Europe
- Consider how additional regions could reduce your Lambda concurrency and possibly keep you in the Lambda "Free Tier"
- Consider how having an infrequently-used region could cause a "cold start" for user
- Generally think minutes, not hours for the timeout until a "cold start" is required. It's best to assume frequent cold starts.
- For critical applications, pay for *provisioned concurrency* which keeps a set number if instances "warm" (you are charged for every GB-second <u>plus</u> a small fee for each request your Lambda function handles while using provisioned concurrency. This can very per AWS region. Some provisioned concurrency usage may be included in the free tier.
