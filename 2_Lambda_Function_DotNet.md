# Create .NET Lambda Function
We are going to demonstrate using C# code. Other options for this runtime included F# and PowerShell.

These instructions were tested on a Windows 11 system.

*See https://awswith.net/2020/02/11/the-simplest-dot-net-core-aws-lambda-function/*

## Prerequisites
1. Install [Visual Studio Code](https://code.visualstudio.com/download) on your workstation
2. Install the [.NET 8 SDK](https://dotnet.microsoft.com/download) (or the latest LTS version)
    - verify the runtimes you have installed: `dotnet --info`
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
    - `cd myip\src\myip`
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
    - `dotnet publish -c Release -r linux-x64 -p:PublishReadyToRun=true -o ./bin/Release/net8.0/publish`
# Package the ZIP file for upload
1. Navigate to your project folder, then to the `src\myip\bin\Release\net8.0\publish` folder
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
2. Configure the handler
    - From the Code tab, click **Edit** (next to *Runtime settings*)
      - `myip::myip.Function::FunctionHandler`
    - Click **Save**
2. In the *Code source* pane, click **Upload from** > **.zip file**
    - Click Upload, select your **LambdaDeploymentPackage.zip** file, and click **Save**
4. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed
