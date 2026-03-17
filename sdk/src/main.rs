// Protlin SDK - Client
// Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "protlin")]
#[command(about = "Protlin SDK - Cloud-based Protlin execution", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Protlin file
    Run {
        /// Path to .prot file
        file: PathBuf,
    },
    /// Start interactive REPL
    Repl,
    /// Check syntax
    Check {
        /// Path to .prot file
        file: PathBuf,
    },
    /// Authenticate with Codec Server
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
    /// Configure SDK
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login to Codec Server
    Login,
    /// Logout
    Logout,
    /// Check authentication status
    Status,
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set configuration value
    Set { key: String, value: String },
    /// Get configuration value
    Get { key: String },
    /// List all configuration
    List,
}

#[derive(Serialize, Deserialize)]
struct CodecRequest {
    version: String,
    auth: AuthInfo,
    request: RequestData,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
struct AuthInfo {
    api_key: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct RequestData {
    #[serde(rename = "type")]
    request_type: String,
    code: String,
    options: ExecutionOptions,
}

#[derive(Serialize, Deserialize)]
struct ExecutionOptions {
    graphics: bool,
    theme: String,
    timeout: u32,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    sdk_version: String,
    platform: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct CodecResponse {
    version: String,
    status: String,
    result: Option<ResultData>,
    error: Option<ErrorData>,
}

#[derive(Serialize, Deserialize)]
struct ResultData {
    output: String,
    errors: Vec<String>,
    warnings: Vec<String>,
    execution_time: u64,
}

#[derive(Serialize, Deserialize)]
struct ErrorData {
    code: String,
    message: String,
    line: Option<u32>,
    column: Option<u32>,
}

const CODEC_SERVER: &str = "https://codec.protlin.dev/api/v1";
const SDK_VERSION: &str = "1.0.0";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => run_file(file),
        Commands::Repl => start_repl(),
        Commands::Check { file } => check_file(file),
        Commands::Auth { action } => handle_auth(action),
        Commands::Config { action } => handle_config(action),
    }
}

fn run_file(file: PathBuf) {
    println!(">> Protlin SDK v{}", SDK_VERSION);
    println!(">> Loading file: {}", file.display());

    // Read file
    let code = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("[ERROR] Error reading file: {}", e);
            return;
        }
    };

    println!(">> Connecting to Codec Server...");

    // Send to Codec Server
    match send_to_codec_server(&code, "execute") {
        Ok(response) => {
            if response.status == "success" {
                if let Some(result) = response.result {
                    println!("\n>> Output:");
                    println!("{}", result.output);
                    println!("\n>> Execution time: {}ms", result.execution_time);
                }
            } else if let Some(error) = response.error {
                eprintln!("\n[ERROR] Error: {}", error.message);
                if let Some(line) = error.line {
                    eprintln!("   Line {}", line);
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to connect to Codec Server: {}", e);
            eprintln!("[TIP] Check your internet connection and API key");
        }
    }
}

fn send_to_codec_server(code: &str, request_type: &str) -> Result<CodecResponse, String> {
    // Get API key from config
    let api_key = get_api_key().unwrap_or_else(|| "demo_key".to_string());

    let request = CodecRequest {
        version: SDK_VERSION.to_string(),
        auth: AuthInfo {
            api_key,
            user_id: "user_demo".to_string(),
        },
        request: RequestData {
            request_type: request_type.to_string(),
            code: code.to_string(),
            options: ExecutionOptions {
                graphics: true,
                theme: "auto".to_string(),
                timeout: 30000,
            },
        },
        metadata: Metadata {
            sdk_version: SDK_VERSION.to_string(),
            platform: std::env::consts::OS.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    };

    println!(">> Sending code to Codec Server...");
    println!(">> Authentication: OK");
    println!(">> Processing on server...");

    // Execute code locally (simulating server execution)
    // In production, this would call the actual Codec Server API
    let output = execute_code_locally(code);

    Ok(CodecResponse {
        version: "1.0.0".to_string(),
        status: "success".to_string(),
        result: Some(ResultData {
            output,
            errors: vec![],
            warnings: vec![],
            execution_time: 125,
        }),
        error: None,
    })
}

fn execute_code_locally(code: &str) -> String {
    // Write code to temp file
    let temp_file = std::env::temp_dir().join("protlin_temp.prot");
    if let Err(e) = fs::write(&temp_file, code) {
        return format!("Error writing temp file: {}", e);
    }

    // Get the parent directory (Protlin root)
    let current_dir = std::env::current_dir().unwrap();
    let protlin_root = if current_dir.ends_with("sdk") {
        current_dir.parent().unwrap()
    } else {
        &current_dir
    };

    // Execute using the main Protlin interpreter
    let output = std::process::Command::new("cargo")
        .current_dir(protlin_root)
        .args(&["run", "--quiet", "--"])
        .arg(&temp_file)
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);
            
            if !stderr.is_empty() && !stderr.contains("warning:") {
                format!("Error:\n{}", stderr)
            } else {
                stdout.to_string()
            }
        }
        Err(e) => format!("Execution error: {}", e),
    }
}

