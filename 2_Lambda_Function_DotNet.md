# Create .NET Lambda Function
We are going to demonstrate using C# code. Other options for this runtime included F# and PowerShell.

These instructions were tested on a Windows 11 system.

## Prerequisites
1. Install [Visual Studio Code](https://code.visualstudio.com/download) on your workstation
3. Install the [.NET 8 SDK](https://dotnet.microsoft.com/download) (or the latest LTS version)
    - verify the runtimes you have installed: `dotnet --info`
4. Create a Lambda project
    - Open Terminal (Powershell or your preferred command-line interface will work)
    - Navigate to the directory where you want to create the project folder
    - Create the project folder and change to it
      - `mkdir MyLambdaProject`
      - `cd MyLambdaProject`
    - Update to latest dotnet templates
      - `dotnet new update`
      - `dotnet new install Amazon.Lambda.Templates`
    - Install global CLI tools
      - `dotnet tool install -g Amazon.Lambda.Tools`
    - use the 'dotnet' command to create a new C# Lambda project
      - `dotnet new lambda.EmptyFunction`
    - use 'dotnet' command to install dependencies (packages)
      - `dotnet add package Amazon.Lambda.Core`
      - `dotnet add package Amazon.Lambda.Serialization.SystemTextJson`
      - FAILS
5. Write the Lambda Code
    - Open Visual Studio Code
    - Open the newly created project folder in VS Code (*Tip*: `code .`)
    - Open `Function.cs`
    - Paste code from [lambda_test.cs](lambda_test.cs)
6. Build the Project
    - In the VS Code terminal run
      - `dotnet build -c Release`

This page needs more work and testing
