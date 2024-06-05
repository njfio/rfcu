

use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use clap::Arg;
use serde::Deserialize;
use tree_sitter::{Parser, Language, Node};
use regex::Regex;

extern "C" { fn tree_sitter_rust() -> Language; }

#[derive(Deserialize)]
struct Settings {
    flowname: String,
    commit_message_flow: String,
    language: String,
    requests: Requests,
    lint_command: Option<String>,
    max_retries: usize,
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

fn main() -> io::Result<()> {
    eprintln!("Reading configuration file...");
    let config_content = fs::read_to_string("/Users/n/.rfcu/config.toml").expect("Failed to read config file");
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
                .value_parser(["improvement", "add_functionality", "add_tests_function", "documentation_whole_file", "documentation_structure", "whole_file"])
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
        let file_path = get_structure_matches.get_one::<String>("file_path").expect("File path is required");

        eprintln!("Reading source code from file: {}", file_path);
        let source_code = fs::read_to_string(file_path).expect("Failed to read source file");

        eprintln!("Initializing parser and setting language: {}", settings.language);
        let mut parser = Parser::new();
        let language = match settings.language.as_str() {
            "rust" => unsafe { tree_sitter_rust() },
            _ => {
                eprintln!("Unsupported language: {}", settings.language);
                std::process::exit(1);
            }
        };

        if let Err(e) = parser.set_language(&language) {
            eprintln!("Error setting language: {:?}", e);
            std::process::exit(1);
        }

        eprintln!("Parsing the source code...");
        let tree = parser.parse(&source_code, None).expect("Error parsing source code");
        let root_node = tree.root_node();

        let mut structures = Vec::new();
        get_structures(&root_node, source_code.as_bytes(), &mut structures);

        eprintln!("Structures found in the source code:");
        for structure in structures {
            println!("{}", structure);
        }
        Ok(())
    } else {
        let mode = matches.get_one::<String>("mode").expect("Mode is required");
        let file_path = matches.get_one::<String>("file_path").expect("File path is required");
        let default_structure_name = String::new();
        let structure_name = matches.get_one::<String>("structure_name").unwrap_or(&default_structure_name);
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
        let user_request = stdin_content.trim().to_string();
        eprintln!("User request: {}", user_request);

        eprintln!("Initializing parser and setting language: {}", settings.language);
        let mut parser = Parser::new();
        let language = match settings.language.as_str() {
            "rust" => unsafe { tree_sitter_rust() },
            _ => {
                eprintln!("Unsupported language: {}", settings.language);
                restore_backup(file_path, &backup_file_path);
                std::process::exit(1);
            }
        };

        if let Err(e) = parser.set_language(&language) {
            eprintln!("Error setting language: {:?}", e);
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

            if mode == "improvement" || mode == "add_tests_function" || mode == "documentation_structure" {
                eprintln!("Parsing the source code...");
                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();
                eprintln!("Root node: {}", root_node.kind());
                eprintln!("Structure name: {}", structure_name);

                let structure_range = find_structure(&root_node, structure_name, source_code.as_bytes());
                (start_byte, end_byte) = structure_range.unwrap();

                // Find the start byte of the attributes and code above the function
                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);

                let original_structure = &source_code[properties_start_byte..end_byte];
                //eprintln!("Original structure found: {}", original_structure);

                //eprintln!("Preparing the request...");
                request = match mode.as_str() {
                    "improvement" => settings.requests.improvement.replace("{structure_code}", original_structure).replace("{user_request}", &user_request).replace("{structure_name}", &structure_name),
                    "add_tests_function" => settings.requests.add_tests_function.replace("{structure_code}", original_structure),
                    "documentation_structure" => settings.requests.documentation_structure.replace("{structure_code}", original_structure).replace("{user_request}", &user_request).replace("{structure_name}", &structure_name),
                    _ => unreachable!(),
                };
            } else if mode == "whole_file" || mode == "documentation_whole_file"  {
                eprintln!("Preparing the whole file request...");
                request = match mode.as_str() {
                    "whole_file" => settings.requests.whole_file.replace("{source_code}", &source_code).replace("{user_request}", &user_request),
                    "documentation_whole_file" => settings.requests.documentation_whole_file.replace("{source_code}", &source_code).replace("{user_request}", &user_request),
                    _ => unreachable!(),
                };
                start_byte = 0;
                end_byte = source_code.len();
            } else if mode == "add_functionality" {
                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();
                eprintln!("Root node: {}", root_node.kind());
                eprintln!("Structure name: {}", structure_name);
                eprintln!("Preparing the request to add functionality...");
                let structure_range = find_structure(&root_node, "main", source_code.as_bytes());
                (start_byte, end_byte) = structure_range.unwrap();

                // Find the start byte of the attributes and code above the function
                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);

                let _original_structure = &source_code[properties_start_byte..end_byte];
                request = settings.requests.add_functionality.replace("{source_code}", &source_code).replace("{user_request}", &user_request);
            } else {
                eprintln!("Invalid mode: {}", mode);
                restore_backup(file_path, &backup_file_path);
                std::process::exit(1);
            }

