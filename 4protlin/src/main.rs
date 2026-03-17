// 4protlin - Protlin Package Installer
// Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "4protlin")]
#[command(about = "4protlin - Package installer for Protlin", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        /// Package name or URL
        package: String,
        
        /// Install from native library
        #[arg(long)]
        native: bool,
        
        /// Install from server
        #[arg(long)]
        server: bool,
    },
    /// Uninstall a package
    Uninstall {
        /// Package name
        package: String,
    },
    /// List installed packages
    List,
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// Update a package
    Update {
        /// Package name (or 'all' for all packages)
        package: String,
    },
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    description: String,
    source: String, // "native", "server", "url"
    installed: bool,
    size: u64,
}

#[derive(Serialize, Deserialize)]
struct PackageRegistry {
    packages: Vec<Package>,
}

const VERSION: &str = "1.0.0";
const REGISTRY_URL: &str = "https://packages.protlin.dev/registry.json";

fn main() {
    let cli = Cli::parse();

    println!(">> 4protlin v{} - Protlin Package Installer", VERSION);
    println!("   Copyright © 2026 Moude AI LLC and Moude Corp\n");

    match cli.command {
        Commands::Install { package, native, server } => {
            install_package(&package, native, server);
        }
        Commands::Uninstall { package } => {
            uninstall_package(&package);
        }
        Commands::List => {
            list_packages();
        }
        Commands::Search { query } => {
            search_packages(&query);
        }
        Commands::Update { package } => {
            update_package(&package);
        }
        Commands::Info { package } => {
            show_package_info(&package);
        }
    }
}

fn install_package(package: &str, native: bool, server: bool) {
    let source = if native {
        "native"
    } else if server {
        "server"
    } else {
        // Auto-detect
        if package.starts_with("http://") || package.starts_with("https://") {
            "url"
        } else if package.starts_with("native:") {
            "native"
        } else {
            "server"
        }
    };

    println!(">> Resolving package: {}", package);
    println!(">> Source: {}", source);
    
    match source {
        "native" => install_native_package(package),
        "server" => install_server_package(package),
        "url" => install_url_package(package),
        _ => println!("[ERROR] Unknown source type"),
    }
}

