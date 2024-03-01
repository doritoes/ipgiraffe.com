# Create Java Lambda Function
Where we demonstrate creating a Lambda function written in Java on the Java 21 runtime.

A Windows 11 workstation was used for the process below.

**IMPORTANT** a free account registration is required to download the Oracle JDK

## Prerequisites
1. Java Development Kit (JDK) - *login required*
    - Java Development Kit (JDK) version 21
    - https://www.oracle.com/java/technologies/downloads/
    - Download the Windows x64 installer
    - Run the installer (Defaults are fine)
      - C:\Program Files\Java\jdk-21\
    - Environment variables
      - Search for "Edit the system environment variables" in the Windows search bar
      - Click **Environment Variables** in the System Properties window
      - Under *System variables*, click **New...**
        - Variable name: **JAVA_HOME**
        - Variable value: set to the JDK installation directory (e.g., C:\Program Files\Java\jdk-21).
        - Click **OK** and then click **OK**
    - Verify from command line
      - `java -version`
      - `javac -version`
2. Install Maven
    - https://maven.apache.org/download.cgi
    - Select and download "Binary zip archive" (for Windows)
    - Unzip the file to your desired installation directory (e.g., `C:\Program Files\Apache\maven`)
    - Environment variables
      - Search for "Edit the system environment variables" in the Windows search bar
      - Click **Environment Variables** in the System Properties window
      - Under *System variables*, click **New...**
        - Variable name: **M2_HOME**
        - Variable value: set to the Maven installation directory (e.g., C:\Program Files\Apache\maven).
      - Under *System variables*, edit **Path**
        - Click **New**
        - Add the path to the bin directory inside the Maven directory (e.g. C:\Program Files\Apache\maven\bin)
    - Test from command line (open a new command prompt if needed)
      - `mvn -version`

## Create the Project
1. Open a terminal and navigate to your desired project directory
2. `mkdir myip_java`
3. `cd myip_java`
4. Create a file `pom.xml` with the contents of [pom.xml](pom.xml)
5. Create a new file named **MyIPHandler.java** with the contents of [lambda_test.java](lambda_test.java)
 
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
