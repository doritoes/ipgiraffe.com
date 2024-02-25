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
      - Windows example
        - `SET GOOS=linux`
        - `SET GOARCH=amd64`
        - `go env`
    - Build command
      - Run the command in your project folder
      - `go build main.go`
      - this produces an executable file named `main`
6. Create the Zip file
    - Create a zip file named **go.zip**
    - Add the execuate we just created: `main`
    - Add the bootstrap script [bootstrap.sh](bootstrap.sh)
7. Create the Lambda function
    - Author from scratch
    - Function name: **myIPFunctionGo**
    - Runtime: **Amazon Linux**
    - Architecture: the architure you build your function for
    - Click **Create function**
    - In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **go.zip** file code, and click **Save**
      - upload a .zip file
      - create a ZIP file containiner your exectable `main`
      - upload the ZIP file
    - Click **Test** then the orange **Test** button
      
Currently this is failing

~~~
2024-02-25T14:36:29.024-05:00
INIT_START Runtime Version: provided:al2023.v12	Runtime Version ARN: arn:aws:lambda:us-east-1::runtime:136ed310b999b334fac0d4a087dd5c3d4f95efe17b434ba3c1eefb429c00f150

Copy
INIT_START Runtime Version: provided:al2023.v12 Runtime Version ARN: arn:aws:lambda:us-east-1::runtime:136ed310b999b334fac0d4a087dd5c3d4f95efe17b434ba3c1eefb429c00f150

2024-02-25T14:36:29.024-05:00
INIT_REPORT Init Duration: 0.21 ms	Phase: init	Status: error	Error Type: Runtime.InvalidEntrypoint

Copy
INIT_REPORT Init Duration: 0.21 ms Phase: init Status: error Error Type: Runtime.InvalidEntrypoint

2024-02-25T14:36:29.043-05:00
INIT_REPORT Init Duration: 0.23 ms	Phase: invoke	Status: error	Error Type: Runtime.InvalidEntrypoint

Copy
INIT_REPORT Init Duration: 0.23 ms Phase: invoke Status: error Error Type: Runtime.InvalidEntrypoint

2024-02-25T14:36:29.053-05:00
START RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c Version: $LATEST

Copy
START RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c Version: $LATEST

2024-02-25T14:36:29.075-05:00
RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c Error: Couldn't find valid bootstrap(s): [/var/task/bootstrap /opt/bootstrap]
Runtime.InvalidEntrypoint

Copy
RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c Error: Couldn't find valid bootstrap(s): [/var/task/bootstrap /opt/bootstrap] Runtime.InvalidEntrypoint

2024-02-25T14:36:29.075-05:00
END RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c

Copy
END RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c

2024-02-25T14:36:29.075-05:00
REPORT RequestId: d619c06a-7b1b-40e8-95d9-5adf1b31983c	Duration: 30.32 ms	Billed Duration: 31 ms	Memory Size: 128 MB	Max Memory Used: 2 MB	

Copy
~~~
