<!--
SPDX-FileCopyrightText: TNG Technology Consulting GmbH <https://www.tngtech.com>

SPDX-License-Identifier: Apache-2.0
-->

# Migration Plan: opossum-file Python to Rust

## Overview

This document outlines the migration of `opossum-file` from Python to Rust, targeting performance, safety, and portability while maintaining full feature parity.

## Project Goals

- **Performance**: Leverage Rust's zero-cost abstractions and efficient memory management
- **Safety**: Compile-time guarantees through Rust's type system
- **Portability**: Single static binary with no runtime dependencies
- **Maintainability**: Clean architecture mirroring the Python design patterns

## Technology Stack

| Concern | Python | Rust |
|---------|--------|------|
| CLI Framework | click | clap (derive macros) |
| Serialization | pydantic | serde + serde_json |
| Error Handling | exceptions | anyhow + thiserror |
| Logging | logging | tracing + tracing-subscriber |
| Testing | pytest + faker | built-in test framework |
| Distribution | pyinstaller | cargo-dist |
| Package Manager | uv | cargo |
| PURL Parsing | packageurl-python | purl (custom crate) |
| DateTime | datetime | chrono |
| UUID | uuid | uuid |

## Project Structure

```
opossum-file.rs/
├── Cargo.toml                    # Workspace configuration
├── purl/                         # PURL parsing library (extractable later)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── parser.rs
│       └── types.rs
├── opossum-file/                 # Binary crate (CLI)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── opossum-lib/                  # Library crate (core logic)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── core/
│       │   ├── mod.rs
│       │   ├── entities/
│       │   │   ├── mod.rs
│       │   │   ├── opossum.rs          # Internal Opossum representation
│       │   │   ├── scan_results.rs     # ScanResults for aggregating data
│       │   │   ├── resource.rs         # Resource enum (File/Directory)
│       │   │   ├── root_resource.rs    # RootResource wrapper
│       │   │   ├── opossum_package.rs  # OpossumPackage struct
│       │   │   ├── metadata.rs
│       │   │   ├── source_info.rs
│       │   │   ├── config.rs
│       │   │   ├── external_attribution_source.rs
│       │   │   ├── frequent_license.rs
│       │   │   └── base_urls_for_sources.rs
│       │   └── services/
│       │       ├── mod.rs
│       │       ├── input_reader.rs
│       │       ├── generate.rs
│       │       ├── merge.rs
│       │       └── write_opossum_file.rs
│       ├── input_formats/
│       │   ├── mod.rs
│       │   ├── scancode/
│       │   │   ├── mod.rs
│       │   │   ├── entities.rs         # ScanCode models
│       │   │   └── reader.rs            # ScanCodeFileReader
│       │   ├── opossum/
│       │   │   ├── mod.rs
│       │   │   ├── entities.rs         # OpossumFileModel
│       │   │   └── reader.rs            # OpossumFileReader
│       │   └── owasp/
│       │       ├── mod.rs
│       │       ├── entities.rs         # OWASP models
│       │       └── reader.rs            # OwaspDependencyScanFileReader
│       └── shared/
│           ├── mod.rs
│           └── constants.rs
├── tests/                        # Integration tests
│   └── data/                     # Test fixtures (copy from Python)
└── .github/
    └── workflows/
        └── release.yml           # cargo-dist release workflow
```

## Dependencies

### Workspace Cargo.toml

```toml
[workspace]
members = ["purl", "opossum-lib", "opossum-file"]
resolver = "2"

[workspace.package]
edition = "2024"
license = "Apache-2.0"
repository = "https://github.com/opossum-tool/opossum-file.rs"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
chrono = "0.4"
uuid = { version = "1.0", features = ["v4", "serde"] }
purl = { path = "purl" }
```

### purl/Cargo.toml

```toml
[package]
name = "purl"
version = "0.1.0"
edition.workspace = true

[dependencies]
serde = { workspace = true }
thiserror = { workspace = true }
url = "2.5"

[dev-dependencies]
pretty_assertions = "1.4"
```

### opossum-lib/Cargo.toml

```toml
[package]
name = "opossum-lib"
version = "0.1.0"
edition.workspace = true

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
chrono = { workspace = true }
uuid = { workspace = true }
purl = { workspace = true }
zip = { version = "2.0", default-features = false, features = ["deflate"] }

[dev-dependencies]
pretty_assertions = "1.4"
```

### opossum-file/Cargo.toml

```toml
[package]
name = "opossum-file"
version = "0.1.0"
edition.workspace = true

[dependencies]
opossum-lib = { path = "../opossum-lib" }
clap = { version = "4.5", features = ["derive"] }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Migration Phases

### Phase 0: PURL Library (Week 1-2)

**Goals:**
- Create a PURL parsing library with full spec compliance
- Designed for future extraction to standalone crate

**Reference:** See [PURL.md](PURL.md) for detailed migration plan.

**Tasks:**
1. Create `purl` crate with basic structure
2. Implement `PackageURL` struct with all components
3. Implement `FromStr` for parsing PURL strings (e.g., `pkg:cargo/clap@4.5`)
4. Implement `Display` for PURL serialization
5. Implement type-specific normalization rules
6. Add validation and error handling
7. Write tests using official test-suite-data.json

**Validation:**
- All ~100+ official test cases pass
- Parse PURLs from OWASP test data
- Round-trip parsing matches original

### Phase 1: Project Setup & Core Types (Week 1-2)

**Goals:**
- Initialize workspace and crates
- Define core data structures with serde derives
- Implement error types

**Tasks:**
1. Create workspace `Cargo.toml` with three crates
2. Implement `OpossumInputFileModel` and related types in `opossum-lib/src/input_formats/opossum/entities.rs`
3. Implement `OpossumOutputFileModel` types
4. Implement `ResourceInFileModel` recursive type (see note below)
5. Create error types using thiserror in `opossum-lib/src/core/errors.rs`
6. Implement camelCase serialization helper (serde rename_all = "camelCase")

**Validation:**
- Deserialize `tests/data/opossum_input.json` successfully
- Round-trip serialization matches original

### Phase 2: Internal Representation (Week 2-3)

**Goals:**
- Implement the `Opossum` internal representation
- Implement conversion from on-disk to internal format

**Tasks:**
1. Implement `ScanResults` struct for aggregating attribution data
2. Implement `RootResource` and `Resource` enum (File/Directory) with children
3. Implement `OpossumPackage` struct with all fields
4. Implement `Metadata`, `SourceInfo`, `Config`, `ExternalAttributionSource`, `FrequentLicense`, `BaseUrlsForSources`
5. Create conversion from `OpossumInputFileModel` to `Opossum`
6. Handle the `resourcesToAttributions` join resolution
7. Build resource tree from flat path structure

**Validation:**
- Unit tests for resource tree construction
- Unit tests for attribution resolution
- Compare with Python test outputs

### Phase 3: ScanCode Reader (Week 3-4)

**Goals:**
- Implement ScanCode JSON parsing and conversion

**Tasks:**
1. Define `ScanCodeFileModel` entities based on Python `scancode_model.py`
2. Handle `extra="allow"` with `#[serde(flatten)]` + `HashMap<String, Value>`
3. Implement field aliases (`#[serde(alias = "...")]`)
4. Implement `ScanCodeFileReader` implementing `InputReader` trait
5. Implement conversion from ScanCode to `Opossum`
6. Port test fixtures from Python

**Validation:**
- Deserialize `tests/data/scancode_input.json`
- Output matches `tests/data/opossum_output.json`

### Phase 4: Opossum File Reader (Week 4)

**Goals:**
- Support reading existing .opossum files

**Tasks:**
1. Implement `.opossum` file format (ZIP with JSON inside)
2. Implement `OpossumFileReader`
3. Handle both input and output file models
4. Handle corrupt files gracefully

**Validation:**
- Round-trip test: read .opossum → write .opossum → compare
- Test with `tests/data/opossum_input_corrupt.opossum`

### Phase 5: OWASP Dependency Scan Reader (Week 4-5)

**Goals:**
- Support OWASP dependency scan JSON files

**Tasks:**
1. Define OWASP model entities (prioritize fields used in conversion)
2. Implement `OwaspDependencyScanFileReader`
3. Implement conversion to `Opossum` using PURL parsing
4. Handle CVSS score extraction

**Validation:**
- Deserialize `tests/data/dependency-check-report.json`
- Verify conversion output

### Phase 6: Merge & Write Operations (Week 5-6)

**Goals:**
- Implement merging multiple Opossum instances
- Write output .opossum files

**Tasks:**
1. Implement `merge_opossums` function
2. Implement `write_opossum_file` function (ZIP creation)
3. Handle conflicts and duplicates
4. Generate UUIDs for attribution IDs

**Validation:**
- Merge multiple test inputs
- Verify output file structure

### Phase 7: CLI Implementation (Week 6)

**Goals:**
- Implement the command-line interface

**Tasks:**
1. Define clap command structure with derive macros
2. Implement `generate` subcommand with multiple input options
3. Handle `multiple=True` for repeating arguments
4. Add logging initialization with tracing-subscriber
5. Error reporting and exit codes
6. Warning when no inputs provided

