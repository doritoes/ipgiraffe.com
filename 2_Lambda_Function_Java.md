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
2. `mkdir my-ip-function`
3. `cd my-ip-function`
4. Create a file **pom.xml** in your project folder root with the contents of [pom.xml](pom.xml)
5. `mkdir src`
6. `mkdir src\main`
7. `mkdir src\main\java`
8. `mkdir src\main\java\com`
9. `mkdir src\main\java\com\doritoes`
10. `cd src\main\java\com\doritoes`
11. Create a file in `src/main/java/com/doritoes` named  **MyRequestHandler.java** with the contents of [lambda_test.java](lambda_test.java)
12. 

## Build
1. Open command line in the project folder (e.g., `my-ip-function`)
2. `mvn clean package`
3. The jar file will be created in the `target` folder
    - similar to `MyRequestHandler-1.0-SNAPSHOT.jar`
    - this is the JAR file containing your compiled Java code
   
# Create function
## Log in to AWS Console and Navigate to Lamba
1. Browse to (https://console.aws.amazon.com) and log in
2. In the search bar enter "Lamba" and click on **Lambda**
## Create our Function
1. Click **Create a function**
    1. Author from scratch
    2. Function name: **myIPFunctionJava**
    3. Runtime: **Java 21**
    4. Change the defult Handler from example.Hello::handleRequest to **com.doritoes.MyRequestHandler::handleRequest**
    6. Architecture: the architure you build your function for
        - make sure it matches the function you built
    7. Leave the rest at defuaults, click **Create function**
2. In the *Code source* pane, click **Upload from** > **.jar file**
    - Click **Upload**, select \file *MyRequestHandler-1.0-SNAPSHOT.jar** within your *target* folder, and click **Save**
3. Click **Test**
    - For the test event
      - Event name: **Test**
      - Test Event JSON: [test_event.json](test_event.json)
        - This provides a dummy IP address for the test to work
      - Click **Save**
    - Click **Test** and the test should succeed
    - **WARNING** our test code current fails the API Gateway test! The Java code can't seem to handle the events properly.

The answer is found here https://georgemao.medium.com/demystifying-java-aws-lambda-handlers-for-api-gateway-c1e77b7e6a8d

~~~
public class ApiGatewayEvent {
    private String sourceIp;
    private String xForwardedFor;
    // ... other fields if needed

    // Getters and setters
}
~~~
