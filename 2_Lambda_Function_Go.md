# Create Go Lambda Function
Where we demonstrate creating a Lambda function written on go on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below.

1. Fetch the example code from [lambda_test.go](lambda_test.go)
2. Install Go Development Environment on your local machine
  - Download the installer from https://go.dev/dl/
  - Default settings are OK
  - Verify installation at command prompt `go version`
3. Create a project folder (e.g., ipgiraffe)
4. Create a new file in your project folder named `main.go`. Insert the example code.
5. Build the Executable
    - Download and initialize dependencies (i.e., AWS Lambda Go libraries)
      - `go mod init github.com/your-username/project-name`
      - `go mod init lambda-test`
      - `go get -u github.com/aws/aws-lambda-go`
      - `go get github.com/aws/aws-lambda-go/events`
      - `go get github.com/aws/aws-lambda-go/lambda`
      - `go.exe get -v -x -u github.com/aws/aws-lambda-go/cmd/build-lambda-zip`
        - in testing, this was required for `build-lambda-zip.exe` to be created 
    - Set environment variables (temporary for this session)
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
    - Build command
      - Run the command in your project folder
      - `go build -o bootstrap main.go`
      - this produces an executable file named `bootstrap`
6. Create the Zip file
    - This is extremely challenging on Windows, and the requirements are strict
    - https://docs.aws.amazon.com/lambda/latest/dg/golang-package.html
    - https://github.com/aws/aws-lambda-go/tree/main/cmd/build-lambda-zip
    - Get the tool
      - `go.exe install github.com/aws/aws-lambda-go/cmd/build-lambda-zip@latest`
    - Create the zip file named **lambda-handler.zip** (example for cmd.exe)
      - `set GOOS=linux`
      - `set GOARCH=amd64`
      - `go build -o bootstrap man.go`
      - `%USERPROFILE%\Go\bin\build-lambda-zip.exe -o lambda-handler.zip bootstrap`
7. Create the Lambda function
    - Author from scratch
    - Function name: **myIPFunctionGo**
    - Runtime: **Amazon Linux**
    - Architecture: the architure you build your function for
      - make sure it matches the function you built
    - Click **Create function**
    - In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **go.zip** file code, and click **Save**
    - Under *Code* > *Runtime settings* click **Edit**
    - Change the handler to **main.handler((
    - ** continue writing here **     
    - Click **Test** then the orange **Test** button
