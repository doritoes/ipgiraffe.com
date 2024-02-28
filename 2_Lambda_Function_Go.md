# Create Go Lambda Function
Where we demonstrate creating a Lambda function written in Go on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below.

## Prerequisites
1. Install Go Development Environment on your local machine
    - Download the installer from https://go.dev/dl/
    - Default settings are OK
    - Verify installation at command prompt `go version`
## Create the Project
1. Create a project folder (e.g., ipgiraffe)
2. Create a new file in your project folder named `main.go`
3.  Insert the example code from [lambda_test.go](lambda_test.go)
## Build the Executable
1. Download and initialize dependencies (i.e., AWS Lambda Go libraries)
    - `go mod init lambda-test`
    - `go get -u github.com/aws/aws-lambda-go`
    - `go get github.com/aws/aws-lambda-go/events`
    - `go get github.com/aws/aws-lambda-go/lambda`
    - `go.exe get -v -x -u github.com/aws/aws-lambda-go/cmd/build-lambda-zip`
      - in testing, this was required for `build-lambda-zip.exe` to be created
2. Set environment variables (temporary for this session)
    - `GOOS=linux`
    - x86_64 architecture: `GOARCH=amd64`
    - arm64 archtecture: `GOARCH=arm64`
    - Windows cmd example
      - `SET GOOS=linux`
      - `SET GOARCH=amd64`
      - `go env`
    - Windows powershell example
      - `$ENV:GOOS = "linux"`
      - `$ENV:GOARCH = "amd64"`
3. Run the build command
    - Run the command in your project folder
    - `go build -o bootstrap main.go`
    - this produces an executable file named `bootstrap`
## Package the ZIP file for upload
Creating the Zip file is extremely challenging on Windows, and the requirements are strict
    - https://docs.aws.amazon.com/lambda/latest/dg/golang-package.html
    - https://github.com/aws/aws-lambda-go/tree/main/cmd/build-lambda-zip
1. Get the tool
    - `go.exe install github.com/aws/aws-lambda-go/cmd/build-lambda-zip@latest`
2. Create the ZIP file named **lambda-handler.zip** (example is for cmd.exe)
    - `set GOOS=linux`
    - `set GOARCH=amd64`
    - `go build -o bootstrap man.go`
    - `%USERPROFILE%\Go\bin\build-lambda-zip.exe -o lambda-handler.zip bootstrap`
# Create function
## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scratch
    2. Function name: **myIPFunctionGo**
    3. Runtime: **Amazon Linux**
    4. Architecture: the architure you build your function for
        - make sure it matches the function you built
    5. Leave the rest at defuaults, click **Create function**
2. In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **lambda-handler.zip** file code, and click **Save**
3. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed
