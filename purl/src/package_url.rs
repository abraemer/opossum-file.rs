use std::collections::BTreeMap;
use std::fmt;

use crate::encode::{encode, encode_qualifier_value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageURL {
    pub r#type: String,
    pub namespace: Option<String>,
    pub name: String,
    pub version: Option<String>,
    pub qualifiers: BTreeMap<String, String>,
    pub subpath: Option<String>,
}

impl PackageURL {
    pub fn new(r#type: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            namespace: None,
            name: name.into(),
            version: None,
            qualifiers: BTreeMap::new(),
            subpath: None,
        }
    }

    pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn qualifier(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.qualifiers.insert(key.into(), value.into());
        self
    }

    pub fn subpath(mut self, subpath: impl Into<String>) -> Self {
        self.subpath = Some(subpath.into());
        self
    }
}

impl fmt::Display for PackageURL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pkg:")?;
        write!(f, "{}/", self.r#type)?;

        if let Some(ref namespace) = self.namespace {
            let encoded_segments: Vec<String> = namespace.split('/').map(encode).collect();
            write!(f, "{}/", encoded_segments.join("/"))?;
        }

        write!(f, "{}", encode(&self.name))?;

        if let Some(ref version) = self.version {
            write!(f, "@{}", encode(version))?;
        }

        if !self.qualifiers.is_empty() {
            write!(f, "?")?;
            let qualifiers: Vec<String> = self
                .qualifiers
                .iter()
                .map(|(k, v)| format!("{}={}", encode(k), encode_qualifier_value(v)))
                .collect();
            write!(f, "{}", qualifiers.join("&"))?;
        }

        if let Some(ref subpath) = self.subpath {
            let encoded_segments: Vec<String> = subpath
                .split('/')
                .filter(|s| !s.is_empty())
                .map(encode)
                .collect();
            if !encoded_segments.is_empty() {
                write!(f, "#{}", encoded_segments.join("/"))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_basic() {
        let purl = PackageURL::new("cargo", "serde");
        assert_eq!(purl.to_string(), "pkg:cargo/serde");
    }

    #[test]
    fn test_display_with_version() {
        let purl = PackageURL::new("cargo", "serde").version("1.0.0");
        assert_eq!(purl.to_string(), "pkg:cargo/serde@1.0.0");
    }

    #[test]
    fn test_display_with_namespace() {
        let purl = PackageURL::new("maven", "commons-io").namespace("org.apache.commons");
        assert_eq!(purl.to_string(), "pkg:maven/org.apache.commons/commons-io");
    }

    #[test]
    fn test_display_with_qualifiers() {
        let mut qualifiers = BTreeMap::new();
        qualifiers.insert(
            "vcs_url".to_string(),
            "https://github.com/serde-rs/serde".to_string(),
        );
        let purl = PackageURL {
            r#type: "cargo".to_string(),
            namespace: None,
            name: "serde".to_string(),
            version: None,
            qualifiers,
            subpath: None,
        };
        assert_eq!(
            purl.to_string(),
            "pkg:cargo/serde?vcs_url=https://github.com/serde-rs/serde"
        );
    }

    #[test]
    fn test_display_with_subpath() {
        let purl = PackageURL::new("cargo", "serde").subpath("src/lib.rs");
        assert_eq!(purl.to_string(), "pkg:cargo/serde#src/lib.rs");
    }

    #[test]
    fn test_display_full() {
        let mut qualifiers = BTreeMap::new();
        qualifiers.insert("checksum".to_string(), "abc123".to_string());
        let purl = PackageURL {
            r#type: "maven".to_string(),
            namespace: Some("org.apache.commons".to_string()),
            name: "commons-io".to_string(),
            version: Some("1.4".to_string()),
            qualifiers,
            subpath: Some("some/path".to_string()),
        };
        assert_eq!(
            purl.to_string(),
            "pkg:maven/org.apache.commons/commons-io@1.4?checksum=abc123#some/path"
        );
    }

    #[test]
    fn test_display_encodes_special_chars() {
        let purl = PackageURL::new("generic", "my@package");
        assert_eq!(purl.to_string(), "pkg:generic/my%40package");
    }

    #[test]
    fn test_display_preserves_colon_in_qualifier_value() {
        let mut qualifiers = BTreeMap::new();
        qualifiers.insert("url".to_string(), "https://example.com".to_string());
        let purl = PackageURL {
            r#type: "generic".to_string(),
            namespace: None,
            name: "pkg".to_string(),
            version: None,
            qualifiers,
            subpath: None,
        };
        assert_eq!(purl.to_string(), "pkg:generic/pkg?url=https://example.com");
    }

    #[test]
    fn test_display_cargo_clap() {
        let purl = PackageURL::new("cargo", "clap").version("4.5.0");
        assert_eq!(purl.to_string(), "pkg:cargo/clap@4.5.0");
    }

    #[test]
    fn test_display_maven_apache_commons_io() {
        let purl = PackageURL::new("maven", "io")
            .namespace("org.apache.commons")
            .version("1.3.4");
        assert_eq!(purl.to_string(), "pkg:maven/org.apache.commons/io@1.3.4");
    }

    #[test]
    fn test_display_with_qualifiers_and_subpath() {
        let purl = PackageURL::new("npm", "lodash")
            .version("4.17.21")
            .qualifier("checksum", "sha256:abc123")
            .subpath("fp/get");
        assert_eq!(
            purl.to_string(),
            "pkg:npm/lodash@4.17.21?checksum=sha256:abc123#fp/get"
        );
    }
}
