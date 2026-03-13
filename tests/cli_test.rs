use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_opossum-file"))
}

fn get_test_data_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("opossum-lib")
        .join("tests")
        .join("data")
}

#[test]
fn test_generate_scancode_file() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--scan-code-json",
            &get_test_data_path()
                .join("scancode_input.json")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
    assert!(output_path.exists());
}

#[test]
fn test_generate_opossum_file() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--opossum-file",
            &get_test_data_path()
                .join("opossum_input.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
    assert!(output_path.exists());
}

#[test]
fn test_generate_owasp_file() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--dependency-check-json",
            &get_test_data_path()
                .join("dependency-check-report.json")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
    assert!(output_path.exists());
}

#[test]
fn test_generate_opossum_file_with_result() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--opossum-file",
            &get_test_data_path()
                .join("opossum_input_with_result.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
}

#[test]
fn test_generate_opossum_file_with_classification() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--opossum-file",
            &get_test_data_path()
                .join("opossum_input_with_classification.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
}

#[test]
fn test_generate_multiple_files() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--scan-code-json",
            &get_test_data_path()
                .join("scancode_input.json")
                .to_string_lossy(),
            "--opossum-file",
            &get_test_data_path()
                .join("opossum_input.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
    assert!(output_path.exists());
}

#[test]
fn test_generate_no_inputs() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args(["generate", "-o", &output_path.to_string_lossy()])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
}

#[test]
fn test_merge_requires_at_least_two_files() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "merge",
            &get_test_data_path()
                .join("opossum_input.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(!status.success());
}

#[test]
fn test_merge_two_files() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "merge",
            &get_test_data_path()
                .join("opossum_input.opossum")
                .to_string_lossy(),
            &get_test_data_path()
                .join("opossum_input_with_classification.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(status.success());
    assert!(output_path.exists());
}

#[test]
fn test_help_output() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("merge"));
}

#[test]
fn test_generate_help() {
    let output = Command::new(get_binary_path())
        .args(["generate", "--help"])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--scan-code-json"));
    assert!(stdout.contains("--opossum-file"));
    assert!(stdout.contains("--dependency-check-json"));
}

#[test]
fn test_merge_help() {
    let output = Command::new(get_binary_path())
        .args(["merge", "--help"])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("INPUT_FILES"));
    assert!(stdout.contains("--output"));
}

#[test]
fn test_corrupt_opossum_file_errors() {
    let output_file = tempfile::NamedTempFile::with_suffix(".opossum").unwrap();
    let output_path = output_file.path();

    let status = Command::new(get_binary_path())
        .args([
            "generate",
            "--opossum-file",
            &get_test_data_path()
                .join("opossum_input_corrupt.opossum")
                .to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
        ])
        .status()
        .expect("Failed to run command");

    assert!(!status.success());
}
