# Create Lambda Function
The first step is to create our function written in Python.

## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scracth
    2. Function name: **myIPFunction**
    3. Runtime: *latest version of Python*
    4. Leave the rest at defaults, click **Create function**
2. In the *Code Source* pane, replace the example function with one of the functions listed below
    - (lambda_test.py) - *easy to read simple version used in testing*
    - (lambda_handler.py) - *full function used to build (www.ipgiraffe.com)
3. Click **Deploy**
4. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: (test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed.

# Important Notes
If you create a function URL and try to test it in you will get an internal server error. This is because our Lambda function depends on the API gateway to provide the IP address.
