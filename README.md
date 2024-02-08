# Iris: Bash Script Management CLI

Iris is a command-line interface (CLI) tool designed to streamline the creation, editing, and management of bash scripts. Leveraging the power of OpenAI's GPT models, Iris provides an intuitive way to generate and modify scripts based on user-provided descriptions. Additionally, Iris integrates version control functionalities, making it easier to track changes and maintain a history of your scripts.

## Features

- **Script Creation**: Generate new bash scripts through a conversational interface that understands your requirements.
- **Script Editing**: Modify existing scripts with ease, using natural language descriptions of the changes you want to make.
- **Version Control**: Automatically commit changes to your scripts, maintaining a history of modifications.
- **Script Listing**: View a list of all your managed scripts in one place.

## Installation

Before installing Iris, ensure you have Rust and Cargo installed on your system. You will also need Git configured for version control features.

1. Clone the Iris repository:
   ```sh
   git clone https://github.com/insprac/iris.git
   ```
2. Navigate to the Iris directory:
   ```sh
   cd iris
   ```
3. Build the project using Cargo:
   ```sh
   cargo build --release
   ```
4. Optionally, add the executable to your system's PATH for easier access.

## Configuration

Iris requires two environment variables to be set:

- `OPENAI_API_KEY`: Your OpenAI API key for accessing GPT models.
- `IRIS_DIR`: The directory where your bash scripts will be stored and managed.

You can set these variables in your `.bashrc`, `.zshrc`, or equivalent shell configuration file:

```sh
export OPENAI_API_KEY="your_openai_api_key_here"
export IRIS_DIR="/path/to/your/scripts/directory"
```

## Usage

After configuring the necessary environment variables, you can use Iris to manage your bash scripts.

### Commands

- **List Scripts**
  ```sh
  iris list
  ```
  Displays a list of all scripts managed by Iris.

- **Initialize Script Directory**
  ```sh
  iris init
  ```
  Prepares the specified `IRIS_DIR` for script management, including initializing a Git repository if one does not already exist.

- **Create New Script**
  ```sh
  iris new --name "script_name" --description "Brief description of the script"
  ```
  Creates a new bash script based on the provided description. If `--name` is omitted, you will be prompted to enter a name.

- **Edit Existing Script**
  ```sh
  iris edit --name "script_name" --description "Description of changes to be made"
  ```
  Edits an existing script. The description should detail the changes you want to apply to the script.

### Examples

- Listing all scripts:
  ```sh
  iris list
  ```

- Initializing the script directory:
  ```sh
  iris init
  ```

- Creating a new script:
  ```sh
  iris new --name "hello_world" --description "A script that prints Hello, World!"
  ```

- Editing an existing script:
  ```sh
  iris edit --name "hello_world" --description "Change the message to print Hello, Iris!"
  ```
