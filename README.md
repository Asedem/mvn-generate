# Maven Generator (mvn-generate)

An easy Q&A-based way to generate new Maven Projects.

## Features:
- Generation of a basic Maven project
- More features coming soon...

## Prerequisites
- Rust installed on your machine

## Installation

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

Run the Maven Generator with the following command:
```sh
mvn-generate <project-name>
```
Follow the on-screen prompts to provide the necessary details for your Maven project:

- Version of Java
- Group ID
- Artifact ID

The generator will create the necessary directories and files for your Maven project based on your inputs.

## Example

```
$ mvn-generate my-new-project
Please use mvn-generate <project-name>
There is already a Directory named my-new-project. Should the project be created within it?
yes (y) | no (n)
y
Enter the version of Java that should be used:
1.8
Provide a groupId for the Project:
com.example
Provide an artifactId for the Project:
myapp
Creating directories:
my-new-project/src/main/java/com/example
my-new-project/src/test/java/com/example
Creating main file: my-new-project/src/main/java/com/example/Example.java
Creating test file: my-new-project/src/test/java/com/example/ExampleTest.java
Creating pom file: my-new-project/pom.xml
```

## Project Structure

The generator will create a basic Maven project structure:

```
my-new-project/
├── pom.xml
└── src
    ├── main
    │   └── java
    │       └── {group_id}
    │           └── {MainClassName}.java
    └── test
        └── java
            └── {group_id}
                └── {MainClassName}Test.java
```

## Let me know if there's anything else you'd like to add or modify!