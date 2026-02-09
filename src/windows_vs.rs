// SPDX-License-Identifier: WTFPL
//! Windows Visual Studio detection and INCLUDE environment variable extraction.
//!
//! This module provides automatic Visual Studio detection on Windows when the INCLUDE
//! environment variable is not set. It uses Microsoft's vswhere.exe tool to locate
//! VS installations and runs vsdevcmd.bat to extract include directories.

use serde::Deserialize;
use std::env;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
struct VsInstance {
    #[serde(rename = "installationPath")]
    installation_path: String,

    #[allow(dead_code)]
    #[serde(rename = "installationVersion")]
    installation_version: String,
}

/// Gets Windows include directories with automatic VS detection fallback.
///
/// Precedence:
/// 1. Uses $INCLUDE if already set (highest priority)
/// 2. Finds VS with vswhere and runs vsdevcmd.bat to get INCLUDE
/// 3. Errors if vswhere not found or VS detection fails
///
/// # Arguments
/// * `vs_version` - Optional VS version filter ("2022", "2026", "17", "18")
///
/// # Returns
/// * `Ok(Vec<String>)` - Include directory paths (forward slashes)
/// * `Err(String)` - Detailed error message
pub fn get_windows_include_dirs_with_fallback(
    vs_version: Option<&str>,
) -> Result<Vec<String>, String> {
    // Priority 1: Check if $INCLUDE already set
    if let Ok(include_var) = env::var("INCLUDE") {
        return parse_include_env(&include_var);
    }

    // Priority 2: Find VS and get INCLUDE via vsdevcmd.bat
    find_vs_and_get_include(vs_version)
}

/// Parses semicolon-separated include paths from environment variable.
///
/// # Arguments
/// * `include_var` - The INCLUDE environment variable value
///
/// # Returns
/// * `Ok(Vec<String>)` - Parsed include directory paths
/// * `Err(String)` - Error if INCLUDE is empty
fn parse_include_env(include_var: &str) -> Result<Vec<String>, String> {
    let dirs: Vec<String> = include_var
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.replace('\\', "/"))
        .collect();

    if dirs.is_empty() {
        Err("INCLUDE environment variable is empty".to_string())
    } else {
        Ok(dirs)
    }
}

/// Finds Visual Studio installation and extracts INCLUDE via vsdevcmd.bat.
///
/// # Arguments
/// * `vs_version` - Optional VS version filter
///
/// # Returns
/// * `Ok(Vec<String>)` - Include directory paths
/// * `Err(String)` - Detailed error message showing what was attempted
fn find_vs_and_get_include(vs_version: Option<&str>) -> Result<Vec<String>, String> {
    // Step 1: Find vswhere.exe
    let vswhere_path = find_vswhere().map_err(|e| {
        format!(
            "INCLUDE environment variable not set.\nTried to find Visual Studio: {}",
            e
        )
    })?;

    // Step 2: Query for VS installation
    let vs_path = query_vswhere(&vswhere_path, vs_version).map_err(|e| {
        format!(
            "INCLUDE environment variable not set.\nTried to find Visual Studio: {}",
            e
        )
    })?;

    // Step 3: Run vsdevcmd.bat -arch=x64 and capture INCLUDE
    let vsdevcmd_path = format!("{}\\Common7\\Tools\\vsdevcmd.bat", vs_path);
    let include_value = run_vsdevcmd_and_capture_include(&vsdevcmd_path).map_err(|e| {
        format!(
            "INCLUDE environment variable not set.\nFound VS at: {}\nvsdevcmd.bat execution failed: {}",
            vs_path, e
        )
    })?;

    // Step 4: Parse INCLUDE value
    parse_include_env(&include_value)
}

