use crate::PackageURL;

fn should_lowercase_namespace(ptype: &str) -> bool {
    matches!(
        ptype,
        "bitbucket"
            | "github"
            | "pypi"
            | "gitlab"
            | "composer"
            | "luarocks"
            | "qpkg"
            | "alpm"
            | "apk"
            | "hex"
    )
}

fn should_lowercase_name(ptype: &str) -> bool {
    matches!(
        ptype,
        "bitbucket"
            | "github"
            | "pypi"
            | "gitlab"
            | "composer"
            | "luarocks"
            | "oci"
            | "npm"
            | "alpm"
            | "apk"
            | "bitnami"
            | "hex"
            | "pub"
    )
}

pub fn normalize_type(ptype: &str) -> String {
    ptype.trim().to_lowercase()
}

pub fn normalize_namespace(namespace: Option<&str>, ptype: &str) -> Option<String> {
    let ns = namespace?.trim();
    if ns.is_empty() {
        return None;
    }

    let ns = ns.trim_start_matches('/');

    let ns = if should_lowercase_namespace(ptype) {
        ns.to_lowercase()
    } else {
        ns.to_string()
    };

    let segments: Vec<&str> = ns.split('/').filter(|s| !s.is_empty()).collect();
    if segments.is_empty() {
        None
    } else {
        Some(segments.join("/"))
    }
}

pub fn normalize_name(name: &str, ptype: &str) -> String {
    let n = name.trim().trim_matches('/');
    if n.is_empty() {
        return String::new();
    }

    if ptype == "pypi" {
        n.replace('_', "-").to_lowercase()
    } else if should_lowercase_name(ptype) {
        n.to_lowercase()
    } else {
        n.to_string()
    }
}

pub fn normalize_version(version: Option<&str>, ptype: &str) -> Option<String> {
    let v = version?.trim();
    if v.is_empty() {
        return None;
    }

    let v = if ptype == "huggingface" || ptype == "oci" {
        v.to_lowercase()
    } else {
        v.to_string()
    };

    Some(v)
}

pub fn normalize_qualifiers(
    qualifiers: &std::collections::BTreeMap<String, String>,
) -> std::collections::BTreeMap<String, String> {
    let mut result = std::collections::BTreeMap::new();
    for (key, value) in qualifiers {
        let normalized_key = key.to_lowercase();
        result.insert(normalized_key, value.clone());
    }
    result
}

pub fn normalize_subpath(subpath: Option<&str>) -> Option<String> {
    let sp = subpath?.trim().trim_matches('/');
    if sp.is_empty() {
        return None;
    }

    let segments: Vec<&str> = sp
        .split('/')
        .filter(|s| {
            let trimmed = s.trim();
            !trimmed.is_empty() && trimmed != "." && trimmed != ".."
        })
        .collect();

    if segments.is_empty() {
        None
    } else {
        Some(segments.join("/"))
    }
}

impl PackageURL {
    pub fn normalize(&self) -> Self {
        let ptype = normalize_type(&self.r#type);
        let namespace = normalize_namespace(self.namespace.as_deref(), &ptype);
        let name = normalize_name(&self.name, &ptype);
        let version = normalize_version(self.version.as_deref(), &ptype);
        let qualifiers = normalize_qualifiers(&self.qualifiers);
        let subpath = normalize_subpath(self.subpath.as_deref());

        Self {
            r#type: ptype,
            namespace,
            name,
            version,
            qualifiers,
            subpath,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_type() {
        assert_eq!(normalize_type("MAVEN"), "maven");
        assert_eq!(normalize_type("cargo"), "cargo");
        assert_eq!(normalize_type("  GITHUB  "), "github");
    }

    #[test]
    fn test_normalize_namespace_github() {
        assert_eq!(
            normalize_namespace(Some("Package-url"), "github"),
            Some("package-url".to_string())
        );
    }

    #[test]
    fn test_normalize_namespace_maven() {
        assert_eq!(
            normalize_namespace(Some("org.apache.commons"), "maven"),
            Some("org.apache.commons".to_string())
        );
    }

    #[test]
    fn test_normalize_name_pypi() {
        assert_eq!(normalize_name("Django_package", "pypi"), "django-package");
    }

    #[test]
    fn test_normalize_name_github() {
        assert_eq!(normalize_name("purl-Spec", "github"), "purl-spec");
    }

    #[test]
    fn test_normalize_name_maven() {
        assert_eq!(normalize_name("HTTPClient", "maven"), "HTTPClient");
    }

    #[test]
    fn test_normalize_purl_github() {
        let purl = PackageURL {
            r#type: "github".to_string(),
            namespace: Some("Package-url".to_string()),
            name: "purl-Spec".to_string(),
            version: Some("244fd47e07d1004f0aed9c".to_string()),
            qualifiers: std::collections::BTreeMap::new(),
            subpath: None,
        };
        let normalized = purl.normalize();
        assert_eq!(normalized.namespace, Some("package-url".to_string()));
        assert_eq!(normalized.name, "purl-spec");
    }

    #[test]
    fn test_normalize_purl_maven_preserves_case() {
        let purl = PackageURL {
            r#type: "maven".to_string(),
            namespace: Some("HTTPClient".to_string()),
            name: "HTTPClient".to_string(),
            version: Some("0.3-3".to_string()),
            qualifiers: std::collections::BTreeMap::new(),
            subpath: None,
        };
        let normalized = purl.normalize();
        assert_eq!(normalized.namespace, Some("HTTPClient".to_string()));
        assert_eq!(normalized.name, "HTTPClient");
    }
}
