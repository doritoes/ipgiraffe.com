# Create Rust Lambda Function
Where we demonstrate creating a Lambda function written in Rust on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below.

WORK IN PROGRESS

## Prerequisites



## Create the Project

3.  Insert the example code from [lambda_test.rs](lambda_test.rs)
## Build the Executable

## Package the ZIP file for upload

lambda-handler-rust.zip
# Create function
## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scratch
    2. Function name: **myIPFunctionRest**
    3. Runtime: **Amazon Linux**
    4. Architecture: the architure you build your function for
        - make sure it matches the function you built
    5. Leave the rest at defuaults, click **Create function**
2. In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **lambda-handler-rust.zip** file code, and click **Save**
3. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed
