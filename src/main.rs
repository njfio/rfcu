use std::fs;
use std::fs::File;
use std::io::{self, stderr, Read, Write};
use std::path::Path;

use clap::Arg;
use regex::Regex;
use serde::Deserialize;
use std::process::{Command, Stdio};
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_rust() -> Language;
}
extern "C" {
    fn tree_sitter_python() -> Language;
}
extern "C" {
    fn tree_sitter_ruby() -> Language;
}
extern "C" {
    fn tree_sitter_powershell() -> Language;
}

#[derive(Deserialize)]
struct Settings {
    flowname: String,
    commit_message_flow: String,
    documentation_flow: String,
    language: String,
    requests: Requests,
    lint_command: Option<String>,
    max_retries: usize,
    python_lint_command: Option<String>,
    ruby_lint_command: Option<String>,
    powershell_lint_command: Option<String>,
}

#[derive(Deserialize)]
struct Requests {
    improvement: String,
    whole_file: String,
    add_functionality: String,
    add_tests_function: String,
    documentation_whole_file: String,
    documentation_structure: String,
}

fn parse_python(source_code: &str) -> Parser {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_python() };
    parser
        .set_language(&language)
        .expect("Error setting Python language");
    parser
}

fn get_powershell_structures(source_code: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)^function\s+([a-zA-Z_]\w*)").unwrap();
    re.captures_iter(source_code)
        .map(|cap| format!("function: {}", &cap[1]))
        .collect()
}

fn get_python_structures(node: &Node, source_code: &[u8], structures: &mut Vec<String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "function_definition" || child.kind() == "class_definition" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = name_node.utf8_text(source_code).unwrap();
                structures.push(format!("{}: {}", child.kind(), name));
            }
        }
        get_python_structures(&child, source_code, structures);
    }
}

fn run_lint_command(command: &str, file_path: &str) -> io::Result<(bool, String)> {
    let command = command.replace("{file_path}", file_path);
    eprintln!("Executing lint command: {}", command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute lint command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let full_output = format!("stdout: {}\nstderr: {}", stdout, stderr);

    Ok((output.status.success(), full_output))
}


#[derive(Debug)]
pub enum LanguageDetectionError {
    UnsupportedExtension,
    PowerShellNotHandled,
}

impl std::error::Error for LanguageDetectionError {}

impl std::fmt::Display for LanguageDetectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageDetectionError::UnsupportedExtension => write!(f, "unsupported file extension"),
            LanguageDetectionError::PowerShellNotHandled => write!(f, "Powershell is not handled"),
        }
    }
}

fn detect_language(file_path: &str) -> Result<Language, LanguageDetectionError> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str());
    match extension {
        Some("rs") => Ok(unsafe { tree_sitter_rust() }),
        Some("py") => Ok(unsafe { tree_sitter_python() }),
        Some("rb") => Ok(unsafe { tree_sitter_ruby() }),
        // Use our new error type here
        Some("ps1") => Err(LanguageDetectionError::PowerShellNotHandled),
        _ => Err(LanguageDetectionError::UnsupportedExtension),
    }
}

