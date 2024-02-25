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
      - `go get github.com/aws/aws-lambda-go/events`
      - `go get github.com/aws/aws-lambda-go/lambda`
    - Set environment variables (temporary for this session)
      - `GOOS=linux`
      - x86_64 architecture: `GOARCH=amd64`
      - arm64 archtecture: `GOARCH=arm64`
      - Windows
        - `SET GOOS=linux`
        - `SET GOARCH=amd64`
        - `go env`
    - Build command
      - Run the command in your project folder
      - `go build main.go`
      - this produces an executable file named `main`
6. Create the Lambda function
    - Name
    - Runtime
    - Architecture
    - Upload code
      - upload a .zip file
      - create a ZIP file containiner your exectable `main`
      - upload the ZIP file
    - click Create Function
      
