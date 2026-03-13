use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

use purl::PackageURL;

use crate::core::entities::{
    ExternalAttributionSource, Metadata, Opossum, OpossumPackage, OpossumPackageBuilder, Resource,
    ResourceType, RootResource, ScanResults, SourceInfo,
};
use crate::core::services::InputReader;
use crate::error::OpossumError;

use super::entities::{
    DependencyModel, EvidenceCollectedModel, OwaspDependencyReportModel, PackageModel,
};

const OWASP_SOURCE_NAME: &str = "Dependency-Check";
const OWASP_PRIORITY: i32 = 40;
const OWASP_CONFIDENCE: i32 = 50;

pub struct OwaspDependencyScanFileReader {
    content: String,
}

impl OwaspDependencyScanFileReader {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn from_file(path: &std::path::Path) -> Result<Self, OpossumError> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::new(content))
    }
}

impl InputReader for OwaspDependencyScanFileReader {
    fn read(&self) -> Result<Opossum, OpossumError> {
        let owasp_data: OwaspDependencyReportModel = serde_json::from_str(&self.content)?;
        convert_to_opossum(owasp_data).map_err(OpossumError::ParseError)
    }
}

fn convert_to_opossum(owasp_data: OwaspDependencyReportModel) -> Result<Opossum, String> {
    Ok(Opossum::new(ScanResults {
        metadata: extract_metadata(&owasp_data),
        resources: extract_resources(&owasp_data)?,
        external_attribution_sources: get_external_attribution_sources(),
        files_with_children: get_files_with_children(&owasp_data),
        ..ScanResults::new(Metadata {
            project_id: String::new(),
            file_creation_date: String::new(),
            project_title: String::new(),
            build_date: None,
            project_version: None,
            expected_release_date: None,
            extra: BTreeMap::new(),
        })
    }))
}

fn get_external_attribution_sources() -> BTreeMap<String, ExternalAttributionSource> {
    let mut sources = BTreeMap::new();
    sources.insert(
        OWASP_SOURCE_NAME.to_string(),
        ExternalAttributionSource {
            name: OWASP_SOURCE_NAME.to_string(),
            priority: OWASP_PRIORITY,
            is_relevant_for_preferred: None,
        },
    );
    sources
}

fn extract_metadata(owasp_data: &OwaspDependencyReportModel) -> Metadata {
    Metadata {
        project_id: owasp_data
            .project_info
            .artifact_id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        file_creation_date: owasp_data.project_info.report_date.clone(),
        project_title: owasp_data.project_info.name.clone(),
        build_date: Some(chrono::Utc::now().to_rfc3339()),
        project_version: None,
        expected_release_date: None,
        extra: BTreeMap::new(),
    }
}

fn extract_resources(owasp_data: &OwaspDependencyReportModel) -> Result<RootResource, String> {
    let mut resources = RootResource::new();

    for dependency in &owasp_data.dependencies {
        let path = extract_path(dependency);
        let resource = Resource::new(path)
            .with_type(ResourceType::File)
            .with_attributions(get_attribution_info(dependency)?);
        resources.add_resource(resource)?;
    }

    Ok(resources)
}

fn get_files_with_children(owasp_data: &OwaspDependencyReportModel) -> Vec<String> {
    owasp_data
        .dependencies
        .iter()
        .filter(|d| d.is_virtual)
        .map(|d| {
            let mut path = d.file_path.clone();
            if !path.ends_with('/') {
                path.push('/');
            }
            path
        })
        .collect()
}

fn extract_path(dependency: &DependencyModel) -> PathBuf {
    if dependency.is_virtual {
        PathBuf::from(&dependency.file_path).join(&dependency.file_name)
    } else {
        PathBuf::from(&dependency.file_path)
    }
}

fn get_attribution_info(dependency: &DependencyModel) -> Result<Vec<OpossumPackage>, String> {
    let builders = get_builders_from_additional_information(dependency)?;
    Ok(builders
        .into_iter()
        .map(|builder| populate_common_information(builder, dependency).build())
        .collect())
}

fn get_builders_from_additional_information(
    dependency: &DependencyModel,
) -> Result<Vec<OpossumPackageBuilder>, String> {
    if let Some(packages) = &dependency.packages {
        Ok(get_attribution_builders_from_packages(packages))
    } else {
        Ok(get_attribution_builders_from_evidence(
            &dependency.evidence_collected,
        ))
    }
}

fn get_attribution_builders_from_packages(packages: &[PackageModel]) -> Vec<OpossumPackageBuilder> {
    packages
        .iter()
        .map(get_attribution_info_from_package)
        .collect()
}

fn get_attribution_info_from_package(package: &PackageModel) -> OpossumPackageBuilder {
    let mut builder = get_base_opossum_package_builder();

    if let Ok(purl) = PackageURL::from_str(&package.id) {
        if let Some(v) = purl.version {
            builder = builder.package_version(v);
        }
        if let Some(ns) = purl.namespace {
            builder = builder.package_namespace(ns);
        }
        builder = builder.package_name(purl.name).package_type(purl.r#type);
    } else {
        builder = builder.package_name(package.id.clone());
    }

    if let Some(url) = &package.url {
        builder = builder.url(url.clone());
    }

    builder
}

fn get_attribution_builders_from_evidence(
    evidence_collected: &EvidenceCollectedModel,
) -> Vec<OpossumPackageBuilder> {
    let namespace = get_first_evidence_value(&evidence_collected.vendor_evidence);
    let name = get_first_evidence_value(&evidence_collected.product_evidence);
    let version = get_first_evidence_value(&evidence_collected.version_evidence);

    if name.is_some() || version.is_some() || namespace.is_some() {
        let mut builder = get_base_opossum_package_builder();
        if let Some(v) = version {
            builder = builder.package_version(v);
        }
        if let Some(ns) = namespace {
            builder = builder.package_namespace(ns);
        }
        if let Some(n) = name {
            builder = builder.package_name(n);
        }
        vec![builder]
    } else {
        vec![]
    }
}

fn get_first_evidence_value(evidences: &[super::entities::EvidenceModel]) -> Option<String> {
    evidences.first().map(|e| e.value.clone())
}

fn get_base_opossum_package_builder() -> OpossumPackageBuilder {
    OpossumPackageBuilder::new(SourceInfo {
        name: OWASP_SOURCE_NAME.to_string(),
        document_confidence: Some(OWASP_CONFIDENCE),
        additional_name: None,
    })
    .attribution_confidence(OWASP_CONFIDENCE)
}

fn populate_common_information(
    mut builder: OpossumPackageBuilder,
    dependency: &DependencyModel,
) -> OpossumPackageBuilder {
    if let Some(follow_up) = extract_follow_up(dependency) {
        builder = builder.follow_up(follow_up);
    }
    if let Some(comment) = extract_comment(dependency) {
        builder = builder.comment(comment);
    }
    if let Some(license) = &dependency.license {
        builder = builder.license_name(license.clone());
    }
    builder
}

fn extract_comment(dependency: &DependencyModel) -> Option<String> {
    dependency
        .vulnerabilities
        .as_ref()
        .map(|vulns| serde_json::to_string_pretty(&vulns).unwrap_or_else(|_| "[]".to_string()))
}

fn extract_follow_up(dependency: &DependencyModel) -> Option<String> {
    if dependency.vulnerabilities.is_some() {
        Some("FOLLOW_UP".to_string())
    } else {
        None
    }
}