fn main() -> io::Result<()> {
    eprintln!("Reading configuration file...");
    let config_content =
        fs::read_to_string("/Users/n/.rfcu/config.toml").expect("Failed to read config file");
    let settings: Settings = toml::from_str(&config_content).expect("Failed to parse config file");

    let matches = clap::Command::new("RFCU")
        .version("1.0")
        .author("Nick <nick@njf.io>")
        .about("Rust Fluent Code Utility")
        .arg(
            Arg::new("file_path")
                .help("The path to the source code file")
                .required(true)
                .global(true)
                .long("file_path")
                .required_unless_present("get_structure"),
        )
        .arg(
            Arg::new("mode")
                .help("The mode of operation")
                .long("mode")
                .value_parser([
                    "improvement",
                    "add_functionality",
                    "add_tests_function",
                    "documentation_whole_file",
                    "documentation_structure",
                    "whole_file",
                ])
                .required(false),
        )
        .arg(
            Arg::new("structure_name")
                .help("The name of the structure to modify (optional)")
                .long("structure_name")
                .required(false),
        )
        .subcommand(
            clap::Command::new("get_structure")
                .about("Retrieve the names of the specified structures from the source code")
                .arg(
                    Arg::new("file_path")
                        .help("The path to the source code file")
                        .required(false),
                ),
        )
        .get_matches();

    if let Some(get_structure_matches) = matches.subcommand_matches("get_structure") {
        let file_path = get_structure_matches
            .get_one::<String>("file_path")
            .expect("File path is required");

        eprintln!("Reading source code from file: {}", file_path);
        let source_code = fs::read_to_string(file_path).expect("Failed to read source file");

        let language_result = detect_language(file_path);
        let mut structures = Vec::new();

        match language_result {
            Ok(language) => {
                eprintln!("Initializing parser and setting language");
                let mut parser = Parser::new();
                parser
                    .set_language(&language)
                    .expect("Error setting language");
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();
                get_structures(&root_node, source_code.as_bytes(), &mut structures);
            }
            _ => {
                eprintln!("Unsupported language");
                std::process::exit(1);
            }
        }

        eprintln!("Structures found in the source code:");
        for structure in structures {
            println!("{}", structure);
        }
        Ok(())
    } else {
        let mode = matches.get_one::<String>("mode").expect("Mode is required");
        let file_path = matches
            .get_one::<String>("file_path")
            .expect("File path is required");
        let default_structure_name = String::new();
        let structure_name = matches
            .get_one::<String>("structure_name")
            .unwrap_or(&default_structure_name);
        let backup_file_path = format!("{}_before_revision", file_path);

        eprintln!("mode: {}", mode);
        eprintln!("file_path: {}", file_path);
        eprintln!("structure_name: {}", structure_name);

        eprintln!("Reading source code from file: {}", file_path);
        let source_code = fs::read_to_string(file_path).expect("Failed to read source file");

        eprintln!("Creating backup file: {}", backup_file_path);
        fs::write(&backup_file_path, &source_code).expect("Failed to create backup file");

        eprintln!("Reading user input from stdin...");
        let mut stdin_content = String::new();
        io::stdin().read_to_string(&mut stdin_content)?;
        let mut user_request = stdin_content.trim().to_string();
        eprintln!("User request: {}", user_request);

        let language_result = detect_language(file_path);
        let mut parser = Parser::new();
        let language_result = match detect_language(file_path) {
            Ok(language) => language, // directly assign the unwrapped value
            Err(e) => {
                eprintln!("Error occurred: {}", e); // print our error message
                restore_backup(file_path, &backup_file_path);
                std::process::exit(1);
            }
        };

        let language = language_result as Language;

        if parser.set_language(&language).is_err() {
            eprintln!("Error setting language: {:?}", language);
            restore_backup(file_path, &backup_file_path);
            std::process::exit(1);
        }

        let mut retries = 0;
        while retries < settings.max_retries {
            retries += 1;
            eprintln!("Attempt {} of {}", retries, settings.max_retries);

            let request;
            let start_byte;
            let end_byte;

            if mode == "improvement"
                || mode == "add_tests_function"
                || mode == "documentation_structure"
            {
                eprintln!("Parsing the source code...");
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();
                eprintln!("Root node: {}", root_node.kind());
                eprintln!("Structure name: {}", structure_name);

                let structure_range =
                    find_structure(&root_node, structure_name, source_code.as_bytes());
                (start_byte, end_byte) = structure_range.unwrap();

                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);
                let original_structure = &source_code[properties_start_byte..end_byte];

                request = match mode.as_str() {
                    "improvement" => settings
                        .requests
                        .improvement
                        .replace("{structure_code}", original_structure)
                        .replace("{user_request}", &user_request)
                        .replace("{structure_name}", &structure_name),
                    "add_tests_function" => settings
                        .requests
                        .add_tests_function
                        .replace("{structure_code}", original_structure),
                    "documentation_structure" => settings
                        .requests
                        .documentation_structure
                        .replace("{structure_code}", original_structure)
                        .replace("{user_request}", &user_request)
                        .replace("{structure_name}", &structure_name),
                    _ => unreachable!(),
                };
            } else if mode == "whole_file" || mode == "documentation_whole_file" {
                eprintln!("Preparing the whole file request...");
                request = match mode.as_str() {
                    "whole_file" => settings
                        .requests
                        .whole_file
                        .replace("{source_code}", &source_code)
                        .replace("{user_request}", &user_request),
                    "documentation_whole_file" => settings
                        .requests
                        .documentation_whole_file
                        .replace("{source_code}", &source_code)
                        .replace("{user_request}", &user_request),
                    _ => unreachable!(),
                };
                start_byte = 0;
                end_byte = source_code.len();
            } else if mode == "add_functionality" {
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();
                eprintln!("Root node: {}", root_node.kind());
                eprintln!("Structure name: {}", structure_name);
                eprintln!("Preparing the request to add functionality...");
                let structure_range = find_structure(&root_node, "main", source_code.as_bytes());
                (start_byte, end_byte) = structure_range.unwrap();

                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);
                let original_structure = &source_code[properties_start_byte..end_byte];
                request = settings
                    .requests
                    .add_functionality
                    .replace("{source_code}", &source_code)
                    .replace("{user_request}", &user_request);
            } else {
                eprintln!("Invalid mode: {}", mode);
                restore_backup(file_path, &backup_file_path);
                std::process::exit(1);
            }

            eprintln!("Sending the request to fluentcli for improvement...");
            let improved_structure = match improve_structure_with_fluentcli(
                &settings.flowname,
                &request,
                &user_request,
                file_path,
            ) {
                Ok(structure) => structure,
                Err(e) => {
                    eprintln!("Error improving structure: {:?}", e);
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
            };

            eprintln!("Improved structure received: {}", improved_structure);

            let updated_code = if mode == "improvement" {
                eprintln!("Updating the source code with the improved structure...");
                let mut updated_code = String::new();
                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);
                updated_code.push_str(&source_code[..properties_start_byte]);
                updated_code.push_str(&improved_structure);
                updated_code.push_str(&source_code[end_byte..]);
                updated_code
            } else if mode == "get_structure" {
                eprintln!("Reading source code from file: {}", file_path);
                let source_code =
                    fs::read_to_string(file_path).expect("Failed to read source file");

                let language_result =
                    detect_language(file_path).expect("Unsupported file extension");
                let mut parser = Parser::new();
                let language = match detect_language(file_path) {
                    Ok(language) => language,
                    _ => {
                        eprintln!("Unsupported language");
                        restore_backup(file_path, &backup_file_path);
                        std::process::exit(1);
                    }
                };

                if parser.set_language(&language).is_err() {
                    eprintln!("Error setting language: {:?}", language);
                    std::process::exit(1);
                }

                eprintln!("Parsing the source code...");
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();

                let mut structures = Vec::new();
                get_structures(&root_node, source_code.as_bytes(), &mut structures);

                eprintln!("Structures found in the source code:");
                for structure in structures {
                    println!("{}", structure);
                }
                break;
            } else if mode == "add_functionality" {

                eprintln!("New functionality received: {}", improved_structure);

                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();

                let updated_code = if language == unsafe { tree_sitter_python() } || language == unsafe { tree_sitter_ruby() } {
                    // For Python, insert above the main function
                    if let Some(main_start) =
                        find_main_function_start(&root_node, source_code.as_bytes())
                    {
                        eprintln!("main function found. Inserting new functionality above main...");
                        let mut updated_code = source_code[..main_start].to_string();
                        updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                        updated_code.push_str(&source_code[main_start..]);
                        eprintln!("Updated code:\n{}", updated_code);
                        updated_code
                    } else {
                        eprintln!(
                            "main function not found. Appending new functionality at the end..."
                        );
                        let mut updated_code = source_code.to_string();
                        updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                        eprintln!("Updated code:\n{}", updated_code);
                        updated_code
                    }
                } else {
                    // For Rust, insert after the main function
                    let main_fn_range = find_main_function(&root_node, source_code.as_bytes());
                    if let Some((_, main_end)) = main_fn_range {
                        eprintln!("main function found. Inserting new functionality after main...");
                        let mut updated_code = source_code[..main_end].to_string();
                        updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                        updated_code.push_str(&source_code[main_end..]);
                        eprintln!("Updated code:\n{}", updated_code);
                        updated_code
                    } else {
                        eprintln!(
                            "main function not found. Appending new functionality at the end..."
                        );
                        let mut updated_code = source_code.to_string();
                        updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                        eprintln!("Updated code:\n{}", updated_code);
                        updated_code
                    }
                };

                updated_code
            } else if mode == "documentation_structure" {
                eprintln!("Replacing or inserting the documentation in the structure...");
                let doc_comment = improved_structure.trim();
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();
                let struct_range =
                    find_struct_node(&root_node, structure_name, source_code.as_bytes());
                if struct_range.is_none() {
                    eprintln!("Structure not found in the source code.");
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
                let (struct_start, struct_end) = struct_range.unwrap();
                eprintln!(
                    "Struct found at byte range: {} - {}",
                    struct_start, struct_end
                );
                let mut updated_code = source_code.to_string();

                let (doc_start, doc_end) = find_documentation_range(&source_code, struct_start);
                eprintln!("Documentation range: {} - {}", doc_start, doc_end);

                if doc_start != doc_end {
                    eprintln!("Replacing existing documentation...");
                    let new_code = format!(
                        "\n{}{}{}\n\n",
                        &source_code[..doc_start],
                        doc_comment,
                        &source_code[doc_end..]
                    );
                    updated_code = new_code;
                } else {
                    eprintln!("Inserting new documentation...");
                    updated_code.insert_str(struct_start, &format!("\n\n{}\n\n", doc_comment));
                }
                eprintln!("Updated code:\n{}", updated_code);
                updated_code
            } else if mode == "documentation_whole_file" {
                eprintln!("Replacing the documentation in the whole file...");
                let mut updated_code = String::new();

                // Detect the language of the file
                let language = detect_language(file_path).expect("Unsupported file extension");

                // Find the documentation range using tree-sitter
                let (doc_start, doc_end) = find_doc_range_tree_sitter(&source_code, &language);

                // Replace the existing documentation with the improved structure
                updated_code.push_str(&source_code[..doc_start]);
                updated_code.push_str(&improved_structure);
                updated_code.push('\n');
                updated_code.push('\n');
                updated_code.push_str(&source_code[doc_end..]);

                eprintln!("Updated code:\n{}", updated_code);
                updated_code
            } else if mode == "add_tests_function" {
                eprintln!("Adding test functions...");
                let test_functions = improved_structure.trim();
                let tree = parser
                    .parse(&source_code, None)
                    .expect("Error parsing source code");
                let root_node = tree.root_node();
                let struct_range =
                    find_structure(&root_node, structure_name, source_code.as_bytes());
                if struct_range.is_none() {
                    eprintln!("Target function not found in the source code.");
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
                let (struct_start, struct_end) = struct_range.unwrap();
                eprintln!(
                    "Target function found at byte range: {} - {}",
                    struct_start, struct_end
                );
                eprintln!(
                    "Target function found at line range: {} - {}",
                    source_code[..struct_start].lines().count(),
                    source_code[..struct_end].lines().count()
                );
                let mut updated_code = source_code.to_string();

                eprintln!("Inserting test functions at byte: {}", struct_end);
                eprintln!(
                    "Inserting test functions at line: {}",
                    source_code[..struct_end].lines().count()
                );
                updated_code.insert_str(struct_end, &format!("\n\n{}\n\n", test_functions));

                eprintln!("Updated code:\n{}", updated_code);
                updated_code
            } else {
                improved_structure
            };
            eprintln!("Writing the updated code to the original file...");
            fs::write(file_path, updated_code.as_bytes())?;
            eprintln!("Updated code written to the original file successfully.");

            let lint_command = match detect_language(file_path) {
                Ok(lang) if lang == unsafe { tree_sitter_rust() } => {
                    settings.lint_command.as_deref()
                }
                Ok(lang) if lang == unsafe { tree_sitter_python() } => {
                    settings.python_lint_command.as_deref()
                }
                Ok(lang) if lang == unsafe { tree_sitter_ruby() } => {
                    settings.ruby_lint_command.as_deref()
                }
                _ => None,
            };

            if let Some(lint_command) = lint_command {
                eprintln!("Linting the updated code...");
                let (success, lint_output) =
                    run_lint_command(lint_command, file_path).expect("Failed to lint the code");

                if success {
                    eprintln!("Linting succeeded.");
                    break;
                } else {
                    eprintln!("Linting failed. Including linting feedback in the payload and retrying...");
                    let request_with_feedback = format!(
                        "{}\nLinting feedback:\n{}",
                        request,
                        lint_output
                    );

                    let improved_structure = match improve_structure_with_fluentcli(
                        &settings.flowname,
                        &request_with_feedback,
                        &user_request,
                        file_path,
                    ) {
                        Ok(structure) => structure,
                        Err(e) => {
                            eprintln!("Error improving structure: {:?}", e);
                            restore_backup(file_path, &backup_file_path);
                            std::process::exit(1);
                        }
                    };

                    eprintln!("Improved structure received: {}", improved_structure);

                    // Repeat the code update process with the new improved structure and continue the loop
                }
            } else {
                break;
            }
        }
        let commit_message =
            generate_commit_message(file_path, mode, &settings.commit_message_flow)
                .unwrap_or_else(|_| "Automated changes made by RFCU".to_string());

        if let Err(e) = commit_changes(file_path, &commit_message) {
            eprintln!("Failed to commit changes: {:?}", e);
            restore_backup(file_path, &backup_file_path);
            std::process::exit(1);
        }

        eprintln!("Changes committed successfully.");

        eprintln!("Cleaning up the backup file...");
        fs::remove_file(&backup_file_path)?; // ... (rest of your existing code goes here)

        Ok(())
    }
}