**Validation:**
- CLI help output matches Python version
- Integration tests for all CLI options

### Phase 8: Testing & Documentation (Week 6-7)

**Goals:**
- Comprehensive test coverage
- Documentation

**Tasks:**
1. Port all Python test fixtures
2. Write unit tests for each module
3. Write integration tests for CLI
4. Add rustdoc comments
5. Create usage examples
6. Test merge logic thoroughly

**Validation:**
- `cargo test` passes
- `cargo doc` generates clean documentation

### Phase 9: Distribution Setup (Week 7)

**Goals:**
- Set up cargo-dist for releases

**Tasks:**
1. Install and configure cargo-dist
2. Create GitHub Actions workflow for releases
3. Test release build locally
4. Configure target platforms (Linux x86_64, macOS x86_64/ARM, Windows)

**Validation:**
- `cargo dist build` succeeds
- Generated binaries run on target platforms

### Phase 10: Final Migration & Cleanup (Week 7-8)

**Goals:**
- Finalize migration

**Tasks:**
1. Performance benchmarks vs Python version
2. Memory usage profiling
3. Update any remaining Python-specific patterns
4. Final documentation review
5. Create release PR

## Key Implementation Details

### InputReader Trait

```rust
pub trait InputReader {
    fn read(&self) -> Result<Opossum>;
}
```

### CamelCase Serialization

Use serde's `rename_all = "camelCase"`:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    project_id: String,
    file_creation_date: String,
    project_title: String,
}
```

### Recursive ResourceInFileModel

Python's `ResourceInFileModel = dict[str, ResourceInFileModel] | int` requires careful Rust modeling:

```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceInFile {
    Directory(BTreeMap<String, ResourceInFile>),
    File(i32),  // file count
}
```

### Field Aliases

Handle Python's field aliases:

```rust
#[derive(Deserialize)]
struct License {
    #[serde(alias = "spdx_license_expression")]
    license_expression_spdx: Option<String>,
}
```

### Extra Fields Handling

For Python's `extra="allow"`:

```rust
#[derive(Deserialize)]
struct ScanCodeFile {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
```

### BaseUrlsForSources Custom Serialization

This model serializes keys with `None` values (non-standard):

```rust
#[derive(Serialize)]
struct BaseUrlsForSources(HashMap<String, Option<String>>);

impl Serialize for BaseUrlsForSources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.0.serialize(serializer)
    }
}
```

### Opossum File Format (ZIP)

The `.opossum` format is a **ZIP file** (not tar.gz) containing:
- `input.json`: OpossumInputFileModel
- `output.json`: OpossumOutputFileModel (optional)

Use the `zip` crate for handling this format.

### Error Handling Strategy

- Use `thiserror` for library errors (typed, descriptive)
- Use `anyhow` in the binary for error reporting
- Chain errors with `.context()` for meaningful messages

### Testing Strategy

- Unit tests in each module with `#[cfg(test)]`
- Integration tests in `tests/` directory
- Property-based testing for serialization roundtrips (optional, using proptest)
- Test fixtures copied from Python project

### Test Files to Port

From Python `tests/`:
- `test_cli.py`
- `core/entities/test_opossum.py`
- `core/services/test_merge_opossums.py` (complex merge logic)
- `input_formats/scancode/services/test_convert_to_opossum.py`
- `input_formats/opossum/services/test_conversion_roundtrip.py`
- `input_formats/opossum/services/test_opossum_file_reader.py`
- `input_formats/owasp_dependency_scan/services/test_convert_to_opossum.py`

## Success Criteria

1. **Functional Parity**: All Python CLI features work identically
2. **Test Coverage**: Same test cases pass as in Python
3. **Performance**: At least 2x faster than Python version on large files
4. **Binary Size**: < 10MB stripped release binary
5. **Startup Time**: < 10ms for `--help` output
6. **Cross-Platform**: Binaries for Linux, macOS (x86_64 + ARM), Windows

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Complex serde models | Start with minimal models, expand incrementally |
| .opossum format edge cases | Test with real files from Python project |
| Performance regression | Benchmark early and often |
| Missing features | Track feature parity with checklist |
| PURL parsing complexity | Keep purl crate minimal, expand as needed |

## References

- Original Python repository: `reference/opossum-file/`
- PURL library migration: `PURL.md`
- PURL specification: https://github.com/package-url/purl-spec
- OpossumUI: https://github.com/opossum-tool/OpossumUI
- cargo-dist book: https://opensource.axo.dev/cargo-dist/
