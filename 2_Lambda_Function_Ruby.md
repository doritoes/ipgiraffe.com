# Create Ruby Lambda Function
Where we demonstrate creating a Lambda function written in Ruby on the Ruby runtime.

1. Create the Lambda function
    - Author from scratch
    - Function name: **myIPFunctionRuby**
    - Runtime: **Ruby 3.2**
    - Architecture: the architure you want to run on
    - Click **Create function**
2. In the *Code Source* pane, replace the example function with the function listed below
    - [lambda_test.rb](lambda_test.rb) - *easy to read simple version used in testing*
3. Click **Deploy**
4. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed.
