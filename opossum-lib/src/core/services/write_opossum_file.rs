use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::entities::{
    Config, ExternalAttributionSource, FrequentLicense, Opossum, OpossumPackage, Resource,
    ResourceType, RootResource,
};
use crate::OpossumError;

const INPUT_JSON_NAME: &str = "input.json";
const OUTPUT_JSON_NAME: &str = "output.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumOutputFileModel {
    pub metadata: OutputMetadata,
    pub manual_attributions: BTreeMap<String, ManualAttributions>,
    pub resources_to_attributions: BTreeMap<String, Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_external_attributions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputMetadata {
    pub project_id: String,
    pub file_creation_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_md5_checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualAttributions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_p_url_appendix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_notice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_review: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_over_origin_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_preferred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumInputFileModel {
    pub metadata: InputMetadata,
    pub resources: serde_json::Value,
    pub external_attributions: BTreeMap<String, OpossumPackageModel>,
    pub resources_to_attributions: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub attribution_breakpoints: Vec<String>,
    #[serde(default)]
    pub external_attribution_sources: BTreeMap<String, ExternalAttributionSourceModel>,
    #[serde(default)]
    pub frequent_licenses: Vec<FrequentLicenseModel>,
    #[serde(default)]
    pub files_with_children: Vec<String>,
    #[serde(default)]
    pub base_urls_for_sources: BTreeMap<String, String>,
    #[serde(default)]
    pub config: Option<ConfigModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputMetadata {
    pub project_id: String,
    pub file_creation_date: String,
    pub project_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumPackageModel {
    pub source: SourceInfoModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_p_u_r_l_appendix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_notice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_preferred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfoModel {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttributionSourceModel {
    pub name: String,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relevant_for_preferred: Option<bool>,
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
pub struct ConfigModel {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub classifications: BTreeMap<i32, String>,
}

pub fn write_opossum_file(opossum: &Opossum, output_path: &Path) -> Result<(), OpossumError> {
    let output_path = ensure_outfile_suffix(output_path);

    let input_model = convert_to_input_model(opossum);
    let output_model = convert_to_output_model(opossum);

    let file = std::fs::File::create(&output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file(INPUT_JSON_NAME, options)?;
    let input_json = serde_json::to_string_pretty(&input_model)?;
    zip.write_all(input_json.as_bytes())?;

    if let Some(output) = output_model {
        zip.start_file(OUTPUT_JSON_NAME, options)?;
        let output_json = serde_json::to_string_pretty(&output)?;
        zip.write_all(output_json.as_bytes())?;
    }

    zip.finish()?;
    Ok(())
}

fn ensure_outfile_suffix(path: &Path) -> std::path::PathBuf {
    if path.extension().map(|e| e != "opossum").unwrap_or(true) {
        path.with_extension("opossum")
    } else {
        path.to_path_buf()
    }
}

fn convert_to_input_model(opossum: &Opossum) -> OpossumInputFileModel {
    let scan_results = &opossum.scan_results;

    OpossumInputFileModel {
        metadata: InputMetadata {
            project_id: scan_results.metadata.project_id.clone(),
            file_creation_date: scan_results.metadata.file_creation_date.clone(),
            project_title: scan_results.metadata.project_title.clone(),
            project_version: scan_results.metadata.project_version.clone(),
        },
        resources: convert_resources_to_json(&scan_results.resources),
        external_attributions: convert_external_attributions(&scan_results.attribution_to_id),
        resources_to_attributions: convert_resources_to_attributions(
            &scan_results.resources,
            &scan_results.attribution_to_id,
        ),
        attribution_breakpoints: scan_results.attribution_breakpoints.clone(),
        external_attribution_sources: convert_external_attribution_sources_model(
            &scan_results.external_attribution_sources,
        ),
        frequent_licenses: convert_frequent_licenses_model(&scan_results.frequent_licenses),
        files_with_children: scan_results.files_with_children.clone(),
        base_urls_for_sources: scan_results.base_urls_for_sources.urls.clone(),
        config: if scan_results.config == Config::default() {
            None
        } else {
            Some(ConfigModel {
                classifications: scan_results.config.classifications.clone(),
            })
        },
    }
}

fn convert_resources_to_json(resources: &RootResource) -> serde_json::Value {
    let mut result = serde_json::Map::new();
    for (name, child) in &resources.children {
        result.insert(name.clone(), convert_resource_to_json(child));
    }
    serde_json::Value::Object(result)
}

fn convert_resource_to_json(resource: &Resource) -> serde_json::Value {
    if resource.children.is_empty() {
        let count = if resource.resource_type == Some(ResourceType::File) {
            1
        } else {
            0
        };
        serde_json::Value::Number(count.into())
    } else {
        let mut result = serde_json::Map::new();
        for (name, child) in &resource.children {
            result.insert(name.clone(), convert_resource_to_json(child));
        }
        serde_json::Value::Object(result)
    }
}

fn convert_external_attributions(
    attribution_to_id: &BTreeMap<OpossumPackage, String>,
) -> BTreeMap<String, OpossumPackageModel> {
    attribution_to_id
        .iter()
        .map(|(pkg, id)| {
            (
                id.clone(),
                OpossumPackageModel {
                    source: SourceInfoModel {
                        name: pkg.source.name.clone(),
                        document_confidence: pkg.source.document_confidence.map(|d| d as f64),
                        additional_name: pkg.source.additional_name.clone(),
                    },
                    attribution_confidence: pkg.attribution_confidence.map(|d| d as f64),
                    comment: pkg.comment.clone(),
                    package_name: pkg.package_name.clone(),
                    package_version: pkg.package_version.clone(),
                    package_namespace: pkg.package_namespace.clone(),
                    package_type: pkg.package_type.clone(),
                    package_p_u_r_l_appendix: pkg.package_purl_appendix.clone(),
                    copyright: pkg.copyright.clone(),
                    license_name: pkg.license_name.clone(),
                    license_text: pkg.license_text.clone(),
                    url: pkg.url.clone(),
                    first_party: pkg.first_party,
                    exclude_from_notice: pkg.exclude_from_notice,
                    pre_selected: pkg.pre_selected,
                    follow_up: pkg.follow_up.clone(),
                    origin_id: pkg.origin_id.clone(),
                    origin_ids: pkg.origin_ids.clone(),
                    criticality: pkg.criticality.clone(),
                    classification: pkg.classification,
                    was_preferred: pkg.was_preferred,
                },
            )
        })
        .collect()
}

fn convert_resources_to_attributions(
    resources: &RootResource,
    attribution_to_id: &BTreeMap<OpossumPackage, String>,
) -> BTreeMap<String, Vec<String>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();

    fn collect_attributions(
        resource: &Resource,
        path: &str,
        attribution_to_id: &BTreeMap<OpossumPackage, String>,
        result: &mut BTreeMap<String, Vec<String>>,
    ) {
        if !resource.attributions.is_empty() {
            let ids: Vec<String> = resource
                .attributions
                .iter()
                .filter_map(|attr| attribution_to_id.get(attr).cloned())
                .collect();
            if !ids.is_empty() {
                result.insert(path.to_string(), ids);
            }
        }

        for (name, child) in &resource.children {
            let child_path = format!("{}/{}", path, name);
            collect_attributions(child, &child_path, attribution_to_id, result);
        }
    }

    for (name, child) in &resources.children {
        let path = format!("/{}", name);
        collect_attributions(child, &path, attribution_to_id, &mut result);
    }

    result
}

fn convert_external_attribution_sources_model(
    sources: &BTreeMap<String, ExternalAttributionSource>,
) -> BTreeMap<String, ExternalAttributionSourceModel> {
    sources
        .iter()
        .map(|(name, source)| {
            (
                name.clone(),
                ExternalAttributionSourceModel {
                    name: source.name.clone(),
                    priority: source.priority,
                    is_relevant_for_preferred: source.is_relevant_for_preferred,
                },
            )
        })
        .collect()
}

fn convert_frequent_licenses_model(licenses: &[FrequentLicense]) -> Vec<FrequentLicenseModel> {
    licenses
        .iter()
        .map(|license| FrequentLicenseModel {
            full_name: license.full_name.clone(),
            short_name: license.short_name.clone(),
            default_text: license.default_text.clone(),
        })
        .collect()
}

fn convert_to_output_model(opossum: &Opossum) -> Option<OpossumOutputFileModel> {
    opossum
        .review_results
        .as_ref()
        .and_then(|review| serde_json::from_value(review.clone()).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entities::{Metadata, ScanResults};
    use std::path::PathBuf;
    use tempfile::tempdir;
    use uuid::Uuid;

    fn create_test_opossum() -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: "test project".to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let scan_results = ScanResults::new(metadata);
        Opossum::new(scan_results)
    }

    #[test]
    fn test_ensure_outfile_suffix_adds_extension() {
        assert_eq!(
            ensure_outfile_suffix(Path::new("output.json")),
            PathBuf::from("output.opossum")
        );
    }

    #[test]
    fn test_ensure_outfile_suffix_keeps_existing() {
        assert_eq!(
            ensure_outfile_suffix(Path::new("output.opossum")),
            PathBuf::from("output.opossum")
        );
    }

    #[test]
    fn test_write_opossum_file_creates_zip() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("test_output.json");
        let opossum = create_test_opossum();

        let result = write_opossum_file(&opossum, &output_path);
        assert!(result.is_ok());

        let expected_path = dir.path().join("test_output.opossum");
        assert!(expected_path.exists());
    }

    #[test]
    fn test_convert_resources_to_json_empty() {
        let resources = RootResource::new();
        let json = convert_resources_to_json(&resources);
        assert!(json.is_object());
        assert!(json.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_convert_resources_to_attributions_empty() {
        let resources = RootResource::new();
        let attribution_to_id: BTreeMap<OpossumPackage, String> = BTreeMap::new();
        let result = convert_resources_to_attributions(&resources, &attribution_to_id);
        assert!(result.is_empty());
    }
}
