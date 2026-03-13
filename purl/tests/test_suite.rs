use std::collections::BTreeMap;
use std::str::FromStr;

use pretty_assertions::assert_eq;
use purl::{PackageURL, PurlError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestCase {
    description: String,
    purl: String,
    canonical_purl: Option<String>,
    #[serde(rename = "type")]
    purl_type: Option<String>,
    namespace: Option<String>,
    name: Option<String>,
    version: Option<String>,
    qualifiers: Option<BTreeMap<String, String>>,
    subpath: Option<String>,
    is_invalid: bool,
}

fn load_test_cases() -> Vec<TestCase> {
    let json_data = include_str!("data/test-suite-data.json");
    serde_json::from_str(json_data).expect("Failed to parse test suite JSON")
}

fn normalize_namespace_for_compare(ns: &str) -> String {
    ns.trim_end_matches('/').to_string()
}

#[test]
fn test_suite() {
    let test_cases = load_test_cases();

    for tc in test_cases {
        let result: Result<PackageURL, PurlError> = PackageURL::from_str(&tc.purl);

        if tc.is_invalid {
            assert!(
                result.is_err(),
                "Expected parsing to fail for '{}': {}",
                tc.purl,
                tc.description
            );
        } else {
            let purl = result.unwrap_or_else(|_| {
                panic!(
                    "Expected parsing to succeed for '{}': {}",
                    tc.purl, tc.description
                )
            });

            if let Some(expected_type) = tc.purl_type {
                assert_eq!(
                    purl.r#type, expected_type,
                    "Type mismatch for '{}': {}",
                    tc.purl, tc.description
                );
            }

            if let Some(ref expected_ns) = tc.namespace {
                let expected = normalize_namespace_for_compare(expected_ns);
                let actual = purl
                    .namespace
                    .as_ref()
                    .map(|ns| normalize_namespace_for_compare(ns))
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "Namespace mismatch for '{}': {}",
                    tc.purl, tc.description
                );
            } else {
                assert_eq!(
                    purl.namespace, None,
                    "Namespace should be None for '{}': {}",
                    tc.purl, tc.description
                );
            }

            if let Some(expected_name) = tc.name {
                assert_eq!(
                    purl.name, expected_name,
                    "Name mismatch for '{}': {}",
                    tc.purl, tc.description
                );
            }

            assert_eq!(
                purl.version, tc.version,
                "Version mismatch for '{}': {}",
                tc.purl, tc.description
            );

            let expected_qualifiers = tc.qualifiers.unwrap_or_default();
            assert_eq!(
                purl.qualifiers, expected_qualifiers,
                "Qualifiers mismatch for '{}': {}",
                tc.purl, tc.description
            );

            assert_eq!(
                purl.subpath, tc.subpath,
                "Subpath mismatch for '{}': {}",
                tc.purl, tc.description
            );

            if let Some(ref expected_canonical) = tc.canonical_purl {
                let canonical = purl.to_string();
                assert_eq!(
                    canonical, *expected_canonical,
                    "Canonical purl mismatch for '{}': {}",
                    tc.purl, tc.description
                );
            }
        }
    }
}