fn retrieve_structures<'a>(
    node: &'a Node<'a>,
    source_code: &'a [u8],
    structures: &mut Vec<String>,
) {
    let cursor = &mut node.walk();
    for child in node.children(cursor) {
        if node.kind() == "function_item"        // Rust
            || node.kind() == "mod_item"         // Rust
            || node.kind() == "struct_item"      // Rust
            || node.kind() == "enum_item"        // Rust
            || node.kind() == "trait_item"       // Rust
            || node.kind() == "impl_item"        // Rust
            || node.kind() == "function_definition"  // Python
            || node.kind() == "class_definition"     // Python
            || node.kind() == "module"
        // Python
        {
            let name_node = child.child_by_field_name("name");
            if let Some(name_node) = name_node {
                let name = std::str::from_utf8(name_node.utf8_text(source_code).unwrap().as_ref())
                    .unwrap();
                structures.push(format!("{}: {}", child.kind(), name));
            }
        }
        retrieve_structures(&child, source_code, structures);
    }
}

fn find_structure<'a>(
    node: &'a Node<'a>,
    structure_name: &str,
    source_code: &'a [u8],
) -> Option<(usize, usize)> {
    let mut cursor = node.walk();
    //eprintln!("Searching for structure in node: {}", node.kind());
    //eprintln!("structure_name: {}", structure_name);
    //eprintln!("Searching for structure: {}", structure_name);
    //eprintln!("Current node name: {}", node.kind());

    if node.kind() == "function_item"        // Rust
        || node.kind() == "mod_item"         // Rust
        || node.kind() == "struct_item"      // Rust
        || node.kind() == "enum_item"        // Rust
        || node.kind() == "trait_item"       // Rust
        || node.kind() == "impl_item"        // Rust
        || node.kind() == "function_definition"  // Python
        || node.kind() == "class_definition"     // Python
        || node.kind() == "module"            // Python
        || node.kind() == "method"            // Ruby
        || node.kind() == "class"             // Ruby
        || node.kind() == "module"            // Ruby

    // Python
    {
        if let Some(name_node) = node.child_by_field_name("name") {
            //eprintln!("Found name node: {}", name_node.kind());
            let name = name_node.utf8_text(source_code).unwrap();
            //eprintln!("Found structure: {}", name);
            if name == structure_name {
                eprintln!("Structure found!");
                return Some((node.start_byte(), node.end_byte()));
            }
        }
    }

    for child in node.children(&mut cursor) {
        //eprintln!("Traversing child node: {}", child.kind());
        if let Some(found) = find_structure(&child, structure_name, source_code) {
            eprintln!("Structure found in child node!");
            return Some(found);
        }
    }

    //aeprintln!("Structure not found in current subtree.");
    None
}

