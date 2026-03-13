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