            eprintln!("Sending the request to fluentcli for improvement...");
            let improved_structure = match improve_structure_with_fluentcli(&settings.flowname, &request, &user_request, file_path) {
                Ok(structure) => structure,
                Err(e) => {
                    eprintln!("Error improving structure: {:?}", e);
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
            };

            eprintln!("Improved structure received: {}", improved_structure);

            let updated_code = if mode == "improvement"  {
                eprintln!("Updating the source code with the improved structure...");
                let mut updated_code = String::new();
                let properties_start_byte = find_properties_start_byte(&source_code, start_byte);
                updated_code.push_str(&source_code[..properties_start_byte]);
                updated_code.push_str(&improved_structure);
                updated_code.push_str(&source_code[end_byte..]);
                updated_code
            } else if mode == "get_structure" {
                eprintln!("Reading source code from file: {}", file_path);
                let source_code = fs::read_to_string(file_path).expect("Failed to read source file");

                eprintln!("Initializing parser and setting language: {}", settings.language);
                let mut parser = Parser::new();
                let language = match settings.language.as_str() {
                    "rust" => unsafe { tree_sitter_rust() },
                    _ => {
                        eprintln!("Unsupported language: {}", settings.language);
                        std::process::exit(1);
                    }
                };

                if let Err(e) = parser.set_language(&language) {
                    eprintln!("Error setting language: {:?}", e);
                    std::process::exit(1);
                }

                eprintln!("Parsing the source code...");
                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();

                let mut structures = Vec::new();
                get_structures(&root_node, source_code.as_bytes(), &mut structures);

                eprintln!("Structures found in the source code:");
                for structure in structures {
                    println!("{}", structure);
                }
                break;
            } else if mode == "add_functionality" {
                eprintln!("Adding new functionality to the script...");
                let request = settings.requests.add_functionality.replace("{source_code}", &source_code);

                eprintln!("Sending the request to fluentcli for improvement...");
                let improved_structure = match improve_structure_with_fluentcli(&settings.flowname, &request, &user_request, file_path) {
                    Ok(structure) => structure,
                    Err(e) => {
                        eprintln!("Error improving structure: {:?}", e);
                        restore_backup(file_path, &backup_file_path);
                        std::process::exit(1);
                    }
                };

                eprintln!("New functionality received: {}", improved_structure);

                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();
                let main_fn_range = find_main_function(&root_node, source_code.as_bytes());

                if let Some((_, main_end)) = main_fn_range {
                    eprintln!("main function found. Inserting new functionality after main...");
                    let mut updated_code = source_code[..main_end].to_string();
                    eprintln!("\n\n\n\n\n\nupdated_code: {}\n\n\n\n\n", updated_code);
                    updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                    updated_code.push_str(&source_code[main_end..]);
                    eprintln!("Updated code:\n{}", updated_code);
                    updated_code
                } else {
                    eprintln!("main function not found. Appending new functionality at the end...");
                    let mut updated_code = source_code.to_string();
                    updated_code.push_str(&format!("\n\n{}\n\n", improved_structure));
                    eprintln!("Updated code:\n{}", updated_code);
                    updated_code
                }
            } else if mode == "documentation_structure" {
                eprintln!("Replacing or inserting the documentation in the structure...");
                let doc_comment = improved_structure.trim();
                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();
                let struct_range = find_struct_node(&root_node, structure_name, source_code.as_bytes());
                if struct_range.is_none() {
                    eprintln!("Structure not found in the source code.");
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
                let (struct_start, struct_end) = struct_range.unwrap();
                eprintln!("Struct found at byte range: {} - {}", struct_start, struct_end);
                let mut updated_code = source_code.to_string();

                // Find the documentation range above the structure
                let (doc_start, doc_end) = find_documentation_range(&source_code, struct_start);
                eprintln!("Documentation range: {} - {}", doc_start, doc_end);

                if doc_start != doc_end {
                    eprintln!("Replacing existing documentation...");
                    let new_code = format!(
                        "{}{}{}",
                        &source_code[..doc_start],
                        doc_comment,
                        &source_code[doc_end..]
                    );
                    updated_code = new_code;
                } else {
                    eprintln!("Inserting new documentation...");
                    updated_code.insert_str(struct_start, &format!("\n{}\n", doc_comment));
                }
                eprintln!("Updated code:\n{}", updated_code);
                updated_code
            } else if mode == "documentation_whole_file" {
                eprintln!("Replacing the documentation in the whole file...");
                let mut updated_code = String::new();

                // Find the start and end of the existing documentation using tree-sitter
                let (doc_start, doc_end) = find_doc_range_tree_sitter(&source_code, &language);

                // Overwrite the existing documentation with the new documentation
                updated_code.push_str(&source_code[..doc_start]);
                updated_code.push_str(&improved_structure);
                updated_code.push('\n');
                updated_code.push_str(&source_code[doc_end..]);
                updated_code
            } else if mode == "add_tests_function" {
                eprintln!("Adding test functions...");
                let test_functions = improved_structure.trim();
                let tree = parser.parse(&source_code, None).expect("Error parsing source code");
                let root_node = tree.root_node();
                let struct_range = find_structure(&root_node, structure_name, source_code.as_bytes());
                if struct_range.is_none() {
                    eprintln!("Target function not found in the source code.");
                    restore_backup(file_path, &backup_file_path);
                    std::process::exit(1);
                }
                let (struct_start, struct_end) = struct_range.unwrap();
                eprintln!("Target function found at byte range: {} - {}", struct_start, struct_end);
                eprintln!("Target function found at line range: {} - {}",
                          source_code[..struct_start].lines().count(),
                          source_code[..struct_end].lines().count()
                );
                let mut updated_code = source_code.to_string();

                // Insert the test functions below the target function
                eprintln!("Inserting test functions at byte: {}", struct_end);
                eprintln!("Inserting test functions at line: {}", source_code[..struct_end].lines().count());
                updated_code.insert_str(struct_end, &format!("\n{}\n", test_functions));

                eprintln!("Updated code:\n{}", updated_code);
                updated_code
            } else {
                improved_structure
            };
            eprintln!("Writing the updated code to the original file...");
            fs::write(file_path, updated_code.as_bytes())?;
            eprintln!("Updated code written to the original file successfully.");

            if let Some(lint_command) = &settings.lint_command {
                eprintln!("Linting the updated code...");
                if lint_code(lint_command).expect("Failed to lint the code") {
                    eprintln!("Linting succeeded.");
                    break;
                } else {
                    eprintln!("Linting failed. Restoring backup and retrying...");
                    restore_backup(file_path, &backup_file_path);
                }
            } else {
                break;
            }
        }
        let commit_message = generate_commit_message(file_path, mode, &settings.commit_message_flow)
            .unwrap_or_else(|_| "Automated changes made by RFCU".to_string());

        if let Err(e) = commit_changes(file_path, &commit_message) {
            eprintln!("Failed to commit changes: {:?}", e);
            restore_backup(file_path, &backup_file_path);
            std::process::exit(1);
        }

        eprintln!("Changes committed successfully.");

        eprintln!("Cleaning up the backup file...");
        fs::remove_file(&backup_file_path)?;// ... (rest of your existing code goes here)

        Ok(())
    }
}