fn find_first_code_line_after_docs(source_code: &str) -> usize {
    for (i, line) in source_code.lines().enumerate() {
        if !line.trim().starts_with("*") && !line.trim().starts_with("/") {
            return source_code.lines().take(i).map(|l| l.len() + 1).sum();
        }
    }
    0
}


fn improve_structure_with_fluentcli(
    flowname: &str,
    request: &str,
    user_request: &str,
    source_code_path: &str,
) -> io::Result<String> {
    eprintln!(
        "Starting fluentcli with flowname: {}, request: {}",
        flowname, request
    );
    let mut child = Command::new("fluent")
        .arg(flowname)
        .arg(request)
        .arg("--additional-context-file")
        .arg(source_code_path)
        .arg("-p") // Parse the code blocks
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fluent");

    eprintln!("Writing user request to fluentcli stdin...");
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(user_request.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    let response = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    eprintln!("Response from fluentcli:\n\n\n\n {}", response);

    // Extract the code block from the response
    let code_block_re =
        Regex::new(r"```(?:rust|python|ruby|javascript|typescript|bash|sh)?\n(.*?)```").unwrap();
    let improved_structure = if let Some(captures) = code_block_re.captures(&response) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        response // In case the response is not in the expected format
    };

    Ok(improved_structure.trim().to_string())
}

