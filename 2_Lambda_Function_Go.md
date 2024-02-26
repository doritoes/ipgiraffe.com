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
    - Create a zip file named **go.zip**
    - Add the executable we just created: `bootstrap`
    - Add the [handler.sh](handler.sh) file
7. Create the Lambda function
    - Author from scratch
    - Function name: **myIPFunctionGo**
    - Runtime: **Amazon Linux**
    - Architecture: the architure you build your function for
    - Click **Create function**
    - In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **go.zip** file code, and click **Save**
    - Under *Code* > *Runtime settings* click **Edit**
    - Change the handler to **main.handler((
    - ** continue writing here **     
    - Click **Test** then the orange **Test** button
      
Currently this is failing

~~~
{
  "errorType": "Runtime.InvalidEntrypoint",
  "errorMessage": "RequestId: 55fac1c7-b7a1-4af5-96f4-c3e79d8a2b20 Error: fork/exec /var/task/bootstrap: no such file or directory"
}

INIT_REPORT Init Duration: 0.79 ms	Phase: init	Status: error	Error Type: Runtime.InvalidEntrypoint
INIT_REPORT Init Duration: 0.61 ms	Phase: invoke	Status: error	Error Type: Runtime.InvalidEntrypoint
START RequestId: 55fac1c7-b7a1-4af5-96f4-c3e79d8a2b20 Version: $LATEST
RequestId: 55fac1c7-b7a1-4af5-96f4-c3e79d8a2b20 Error: fork/exec /var/task/bootstrap: no such file or directory
Runtime.InvalidEntrypoint
END RequestId: 55fac1c7-b7a1-4af5-96f4-c3e79d8a2b20
REPORT RequestId: 55fac1c7-b7a1-4af5-96f4-c3e79d8a2b20	Duration: 10.05 ms	Billed Duration: 11 ms	Memory Size: 128 MB	Max Memory Used: 3 MB	
~~~
