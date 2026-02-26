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
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(windows)]
mod windows_vs;

#[derive(Parser, Debug)]
#[command(name = "get-system-include-dirs")]
#[command(about = "Extract system include directories from C++ compiler", long_about = None)]
struct Args {
    /// Path to the C++ compiler to query
    #[arg(short, long)]
    compiler: Option<PathBuf>,

    /// Output file path (use '-' for stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Visual Studio version (e.g., "2022", "2026", "17", "18")
    /// Only used on Windows when no compiler is specified
    #[cfg(windows)]
    #[arg(long)]
    vs_version: Option<String>,

    /// Extra arguments forwarded verbatim to the compiler (gcc-like only, requires --compiler)
    #[arg(last = true)]
    compiler_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    match get_include_dirs(
        args.compiler,
        args.compiler_args,
        #[cfg(windows)]
        args.vs_version,
    ) {
        Ok(dirs) => {
            if let Err(e) = write_output(&dirs, args.output) {
                eprintln!("Error writing output: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Writes include directories to the specified output destination.
///
/// # Arguments
///
/// * `dirs` - Vector of include directory paths to write
/// * `output` - Optional output destination: `None` or `Some("-")` for stdout, `Some(path)` for file
///
/// # Returns
///
/// * `Ok(())` - Successfully wrote output
/// * `Err(io::Error)` - Failed to write output
fn write_output(dirs: &[String], output: Option<String>) -> io::Result<()> {
    let content = dirs.join("\n");

    match output.as_deref() {
        None | Some("-") => {
            // Write to stdout
            println!("{}", content);
        }
        Some(path) => {
            // Write to file
            let mut file = File::create(path)?;
            writeln!(file, "{}", content)?;
        }
    }

    Ok(())
}

/// Gets system include directories using the specified compiler or platform defaults.
///
/// # Arguments
///
/// * `compiler` - Optional path to a C++ compiler. If `None`, uses platform-specific defaults.
/// * `compiler_args` - Extra arguments forwarded verbatim to the compiler. Only applied when
///   `compiler` is explicitly specified and is non-MSVC-like; otherwise a warning is emitted.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error message if the operation fails
///
/// # Platform behavior
///
/// - **Windows (no compiler, or MSVC-like compiler)**: Uses `$INCLUDE` env var or auto-detects
///   Visual Studio via `vswhere.exe` and `vsdevcmd.bat`
/// - **Unix-like (no compiler specified)**: Uses `/usr/bin/c++`
/// - **Compiler specified (non-MSVC-like)**: Invokes the compiler with `-v` to extract include directories
fn get_include_dirs(
    compiler: Option<PathBuf>,
    compiler_args: Vec<String>,
    #[cfg(windows)] vs_version: Option<String>,
) -> Result<Vec<String>, String> {
    // Warn if extra args provided but cannot be applied
    if !compiler_args.is_empty() && compiler.is_none() {
        eprintln!("warning: compiler args ignored — no --compiler specified");
    }
    #[cfg(windows)]
    if !compiler_args.is_empty() && compiler.as_deref().is_some_and(is_msvc_like_compiler) {
        eprintln!("warning: compiler args ignored for MSVC-like compilers");
    }

    #[cfg(windows)]
    if compiler.as_deref().is_none_or(is_msvc_like_compiler) {
        // On Windows with no compiler, or with an MSVC-like compiler, use $INCLUDE or auto-detect VS
        return windows_vs::get_windows_include_dirs_with_fallback(vs_version.as_deref());
    }

    // Only forward extra args when compiler was explicitly specified
    let compiler_is_explicit = compiler.is_some();
    let compiler_path = compiler.unwrap_or_else(|| {
        if cfg!(unix) {
            PathBuf::from("/usr/bin/c++")
        } else {
            PathBuf::from("c++")
        }
    });
    let effective_args: &[String] = if compiler_is_explicit { &compiler_args } else { &[] };

    get_compiler_include_dirs(&compiler_path, effective_args)
}

/// Returns `true` if the compiler filename matches a known MSVC-like compiler.
///
/// Matches (case-insensitive): `cl`, `cl.exe`, `clang-cl`, `clang-cl.exe`
///
/// # Arguments
///
/// * `compiler` - Path to the compiler executable
#[cfg(windows)]
fn is_msvc_like_compiler(compiler: &Path) -> bool {
    compiler
        .file_name()
        .and_then(|n| n.to_str())
        .map(|name| {
            let lower = name.to_ascii_lowercase();
            matches!(lower.as_str(), "cl" | "cl.exe" | "clang-cl" | "clang-cl.exe")
        })
        .unwrap_or(false)
}

/// Extracts include directories by invoking a gcc-like compiler with verbose flags.
///
/// Runs the compiler with `-v -E -x c++ [extra_args] -` arguments to generate verbose output
/// about its configuration, then parses the stderr output to extract include directories.
///
/// # Arguments
///
/// * `compiler` - Path to the C++ compiler executable
/// * `extra_args` - Additional arguments inserted after `-x c++` and before the stdin sentinel `-`
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of include directory paths
/// * `Err(String)` - An error if the compiler fails to execute or no directories are found
fn get_compiler_include_dirs(compiler: &PathBuf, extra_args: &[String]) -> Result<Vec<String>, String> {
    // Run compiler with -v flag to get verbose output
    // We need to provide some input, so we use echo with a simple C++ snippet
    let output = Command::new(compiler)
        .arg("-v")
        .arg("-E")
        .arg("-x")
        .arg("c++")
        .args(extra_args)
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
