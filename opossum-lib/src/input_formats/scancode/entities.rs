use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScancodeModel {
    pub files: Vec<FileModel>,
    pub headers: Vec<HeaderModel>,
    #[serde(default)]
    pub license_references: Option<Vec<LicenseReferenceModel>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderModel {
    pub tool_name: String,
    pub tool_version: String,
    pub options: OptionsModel,
    pub notice: String,
    pub start_timestamp: String,
    pub end_timestamp: String,
    pub output_format_version: String,
    pub duration: f64,
    pub message: Option<serde_json::Value>,
    #[serde(default)]
    pub errors: Vec<serde_json::Value>,
    #[serde(default)]
    pub warnings: Vec<serde_json::Value>,
    #[serde(default)]
    pub extra_data: Option<ExtraDataModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsModel {
    pub input: Vec<String>,
    #[serde(default, rename = "--strip-root")]
    pub strip_root: bool,
    #[serde(default, rename = "--full-root")]
    pub full_root: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraDataModel {
    #[serde(default)]
    pub files_count: Option<i32>,
    #[serde(default)]
    pub spdx_license_list_version: Option<String>,
    #[serde(default)]
    pub system_environment: Option<SystemEnvironmentModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEnvironmentModel {
    #[serde(default)]
    pub operating_system: Option<String>,
    #[serde(default)]
    pub cpu_architecture: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub platform_version: Option<String>,
    #[serde(default)]
    pub python_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileTypeModel {
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModel {
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: FileTypeModel,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub base_name: Option<String>,
    #[serde(default)]
    pub extension: Option<String>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub file_type_str: Option<String>,
    #[serde(default)]
    pub programming_language: Option<String>,
    #[serde(default)]
    pub is_binary: bool,
    #[serde(default)]
    pub is_text: bool,
    #[serde(default)]
    pub is_archive: bool,
    #[serde(default)]
    pub is_media: bool,
    #[serde(default)]
    pub is_source: bool,
    #[serde(default)]
    pub is_script: bool,
    #[serde(default)]
    pub package_data: Option<Vec<PackageDataModel>>,
    #[serde(default)]
    pub for_packages: Option<Vec<String>>,
    #[serde(default)]
    pub detected_license_expression: Option<String>,
    #[serde(default)]
    pub detected_license_expression_spdx: Option<String>,
    #[serde(default)]
    pub license_detections: Option<Vec<FileBasedLicenseDetectionModel>>,
    #[serde(default)]
    pub license_clues: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub percentage_of_license_text: Option<f64>,
    #[serde(default)]
    pub copyrights: Option<Vec<CopyrightModel>>,
    #[serde(default)]
    pub holders: Option<Vec<HolderModel>>,
    #[serde(default)]
    pub authors: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub emails: Option<Vec<EmailModel>>,
    #[serde(default)]
    pub urls: Option<Vec<UrlModel>>,
    #[serde(default)]
    pub files_count: Option<i32>,
    #[serde(default)]
    pub dirs_count: Option<i32>,
    #[serde(default)]
    pub size_count: Option<i64>,
    #[serde(default)]
    pub scan_errors: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBasedLicenseDetectionModel {
    #[serde(default)]
    pub identifier: Option<String>,
    pub license_expression: String,
    #[serde(alias = "spdx_license_expression")]
    pub license_expression_spdx: String,
    #[serde(default)]
    pub matches: Vec<MatchModel>,
    #[serde(default)]
    pub detection_count: Option<i32>,
    #[serde(default)]
    pub reference_matches: Option<Vec<MatchModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchModel {
    #[serde(default)]
    pub license_expression: Option<String>,
    #[serde(alias = "spdx_license_expression")]
    pub license_expression_spdx: String,
    #[serde(default)]
    pub from_file: Option<String>,
    pub start_line: i32,
    pub end_line: i32,
    #[serde(default)]
    pub matcher: Option<String>,
    pub score: f64,
    #[serde(default)]
    pub matched_length: Option<i32>,
    #[serde(default)]
    pub match_coverage: Option<f64>,
    #[serde(default)]
    pub rule_relevance: Option<i32>,
    #[serde(default)]
    pub rule_identifier: Option<String>,
    #[serde(default)]
    pub rule_url: Option<String>,
    #[serde(default)]
    pub matched_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyrightModel {
    pub copyright: String,
    #[serde(default)]
    pub start_line: Option<i32>,
    #[serde(default)]
    pub end_line: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolderModel {
    pub holder: String,
    #[serde(default)]
    pub start_line: Option<i32>,
    #[serde(default)]
    pub end_line: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlModel {
    pub url: String,
    #[serde(default)]
    pub start_line: Option<i32>,
    #[serde(default)]
    pub end_line: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailModel {
    pub email: String,
    #[serde(default)]
    pub start_line: Option<i32>,
    #[serde(default)]
    pub end_line: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDataModel {
    #[serde(default)]
    pub purl: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "type")]
    pub package_type: Option<String>,
    #[serde(default)]
    pub namespace: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub qualifiers: Option<serde_json::Value>,
    #[serde(default)]
    pub subpath: Option<String>,
    #[serde(default)]
    pub primary_language: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub homepage_url: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub repository_homepage_url: Option<String>,
    #[serde(default)]
    pub code_view_url: Option<String>,
    #[serde(default)]
    pub vcs_url: Option<String>,
    #[serde(default)]
    pub copyright: Option<String>,
    #[serde(default)]
    pub holder: Option<String>,
    #[serde(default)]
    pub declared_license_expression: Option<String>,
    #[serde(default)]
    pub declared_license_expression_spdx: Option<String>,
    #[serde(default)]
    pub other_license_expression: Option<String>,
    #[serde(default)]
    pub other_license_expression_spdx: Option<String>,
    #[serde(default)]
    pub license_detections: Option<Vec<FileBasedLicenseDetectionModel>>,
    #[serde(default)]
    pub notice_text: Option<String>,
    #[serde(default)]
    pub dependencies: Option<Vec<DependencyModel>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyModel {
    #[serde(default)]
    pub purl: Option<String>,
    #[serde(default)]
    pub extracted_requirement: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub is_runtime: bool,
    #[serde(default)]
    pub is_optional: bool,
    #[serde(default, alias = "is_resolved")]
    pub is_pinned: bool,
    #[serde(default)]
    pub is_direct: bool,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseReferenceModel {
    pub spdx_license_key: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub short_name: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub homepage_url: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
