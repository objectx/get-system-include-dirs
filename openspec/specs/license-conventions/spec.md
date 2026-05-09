# Spec: License Conventions

## Purpose

Encodes the project's dual `MIT OR Apache-2.0` licensing convention as machine-checkable requirements. Defines the license file layout at repo root, the `Cargo.toml` `license` SPDX expression, the per-source-file `SPDX-License-Identifier` header marker, and the canonical bodies of `LICENSE-MIT` and `LICENSE-APACHE`, so that drift from the convention is caught by `openspec validate --strict` and by per-requirement scenarios rather than slipping through code review.

## Requirements

### Requirement: License file layout

The repository SHALL provide both `LICENSE-MIT` and `LICENSE-APACHE` at its root, and SHALL NOT contain any other license-named file. The forbidden alternates include `COPYING`, `LICENSE`, `LICENSE.md`, `LICENCE`, `LICENCE.md`, and any case or spelling variant of the same. The two named files together MUST express the project's dual `MIT OR Apache-2.0` licensing.

#### Scenario: Both license files present at root

- **WHEN** the repository root is listed
- **THEN** a regular file named `LICENSE-MIT` SHALL exist at the root
- **THEN** a regular file named `LICENSE-APACHE` SHALL exist at the root

#### Scenario: No legacy or alternate license file remains

- **WHEN** the repository root is listed
- **THEN** no file named `COPYING` SHALL exist
- **THEN** no file named `LICENSE`, `LICENSE.md`, `LICENCE`, or `LICENCE.md` SHALL exist
- **THEN** no other license-named file SHALL exist beyond `LICENSE-MIT` and `LICENSE-APACHE`

---

### Requirement: Cargo manifest license field

The `Cargo.toml` `[package]` table SHALL set the `license` field to the SPDX expression `MIT OR Apache-2.0` exactly. The `license-file` field SHALL NOT be set.

#### Scenario: license expression matches SPDX dual form

- **WHEN** `Cargo.toml` is read
- **THEN** the `[package]` section SHALL contain `license = "MIT OR Apache-2.0"` exactly (case-sensitive, including the surrounding double quotes)

#### Scenario: license-file is not used

- **WHEN** `Cargo.toml` is read
- **THEN** the `[package]` section SHALL NOT contain a `license-file` key
- **THEN** the project's license SHALL be expressed via the `license` SPDX expression rather than via a path to a file

---

### Requirement: SPDX header in source files

Every `*.rs` file under `src/` SHALL contain `// SPDX-License-Identifier: MIT OR Apache-2.0` as one of its first two lines. No `*.rs` file under `src/` SHALL contain a `SPDX-License-Identifier:` line whose value differs from `MIT OR Apache-2.0`.

#### Scenario: Top-of-file SPDX marker is present

- **WHEN** any file matching `src/**/*.rs` is opened
- **THEN** one of its first two lines SHALL be exactly `// SPDX-License-Identifier: MIT OR Apache-2.0`

#### Scenario: No legacy or divergent SPDX value remains

- **WHEN** any file matching `src/**/*.rs` is searched for the substring `SPDX-License-Identifier:`
- **THEN** every match SHALL be followed by ` MIT OR Apache-2.0`
- **THEN** no match SHALL be followed by `WTFPL`, `MIT`, `Apache-2.0`, or any other identifier value differing from the dual expression

---

### Requirement: Canonical license texts

`LICENSE-MIT` SHALL contain the SPDX canonical MIT license template body, with a single `Copyright (c) <years> <holder>` line filled in by the project. `LICENSE-APACHE` SHALL contain the unmodified SPDX canonical Apache-2.0 license text. The body of `LICENSE-APACHE` SHALL NOT be edited to insert project-specific text — the project's copyright is recorded in `LICENSE-MIT`'s copyright line and in source-file SPDX headers, not in the Apache body.

#### Scenario: MIT body matches SPDX template aside from the copyright line

- **WHEN** `LICENSE-MIT` is read
- **THEN** its body SHALL be the SPDX MIT license template
- **THEN** the only project-specific deviation from the SPDX template SHALL be a single `Copyright (c) <years> <holder>` line filled in with project-specific year(s) and holder name
- **THEN** the year and holder values SHALL NOT be pinned by this spec; only the *presence* of a copyright line SHALL be normative

#### Scenario: Apache body is byte-equivalent to canonical

- **WHEN** `LICENSE-APACHE` is read
- **THEN** its body SHALL be byte-equivalent to the SPDX canonical Apache-2.0 license text
- **THEN** no project-specific text SHALL be inserted into the body of the Apache-2.0 license
- **THEN** the optional appendix copyright placeholder SHALL remain in the canonical template wording (it is not filled in for this project)
