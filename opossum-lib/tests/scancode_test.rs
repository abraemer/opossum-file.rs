use std::path::Path;

use opossum_lib::core::services::InputReader;
use opossum_lib::input_formats::ScanCodeFileReader;

#[test]
fn test_scancode_file_reader_parses_scancode_input() {
    let test_file = Path::new("tests/data/scancode_input.json");
    let reader = ScanCodeFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read scancode data");

    assert!(!opossum.scan_results.resources.children.is_empty());
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("SC"));
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("SC-P"));
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("SC-D"));
}

#[test]
fn test_scancode_file_reader_has_correct_metadata() {
    let test_file = Path::new("tests/data/scancode_input.json");
    let reader = ScanCodeFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read scancode data");

    assert_eq!(opossum.scan_results.metadata.project_title, "ScanCode file");
    assert!(!opossum.scan_results.metadata.project_id.is_empty());
    assert!(!opossum.scan_results.metadata.file_creation_date.is_empty());
}

#[test]
fn test_scancode_file_reader_has_resources() {
    let test_file = Path::new("tests/data/scancode_input.json");
    let reader = ScanCodeFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read scancode data");

    let all_resources: Vec<_> = opossum.scan_results.resources.all_resources().collect();
    assert!(!all_resources.is_empty());

    let src_resource = all_resources
        .iter()
        .find(|r| r.path.to_str() == Some("src/index.tsx"));
    assert!(src_resource.is_some());

    let src_resource = src_resource.unwrap();
    assert!(!src_resource.attributions.is_empty());

    let has_license = src_resource
        .attributions
        .iter()
        .any(|a| a.license_name.as_deref() == Some("Apache-2.0"));
    assert!(has_license);
}

#[test]
fn test_scancode_file_reader_has_copyrights() {
    let test_file = Path::new("tests/data/scancode_input.json");
    let reader = ScanCodeFileReader::from_file(test_file).expect("Failed to create reader");

    let opossum = reader.read().expect("Failed to read scancode data");

    let all_resources: Vec<_> = opossum.scan_results.resources.all_resources().collect();
    let src_resource = all_resources
        .iter()
        .find(|r| r.path.to_str() == Some("src/index.tsx"));
    assert!(src_resource.is_some());

    let src_resource = src_resource.unwrap();
    let has_copyright = src_resource.attributions.iter().any(|a| {
        a.copyright
            .as_ref()
            .is_some_and(|c| c.contains("TNG Technology Consulting"))
    });
    assert!(has_copyright);
}
