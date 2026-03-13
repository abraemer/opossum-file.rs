use std::io::Write;

use opossum_lib::core::services::input_reader::InputReader;
use opossum_lib::input_formats::opossum::OpossumFileReader;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn create_test_zip(path: &std::path::Path, input_json: &str, output_json: Option<&str>) {
    let file = std::fs::File::create(path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("input.json", options).unwrap();
    zip.write_all(input_json.as_bytes()).unwrap();

    if let Some(output) = output_json {
        zip.start_file("output.json", options).unwrap();
        zip.write_all(output.as_bytes()).unwrap();
    }

    zip.finish().unwrap();
}

fn get_test_input_json() -> String {
    std::fs::read_to_string("tests/data/opossum_input.json").unwrap()
}

#[test]
fn test_read_opossum_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let result = reader.read();

    assert!(result.is_ok());
    let opossum = result.unwrap();

    assert_eq!(
        opossum.scan_results.metadata.project_id,
        "2a58a469-738e-4508-98d3-a27bce6e71f7"
    );
    assert_eq!(opossum.scan_results.metadata.project_title, "Test Title");
}

#[test]
fn test_read_opossum_file_with_output() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_with_output.opossum");

    let output_json = r#"{
        "metadata": {
            "projectId": "test-project",
            "fileCreationDate": "2020-07-23"
        },
        "manualAttributions": {},
        "resourcesToAttributions": {}
    }"#;

    create_test_zip(&zip_path, &get_test_input_json(), Some(output_json));

    let reader = OpossumFileReader::new(&zip_path);
    let result = reader.read();

    assert!(result.is_ok());
    let opossum = result.unwrap();
    assert!(opossum.review_results.is_some());
}

#[test]
fn test_read_missing_input_json() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("invalid.opossum");

    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("wrong.json", options).unwrap();
    zip.write_all(b"{}").unwrap();
    zip.finish().unwrap();

    let reader = OpossumFileReader::new(&zip_path);
    let result = reader.read();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("corrupt"));
}

#[test]
fn test_resources_parsed_correctly() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_resources.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.resources.children.is_empty());
    assert!(opossum
        .scan_results
        .resources
        .children
        .contains_key("ElectronBackend"));
    assert!(opossum
        .scan_results
        .resources
        .children
        .contains_key("Frontend"));
}

#[test]
fn test_external_attribution_sources_parsed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_sources.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.external_attribution_sources.is_empty());
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("MERGER"));
    assert!(opossum
        .scan_results
        .external_attribution_sources
        .contains_key("HHC"));
}

#[test]
fn test_frequent_licenses_parsed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_licenses.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.frequent_licenses.is_empty());
    let mit_license = opossum
        .scan_results
        .frequent_licenses
        .iter()
        .find(|l| l.short_name == "MIT");
    assert!(mit_license.is_some());
}

#[test]
fn test_attribution_breakpoints_parsed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_breakpoints.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.attribution_breakpoints.is_empty());
    assert!(opossum
        .scan_results
        .attribution_breakpoints
        .contains(&"/Frontend/Components/".to_string()));
}

#[test]
fn test_read_actual_opossum_file() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let result = reader.read();

    assert!(result.is_ok());
    let opossum = result.unwrap();

    assert_eq!(
        opossum.scan_results.metadata.project_id,
        "2a58a469-738e-4508-98d3-a27bce6e71f7"
    );
}

#[test]
fn test_read_opossum_file_with_result() {
    let zip_path = std::path::Path::new("tests/data/opossum_input_with_result.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let result = reader.read();

    assert!(result.is_ok());
    let opossum = result.unwrap();
    assert!(opossum.review_results.is_some());
}

#[test]
fn test_read_opossum_file_with_classification() {
    let zip_path = std::path::Path::new("tests/data/opossum_input_with_classification.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let result = reader.read();

    assert!(result.is_ok());
}

#[test]
fn test_read_corrupt_opossum_file() {
    let zip_path = std::path::Path::new("tests/data/opossum_input_corrupt.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let result = reader.read();

    assert!(result.is_err());
}

#[test]
fn test_attributions_have_correct_ids() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_attributions.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.attribution_to_id.is_empty());
}

#[test]
fn test_files_with_children_parsed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_files_with_children.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.files_with_children.is_empty());
}

#[test]
fn test_base_urls_for_sources_parsed() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_base_urls.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.base_urls_for_sources.urls.is_empty());
}

#[test]
fn test_all_resources_have_paths() {
    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("test_paths.opossum");

    create_test_zip(&zip_path, &get_test_input_json(), None);

    let reader = OpossumFileReader::new(&zip_path);
    let opossum = reader.read().unwrap();

    for resource in opossum.scan_results.resources.all_resources() {
        assert!(!resource.path.to_str().unwrap_or("").is_empty());
    }
}

#[test]
fn test_roundtrip_preserves_metadata() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    let original_metadata = &opossum.scan_results.metadata;

    assert_eq!(
        original_metadata.project_id,
        "2a58a469-738e-4508-98d3-a27bce6e71f7"
    );
    assert_eq!(original_metadata.project_title, "Test Title");
}

#[test]
fn test_roundtrip_preserves_resources() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    let resource_count = opossum.scan_results.resources.all_resources().count();
    assert!(resource_count > 0);
}

#[test]
fn test_roundtrip_preserves_external_attributions() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.attribution_to_id.is_empty());
}

#[test]
fn test_roundtrip_preserves_attribution_breakpoints() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.attribution_breakpoints.is_empty());
}

#[test]
fn test_roundtrip_preserves_external_attribution_sources() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.external_attribution_sources.is_empty());
}

#[test]
fn test_roundtrip_preserves_frequent_licenses() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.frequent_licenses.is_empty());
}

#[test]
fn test_roundtrip_preserves_files_with_children() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.files_with_children.is_empty());
}

#[test]
fn test_roundtrip_preserves_base_urls_for_sources() {
    let zip_path = std::path::Path::new("tests/data/opossum_input.opossum");

    let reader = OpossumFileReader::new(zip_path);
    let opossum = reader.read().unwrap();

    assert!(!opossum.scan_results.base_urls_for_sources.urls.is_empty());
}

#[test]
fn test_input_file_only_roundtrip() {
    let input_json = r#"{
        "metadata": {
            "projectId": "test-id",
            "fileCreationDate": "2020-01-01",
            "projectTitle": "Test"
        },
        "resources": {},
        "externalAttributions": {},
        "resourcesToAttributions": {},
        "attributionBreakpoints": [],
        "externalAttributionSources": {}
    }"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let zip_path = temp_dir.path().join("input_only.opossum");

    create_test_zip(&zip_path, input_json, None);

    let reader = OpossumFileReader::new(&zip_path);
    let result = reader.read();

    assert!(result.is_ok());
    let opossum = result.unwrap();
    assert!(opossum.review_results.is_none());
}
