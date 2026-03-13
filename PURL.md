<!--
SPDX-FileCopyrightText: TNG Technology Consulting GmbH <https://www.tngtech.com>

SPDX-License-Identifier: Apache-2.0
-->

# Migration Plan: packageurl-python to Rust

## Overview

This document outlines the migration of `packageurl-python` to Rust. This library parses and builds Package URLs (purl) as specified at https://github.com/package-url/purl-spec.

## Project Goals

- **Correctness**: Full spec compliance with official test suite
- **Performance**: Fast parsing with minimal allocations
- **Ergonomics**: Clean Rust API with proper error handling
- **Extractability**: Designed as standalone crate, separate from opossum-file

## Library Analysis

### Core Functionality

| Feature | Description | Complexity |
|---------|-------------|------------|
| `PackageURL` struct | Holds purl components | Low |
| `from_string()` | Parse purl string to struct | Medium |
| `to_string()` | Serialize struct to purl string | Low |
| Normalization | Type-specific rules for name/namespace | Medium |
| Validation | Check components against type-specific rules | Medium |

### PURL Components

```
pkg:type/namespace/name@version?qualifiers#subpath
```

| Component | Required | Description |
|-----------|----------|-------------|
| type | Yes | Package type (e.g., maven, npm, cargo) |
| namespace | No | Package namespace (e.g., org.apache.commons) |
| name | Yes | Package name |
| version | No | Package version |
| qualifiers | No | Key-value pairs (e.g., arch=i386) |
| subpath | No | Subpath within package |

### Type-Specific Normalization

Different package types have different normalization rules:

| Type | Name Casing | Namespace Casing | Special Rules |
|------|-------------|------------------|---------------|
| maven | Preserve | Preserve | - |
| github | Lowercase | Lowercase | - |
| pypi | Lowercase | Lowercase | `_` → `-` in name |
| npm | Lowercase | Lowercase | Namespace starts with `@` |
| golang | Preserve | Preserve | - |
| cargo | Preserve | Preserve | - |

### Dependencies

| Python | Rust |
|--------|------|
| urllib.parse (quote/unquote) | percent-encoding crate |
| No other deps | - |

## Technology Stack

| Concern | Python | Rust |
|---------|--------|------|
| Parsing | Manual string parsing | Manual or nom |
| Percent encoding | urllib.parse | percent-encoding |
| Error Handling | ValueError | thiserror |
| Data Structure | namedtuple | struct |

## Project Structure

```
purl/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── normalize.rs           # Type-specific normalization
│   ├── parse.rs               # Parsing logic
│   └── package_url.rs         # PackageURL struct
├── tests/
│   ├── test_packageurl.rs     # Unit tests
│   └── data/
│       └── test-suite-data.json  # Official test suite
└── LICENSE
```

## Dependencies

### Cargo.toml

```toml
[package]
name = "purl"
version = "0.1.0"
edition = "2024"
license = "MIT"
description = "Parse and build Package URLs (purl)"

[dependencies]
thiserror = "2.0"
percent-encoding = "2.3"

[dev-dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pretty_assertions = "1.4"
```

## Migration Phases

### Phase 1: Core Types & Errors (Week 1, Days 1-2)

**Goals:**
- Define core data structures
- Implement error types

**Tasks:**
1. Create `PurlError` enum with thiserror
2. Define `PackageURL` struct:
   ```rust
   pub struct PackageURL {
       pub r#type: String,
       pub namespace: Option<String>,
       pub name: String,
       pub version: Option<String>,
       pub qualifiers: BTreeMap<String, String>,
       pub subpath: Option<String>,
   }
   ```
3. Implement `Display` for `PackageURL` (serialization)
4. Implement `PartialEq`, `Eq`, `Hash`

**Validation:**
- Struct compiles
- Display produces correct output for simple cases

### Phase 2: Serialization (Week 1, Days 3-4)

**Goals:**
- Implement `to_string()` with proper encoding

**Tasks:**
1. Implement percent-encoding helper (preserve `:` in values)
2. Implement type-specific normalization functions
3. Implement qualifier serialization
4. Build purl string from components

**Validation:**
- Unit tests for each component serialization
- Compare output with Python version

### Phase 3: Parsing (Week 1, Days 5-7 + Week 2, Days 1-2)

**Goals:**
- Implement `from_string()` parser

**Tasks:**
1. Parse scheme (`pkg:`)
2. Parse type (must be alphanumeric + `._-`)
3. Handle namespace/name separation
4. Handle `@version` parsing
5. Handle `?qualifiers` parsing
6. Handle `#subpath` parsing
7. Special handling for npm `@namespace`
8. Percent-decode components

**Validation:**
- Parse all valid test cases from test-suite-data.json
- Reject all invalid test cases

### Phase 4: Normalization (Week 2, Days 3-4)

**Goals:**
- Implement type-specific normalization rules

**Tasks:**
1. Implement `normalize_type()` - lowercase
2. Implement `normalize_namespace()` - type-specific casing, segment splitting
3. Implement `normalize_name()` - type-specific casing, special chars
4. Implement `normalize_version()` - percent encoding
5. Implement `normalize_qualifiers()` - key validation, sorting
6. Implement `normalize_subpath()` - segment handling

**Validation:**
- All normalization test cases pass
- Canonical purl matches expected output

### Phase 5: Test Suite Integration (Week 2, Days 5-7)

**Goals:**
- Full compliance with official test suite