fn lint_code(lint_command: &str) -> io::Result<bool> {
    eprintln!("Executing lint command: {}", lint_command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(lint_command)
        .output()
        .expect("Failed to execute lint command");

    Ok(output.status.success())
}

fn commit_changes(file_path: &str, message: &str) -> io::Result<()> {
    eprintln!("Adding changes to git...");
    Command::new("git")
        .args(&["add", file_path])
        .output()
        .expect("Failed to add changes to git");

    eprintln!("Committing changes to git...");
    Command::new("git")
        .args(&["commit", "-m", message])
        .output()
        .expect("Failed to commit changes");

    Ok(())
}

fn restore_backup(file_path: &str, backup_file_path: &str) {
    eprintln!("Restoring backup file: {}", backup_file_path);
    if let Err(e) = fs::copy(backup_file_path, file_path) {
        eprintln!("Failed to restore backup file: {:?}", e);
    } else {
        eprintln!("Backup restored successfully.");
    }
}

fn generate_commit_message(file_path: &str, mode: &str, flowname: &str) -> io::Result<String> {
    eprintln!("Generating detailed commit message using FluentCLI...");
    let request = format!("Generate a commit message for the changes made in {} mode to the file {} on a single line, it should be succinct. Only ouptut the commit message.", mode, file_path);

    let mut child = Command::new("fluent")
        .arg(flowname)
        .arg(request)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fluent");

    let output = child.wait_with_output().expect("Failed to read stdout");

    let response = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    eprintln!("Response from fluentcli: {}", response);

    Ok(response.trim().to_string())
}

fn find_struct_node<'a>(
    node: &'a Node<'a>,
    struct_name: &str,
    source_code: &'a [u8],
) -> Option<(usize, usize)> {
    //eprintln!("Searching for struct: {}", struct_name);
    //eprintln!("Current node kind: {}", node.kind());

    if node.kind() == "function_item"        // Rust
        || node.kind() == "mod_item"         // Rust
        || node.kind() == "struct_item"      // Rust
        || node.kind() == "enum_item"        // Rust
        || node.kind() == "trait_item"       // Rust
        || node.kind() == "impl_item"        // Rust
        || node.kind() == "function_definition"  // Python
        || node.kind() == "class_definition"     // Python
        || node.kind() == "module"               // Python
        || node.kind() == "method"            // Ruby
        || node.kind() == "class"             // Ruby
        || node.kind() == "module"            // Ruby
    // Python
    {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source_code).unwrap();
            //eprintln!("Found function: {}", name);
            if name == struct_name {
                eprintln!("Struct found!");
                return Some((node.start_byte(), node.end_byte()));
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(struct_range) = find_struct_node(&child, struct_name, source_code) {
            return Some(struct_range);
        }
    }

    //eprintln!("Struct not found in current subtree.");
    None
}