fn retrieve_structures<'a>(node: &'a Node<'a>, source_code: &'a [u8], structures: &mut Vec<String>) {
    let cursor = &mut node.walk();
    for child in node.children(cursor) {
        if node.kind() == "function_item"
            || node.kind() == "mod_item"
            || node.kind() == "struct_item"
            || node.kind() == "enum_item"
            || node.kind() == "trait_item"
            || node.kind() == "impl_item"
        {
            let name_node = child.child_by_field_name("name");
            if let Some(name_node) = name_node {
                let name = std::str::from_utf8(name_node.utf8_text(source_code).unwrap().as_ref()).unwrap();
                structures.push(format!("{}: {}", child.kind(), name));
            }
        }
        retrieve_structures(&child, source_code, structures);
    }
}

fn find_structure<'a>(node: &'a Node<'a>, structure_name: &str, source_code: &'a [u8]) -> Option<(usize, usize)> {
    let mut cursor = node.walk();
    //eprintln!("Searching for structure in node: {}", node.kind());
    //eprintln!("structure_name: {}", structure_name);
    //eprintln!("Searching for structure: {}", structure_name);
    //eprintln!("Current node name: {}", node.kind());

    if node.kind() == "function_item"
        || node.kind() == "mod_item"
        || node.kind() == "struct_item"
        || node.kind() == "enum_item"
        || node.kind() == "trait_item"
        || node.kind() == "impl_item"
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


fn improve_structure_with_fluentcli(flowname: &str, request: &str, user_request: &str, source_code_path: &str) -> io::Result<String> {
    eprintln!("Starting fluentcli with flowname: {}, request: {}", flowname, request);
    let mut child = Command::new("fluent")
        .arg(flowname)
        .arg(request)
        .arg("--additional-context-file")
        .arg(source_code_path)
        .arg("-p")  // Parse the code blocks
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fluent");

    eprintln!("Writing user request to fluentcli stdin...");
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(user_request.as_bytes()).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    let response = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    eprintln!("Response from fluentcli:\n\n\n\n {}", response);

    // Extract the code block from the response
    let code_block_re = Regex::new(r"```(?:rust|python|ruby|javascript|typescript|bash|sh)?\n(.*?)```").unwrap();
    let improved_structure = if let Some(captures) = code_block_re.captures(&response) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        response  // In case the response is not in the expected format
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
    let request = format!("Generate a commit message for the changes made in {} mode to the file {} on a single line, it should be succinct.", mode, file_path);

    let child = Command::new("fluent")
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

fn find_struct_node<'a>(node: &'a Node<'a>, struct_name: &str, source_code: &'a [u8]) -> Option<(usize, usize)> {
    //eprintln!("Searching for struct: {}", struct_name);
    //eprintln!("Current node kind: {}", node.kind());

    if node.kind() == "function_item"
        || node.kind() == "mod_item"
        || node.kind() == "struct_item"
        || node.kind() == "enum_item"
        || node.kind() == "trait_item"
        || node.kind() == "impl_item" {
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
    parser.set_language(language).expect("Error setting language");
    let tree = parser.parse(source_code, None).expect("Error parsing source code");
    let root_node = tree.root_node();

    let mut doc_start = std::usize::MAX;
    let mut doc_end = 0;
    let mut cursor = root_node.walk();

    eprintln!("Searching for documentation range using tree-sitter...");

    for node in root_node.children(&mut cursor) {
        if node.kind() == "block_comment" {
            doc_start = node.start_byte();
            doc_end = node.end_byte();
            break;
        } else if !node.is_extra() {
            break;
        }
    }

    if doc_start == std::usize::MAX {
        eprintln!("No documentation block found.");
        (0, 0)
    } else {
        eprintln!("Documentation range:");
        eprintln!("  Start byte: {}", doc_start);
        eprintln!("  End byte: {}", doc_end);
        eprintln!("  Start line: {}", source_code[..doc_start].lines().count());
        eprintln!("  End line: {}", source_code[..doc_end].lines().count());
        (doc_start, doc_end)
    }
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


fn get_structures<'a>(node: &'a Node<'a>, source_code: &'a [u8], structures: &mut Vec<String>) {
    let mut cursor = node.walk();

    if node.kind() == "function_item"
        || node.kind() == "mod_item"
        || node.kind() == "struct_item"
        || node.kind() == "enum_item"
        || node.kind() == "trait_item"
        || node.kind() == "impl_item"
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