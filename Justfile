# Binary name derived from Cargo.toml
set shell := ["nu", "-c"]

bin_name := `cargo metadata --no-deps --format-version 1 | from json | get packages.0.name`

# Target triples
target_macos_x86 := "x86_64-apple-darwin"
target_macos_arm := "aarch64-apple-darwin"
target_linux := "x86_64-unknown-linux-gnu"
target_windows := "x86_64-pc-windows-msvc"

# Build a release binary for a specific target
build target:
    #!/usr/bin/env nu
    let tool = if ("{{ target }}" | str contains "linux") { "cross" } else { "cargo" }
    let suffix = if ("{{ target }}" | str contains "windows") { ".exe" } else { "" }
    run-external $tool "build" "--release" "--target" "{{ target }}"
    mkdir dist/{{ target }}
    cp $"target/{{ target }}/release/{{ bin_name }}($suffix)" $"dist/{{ target }}/{{ bin_name }}($suffix)"

# Build all cross-compilable targets (macOS x86_64, macOS arm64, Linux)
build-all: (build target_macos_x86) (build target_macos_arm) (build target_linux)

# Build for Windows (run on a Windows machine)
build-windows: (build target_windows)

# Clean build artifacts for a specific target
_clean target:
    #!/usr/bin/env nu
    rm -rf $"target/{{ target }}/release"
    rm -rf $"dist/{{ target }}"

# Clean and rebuild a release binary for a specific target
rebuild target: (_clean target) (build target)

# Clean and rebuild all cross-compilable targets
rebuild-all: clean build-all

# Run clippy with default lints
check:
    cargo clippy

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test

# Clean build and dist artifacts
clean:
    cargo clean
    rm -rf dist
