# opossum-file

A Rust CLI tool for converting software scan results to the Opossum format for use with [OpossumUI](https://github.com/opossum-tool/OpossumUI).

## Overview

This tool converts various input formats into `.opossum` files:

- **ScanCode JSON** - Output from [ScanCode Toolkit](https://github.com/nexB/scancode-toolkit)
- **OWASP Dependency Check JSON** - Output from [OWASP Dependency-Check](https://owasp.org/www-project-dependency-check/)
- **Opossum files** - Existing `.opossum` files for merging

## Installation

### From Source

```bash
cargo install --path opossum-file
```

### From Release

Download the latest release from the [releases page](https://github.com/opossum-tool/opossum-file.rs/releases).

## Usage

### Generate Opossum File

Convert scan results to an Opossum file:

```bash
opossum-file generate \
  --scan-code-json scan-results.json \
  --output output.opossum
```

Multiple input files can be combined:

```bash
opossum-file generate \
  --scan-code-json scan1.json \
  --scan-code-json scan2.json \
  --dependency-check-json owasp-report.json \
  --opossum-file existing.opossum \
  --output merged.opossum
```

Optional metadata:

```bash
opossum-file generate \
  --scan-code-json scan.json \
  --output output.opossum \
  --project-title "My Project" \
  --project-id "my-project-001"
```

### Merge Opossum Files

Merge multiple `.opossum` files:

```bash
opossum-file merge file1.opossum file2.opossum -o merged.opossum
```

### Help

```bash
opossum-file --help
opossum-file generate --help
opossum-file merge --help
```

## Output Format

The `.opossum` file is a ZIP archive containing:

- `input.json` - Attribution data for OpossumUI
- `output.json` - Review results (if present)

## Building

```bash
# Build
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings
```

## Project Structure

```
opossum-file.rs/
├── purl/              # PURL parsing library
├── opossum-lib/       # Core library
└── opossum-file/      # CLI binary
```

## License

Apache-2.0
