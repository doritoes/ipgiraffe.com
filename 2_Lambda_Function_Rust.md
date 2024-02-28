# Create Rust Lambda Function
Where we demonstrate creating a Lambda function written in Rust on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below. Note that you can also Rust install in WSL (Windows Subsystem for Linux). See the installation link below.

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
    - Run `rustup update`
2. Windows: Install the "Build Tools for Visual Studio"
    - https://visualstudio.microsoft.com/downloads/
    - Select the "C++ build tools" workload

## Create the Project
1. Open a terminal and navigate to your desired project directory
2. `mkdir ip_rust`
3. `cd ip_rust`
4. `cargo init --bin`
7. Edit the `Cargo.toml` file to add the following dependencies under `[dependencies]`
    - serde = "1.0"
    - serde_json = "1.0"
    - lambda_runtime = "0.6"
    - tokio = { version = "1", features = ["rt-multi-thread"] } # Required for async in Lambda
    - Didn't work in build, trying with this removed aws_lambda_events = { version = "0.5", features = ["rustls-tls"] }
    - Trying aws_lambda_events = { version = "0.15" }
8.  Edit the handler `src/main.rs` and insert the example code from [lambda_test.rs](lambda_test.rs)
 
## Cross-Compilation for Amazon Linux 2023
1. Install the Rust target specification for compiling to 64-bit Amazon Linux 2023 (using the MUSL C library)
    - `rustup target add x86_64-unknown-linux-musl`
2. Navigate to your project's root directory
3. Build with Cross-Compilation
    - `cargo build --release --target x86_64-unknown-linux-musl`
4. Locate compiled binary
    - located in the target/x86_64-unknown-linux-musl/release directory within your project
    - The filename will likely be the same as your project name (e.g., ip_lambda_rust)

STUCK at build error

~~~
error: failed to select a version for `aws_lambda_events`.
    ... required by package `ip_lambda_rust v0.1.0 (C:\Tools\rust\ip_lambda_rust)`
versions that meet the requirements `^0.5` are: 0.5.0

the package `ip_lambda_rust` depends on `aws_lambda_events`, with features: `rustls-tls` but `aws_lambda_events` does not have these features.


failed to select a version for `aws_lambda_events` which could resolve this conflictw
~~~

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
