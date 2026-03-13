use std::path::Path;

use opossum_lib::core::services::InputReader;
use opossum_lib::input_formats::OwaspDependencyScanFileReader;

#[test]
fn test_owasp_file_reader_parses_owasp_input() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    assert!(!opossum.scan_results.resources.children.is_empty());
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("Dependency-Check"));
}

#[test]
fn test_owasp_file_reader_has_correct_metadata() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    assert!(!opossum.scan_results.metadata.project_id.is_empty());
    assert!(!opossum.scan_results.metadata.file_creation_date.is_empty());
    assert!(opossum.scan_results.metadata.build_date.is_some());
}

#[test]
fn test_owasp_file_reader_has_resources() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    let all_resources: Vec<_> = opossum.scan_results.resources.all_resources().collect();
    assert!(!all_resources.is_empty());

    let dotzlib_resource = all_resources
        .iter()
        .find(|r| r.path.to_str() == Some("contrib/dotzlib/DotZLib/DotZLib.csproj"));
    assert!(dotzlib_resource.is_some());

    let dotzlib_resource = dotzlib_resource.unwrap();
    assert!(!dotzlib_resource.attributions.is_empty());
}

#[test]
fn test_owasp_file_reader_has_virtual_dependencies() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    assert!(!opossum.scan_results.files_with_children.is_empty());
}

#[test]
fn test_owasp_file_reader_has_vulnerability_info() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    let all_resources: Vec<_> = opossum.scan_results.resources.all_resources().collect();

    let async_resource = all_resources.iter().find(|r| {
        r.path
            .to_str()
            .map(|s| s.contains("async:2.6.3"))
            .unwrap_or(false)
    });
    assert!(async_resource.is_some());

    let async_resource = async_resource.unwrap();
    let has_follow_up = async_resource
        .attributions
        .iter()
        .any(|a| a.follow_up.as_deref() == Some("FOLLOW_UP"));
    assert!(has_follow_up);

    let has_comment = async_resource
        .attributions
        .iter()
        .any(|a| a.comment.is_some());
    assert!(has_comment);
}

#[test]
fn test_owasp_file_reader_has_purl_info() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    let all_resources: Vec<_> = opossum.scan_results.resources.all_resources().collect();

    let async_resource = all_resources.iter().find(|r| {
        r.path
            .to_str()
            .map(|s| s.contains("async:2.6.3"))
            .unwrap_or(false)
    });
    assert!(async_resource.is_some());

    let async_resource = async_resource.unwrap();
    let has_purl_info = async_resource.attributions.iter().any(|a| {
        a.package_name.as_deref() == Some("async")
            && a.package_version.as_deref() == Some("2.6.3")
            && a.package_type.as_deref() == Some("npm")
    });
    assert!(has_purl_info);
}

#[test]
fn test_owasp_no_review_results() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    assert!(opossum.review_results.is_none());
}

#[test]
fn test_owasp_has_correct_external_attribution_source() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    assert_eq!(opossum.scan_results.external_attribution_sources.len(), 1);
    let source = opossum
        .scan_results
        .external_attribution_sources
        .get("Dependency-Check");
    assert!(source.is_some());
    let source = source.unwrap();
    assert_eq!(source.name, "Dependency-Check");
    assert_eq!(source.priority, 40);
}

#[test]
fn test_owasp_all_attributions_have_correct_source() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    for resource in opossum.scan_results.resources.all_resources() {
        for attribution in &resource.attributions {
            assert_eq!(attribution.source.name, "Dependency-Check");
            assert_eq!(attribution.attribution_confidence, Some(50));
        }
    }
}

#[test]
fn test_owasp_attribution_count() {
    let test_file = Path::new("tests/data/dependency-check-report.json");
    let reader =
        OwaspDependencyScanFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read OWASP data");

    let attribution_count: usize = opossum
        .scan_results
        .resources
        .all_resources()
        .map(|r| r.attributions.len())
        .sum();

    assert!(attribution_count > 0);
}
