use std::collections::BTreeMap;
use std::str::FromStr;

use crate::encode::decode;
use crate::error::PurlError;
use crate::normalize::{
    normalize_name, normalize_namespace, normalize_qualifiers, normalize_subpath, normalize_version,
};
use crate::PackageURL;

const SCHEME: &str = "pkg";

fn is_valid_type_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_'
}

fn is_valid_qualifier_key_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_'
}

fn validate_qualifier_key(key: &str) -> Result<(), PurlError> {
    if key.is_empty() {
        return Err(PurlError::InvalidQualifierKey(key.to_string()));
    }
    let mut chars = key.chars();
    let first = chars.next().unwrap();
    if first.is_ascii_digit() {
        return Err(PurlError::InvalidQualifierKey(key.to_string()));
    }
    if !key.chars().all(is_valid_qualifier_key_char) {
        return Err(PurlError::InvalidQualifierKey(key.to_string()));
    }
    Ok(())
}

fn parse_qualifiers(s: &str) -> Result<BTreeMap<String, String>, PurlError> {
    let mut map = BTreeMap::new();
    if s.is_empty() {
        return Ok(map);
    }
    for pair in s.split('&') {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = decode(key);
        let key_lower = key.to_lowercase();
        validate_qualifier_key(&key_lower)?;
        let value = decode(value);
        map.insert(key_lower, value);
    }
    Ok(map)
}

impl FromStr for PackageURL {
    type Err = PurlError;

    fn from_str(purl: &str) -> Result<Self, Self::Err> {
        let (scheme, remainder) = purl.split_once(':').ok_or(PurlError::MissingScheme)?;
        if scheme != SCHEME {
            return Err(PurlError::MissingScheme);
        }

        let remainder = remainder.trim_start_matches('/');

        let (type_str, remainder) = remainder.split_once('/').ok_or(PurlError::MissingType)?;
        if type_str.is_empty() {
            return Err(PurlError::MissingType);
        }
        if !type_str.chars().all(is_valid_type_char) {
            return Err(PurlError::InvalidType(type_str.to_string()));
        }
        let type_str = type_str.to_lowercase();

        let (path_rest, qualifiers_str, subpath_str) = split_path_qualifiers_subpath(remainder);

        let (namespace, name, version) = parse_namespace_name_version(&type_str, path_rest)?;

        let qualifiers = parse_qualifiers(qualifiers_str)?;
        let subpath = if subpath_str.is_empty() {
            None
        } else {
            normalize_subpath(Some(&decode(subpath_str)))
        };

        let namespace = normalize_namespace(namespace.as_deref(), &type_str);
        let name = normalize_name(&name, &type_str);
        let version = normalize_version(version.as_deref(), &type_str);
        let qualifiers = normalize_qualifiers(&qualifiers);

        Ok(PackageURL {
            r#type: type_str,
            namespace,
            name,
            version,
            qualifiers,
            subpath,
        })
    }
}

fn split_path_qualifiers_subpath(s: &str) -> (&str, &str, &str) {
    let (after_hash, subpath) = match s.split_once('#') {
        Some((before, after)) => (before, after),
        None => (s, ""),
    };
    let (path, qualifiers) = match after_hash.split_once('?') {
        Some((before, after)) => (before, after),
        None => (after_hash, ""),
    };
    (path, qualifiers, subpath)
}

fn parse_namespace_name_version(
    ptype: &str,
    path: &str,
) -> Result<(Option<String>, String, Option<String>), PurlError> {
    let path = path.trim_end_matches('/');

    if ptype == "npm" && path.starts_with('@') {
        return parse_npm_path(path);
    }

    let (path_before_at, version) = match path.rsplit_once('@') {
        Some((before, after)) => (before, Some(decode(after))),
        None => (path, None),
    };

    let parts: Vec<&str> = path_before_at
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return Err(PurlError::MissingName);
    }

    let name = decode(parts.last().unwrap());
    let namespace = if parts.len() > 1 {
        let ns: Vec<String> = parts[..parts.len() - 1].iter().map(|s| decode(s)).collect();
        Some(ns.join("/"))
    } else {
        None
    };

    Ok((namespace, name, version))
}