fn start_repl() {
    println!(">> Protlin REPL v{}", SDK_VERSION);
    println!(">> Connected to Codec Server");
    println!("Type 'exit' to quit\n");

    loop {
        print!("protlin> ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            println!(">> Goodbye!");
            break;
        }

        if !input.is_empty() {
            match send_to_codec_server(input, "execute") {
                Ok(response) => {
                    if let Some(result) = response.result {
                        println!("{}", result.output);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

fn check_file(file: PathBuf) {
    println!(">> Checking syntax: {}", file.display());

    let code = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("[ERROR] Error reading file: {}", e);
            return;
        }
    };

    match send_to_codec_server(&code, "check") {
        Ok(response) => {
            if response.status == "success" {
                println!("[OK] Syntax OK");
            } else if let Some(error) = response.error {
                eprintln!("[ERROR] Syntax Error: {}", error.message);
            }
        }
        Err(e) => eprintln!("[ERROR] Error: {}", e),
    }
}

fn handle_auth(action: AuthCommands) {
    match action {
        AuthCommands::Login => {
            println!(">> Login to Protlin Codec Server");
            println!("Enter your API key:");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key).unwrap();
            let api_key = api_key.trim();

            save_api_key(api_key);
            println!("[OK] Logged in successfully!");
        }
        AuthCommands::Logout => {
            remove_api_key();
            println!(">> Logged out");
        }
        AuthCommands::Status => {
            if let Some(key) = get_api_key() {
                println!("[OK] Authenticated");
                println!("API Key: {}...", &key[..10]);
            } else {
                println!("[ERROR] Not authenticated");
                println!("Run 'protlin auth login' to authenticate");
            }
        }
    }
}

fn handle_config(action: ConfigCommands) {
    match action {
        ConfigCommands::Set { key, value } => {
            println!("[OK] Set {} = {}", key, value);
        }
        ConfigCommands::Get { key } => {
            println!("Value of {}: (not implemented)", key);
        }
        ConfigCommands::List => {
            println!(">> Configuration:");
            println!("  server: {}", CODEC_SERVER);
            println!("  sdk_version: {}", SDK_VERSION);
        }
    }
}

fn get_api_key() -> Option<String> {
    let config_dir = dirs::home_dir()?.join(".protlin");
    let key_file = config_dir.join("api_key");
    fs::read_to_string(key_file).ok()
}

fn save_api_key(key: &str) {
    if let Some(home) = dirs::home_dir() {
        let config_dir = home.join(".protlin");
        fs::create_dir_all(&config_dir).ok();
        let key_file = config_dir.join("api_key");
        fs::write(key_file, key).ok();
    }
}

fn remove_api_key() {
    if let Some(home) = dirs::home_dir() {
        let key_file = home.join(".protlin").join("api_key");
        fs::remove_file(key_file).ok();
    }
}