fn find_doc_node<'a>(node: &Node<'a>) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    let mut prev_node: Option<Node<'a>> = None;

    for child in node.children(&mut cursor) {
        if child.kind() == "block_comment" {
            prev_node = Some(child);
        } else if child.kind() == "function_item" {
            return prev_node;
        }
    }

    None
}

fn find_doc_range<'a>(node: &Node<'a>, source_code: &'a [u8]) -> Option<(usize, usize)> {
    let mut cursor = node.walk();
    let mut doc_start = None;
    let mut doc_end = None;

    for child in node.children(&mut cursor) {
        if child.kind() == "block_comment" {
            if doc_start.is_none() {
                doc_start = Some(child.start_byte());
            }
            doc_end = Some(child.end_byte());
        } else if child.kind() == "function_item" {
            break;
        }
    }

    if let (Some(start), Some(end)) = (doc_start, doc_end) {
        Some((start, end))
    } else {
        None
    }
}

fn find_doc_end(source_code: &str) -> usize {
    let mut doc_end = 0;
    for (i, line) in source_code.lines().enumerate() {
        if line.trim().starts_with("//!") || line.trim().starts_with("/*!") {
            doc_end = source_code.lines().take(i + 1).map(|l| l.len() + 1).sum();
        } else if line.trim().starts_with("//") || line.trim().starts_with("/*") {
            continue;
        } else {
            break;
        }
    }
    doc_end
}

fn find_struct_start<'a>(node: &Node<'a>, source_code: &'a [u8]) -> usize {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return node.start_byte();
        }
    }
    node.start_byte()
}

fn find_doc_start(source_code: &str, struct_start: usize) -> Option<usize> {
    let mut doc_start = struct_start;
    let lines: Vec<&str> = source_code[..struct_start].lines().collect();

    for line in lines.iter().rev() {
        if line.trim().starts_with("/**") {
            doc_start = source_code[..struct_start].find(line).unwrap();
            break;
        } else if !line.trim().is_empty() {
            break;
        }
    }

    if doc_start < struct_start {
        Some(doc_start)
    } else {
        None
    }
}

// Update the find_doc_node function
fn update_documentation(
    source_code: &str,
    struct_range: (usize, usize),
    improved_structure: &str,
) -> String {
    let doc_comment = format!("/**\n{}\n*/", improved_structure.trim());
    let (struct_start, struct_end) = struct_range;
    let (doc_start, doc_end) = find_documentation_range(source_code, struct_start);

    let mut updated_code = source_code.to_string();
    updated_code.replace_range(doc_start..doc_end, &doc_comment);
    updated_code
}

fn find_documentation_range(source_code: &str, struct_start: usize) -> (usize, usize) {
    let mut doc_start = struct_start;
    let mut doc_end = struct_start;

    // Split the source code into lines and iterate backwards from the struct start line
    let lines: Vec<&str> = source_code[..struct_start].lines().collect();
    eprintln!("Total lines before struct: {}", lines.len());
    for i in (0..lines.len()).rev() {
        let line = lines[i].trim();
        eprintln!("Checking line {} for doc start: {}", i, line);
        if line.starts_with("/**") {
            // Found the start of the documentation block
            doc_start = source_code[..source_code[..struct_start].rfind(line).unwrap()].len();
            eprintln!("Doc start found at line {}, byte {}", i, doc_start);
            break;
        } else if line.starts_with("*/") || line.starts_with("*") {
            continue;
        } else if !line.is_empty() {
            break;
        }
    }

    if doc_start < struct_start {
        // Iterate from the found doc_start to struct_start to find the end of the documentation block
        let doc_lines: Vec<&str> = source_code[doc_start..struct_start].lines().collect();
        eprintln!("Total lines in doc block: {}", doc_lines.len());
        for (i, line) in doc_lines.iter().enumerate() {
            eprintln!("Checking doc line {} for doc end: {}", i, line);
            if line.trim().ends_with("*/") {
                // Found the end of the documentation block
                doc_end = doc_start + source_code[doc_start..].find(line).unwrap() + line.len();
                eprintln!("Doc end found at byte {}", doc_end);
                break;
            }
        }
    }

    if doc_start == struct_start || doc_end == struct_start {
        eprintln!("No documentation block found.");
        return (struct_start, struct_start);
    }

    (doc_start, doc_end)
}

