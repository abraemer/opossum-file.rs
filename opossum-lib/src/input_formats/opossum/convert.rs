use std::collections::{BTreeMap, HashSet};

use crate::core::entities::base_urls_for_sources::BaseUrlsForSources;
use crate::core::entities::config::Config;
use crate::core::entities::external_attribution_source::ExternalAttributionSource;
use crate::core::entities::frequent_license::FrequentLicense;
use crate::core::entities::metadata::Metadata;
use crate::core::entities::opossum::Opossum;
use crate::core::entities::opossum_package::OpossumPackage;
use crate::core::entities::resource::{Resource, ResourceType};
use crate::core::entities::root_resource::RootResource;
use crate::core::entities::scan_results::ScanResults;
use crate::core::entities::source_info::SourceInfo;

use super::entities::{
    BaseUrlsForSourcesModel, ConfigModel, FrequentLicenseModel, MetadataModel,
    OpossumInputFileModel, OpossumOutputFileModel, OpossumPackageModel, ResourceInFile,
    SourceInfoModel,
};

pub fn convert_to_opossum(
    input_file: OpossumInputFileModel,
    output_file: Option<OpossumOutputFileModel>,
) -> Opossum {
    let scan_results = convert_to_scan_results(&input_file);
    let review_results =
        output_file.map(|o| serde_json::to_value(o).unwrap_or(serde_json::Value::Null));
    Opossum::new(scan_results).with_review_results(review_results)
}

fn convert_to_scan_results(input_file: &OpossumInputFileModel) -> ScanResults {
    let (resources, used_attribution_ids) = convert_to_resource_tree(
        &input_file.resources,
        &input_file.external_attributions,
        &input_file.resources_to_attributions,
    );

    let frequent_licenses: Vec<FrequentLicense> = input_file
        .frequent_licenses
        .as_ref()
        .map(|licenses| licenses.iter().map(convert_frequent_license).collect())
        .unwrap_or_default();

    let base_urls_for_sources =
        convert_base_urls_for_sources(input_file.base_urls_for_sources.as_ref());

    let external_attribution_sources: BTreeMap<String, ExternalAttributionSource> = input_file
        .external_attribution_sources
        .iter()
        .map(|(name, source)| (name.clone(), convert_external_attribution_source(source)))
        .collect();

    let attribution_to_id = convert_to_attribution_with_id(&input_file.external_attributions);

    let unassigned_attributions =
        get_unassigned_attributions(&used_attribution_ids, &input_file.external_attributions);

    let config = convert_config(input_file.config.as_ref());

    let mut scan_results = ScanResults::new(convert_metadata(&input_file.metadata));
    scan_results.resources = resources;
    scan_results.attribution_breakpoints = input_file.attribution_breakpoints.clone();
    scan_results.external_attribution_sources = external_attribution_sources;
    scan_results.config = config;
    scan_results.frequent_licenses = frequent_licenses;
    scan_results.files_with_children = input_file.files_with_children.clone().unwrap_or_default();
    scan_results.base_urls_for_sources = base_urls_for_sources;
    scan_results.attribution_to_id = attribution_to_id;
    scan_results.unassigned_attributions = unassigned_attributions;
    scan_results
}

fn convert_metadata(metadata: &MetadataModel) -> Metadata {
    Metadata {
        project_id: metadata.project_id.clone(),
        file_creation_date: metadata.file_creation_date.clone(),
        project_title: metadata.project_title.clone(),
        project_version: metadata.project_version.clone(),
        expected_release_date: metadata.expected_release_date.clone(),
        build_date: metadata.build_date.clone(),
        extra: BTreeMap::new(),
    }
}

fn convert_base_urls_for_sources(model: Option<&BaseUrlsForSourcesModel>) -> BaseUrlsForSources {
    match model {
        Some(m) => BaseUrlsForSources {
            urls: m
                .urls
                .iter()
                .filter_map(|(k, v)| v.as_ref().map(|url| (k.clone(), url.clone())))
                .collect(),
        },
        None => BaseUrlsForSources::default(),
    }
}

fn convert_config(model: Option<&ConfigModel>) -> Config {
    match model {
        Some(m) => Config {
            classifications: m.classifications.clone().unwrap_or_default(),
            extra: BTreeMap::new(),
        },
        None => Config::default(),
    }
}

fn convert_external_attribution_source(
    source: &super::entities::ExternalAttributionSourceModel,
) -> ExternalAttributionSource {
    ExternalAttributionSource {
        name: source.name.clone(),
        priority: source.priority,
        is_relevant_for_preferred: source.is_relevant_for_preferred,
    }
}

fn convert_frequent_license(license: &FrequentLicenseModel) -> FrequentLicense {
    FrequentLicense {
        full_name: license.full_name.clone(),
        short_name: license.short_name.clone(),
        default_text: license.default_text.clone(),
    }
}

