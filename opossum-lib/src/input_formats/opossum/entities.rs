use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type OpossumPackageIdentifier = String;
pub type ResourcePath = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceInFile {
    Directory(BTreeMap<String, ResourceInFile>),
    FileCount(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumInputFileModel {
    pub metadata: MetadataModel,
    pub resources: ResourceInFile,
    pub external_attributions: BTreeMap<OpossumPackageIdentifier, OpossumPackageModel>,
    pub resources_to_attributions: BTreeMap<ResourcePath, Vec<OpossumPackageIdentifier>>,
    #[serde(default)]
    pub config: Option<ConfigModel>,
    #[serde(default)]
    pub attribution_breakpoints: Vec<String>,
    #[serde(default)]
    pub external_attribution_sources: BTreeMap<String, ExternalAttributionSourceModel>,
    pub frequent_licenses: Option<Vec<FrequentLicenseModel>>,
    pub files_with_children: Option<Vec<String>>,
    pub base_urls_for_sources: Option<BaseUrlsForSourcesModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseUrlsForSourcesModel {
    #[serde(flatten)]
    pub urls: BTreeMap<String, Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigModel {
    pub classifications: Option<BTreeMap<i32, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrequentLicenseModel {
    pub full_name: String,
    pub short_name: String,
    pub default_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfoModel {
    pub name: String,
    #[serde(default)]
    pub document_confidence: Option<f64>,
    pub additional_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumPackageModel {
    pub source: SourceInfoModel,
    pub attribution_confidence: Option<i32>,
    pub comment: Option<String>,
    pub package_name: Option<String>,
    pub package_version: Option<String>,
    pub package_namespace: Option<String>,
    pub package_type: Option<String>,
    pub package_p_u_r_l_appendix: Option<String>,
    pub copyright: Option<String>,
    pub license_name: Option<String>,
    pub license_text: Option<String>,
    pub url: Option<String>,
    pub first_party: Option<bool>,
    pub exclude_from_notice: Option<bool>,
    pub pre_selected: Option<bool>,
    pub follow_up: Option<String>,
    pub origin_id: Option<String>,
    pub origin_ids: Option<Vec<String>>,
    pub criticality: Option<String>,
    pub classification: Option<i32>,
    pub was_preferred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataModel {
    pub project_id: String,
    pub file_creation_date: String,
    pub project_title: String,
    pub project_version: Option<String>,
    pub expected_release_date: Option<String>,
    pub build_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttributionSourceModel {
    pub name: String,
    pub priority: i32,
    pub is_relevant_for_preferred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumOutputFileModel {
    pub metadata: OutputMetadata,
    pub manual_attributions: BTreeMap<String, ManualAttributions>,
    pub resources_to_attributions: BTreeMap<String, Vec<String>>,
    pub resolved_external_attributions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputMetadata {
    pub project_id: String,
    pub file_creation_date: String,
    pub input_file_md5_checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualAttributions {
    pub package_name: Option<String>,
    pub package_version: Option<String>,
    pub package_namespace: Option<String>,
    pub package_type: Option<String>,
    pub package_p_u_r_l_appendix: Option<String>,
    pub url: Option<String>,
    pub license_name: Option<String>,
    pub license_text: Option<String>,
    pub attribution_confidence: Option<f64>,
    pub comment: Option<String>,
    pub criticality: Option<String>,
    pub copyright: Option<String>,
    pub first_party: Option<bool>,
    pub pre_selected: Option<bool>,
    pub exclude_from_notice: Option<bool>,
    pub follow_up: Option<String>,
    pub origin_id: Option<String>,
    pub origin_ids: Option<Vec<String>>,
    pub needs_review: Option<bool>,
    pub preferred: Option<bool>,
    pub preferred_over_origin_ids: Option<Vec<String>>,
    pub was_preferred: Option<bool>,
}
