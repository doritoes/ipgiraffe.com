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
    - Record name: www, ie. www.ipgiraffe.com
    - Record type: **CNAME**
    - Alias: **OFF**
    - Value: **Your domain**, i.e. ipgiraffe.com
    - TTL: **300** (default) or set to 60 seconds for your lab testing
    - Routing policy: **Simple routing** (default)
    - Click **Create records**



## Testing
1. From inside Route 353 click **Hosted zones** from the menu on the left
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
3. nslookup
    - from a command line test both DNS lookups (allow type for DNS propagation)
      - ```nslookup ipgiraffe.com```
      - ```nslookup www.ipgiraffe.com```
4. Browser testing:
    - HTTP to the domain name (i.e., http://ipgiraffe.com which should redirect to https)
    - HTTPS to the domain name (i.e., https://ipgiraffe.com)
    - HTTP to the alias (i.e., http://www.ipgiraffe.com which should redirect to https)
    - HTTPS to the alias (i.e., https://www.ipgiraffe.com)
5. DNS global propagation
    - Visit https://dnschecker.org/
    - Enter your domain name and click Search
    - Examples:
      - https://dnschecker.org/#A/ipgiraffe.com
      - https://dnschecker.org/#CNAME/www.ipgiraffe.com
