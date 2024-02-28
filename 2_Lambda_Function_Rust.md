# Create Rust Lambda Function
Where we demonstrate creating a Lambda function written in Rust on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below. Note that you can also Rust install in WSL (Windows Subsystem for Linux). See the installation link below.

WORK IN PROGRESS

## Prerequisites
1. Install Rust on your local machine
    - https://www.rust-lang.org/tools/install
    - Rust is installed and managed by the `rustup` tool
    - Download RUSTUP-INIT.EXT (64-BIT)
    - Run the program
    - Select (1) Quick install of Rust Visual C++ prerequisites
    - Click Install for Visual Studio
    - Be very patient for all of the Visual Studio components and packages are installed
    - Select (1) Proceed with installation (default)
    - Re-open your terminal/command prompt so the commands will work


## Create the Project
1. Open a terminal and navigate to your desired project directory
3. `cargo new ip_lambda_rust`
4. `cd ip_lambda_rust`
5. Edit the `Cargo.toml` file to add the following dependencies under `[dependencies]`
    - serde = "1.0"
    - serde_json = "1.0"
    - lambda_runtime = "0.6"
    - tokio = { version = "1", features = ["rt-multi-thread"] } # Required for async in Lambda
    - aws_lambda_events = { version = "0.5", features = ["rustls-tls"] } 
7.  Edit the handler `src/main.rs` and insert the example code from [lambda_test.rs](lambda_test.rs)
 
## Cross-Compilation for Amazon Linux 2023


## Package the ZIP file for upload
Create the ZIP file **lambda-handler-rust.zip** containing your compiled binary

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