fn find_doc_range_tree_sitter(source_code: &str, language: &Language) -> (usize, usize) {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Error setting language");
    let tree = parser
        .parse(source_code, None)
        .expect("Error parsing source code");
    let root_node = tree.root_node();

    let mut doc_start = None;
    let mut doc_end = None;
    let mut cursor = root_node.walk();

    eprintln!("Searching for documentation range using tree-sitter...");
    eprintln!("Root node kind: {}", root_node.kind());
    eprintln!("Root node range: {} - {}", root_node.start_byte(), root_node.end_byte());
    eprintln!("Root node children: {} children", root_node.child_count());
    for node in root_node.children(&mut cursor) {
        eprintln!("Node kind: {}", node.kind());
        if node.kind() == "comment" || node.kind() == "doc_string" || node.kind() == "block_comment" || node.kind() == "documentation" || node.kind() == "expression_statement" {
            eprintln!("Node range: {} - {}", node.start_byte(), node.end_byte());
            eprintln!("Node kind inner: {}", node.kind());
            if doc_start.is_none() {
                doc_start = Some(node.start_byte());
            }
            doc_end = Some(node.end_byte());
        } else if !node.is_extra() {
            eprint!("Skipping node: {}", node.kind());
            break;
        }
    }

    match (doc_start, doc_end) {
        (Some(start), Some(end)) => (start, end),
        _ => (0, 0),  // If no documentation is found, return 0, 0 to add at the beginning
    }
}



fn find_insert_position<'a>(node: Node<'a>, source_code: &'a [u8]) -> usize {
    let mut cursor = node.walk();
    let mut prev_node: Option<Node<'a>> = None;
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            if let Some(prev) = prev_node {
                if is_terminating_node(&prev) {
                    return prev.end_byte();
                }
            }
            return node.start_byte();
        }
        prev_node = Some(child);
    }
    if let Some(prev) = prev_node {
        if is_terminating_node(&prev) {
            return prev.end_byte();
        }
    }
    node.start_byte()
}

fn is_terminating_node(node: &Node) -> bool {
    match node.kind() {
        "]" | ")" | "}" | ";" => true,
        _ => false,
    }
}

fn insert_test_functions(source_code: &str, test_functions: &str) -> String {
    let mut updated_code = source_code.to_string();

    // Check if a #[cfg(test)] block already exists
    if let Some((test_block_start, test_block_end)) = find_cfg_test_block(&source_code) {
        let start_line = source_code[..test_block_start].lines().count();
        let end_line = source_code[..test_block_end].lines().count();
        eprintln!(
            "Existing #[cfg(test)] block found at byte range: {} - {}",
            test_block_start, test_block_end
        );
        eprintln!(
            "Existing #[cfg(test)] block found at line range: {} - {}",
            start_line, end_line
        );
        eprintln!("Inserting test functions into the existing block...");

        eprintln!("Content being inserted:\n{}", test_functions);
        eprintln!("Insertion point: {}", test_block_end - 1);

        updated_code.insert_str(test_block_end - 1, &format!("\n\n{}\n", test_functions));
    } else {
        eprintln!("No #[cfg(test)] block found.");
        eprintln!("Creating a new #[cfg(test)] block and inserting test functions...");

        // Find the last function in the source code
        if let Some((last_fn_start, last_fn_end)) = find_last_function(&source_code) {
            let start_line = source_code[..last_fn_start].lines().count();
            let end_line = source_code[..last_fn_end].lines().count();
            eprintln!(
                "Last function found at byte range: {} - {}",
                last_fn_start, last_fn_end
            );
            eprintln!(
                "Last function found at line range: {} - {}",
                start_line, end_line
            );

            eprintln!("Content being inserted:\n{}", test_functions);
            eprintln!("Insertion point: {}", last_fn_end);

            updated_code.insert_str(
                last_fn_end,
                &format!("\n\n#[cfg(test)]\nmod tests {{\n{}\n}}\n", test_functions),
            );
        } else {
            eprintln!("No functions found in the source code.");
            eprintln!("Appending test functions at the end of the file...");

            eprintln!("Content being inserted:\n{}", test_functions);
            eprintln!("Insertion point: {}", source_code.len());

            updated_code.push_str(&format!(
                "\n\n#[cfg(test)]\nmod tests {{\n{}\n}}\n",
                test_functions
            ));
        }
    }

    updated_code
}

