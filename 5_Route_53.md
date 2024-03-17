# Update DNS
The final step to bring (https://www.ipgiraffe.com) is pointing our domain to the CloudFront instance.

## Log in to AWS Console and Navigate to CloudFront
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Route" and click on **Route 53**
   
## Create DNS Records
1. Click **Hosted zones** from the menu on the left
2. Click on your domain (ex., ipgiraffe.com)
3. Create an A Record for ipgiraffe.com
    - Click **Create record**
    - Record name: leave subdomain empty, i.e. just ipgiraffe.com
    - Record type: leave default **A**
    - Alias: **Yes**
    - Route traffic to: **Alias to CloudFront distribution**
    - Choose distribution: click in the box and choose your CloudFront distribution from the drop list
    - Routing policy: **Simple routing** (default)
    - Click **Create records**
4. Create CNAME record for www.ipgiraffe.com
    - Click **Create record**
    - Record name: www (i.e., www.ipgiraffe.com)
    - Record type: **CNAME**
    - Alias: **OFF**
    - Value: **Your domain** (i.e., ipgiraffe.com)
    - TTL: **300** (default) or set to 60 seconds for your lab testing
    - Routing policy: **Simple routing** (default)
    - Click **Create records**

## Testing
You can test DNS records from inside Route 53.
1. Click **Hosted zones** from the menu on the left
2. Click on your domain (ex., ipgiraffe.com)
3. Click **Test record**
    - First test - A Record
      - Record name: *leave blank*
      - Record type: **A**
      - Click **Get response**
      - Note the CloudFront IP addresses returned
    - Second test - CNAME
      - Record name: ***www***
      - Record type: **CNAME**
      - Click **Get response**
      - Note the alias points to ipgiraffe.com
