
# RFCU - Rust Fluent Code Utility

![DALL·E 2024-06-05 14 14 24 - Create a logo for RFCU (Rust Fluent Code Utility)  The logo should feature a clean, modern design with a combination of a rust-colored gear symbolizin copy](https://github.com/njfio/rfcu/assets/7220/1a70041a-dcb4-4688-a730-dd758a5cd52f)


**Streamline your Rust development workflow with the power of AI code assistance and automated tasks.**

## Overview

RFCU (Rust Fluent Code Utility) is a command-line tool designed to enhance your Rust coding experience. It leverages the capabilities of [FluentCI](https://github.com/fluentci-project/fluentci) to provide AI-powered code improvements, documentation generation, and other useful features.

## Features

- **AI-driven Code Improvements:** Request improvements to your functions, structures, or entire files using natural language prompts. RFCU interacts with FluentCI to provide intelligent code suggestions.
- **Automated Documentation Generation:** Effortlessly generate documentation for your codebase. Specify the desired level of detail, and RFCU will handle the rest.
- **Test Function Generation:** Quickly create test functions for your code with a simple command.
- **Code Structure Retrieval:** Easily retrieve the names and types of structures within your source code.
- **Linting and Code Style Enforcement:** Configure RFCU to automatically lint your code and ensure consistent style using your preferred linting tool.
- **Git Integration:** Seamlessly commit changes made by RFCU with automatically generated commit messages.
- **Customizable Configuration:** Tailor RFCU's behavior to your specific needs using a configuration file.

## Installation

1. **Prerequisites:** Ensure you have Rust and Cargo installed on your system.
2. **Install FluentCI:** Follow the instructions in the [FluentCI repository](https://github.com/fluentci-project/fluentci) to install FluentCI on your system.
3. **Install RFCU:**
   ```bash
   cargo install rfcu
   ```

## Configuration

RFCU uses a configuration file (`config.toml`) to customize its behavior. Create a `config.toml` file in your home directory under the `.rfcu` directory (`~/.rfcu/config.toml`) with the following structure:

```toml
flowname = "your_flow_name"
commit_message_flow = "your_commit_message_flow_name"
documentation_flow = "your_documentation_flow_name"
language = "rust"
max_retries = 5
lint_command = "cargo fmt && cargo clippy"

[requests]
improvement = "Improve this Rust code: ```rust\n{structure_code}\n```\nUser request: {user_request}\nStructure name: {structure_name}"
whole_file = "Improve this Rust code: ```rust\n{source_code}\n```\nUser request: {user_request}"
add_functionality = "Add this functionality to the Rust code: ```rust\n{source_code}\n```\nUser request: {user_request}"
add_tests_function = "Add tests for this Rust code: ```rust\n{structure_code}\n```"
documentation_whole_file = "Document this Rust code with a brief description at the top: ```rust\n{source_code}\n```\nUser request: {user_request}"
documentation_structure = "Document this structure in Rust: ```rust\n{structure_code}\n```\nUser request: {user_request}\nStructure name: {structure_name}"

```

- **flowname:** The name of the FluentCI flow to use for code improvements.
- **commit_message_flow:** The name of the FluentCI flow to use for generating commit messages.
- **documentation_flow:** The name of the FluentCI flow to use for generating documentation.
- **language:** The programming language of your source code (currently only "rust" is supported).
- **max_retries:** The maximum number of times RFCU should retry improving the code if linting fails.
- **requests:** A section containing the request templates for different modes of operation.
- **lint_command:** The command to execute for linting the code (optional).

## Usage

```
rfcu --file-path <file_path> --mode <mode> [--structure-name <structure_name>]
```

**Arguments:**

- **--file-path:** The path to the Rust source code file.
- **--mode:** The mode of operation. Supported modes:
    - `improvement`: Request AI-powered code improvements.
    - `add_functionality`: Add new functionality to your code.
    - `add_tests_function`: Generate test functions.
    - `documentation_whole_file`: Generate documentation for the entire file.
    - `documentation_structure`: Generate documentation for a specific structure.
    - `whole_file`: Request improvements for the whole file.
- **--structure-name:** The name of the structure to modify (optional, required for `improvement`, `add_tests_function`, and `documentation_structure` modes).

**Example:**

```bash
# Request code improvements for the function "my_function" in the file "main.rs"
rfcu --file-path src/main.rs --mode improvement --structure-name my_function

# Generate documentation for the entire file "lib.rs"
rfcu --file-path src/lib.rs --mode documentation_whole_file
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve RFCU.

## License

This project is licensed under the MIT License.