fn find_cfg_test_block(source_code: &str) -> Option<(usize, usize)> {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_rust() };
    parser
        .set_language(&language)
        .expect("Error setting language");
    let tree = parser
        .parse(source_code, None)
        .expect("Error parsing source code");
    let root_node = tree.root_node();

    eprintln!("Searching for #[cfg(test)] block...");

    let mut cursor = root_node.walk();
    for node in root_node.children(&mut cursor) {
        if node.kind() == "attribute_item" {
            let attribute_text = node.utf8_text(source_code.as_bytes()).unwrap();
            eprintln!("Found attribute: {}", attribute_text);
            if attribute_text.contains("#[cfg(test)]") {
                eprintln!("Found #[cfg(test)] attribute");
                if let Some(sibling) = node.next_sibling() {
                    eprintln!("Next sibling kind: {}", sibling.kind());
                    if sibling.kind() == "mod_item" {
                        eprintln!("Found #[cfg(test)] block");
                        return Some((sibling.start_byte(), sibling.end_byte()));
                    }
                }
            }
        }
    }

    eprintln!("No #[cfg(test)] block found");
    None
}

fn find_last_function(source_code: &str) -> Option<(usize, usize)> {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_rust() };
    parser
        .set_language(&language)
        .expect("Error setting language");
    let tree = parser
        .parse(source_code, None)
        .expect("Error parsing source code");
    let root_node = tree.root_node();

    eprintln!("Searching for the last function...");

    let mut last_fn_range = None;
    let mut cursor = root_node.walk();
    for node in root_node.children(&mut cursor) {
        if node.kind() == "function_item" {
            eprintln!(
                "Found function: {}",
                node.utf8_text(source_code.as_bytes()).unwrap()
            );
            last_fn_range = Some((node.start_byte(), node.end_byte()));
        }
    }

    if last_fn_range.is_none() {
        eprintln!("No functions found");
    }

    last_fn_range
}

fn find_properties_start_byte(source_code: &str, start_byte: usize) -> usize {
    let mut properties_start_byte = start_byte;
    let mut found_closing_brace = false;

    for i in (0..start_byte).rev() {
        if source_code.as_bytes()[i] == b'}' {
            found_closing_brace = true;
        } else if source_code.as_bytes()[i] == b'#' && source_code.as_bytes()[i + 1] == b'[' {
            if found_closing_brace {
                break;
            } else {
                properties_start_byte = i;
                break;
            }
        }
    }

    properties_start_byte
}

fn find_doc_end_tree_sitter(source_code: &str, language: &Language) -> usize {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Error setting language");
    let tree = parser
        .parse(source_code, None)
        .expect("Error parsing source code");
    let root_node = tree.root_node();

    let mut doc_end = 0;
    let mut cursor = root_node.walk();
    for node in root_node.children(&mut cursor) {
        if node.kind() == "comment" {
            doc_end = node.end_byte();
        } else {
            break;
        }
    }
    doc_end
}

fn get_structures<'a>(node: &'a Node<'a>, source_code: &'a [u8], structures: &mut Vec<String>) {
    let mut cursor = node.walk();

    if node.kind() == "function_item"
        || node.kind() == "mod_item"
        || node.kind() == "struct_item"
        || node.kind() == "enum_item"
        || node.kind() == "trait_item"
        || node.kind() == "impl_item"
        || node.kind() == "function_definition"  // Python
        || node.kind() == "class_definition"     // Python
        || node.kind() == "module"                // Python
        || node.kind() == "method"            // Ruby
        || node.kind() == "class"             // Ruby
        || node.kind() == "module"            // Ruby
    // Python
    {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source_code).unwrap();
            structures.push(format!("{}", name));
        }
    }

    for child in node.children(&mut cursor) {
        get_structures(&child, source_code, structures);
    }
}

fn find_main_function<'a>(node: &'a Node<'a>, source_code: &'a [u8]) -> Option<(usize, usize)> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "function_item" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = name_node.utf8_text(source_code).unwrap();
                if name == "main" {
                    return Some((child.start_byte(), child.end_byte()));
                }
            }
        }

        if let Some(range) = find_main_function(&child, source_code) {
            return Some(range);
        }
    }

    None
}

fn find_main_function_start<'a>(node: &'a Node<'a>, source_code: &'a [u8]) -> Option<usize> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "function_definition" || child.kind() == "class_definition" || child.kind() == "method_definition" || child.kind() == "method" {
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = name_node.utf8_text(source_code).unwrap();
                if name == "main" {
                    return Some(child.start_byte());
                }
            }
        }

        if let Some(start_byte) = find_main_function_start(&child, source_code) {
            return Some(start_byte);
        }
    }

    None
}