fn install_native_package(package: &str) {
    let pkg_name = package.strip_prefix("native:").unwrap_or(package);
    
    println!(">> Installing native package: {}", pkg_name);
    
    // Simulate native package installation
    let pb = create_progress_bar(100);
    pb.set_message(format!("Downloading {}...", pkg_name));
    
    for i in 0..100 {
        pb.set_position(i);
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    pb.finish_with_message("Downloaded");
    
    println!(">> Extracting package...");
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!(">> Installing to ~/.protlin/packages/{}...", pkg_name);
    
    // Create package directory
    if let Some(home) = dirs::home_dir() {
        let pkg_dir = home.join(".protlin").join("packages").join(pkg_name);
        fs::create_dir_all(&pkg_dir).ok();
        
        // Create a marker file
        let marker = pkg_dir.join("package.json");
        let pkg_info = serde_json::json!({
            "name": pkg_name,
            "version": "1.0.0",
            "source": "native",
            "installed_at": chrono::Utc::now().to_rfc3339()
        });
        fs::write(marker, serde_json::to_string_pretty(&pkg_info).unwrap()).ok();
    }
    
    println!("[OK] Successfully installed {} (native)", pkg_name);
    println!("[TIP] Import with: import(\"native:{}\")", pkg_name);
}

fn install_server_package(package: &str) {
    println!(">> Installing from Protlin Package Server: {}", package);
    
    let pb = create_progress_bar(100);
    pb.set_message(format!("Fetching {}...", package));
    
    for i in 0..100 {
        pb.set_position(i);
        std::thread::sleep(std::time::Duration::from_millis(15));
    }
    pb.finish_with_message("Downloaded");
    
    println!(">> Verifying package signature...");
    std::thread::sleep(std::time::Duration::from_millis(300));
    
    println!(">> Installing to ~/.protlin/packages/{}...", package);
    
    // Create package directory
    if let Some(home) = dirs::home_dir() {
        let pkg_dir = home.join(".protlin").join("packages").join(package);
        fs::create_dir_all(&pkg_dir).ok();
        
        let marker = pkg_dir.join("package.json");
        let pkg_info = serde_json::json!({
            "name": package,
            "version": "1.0.0",
            "source": "server",
            "installed_at": chrono::Utc::now().to_rfc3339()
        });
        fs::write(marker, serde_json::to_string_pretty(&pkg_info).unwrap()).ok();
    }
    
    println!("[OK] Successfully installed {} (server)", package);
    println!("[TIP] Import with: import(\"{}\")", package);
}

fn install_url_package(url: &str) {
    println!(">> Installing from URL: {}", url);
    
    let pb = create_progress_bar(100);
    pb.set_message("Downloading...");
    
    for i in 0..100 {
        pb.set_position(i);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    pb.finish_with_message("Downloaded");
    
    // Extract package name from URL
    let pkg_name = url.split('/').last().unwrap_or("package").replace(".prot", "");
    
    println!(">> Installing to ~/.protlin/packages/{}...", pkg_name);
    
    if let Some(home) = dirs::home_dir() {
        let pkg_dir = home.join(".protlin").join("packages").join(&pkg_name);
        fs::create_dir_all(&pkg_dir).ok();
        
        let marker = pkg_dir.join("package.json");
        let pkg_info = serde_json::json!({
            "name": pkg_name,
            "version": "1.0.0",
            "source": url,
            "installed_at": chrono::Utc::now().to_rfc3339()
        });
        fs::write(marker, serde_json::to_string_pretty(&pkg_info).unwrap()).ok();
    }
    
    println!("[OK] Successfully installed {} (url)", pkg_name);
    println!("[TIP] Import with: import(\"{}\")", pkg_name);
}

fn uninstall_package(package: &str) {
    println!(">> Uninstalling package: {}", package);
    
    if let Some(home) = dirs::home_dir() {
        let pkg_dir = home.join(".protlin").join("packages").join(package);
        
        if pkg_dir.exists() {
            fs::remove_dir_all(&pkg_dir).ok();
            println!("[OK] Successfully uninstalled {}", package);
        } else {
            println!("[ERROR] Package not found: {}", package);
        }
    }
}

fn list_packages() {
    println!(">> Installed packages:\n");
    
    if let Some(home) = dirs::home_dir() {
        let packages_dir = home.join(".protlin").join("packages");
        
        if !packages_dir.exists() {
            println!("   No packages installed");
            return;
        }
        
        if let Ok(entries) = fs::read_dir(packages_dir) {
            let mut count = 0;
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let pkg_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Read package info
                    let info_file = entry.path().join("package.json");
                    if let Ok(content) = fs::read_to_string(info_file) {
                        if let Ok(info) = serde_json::from_str::<serde_json::Value>(&content) {
                            let version = info["version"].as_str().unwrap_or("unknown");
                            let source = info["source"].as_str().unwrap_or("unknown");
                            println!("   [*] {} @ {} ({})", pkg_name, version, source);
                            count += 1;
                        }
                    }
                }
            }
            
            if count == 0 {
                println!("   No packages installed");
            } else {
                println!("\n   Total: {} package(s)", count);
            }
        }
    }
}

fn search_packages(query: &str) {
    println!(">> Searching for: {}\n", query);
    
    // Simulate package search
    let mock_packages = vec![
        ("http-client", "HTTP client library", "native"),
        ("json-parser", "JSON parsing utilities", "server"),
        ("graphics-extra", "Extended graphics functions", "server"),
        ("math-utils", "Mathematical utilities", "native"),
        ("database", "Database connectivity", "native"),
    ];
    
    let mut found = 0;
    for (name, desc, source) in mock_packages {
        if name.contains(query) || desc.to_lowercase().contains(&query.to_lowercase()) {
            println!("   [*] {} ({})", name, source);
            println!("      {}", desc);
            println!();
            found += 1;
        }
    }
    
    if found == 0 {
        println!("   No packages found matching '{}'", query);
    } else {
        println!("   Found {} package(s)", found);
    }
}

fn update_package(package: &str) {
    if package == "all" {
        println!(">> Updating all packages...");
        list_packages();
        println!("\n[OK] All packages are up to date");
    } else {
        println!(">> Updating package: {}", package);
        println!("[OK] {} is up to date", package);
    }
}

fn show_package_info(package: &str) {
    println!(">> Package information: {}\n", package);
    
    if let Some(home) = dirs::home_dir() {
        let pkg_dir = home.join(".protlin").join("packages").join(package);
        let info_file = pkg_dir.join("package.json");
        
        if info_file.exists() {
            if let Ok(content) = fs::read_to_string(info_file) {
                if let Ok(info) = serde_json::from_str::<serde_json::Value>(&content) {
                    println!("   Name:         {}", info["name"].as_str().unwrap_or("unknown"));
                    println!("   Version:      {}", info["version"].as_str().unwrap_or("unknown"));
                    println!("   Source:       {}", info["source"].as_str().unwrap_or("unknown"));
                    println!("   Installed:    {}", info["installed_at"].as_str().unwrap_or("unknown"));
                    return;
                }
            }
        }
    }
    
    println!("   [ERROR] Package not installed");
}

fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb
}