fn convert_to_resource_tree(
    resources: &ResourceInFile,
    external_attributions: &BTreeMap<String, OpossumPackageModel>,
    resources_to_attributions: &BTreeMap<String, Vec<String>>,
) -> (RootResource, HashSet<String>) {
    let mut used_attribution_ids = HashSet::new();

    match resources {
        ResourceInFile::Directory(children) => {
            let mut root = RootResource::new();
            for (name, child) in children {
                let path = std::path::PathBuf::from(name);
                let resource = generate_child_resource(
                    path,
                    child,
                    external_attributions,
                    resources_to_attributions,
                    &mut used_attribution_ids,
                );
                if let Err(e) = root.add_resource(resource) {
                    tracing::warn!("Failed to add resource: {}", e);
                }
            }
            (root, used_attribution_ids)
        }
        ResourceInFile::FileCount(_) => {
            panic!("Root node must not be of file type");
        }
    }
}

fn generate_child_resource(
    path: std::path::PathBuf,
    to_insert: &ResourceInFile,
    external_attributions: &BTreeMap<String, OpossumPackageModel>,
    resources_to_attributions: &BTreeMap<String, Vec<String>>,
    used_attribution_ids: &mut HashSet<String>,
) -> Resource {
    let path_str = format!("/{}", path.to_string_lossy());
    let (attributions, attribution_ids) =
        get_applicable_attributions(&path_str, external_attributions, resources_to_attributions);
    used_attribution_ids.extend(attribution_ids);

    match to_insert {
        ResourceInFile::FileCount(_) => Resource::new(path.clone())
            .with_type(ResourceType::File)
            .with_attributions(attributions),
        ResourceInFile::Directory(children) => {
            let mut resource = Resource::new(path.clone())
                .with_type(ResourceType::Directory)
                .with_attributions(attributions);

            for (name, child) in children {
                let child_path = path.join(name);
                let child_resource = generate_child_resource(
                    child_path,
                    child,
                    external_attributions,
                    resources_to_attributions,
                    used_attribution_ids,
                );
                if let Err(e) = resource.add_resource(child_resource) {
                    tracing::warn!("Failed to add child resource: {}", e);
                }
            }
            resource
        }
    }
}

fn get_applicable_attributions(
    path: &str,
    external_attributions: &BTreeMap<String, OpossumPackageModel>,
    resources_to_attributions: &BTreeMap<String, Vec<String>>,
) -> (Vec<OpossumPackage>, Vec<String>) {
    match resources_to_attributions.get(path) {
        Some(attribution_ids) => {
            let attributions: Vec<OpossumPackage> = attribution_ids
                .iter()
                .filter_map(|id| external_attributions.get(id).map(convert_package))
                .collect();
            (attributions, attribution_ids.clone())
        }
        None => (Vec::new(), Vec::new()),
    }
}

fn convert_to_attribution_with_id(
    external_attributions: &BTreeMap<String, OpossumPackageModel>,
) -> BTreeMap<OpossumPackage, String> {
    let mut result = BTreeMap::new();
    for (package_identifier, package) in external_attributions {
        let converted_package = convert_package(package);
        if result.contains_key(&converted_package) {
            panic!("An attribution was duplicated in the scan breaking internal assertions");
        }
        result.insert(converted_package, package_identifier.clone());
    }
    result
}

fn get_unassigned_attributions(
    used_attribution_ids: &HashSet<String>,
    external_attributions: &BTreeMap<String, OpossumPackageModel>,
) -> HashSet<OpossumPackage> {
    external_attributions
        .iter()
        .filter(|(id, _)| !used_attribution_ids.contains(*id))
        .map(|(_, package)| convert_package(package))
        .collect()
}

fn convert_package(package: &OpossumPackageModel) -> OpossumPackage {
    OpossumPackage {
        source: convert_source(&package.source),
        attribution_confidence: package.attribution_confidence.map(|d| d as i32),
        comment: package.comment.clone(),
        package_name: package.package_name.clone(),
        package_version: package.package_version.clone(),
        package_namespace: package.package_namespace.clone(),
        package_type: package.package_type.clone(),
        package_purl_appendix: package.package_p_u_r_l_appendix.clone(),
        copyright: package.copyright.clone(),
        license_name: package.license_name.clone(),
        license_text: package.license_text.clone(),
        url: package.url.clone(),
        first_party: package.first_party,
        exclude_from_notice: package.exclude_from_notice,
        pre_selected: package.pre_selected,
        follow_up: package.follow_up.clone(),
        origin_id: package.origin_id.clone(),
        origin_ids: package.origin_ids.clone(),
        criticality: package.criticality.clone(),
        classification: package.classification,
        was_preferred: package.was_preferred,
    }
}

fn convert_source(source: &SourceInfoModel) -> SourceInfo {
    SourceInfo {
        name: source.name.clone(),
        document_confidence: source.document_confidence.map(|d| d as i32),
        additional_name: source.additional_name.clone(),
    }
}