/// Locates vswhere.exe at the standard installation path.
///
/// # Returns
/// * `Ok(PathBuf)` - Path to vswhere.exe
/// * `Err(String)` - Error if vswhere.exe not found
fn find_vswhere() -> Result<PathBuf, String> {
    let vswhere_path = PathBuf::from(
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe",
    );

    if vswhere_path.exists() {
        Ok(vswhere_path)
    } else {
        Err(format!(
            "vswhere.exe not found at standard location.\nExpected: {}",
            vswhere_path.display()
        ))
    }
}

/// Queries vswhere for Visual Studio installation path.
///
/// # Arguments
/// * `vswhere_path` - Path to vswhere.exe
/// * `vs_version` - Optional VS version filter
///
/// # Returns
/// * `Ok(String)` - VS installation path
/// * `Err(String)` - Error if VS not found or vswhere fails
fn query_vswhere(vswhere_path: &PathBuf, vs_version: Option<&str>) -> Result<String, String> {
    let mut cmd = Command::new(vswhere_path);
    cmd.arg("-format").arg("json");
    cmd.arg("-utf8");

    // Apply version filter if specified
    if let Some(ver) = vs_version {
        let version_range = map_version_to_range(ver)?;
        cmd.arg("-version").arg(version_range);
    } else {
        cmd.arg("-latest");
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute vswhere.exe: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "vswhere.exe failed with exit code: {}",
            output.status
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let instances: Vec<VsInstance> = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse vswhere JSON output: {}", e))?;

    instances
        .first()
        .map(|inst| inst.installation_path.clone())
        .ok_or_else(|| {
            if let Some(ver) = vs_version {
                format!("No Visual Studio installation found for version: {}", ver)
            } else {
                "No Visual Studio installation found".to_string()
            }
        })
}

/// Maps friendly version strings to vswhere version ranges.
///
/// # Arguments
/// * `version` - Version string ("2022", "17", "[17.0,18.0)", etc.)
///
/// # Returns
/// * `Ok(String)` - Version range for vswhere
/// * `Err(String)` - Error if version format is invalid
fn map_version_to_range(version: &str) -> Result<String, String> {
    let range = match version {
        "2026" | "18" => "[18.0,19.0)".to_string(),
        "2022" | "17" => "[17.0,18.0)".to_string(),
        "2019" | "16" => "[16.0,17.0)".to_string(),
        "2017" | "15" => "[15.0,16.0)".to_string(),
        custom if custom.starts_with('[') => custom.to_string(),
        _ => {
            return Err(format!(
                "Unknown VS version: {}. Use: 2017, 2019, 2022, 2026 or range like [17.0,18.0)",
                version
            ))
        }
    };
    Ok(range)
}

/// Runs vsdevcmd.bat with x64 architecture and captures INCLUDE variable.
///
/// # Arguments
/// * `vsdevcmd_path` - Path to vsdevcmd.bat
///
/// # Returns
/// * `Ok(String)` - INCLUDE environment variable value
/// * `Err(String)` - Error if vsdevcmd.bat fails or INCLUDE not found
fn run_vsdevcmd_and_capture_include(vsdevcmd_path: &str) -> Result<String, String> {
    // Build raw command line for Windows cmd
    // Must use call to execute batch file and continue to next command
    let cmdline = format!(
        "cmd /c \"call \"{}\" -arch=x64 >nul 2>&1 && set INCLUDE\"",
        vsdevcmd_path
    );

    let output = Command::new("cmd")
        .raw_arg(&cmdline[4..]) // Skip "cmd " prefix as Command::new already specifies cmd
        .output()
        .map_err(|e| format!("Failed to execute vsdevcmd.bat: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Try to parse INCLUDE even if exit code is non-zero
    // (vsdevcmd.bat may set variables but return non-zero exit code)

    // Parse "INCLUDE=..." from output
    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("INCLUDE=") {
            return Ok(value.to_string());
        }
    }

    // If we couldn't find INCLUDE, include both stdout and stderr in error
    Err(format!(
        "Could not find INCLUDE variable in vsdevcmd.bat output.\nExit code: {}\nStderr: {}\nStdout: {}",
        output.status, stderr.trim(), stdout.trim()
    ))
}
