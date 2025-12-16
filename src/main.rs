// SPDX-License-Identifier: WTFPL
//! A cross-platform utility to extract system include directories from C++ compilers.
//!
//! This tool queries a C++ compiler to discover its default system include directories.
//! It supports gcc-like compilers (gcc, clang, etc.) and provides platform-specific fallbacks:
//!
//! - **Unix-like platforms**: Uses `/usr/bin/c++` as the default compiler when none is specified
//! - **Windows**: Parses the `INCLUDE` environment variable (`;` separated paths) when no compiler is specified
//!
//! For gcc-like compilers, the tool invokes the compiler with `-v -E -x c++ -` and parses
//! the output to extract include directory paths.

use clap::Parser;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "get-system-include-dirs")]
#[command(about = "Extract system include directories from C++ compiler", long_about = None)]
struct Args {
    /// Path to the C++ compiler to query
    #[arg(short, long)]
    compiler: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match get_include_dirs(args.compiler) {
        Ok(dirs) => {
            for dir in dirs {
                println!("{}", dir);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Gets system include directories using the specified compiler or platform defaults.
///
/// # Arguments
///
/// * `compiler` - Optional path to a C++ compiler. If `None`, uses platform-specific defaults.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error message if the operation fails
///
/// # Platform behavior
///
/// - **Windows (no compiler specified)**: Parses the `INCLUDE` environment variable
/// - **Unix-like (no compiler specified)**: Uses `/usr/bin/c++`
/// - **Compiler specified**: Invokes the compiler with `-v` to extract include directories
fn get_include_dirs(compiler: Option<PathBuf>) -> Result<Vec<String>, String> {
    if cfg!(windows) && compiler.is_none() {
        // On Windows without a specified compiler, parse $INCLUDE
        return get_windows_include_dirs();
    }

    // Unix-like platforms or when compiler is specified
    let compiler_path = compiler.unwrap_or_else(|| {
        if cfg!(unix) {
            PathBuf::from("/usr/bin/c++")
        } else {
            PathBuf::from("c++")
        }
    });

    // On Windows, check if the compiler is MSVC-like
    if cfg!(windows) && is_msvc_like_compiler(&compiler_path) {
        return get_windows_include_dirs();
    }

    get_compiler_include_dirs(&compiler_path)
}

/// Checks if a compiler is MSVC-like based on its filename.
///
/// MSVC-like compilers include: cl, cl.exe, clang-cl, clang-cl.exe
///
/// # Arguments
///
/// * `compiler` - Path to the compiler executable
///
/// # Returns
///
/// `true` if the compiler filename matches the pattern `cl(?:\.exe)$`
fn is_msvc_like_compiler(compiler: &PathBuf) -> bool {
    if let Some(filename) = compiler.file_name() {
        if let Some(name) = filename.to_str() {
            let msvc_pattern = Regex::new(r"cl(?:\.exe)?$").unwrap();
            return msvc_pattern.is_match(name);
        }
    }
    false
}

/// Extracts include directories from the Windows `INCLUDE` environment variable.
///
/// Parses semicolon-separated paths from the `INCLUDE` environment variable,
/// filtering out empty entries.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error if the `INCLUDE` environment variable is not set
fn get_windows_include_dirs() -> Result<Vec<String>, String> {
    match env::var("INCLUDE") {
        Ok(include_var) => {
            let dirs: Vec<String> = include_var
                .split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.replace('\\', "/"))
                .collect();
            Ok(dirs)
        }
        Err(_) => Err("INCLUDE environment variable not set".to_string()),
    }
}

/// Extracts include directories by invoking a gcc-like compiler with verbose flags.
///
/// Runs the compiler with `-v -E -x c++ -` arguments to generate verbose output
/// about its configuration, then parses the stderr output to extract include directories.
///
/// # Arguments
///
/// * `compiler` - Path to the C++ compiler executable
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error if the compiler fails to execute or no directories are found
fn get_compiler_include_dirs(compiler: &PathBuf) -> Result<Vec<String>, String> {
    // Run compiler with -v flag to get verbose output
    // We need to provide some input, so we use echo with a simple C++ snippet
    let output = Command::new(compiler)
        .arg("-v")
        .arg("-E")
        .arg("-x")
        .arg("c++")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let output = output.map_err(|e| format!("Failed to execute compiler: {}", e))?;

    // gcc-like compilers write -v output to stderr
    let stderr = String::from_utf8_lossy(&output.stderr);

    parse_include_dirs(&stderr)
}

/// Parses include directories from gcc-like compiler verbose output.
///
/// Extracts directory paths from the section between `#include <...> search starts here:`
/// and `End of search list.` in the compiler's output. Also handles platform-specific
/// annotations like `(framework directory)` on macOS.
///
/// # Arguments
///
/// * `compiler_output` - The stderr output from running the compiler with `-v`
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error if no include directories are found in the output
fn parse_include_dirs(compiler_output: &str) -> Result<Vec<String>, String> {
    let mut dirs = Vec::new();
    let mut in_include_section = false;
    let annotation_pattern = Regex::new(r"\s*\(.*\)$").unwrap();

    for line in compiler_output.lines() {
        let trimmed = line.trim();

        // Start of include directory section
        if trimmed.contains("#include <...> search starts here:") {
            in_include_section = true;
            continue;
        }

        // End of include directory section
        if trimmed.contains("End of search list.") {
            break;
        }

        // Collect directory paths
        if in_include_section && !trimmed.is_empty() {
            // Remove trailing annotations like "(framework directory)" on macOS
            let cleaned = annotation_pattern.replace(trimmed, "");
            let path = cleaned.trim();

            if !path.is_empty() {
                // Normalize path separators to forward slashes
                let normalized = path.replace('\\', "/");
                dirs.push(normalized);
            }
        }
    }

    if dirs.is_empty() {
        Err("No include directories found in compiler output".to_string())
    } else {
        Ok(dirs)
    }
}
