# Create API Gateway
The next step is to create the API Gateway. This acts as a "front door" for your Lambda function, providing a managed HTTP endpoint that clients can interact with to trigger the Lambda and receive responses.

## Log in to AWS Console and Navigate to API Gateway
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "API Gateway" and click on **API Gateway**

## Create our API
1. Next to "REST API", click **Build**
2. New API
3. API name: **myIP-api**
4. API endpoint type: **Edge-optimized** (we will be configuring CloudFront shortly)
5. Click **Create API**

## Configure our Resource
1. The "Resources" pane will be shown by default
2. On the right click **Create method**
    - Method type: **GET**
    - Integration type: **Lambda function** (default)
    - Lambda proxy integration: **OFF** (default)
    - Lambda function: Click in the the text box and the function you created will be listed; select it
    - Click **Create method**
### Method Request
A diagram of the client and how it communicates to/from the Lambda integration.
1. Click on **Method request** or select it from the ribbon menu below the diagram.

The default settings are what we want: no authorization or API key requirement

### Integration Request
1. Click on **Integration request** or select it from the ribbon menu.
2. Click **Edit**
  1. Request body passthrough: **Never**
  2. Expand Mapping templates
  3. Click **Add mapping template**
  4. Content type: <u>type in</u> **application/json** (this is IMPORTANT, you need to fill out this field)
  5. Template body: [mapping_template.vtl](mapping_template.vtl)
3. Click **Save**
4. In the ribbon bar, click the **Right >** arrow a few times to expose the *Test* tab
5. Click **Test** on the ribbon bar
6. Click the orange **Test** button. The test should succeed.
7. Click **Deploy API**
    - Stage: **New Stage**
    - Name: **Prod**
    - Click **Deploy**
8. Copy the Invoke URL to a browser.

The currrent results you will get are JSON. For example:
~~~
{"statusCode": 200, "body": "\n    <html>\n    <body>\n       <h1>Your IP Address</h1>\n       <p>123.45.67.89</p>\n    </body>\n    </html>\n    ", "headers": {"Content-Type": "text/html"}}
~~~

### Integration Response
1. Click on **Integration response** or select it from the ribbon menu.
2. Click **Edit**
3. Expand Mapping templates
4. Customize the template
    - Content type: <u>type in</u> **text/html**
	  - Template body: [response_template.vtl](response_template.vtl)
    - Click **Save**
5. Click **Deploy API** and deploy to Prod
6. Copy the Invoke URL to a browser. (you may need to refresh the page)

You will now get HTML, but it is still interpreted as JSON by the browser.

~~~
    <html>
    <body>
       <h1>Your IP Address</h1>
       <p>123.45.67.89</p>
    </body>
    </html>
~~~

### Method Response
1. Click on **Integration response** or select it from the ribbon menu.
2. Next to Response 200 click **Edit**
3. Change Content-Type from application/json to **text/html**
4. Click **Save**
5. Click **Deploy API** and deploy to Prod
6. Copy the Invoke URL to a browser. (you may need to refresh the page or be patient as the changes get deployed)

You will now have a web page.
