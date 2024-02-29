# Create Rust Lambda Function
Where we demonstrate creating a Lambda function written in Rust on the Amazon Linux runtime.

A Windows 11 workstation was used for the process below. It is a challenge to cross-compile for Amazon Linux 2023 from Windows 11. Therefore are going to install under WSL (Windows Subsystem for Linux).

*See [https://docs.aws.amazon.com/linux/al2023/ug/rust.html](https://stratusgrid.com/blog/aws-lambda-rust-how-to-deploy-aws-lambda-functions)*

## Prerequisites
1. Install WSL (Windows 11 and Windows 10 2004 or later)
    - Search for "Turn Windows Features on or off" in your Start menu
    - Check the box next to "Windows Subsystem for Linux" and click *OK*
    - Restart your computer as prompted
2. Install a Linux Distribution
    - Open the Microsoft Store app
    - Search for your preferred Linux distribution. Ubuntu is a popular and beginner-friendly choice.
    - Click "Get" to install the distribution
3. Setup WSL
    - Initial Launch: After the installation, open a new terminal/command prompt, type wsl, and press enter. This will initialize and launch your chosen Linux distribution.
    - Create a User: You'll be asked to create a Unix username and password
4. Install Rust in WSL
    - Open a terminal within WSL
    - Follow the Rust installation instructions
      - https://www.rust-lang.org/tools/install
      - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
      - Answer 1 to continue the installation
      - Either restart your terminal or follow the instructions to run `source "$HOME/.cargo/env"`
  - Check for updates `rustup updates`
  - Install the Cross-Compilation Target
    - `rustup target add x86_64-unknown-linux-musl`

## Create the Project
1. Open a terminal and navigate to your desired project directory
2. `mkdir myip_rust`
3. `cd myip_rust`
4. `cargo init --bin`
5.  Configure Cargo
    - Create `.cargo/config.toml` file
    - `[target.x86_64-unknown-linux-musl]`
    - `linker = "x86_64-linux-gnu-gcc"`
6. Edit the `Cargo.toml` file to add the dependencies under `[dependencies]`
    - [dependencies.txt](dependencies.txt)
7. better way: cargo add aws_lambda_events http lambda_runtime serde serde_json tokio
7.  Edit the handler `src/main.rs` and insert the example code from [lambda_test.rs](lambda_test.rs)
 
## Cross-Compilation for Amazon Linux 2023
1. Install the Rust target specification for compiling to 64-bit Amazon Linux 2023 (using the MUSL C library)
    - `rustup target add x86_64-unknown-linux-musl`
2. Navigate to your project's root directory
3. Build with Cross-Compilation
    - `cargo build --release --target x86_64-unknown-linux-musl`

## Package the ZIP file for upload
4. Locate compiled binary
    - located in the target/x86_64-unknown-linux-musl/release directory within your project
    - The filename will likely be the same as your project name (e.g., ip_lambda_rust)
5. Create the bootstrap file
    - `cp ip_lambda_rust boostrap`
6. Create the ZIP file
    - `zip lambda-handler-rust.zip boostrap`
5. Copy the ZIP file back to your Windows host
    - From Windows: Windows-R, `\\wsl$`
    - Open your Linux distribution, then home directory, your user directory, and nagivate from there
    - Alternatively, you can copy the file from within WSL back to a folder on the host
      - If your have a directory in c:\Tools
      - `cp <filename> /mnt/c/Tools`

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