**Tasks:**
1. Port `test-suite-data.json` test file
2. Create test harness to load and run JSON tests
3. Fix any discrepancies found
4. Add edge case tests

**Validation:**
- All 100+ test cases pass
- Invalid purls are properly rejected

## Key Implementation Details

### PackageURL Struct

```rust
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageURL {
    pub r#type: String,
    pub namespace: Option<String>,
    pub name: String,
    pub version: Option<String>,
    pub qualifiers: BTreeMap<String, String>,
    pub subpath: Option<String>,
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum PurlError {
    #[error("Missing required scheme 'pkg:'")]
    MissingScheme,
    
    #[error("Missing required type component")]
    MissingType,
    
    #[error("Missing required name component")]
    MissingName,
    
    #[error("Invalid type: {0}")]
    InvalidType(String),
    
    #[error("Invalid qualifier key: {0}")]
    InvalidQualifierKey(String),
    
    #[error("Invalid percent encoding: {0}")]
    InvalidEncoding(String),
}
```

### Parsing Approach

Manual parsing (no nom) for clarity:

```rust
impl PackageURL {
    pub fn from_string(purl: &str) -> Result<Self, PurlError> {
        // 1. Check scheme
        let remainder = purl.strip_prefix("pkg:")
            .ok_or(PurlError::MissingScheme)?;
        
        // 2. Parse type
        let (type_str, remainder) = split_type(remainder)?;
        
        // 3. Use urlsplit-like logic for rest
        // 4. Handle @version, ?qualifiers, #subpath
        // 5. Normalize components
        // 6. Return struct
    }
}
```

### Normalization Rules

```rust
pub fn normalize_namespace(namespace: &str, ptype: &str) -> Option<String> {
    let ns = namespace.trim().trim_matches('/');
    
    // Type-specific lowercase
    let ns = match ptype {
        "github" | "pypi" | "npm" | "gitlab" | "bitbucket" 
        | "composer" | "luarocks" | "qpkg" | "alpm" | "apk" | "hex" => ns.to_lowercase(),
        _ => ns.to_string(),
    };
    
    // Split and encode segments
    let segments: Vec<&str> = ns.split('/').filter(|s| !s.is_empty()).collect();
    if segments.is_empty() { None } else { Some(segments.join("/")) }
}

pub fn normalize_name(name: &str, ptype: &str) -> Option<String> {
    let n = name.trim().trim_matches('/');
    
    let n = match ptype {
        "pypi" => n.replace('_', "-").to_lowercase(),
        "github" | "npm" | "gitlab" | "bitbucket" 
        | "composer" | "luarocks" | "oci" | "alpm" | "apk" | "bitnami" | "hex" | "pub" => n.to_lowercase(),
        _ => n.to_string(),
    };
    
    if n.is_empty() { None } else { Some(n) }
}
```

### Qualifier Handling

```rust
pub fn parse_qualifiers(s: &str) -> Result<BTreeMap<String, String>, PurlError> {
    let mut map = BTreeMap::new();
    
    if s.is_empty() {
        return Ok(map);
    }
    
    for pair in s.split('&') {
        let (key, value) = pair.split_once('=')
            .ok_or_else(|| PurlError::InvalidQualifierKey(pair.to_string()))?;
        
        validate_qualifier_key(key)?;
        map.insert(key.to_lowercase(), percent_decode(value)?);
    }
    
    Ok(map)
}

fn validate_qualifier_key(key: &str) -> Result<(), PurlError> {
    if key.is_empty() {
        return Err(PurlError::InvalidQualifierKey("empty key".to_string()));
    }
    if key.starts_with(|c: char| c.is_ascii_digit()) {
        return Err(PurlError::InvalidQualifierKey(format!("key starts with digit: {}", key)));
    }
    if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') {
        return Err(PurlError::InvalidQualifierKey(format!("invalid character in key: {}", key)));
    }
    Ok(())
}
```

## Test Coverage

Port tests from Python test suite:

| Test File | Test Count | Focus |
|-----------|------------|-------|
| `test-suite-data.json` | ~100+ | Official spec compliance |
| `test_packageurl.py` | ~50 | Additional edge cases |

### Test JSON Format

```json
{
  "description": "valid maven purl",
  "purl": "pkg:maven/org.apache.commons/io@1.3.4",
  "canonical_purl": "pkg:maven/org.apache.commons/io@1.3.4",
  "type": "maven",
  "namespace": "org.apache.commons",
  "name": "io",
  "version": "1.3.4",
  "qualifiers": null,
  "subpath": null,
  "is_invalid": false
}
```

## Success Criteria

1. **Spec Compliance**: All official test-suite-data.json tests pass
2. **Round-trip**: Parse → Serialize → Parse yields identical result
3. **Error Messages**: Clear, actionable error messages
4. **Performance**: Parse 10,000 purls in < 10ms
5. **No Panics**: All errors returned as `Result`, no panics

## Timeline

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1: Core Types | 2 days | 2 days |
| Phase 2: Serialization | 2 days | 4 days |
| Phase 3: Parsing | 4 days | 8 days |
| Phase 4: Normalization | 2 days | 10 days |
| Phase 5: Test Suite | 3 days | 13 days |

**Total: ~2.5 weeks**

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Complex parsing edge cases | Reference Python implementation |
| Percent encoding nuances | Use well-tested percent-encoding crate |
| Type-specific rules | Document rules per type in code comments |

## References

- Original Python repository: `reference/packageurl-python/`
- PURL specification: https://github.com/package-url/purl-spec
- Test suite data: `tests/data/test-suite-data.json`
