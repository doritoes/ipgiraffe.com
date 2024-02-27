# Create .NET Lambda Function
We are going to demonstrate using C# code. Other options for this runtime included F# and PowerShell.

These instructions were tested on a Windows 11 system.

*See https://awswith.net/2020/02/11/the-simplest-dot-net-core-aws-lambda-function/*

## Prerequisites
1. Install [Visual Studio Code](https://code.visualstudio.com/download) on your workstation
3. Install the [.NET 8 SDK](https://dotnet.microsoft.com/download) (or the latest LTS version)
    - verify the runtimes you have installed: `dotnet --info`
    - install the base Linux-x64 runtime components for .NET 8 installed
      - Download Page: https://dotnet.microsoft.com/en-us/download
      - Under .NET 8, find the section for "Run apps - Runtime".
      - Choose the download specifically for Linux-x64. The installer file name will likely include a linux-x64 Runtime Identifier (RID).
      - Install:  Follow the installation instructions provided for your Linux distribution
      - Verify Installation: `dotnet --list-runtimes`
      - You should now see a Microsoft.NETCore.App runtime with the Linux-x64 RID listed

## Create a Lambda Project
1. Open Terminal (Powershell or your preferred command-line interface will work)
2. Navigate to the directory where you want to create the project folder
3. Create the a base folder for your dotnet work
    - `mkdir dotnet`
    - `cd dotnet`
4. Update to latest dotnet templates
    - `dotnet new update`
5. Apply templates
    - `dotnet new globaljson`
6. Install global CLI tools
    -  `dotnet tool install -g Amazon.Lambda.Tools`
    -  `dotnet new install Amazon.Lambda.Templates`
8. Create project and switch directory
    - `dotnet new lambda.EmptyFunction -n myip`
    - `cd src\myip`
    - `cd myip`
9. Add packages to the project
    - `dotnet add package Amazon.Lambda.Serialization.Json`
    - `dotnet add package Amazon.Lambda.Serialization.SystemTextJson`
    - `dotnet add package Amazon.Lambda.CloudwatchEvents`
    - `dotnet add package Amazon.Lambda.Core`
    - `dotnet add package AWSSDK.Lambda`
    - `dotnet add package Amazon.Lambda.APIGatewayEvents`
10. Write the Lambda Code
    - Open Visual Studio Code
    - Open the newly created project folder in VS Code (*Tip*: `code .`)
    - Edit `myip\src\myip\Function.cs`
    - Paste code from [lambda_test.cs](lambda_test.cs)
11. Build the Project
    - `dotnet build -c Release`
12. Publish the Project
    - `dotnet publish -c Release -r net8.0 -o ./bin/Release/net8.0/publish`
# Package the ZIP file for upload
1. Navigate to your project folder, then to the `bin/Release/net8.0/publish` folder
2. Create a zip file using using your preferred compression tool
    - Name: **LambdaDeploymentPackage.zip**
    - Contents: *all the files in the `bin/Release/net8.0/publish` folder
    - Dependencies: *The zip file must contain all your code files and any required .NET assemblies (dependencies) your project uses*
    - Flat Structure: *Place the files directly at the root level of the zip file, not within an additional folder*

# Create function
## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scratch
    2. Function name: **myIPFunction**
    3. Runtime: *latest version of .NET*
    4. Leave the rest at defaults, click **Create function**
2. In the *Code source* pane, click **Upload from** > **.zip file**
    - Click Upload, select your **LambdaDeploymentPackage.zip** file, and click **Save**
4. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed.

FAILS at test

~~~
Executing function: failed (logs )
Details
The area below shows the last 4 KB of the execution log.
{
  "errorType": "Runtime.ExitError",
  "errorMessage": "RequestId: dde1acb5-7158-4518-9547-a5beae8996d2 Error: Runtime exited with error: exit status 106"
}
Summary
Code SHA-256
6CdlPrQ8PDxEZ7m8JPSzZgL66zvivIKbI8H+99q0v9U=
Execution time
1 minute ago (February 27, 2024 at 09:19 AM EST)
Request ID
dde1acb5-7158-4518-9547-a5beae8996d2
Function version
$LATEST
Duration
35.50 ms
Billed duration
36 ms
Resources configured
512 MB
Max memory used
3 MB
Log output
The section below shows the logging calls in your code. Click here  to view the corresponding CloudWatch log group.
Error: .NET binaries for Lambda function are not correctly installed in the /var/task directory of the image when the image was built. The /var/task directory is missing the required .runtimeconfig.json file.
INIT_REPORT Init Duration: 19.27 ms	Phase: invoke	Status: error	Error Type: Runtime.ExitError
START RequestId: dde1acb5-7158-4518-9547-a5beae8996d2 Version: $LATEST
RequestId: dde1acb5-7158-4518-9547-a5beae8996d2 Error: Runtime exited with error: exit status 106
Runtime.ExitError
END RequestId: dde1acb5-7158-4518-9547-a5beae8996d2
REPORT RequestId: dde1acb5-7158-4518-9547-a5beae8996d2	Duration: 35.50 ms	Billed Duration: 36 ms	Memory Size: 512 MB	Max Memory Used: 3 MB	

Test event
~~~