fn parse_npm_path(path: &str) -> Result<(Option<String>, String, Option<String>), PurlError> {
    let (path_before_at, version) = match path.rsplit_once('@') {
        Some((before, after)) => {
            if before.contains('/') {
                (before, Some(decode(after)))
            } else {
                (path, None)
            }
        }
        None => (path, None),
    };

    let (namespace, name) = match path_before_at.split_once('/') {
        Some((ns, n)) => {
            let ns = decode(ns);
            let n = decode(n);
            if n.is_empty() {
                return Err(PurlError::MissingName);
            }
            (Some(ns), n)
        }
        None => {
            let name = decode(path_before_at);
            if name.is_empty() {
                return Err(PurlError::MissingName);
            }
            (None, name)
        }
    };

    Ok((namespace, name, version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_clap() {
        let purl: PackageURL = "pkg:cargo/clap@4.5.0".parse().unwrap();
        assert_eq!(purl.r#type, "cargo");
        assert_eq!(purl.namespace, None);
        assert_eq!(purl.name, "clap");
        assert_eq!(purl.version, Some("4.5.0".to_string()));
        assert!(purl.qualifiers.is_empty());
        assert_eq!(purl.subpath, None);
    }

    #[test]
    fn test_parse_maven_apache_commons() {
        let purl: PackageURL = "pkg:maven/org.apache.commons/io@1.3.4".parse().unwrap();
        assert_eq!(purl.r#type, "maven");
        assert_eq!(purl.namespace, Some("org.apache.commons".to_string()));
        assert_eq!(purl.name, "io");
        assert_eq!(purl.version, Some("1.3.4".to_string()));
    }

    #[test]
    fn test_parse_npm_encoded_at() {
        let purl: PackageURL = "pkg:npm/%40angular/core@18.0.0".parse().unwrap();
        assert_eq!(purl.r#type, "npm");
        assert_eq!(purl.namespace, Some("@angular".to_string()));
        assert_eq!(purl.name, "core");
        assert_eq!(purl.version, Some("18.0.0".to_string()));
    }

    #[test]
    fn test_parse_github_with_subpath() {
        let purl: PackageURL = "pkg:github/package-url/purl-spec@244fd47e07d1004#spec/README.md"
            .parse()
            .unwrap();
        assert_eq!(purl.r#type, "github");
        assert_eq!(purl.namespace, Some("package-url".to_string()));
        assert_eq!(purl.name, "purl-spec");
        assert_eq!(purl.version, Some("244fd47e07d1004".to_string()));
        assert_eq!(purl.subpath, Some("spec/README.md".to_string()));
    }

    #[test]
    fn test_parse_missing_scheme() {
        let result: Result<PackageURL, _> = "cargo/clap@4.5.0".parse();
        assert!(matches!(result, Err(PurlError::MissingScheme)));
    }

    #[test]
    fn test_parse_invalid_scheme() {
        let result: Result<PackageURL, _> = "npm:lodash@4.17.21".parse();
        assert!(matches!(result, Err(PurlError::MissingScheme)));
    }

    #[test]
    fn test_parse_missing_type() {
        let result: Result<PackageURL, _> = "pkg:/lodash".parse();
        assert!(matches!(result, Err(PurlError::MissingType)));
    }

    #[test]
    fn test_parse_missing_name() {
        let result: Result<PackageURL, _> = "pkg:cargo/".parse();
        assert!(matches!(result, Err(PurlError::MissingName)));
    }

    #[test]
    fn test_parse_invalid_type_chars() {
        let result: Result<PackageURL, _> = "pkg:car go/serde".parse();
        assert!(matches!(result, Err(PurlError::InvalidType(_))));
    }

    #[test]
    fn test_parse_with_qualifiers() {
        let purl: PackageURL = "pkg:cargo/serde?vcs_url=https://github.com"
            .parse()
            .unwrap();
        assert_eq!(
            purl.qualifiers.get("vcs_url").unwrap(),
            "https://github.com"
        );
    }

    #[test]
    fn test_parse_multiple_qualifiers() {
        let purl: PackageURL = "pkg:cargo/serde?checksum=abc123&vcs_url=https://github.com"
            .parse()
            .unwrap();
        assert_eq!(purl.qualifiers.get("checksum").unwrap(), "abc123");
        assert_eq!(
            purl.qualifiers.get("vcs_url").unwrap(),
            "https://github.com"
        );
    }

    #[test]
    fn test_parse_full_purl() {
        let purl: PackageURL =
            "pkg:maven/org.apache.commons/commons-io@1.4?checksum=abc123#some/path"
                .parse()
                .unwrap();
        assert_eq!(purl.r#type, "maven");
        assert_eq!(purl.namespace, Some("org.apache.commons".to_string()));
        assert_eq!(purl.name, "commons-io");
        assert_eq!(purl.version, Some("1.4".to_string()));
        assert_eq!(purl.qualifiers.get("checksum").unwrap(), "abc123");
        assert_eq!(purl.subpath, Some("some/path".to_string()));
    }

    #[test]
    fn test_roundtrip_cargo_clap() {
        let purl_str = "pkg:cargo/clap@4.5.0";
        let purl: PackageURL = purl_str.parse().unwrap();
        assert_eq!(purl.to_string(), purl_str);
    }

    #[test]
    fn test_roundtrip_maven() {
        let purl_str = "pkg:maven/org.apache.commons/io@1.3.4";
        let purl: PackageURL = purl_str.parse().unwrap();
        assert_eq!(purl.to_string(), purl_str);
    }

    #[test]
    fn test_type_normalized_to_lowercase() {
        let purl: PackageURL = "pkg:CARGO/serde".parse().unwrap();
        assert_eq!(purl.r#type, "cargo");
        assert_eq!(purl.to_string(), "pkg:cargo/serde");
    }
}
