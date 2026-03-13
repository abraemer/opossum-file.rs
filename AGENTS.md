# AGENTS.md

## Project Goal

Migrate `opossum-file` (Python CLI) to Rust. The tool converts ScanCode JSON, Opossum files, and OWASP Dependency Scan JSON into `.opossum` format for OpossumUI.

## Repository Structure

```
opossum-file.rs/
├── MIGRATION.md          # Main migration plan (opossum-file)
├── PURL.md               # PURL library migration plan
├── reference/
│   ├── opossum-file/     # Python source to migrate from
│   └── packageurl-python/ # PURL library to migrate from
├── purl/                  # PURL crate (to be created)
├── opossum-lib/           # Core library (to be created)
└── opossum-file/          # CLI binary (to be created)
```

## Migration Order

1. **Phase 0**: Create `purl/` crate (see PURL.md) - PURL parsing
2. **Phase 1-10**: Create `opossum-lib/` and `opossum-file/` (see MIGRATION.md)

## Getting Started

1. Read `PURL.md` for PURL library migration
2. Read `MIGRATION.md` for main project migration
3. Reference `reference/opossum-file/src/opossum_lib/` for Python implementation
4. Reference `reference/packageurl-python/src/packageurl/` for PURL implementation

## Tech Stack

- **CLI**: clap
- **Serialization**: serde + serde_json
- **Errors**: thiserror (lib) + anyhow (binary)
- **Logging**: tracing
- **File format**: zip crate (`.opossum` = ZIP with JSON inside)
- **Distribution**: cargo-dist

## Commands

```bash
# Build
cargo build

# Test
cargo test

# Run CLI
cargo run -- generate --scan-code-json input.json -o output.opossum
```

## Conventions

- Follow Rust 2024 edition idioms
- Use camelCase serialization (`#[serde(rename_all = "camelCase")]`)
- No comments in code unless requested
- Keep implementations minimal and correct
