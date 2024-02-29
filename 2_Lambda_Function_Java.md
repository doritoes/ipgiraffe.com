# Create Java Lambda Function
Where we demonstrate creating a Lambda function written in Java on the Java 21 runtime.

A Windows 11 workstation was used for the process below.

**IMPORTANT** a free account registration is required to download the Oracle JDK

## Prerequisites
1. Java Development Kit (JDK) - *login required*
    - Java Development Kit (JDK) version 21
    - https://www.oracle.com/java/technologies/downloads/
    - Locate the "Java SE 21" section
    - Download the Windows x64 installer
    - Run the installer (Defaults are fine)
    - Environment variables
      - Search for "Environment Variables": Search for "Edit the system environment variables" in the Windows search bar
      - Click "Environment Variables" in the System Properties window.
      - JAVA_HOME: Create a new system variable named JAVA_HOME. Set its value to the JDK installation directory (e.g., C:\Program Files\Java\jdk-21).
      - Path: Edit the existing Path system variable. Add a new entry pointing to the bin directory within your JDK installation (e.g., C:\Program Files\Java\jdk-21\bin).
    - Verify
      - `java -version`
      - `javac -version`
- AWS Toolkit for VS Code: Install this extension in VS Code to help with AWS development and deployment tasks.
    - todo
- Maven or Gradle (Build Tool): Choose either Maven or Gradle to manage your project dependencies and build process. I'll provide instructions for Maven.
    - todo

## Create the Project
todo
 
## Build
todo

## Package the ZIP file for upload
todo 
# Create function
todo

## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scratch
    2. Function name: **myIPFunctionRest**
    3. Runtime: **Java 21**
    4. Change the defult Handler from example.Hello::handleRequest to **MyIPHandler::handleRequest**
    5. Architecture: the architure you build your function for
        - make sure it matches the function you built
    6. Leave the rest at defuaults, click **Create function**
2. In the *Code source* pane, click **Upload from** > **.zip file**
    - Click **Upload**, select your **lambda-handler-java.zip** file code, and click **Save**
3. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed
