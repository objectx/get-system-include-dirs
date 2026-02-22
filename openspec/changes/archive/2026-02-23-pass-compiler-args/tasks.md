## 1. CLI: Add compiler_args field to Args

- [x] 1.1 Add `compiler_args: Vec<String>` with `#[arg(last = true)]` to the `Args` struct in `src/main.rs`
- [x] 1.2 Verify `cargo build` — help text shows `[-- <COMPILER_ARGS>...]` correctly

## 2. Thread args through call stack

- [x] 2.1 Add `compiler_args: Vec<String>` parameter to `get_include_dirs` signature
- [x] 2.2 Add `extra_args: &[String]` parameter to `get_compiler_include_dirs` signature
- [x] 2.3 Update the call site in `main()` to pass `args.compiler_args`

## 3. Warnings for inapplicable args

- [x] 3.1 In `get_include_dirs`, if `compiler_args` is non-empty and `compiler` is `None`, emit warning to stderr: `"warning: compiler args ignored — no --compiler specified"`
- [x] 3.2 On Windows, if `compiler_args` is non-empty and the compiler is MSVC-like, emit warning to stderr: `"warning: compiler args ignored for MSVC-like compilers"`

## 4. Apply extra args in compiler invocation

- [x] 4.1 In `get_compiler_include_dirs`, insert `.args(extra_args)` after `.arg("c++")` and before `.arg("-")`
- [x] 4.2 Verify `cargo build` — no warnings, no errors

## 5. Smoke test

- [x] 5.1 Test passthrough: `get-system-include-dirs --compiler clang++ -- --verbose` (expect normal output, no crash)
- [x] 5.2 Test cross-compilation (if toolchain available): `get-system-include-dirs --compiler clang++ -- --target x86_64-unknown-linux-gnu`
- [x] 5.3 Test warning path: `get-system-include-dirs -- --some-flag` (expect warning + output)
