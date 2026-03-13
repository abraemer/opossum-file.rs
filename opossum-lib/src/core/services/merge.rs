use std::collections::{BTreeMap, HashSet};

use uuid::Uuid;

use crate::core::entities::{
    BaseUrlsForSources, Config, ExternalAttributionSource, FrequentLicense, Metadata, Opossum,
    OpossumPackage, RootResource, ScanResults,
};

pub fn merge_opossums(opossums: Vec<Opossum>) -> Result<Opossum, String> {
    if opossums.len() < 2 {
        return Err(format!(
            "You need to provide at least 2 opossums for merging. Got: {}",
            opossums.len()
        ));
    }
    let scan_results = merge_scan_results(&opossums);
    let review_results = handle_review_results(&opossums, &scan_results);
    Ok(Opossum::new(scan_results).with_review_results(review_results))
}

fn handle_review_results(
    opossums: &[Opossum],
    scan_results: &ScanResults,
) -> Option<serde_json::Value> {
    let review_results: Vec<_> = opossums
        .iter()
        .filter_map(|o| o.review_results.as_ref())
        .collect();

    if review_results.is_empty() {
        return None;
    }
    if review_results.len() > 1 {
        tracing::warn!(
            "More than one .opossum input contains review results. This is currently unsupported. Got: {}",
            review_results.len()
        );
        return None;
    }

    if let Some(review) = review_results.first() {
        let mut review = (*review).clone();
        if let Some(obj) = review.as_object_mut() {
            obj.insert(
                "projectId".to_string(),
                serde_json::Value::String(scan_results.metadata.project_id.clone()),
            );
            obj.insert(
                "fileCreationDate".to_string(),
                serde_json::Value::String(scan_results.metadata.file_creation_date.clone()),
            );
        }
        Some(review)
    } else {
        None
    }
}

fn merge_scan_results(opossums: &[Opossum]) -> ScanResults {
    let scan_results: Vec<_> = opossums.iter().map(|o| &o.scan_results).collect();
    let resources = merge_resources(&scan_results);
    let unassigned_attributions_raw = merge_unassigned_attributions(&scan_results);
    let unassigned_attributions =
        remove_assigned_attributions(&resources, &unassigned_attributions_raw);

    ScanResults {
        metadata: merge_metadata(&scan_results),
        resources,
        attribution_breakpoints: merge_attribution_breakpoints(&scan_results),
        external_attribution_sources: merge_external_attribution_sources(&scan_results),
        frequent_licenses: merge_frequent_licenses(&scan_results),
        config: merge_config(&scan_results),
        files_with_children: merge_files_with_children(&scan_results),
        base_urls_for_sources: merge_base_urls_for_sources(&scan_results),
        attribution_to_id: merge_attribution_to_id(&scan_results),
        unassigned_attributions,
    }
}

fn merge_metadata(scan_results: &[&ScanResults]) -> Metadata {
    let merged_titles: String = scan_results
        .iter()
        .map(|res| res.metadata.project_title.as_str())
        .collect::<Vec<_>>()
        .join(" | ");

    Metadata {
        project_id: Uuid::new_v4().to_string(),
        project_title: format!("Merged from: {}", merged_titles),
        file_creation_date: chrono::Utc::now().to_rfc3339(),
        project_version: None,
        expected_release_date: None,
        build_date: None,
        extra: BTreeMap::new(),
    }
}

fn merge_resources(scan_results: &[&ScanResults]) -> RootResource {
    let mut new_root = RootResource::new();
    for scan_result in scan_results {
        for resource in scan_result.resources.all_resources() {
            if let Err(e) = new_root.add_resource(resource.clone()) {
                tracing::warn!("Failed to add resource during merge: {}", e);
            }
        }
    }
    new_root
}

fn merge_unique_order_preserving<T: Clone + std::hash::Hash + Eq>(lists: &[Vec<T>]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for list in lists {
        for item in list {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
    }
    result
}

fn merge_attribution_breakpoints(scan_results: &[&ScanResults]) -> Vec<String> {
    merge_unique_order_preserving(
        &scan_results
            .iter()
            .map(|sr| sr.attribution_breakpoints.clone())
            .collect::<Vec<_>>(),
    )
}

fn merge_frequent_licenses(scan_results: &[&ScanResults]) -> Vec<FrequentLicense> {
    merge_unique_order_preserving(
        &scan_results
            .iter()
            .filter(|sr| !sr.frequent_licenses.is_empty())
            .map(|sr| sr.frequent_licenses.clone())
            .collect::<Vec<_>>(),
    )
}

fn merge_files_with_children(scan_results: &[&ScanResults]) -> Vec<String> {
    merge_unique_order_preserving(
        &scan_results
            .iter()
            .filter(|sr| !sr.files_with_children.is_empty())
            .map(|sr| sr.files_with_children.clone())
            .collect::<Vec<_>>(),
    )
}

fn merge_dict_warn_on_overwrite<K, V>(dicts: &[BTreeMap<K, V>], message: &str) -> BTreeMap<K, V>
where
    K: Clone + Ord + std::fmt::Debug,
    V: Clone + PartialEq + std::fmt::Debug,
{
    let mut merged: BTreeMap<K, V> = BTreeMap::new();
    for incoming in dicts {
        for (key, value) in incoming {
            if let Some(existing) = merged.get(key)
                && existing != value
            {
                tracing::warn!(
                    "{} Overwriting '{:?}' with '{:?}' for key '{:?}'",
                    message,
                    existing,
                    value,
                    key
                );
            }
            merged.insert(key.clone(), value.clone());
        }
    }
    merged
}

fn merge_base_urls_for_sources(scan_results: &[&ScanResults]) -> BaseUrlsForSources {
    let urls: Vec<_> = scan_results
        .iter()
        .map(|sr| sr.base_urls_for_sources.urls.clone())
        .collect();
    BaseUrlsForSources {
        urls: merge_dict_warn_on_overwrite(&urls, "[Merge base Urls for sources]"),
    }
}

fn merge_config(scan_results: &[&ScanResults]) -> Config {
    let configs: Vec<_> = scan_results
        .iter()
        .filter(|sr| sr.config != Config::default())
        .map(|sr| sr.config.clone())
        .collect();

    if configs.is_empty() {
        return Config::default();
    }

    let classifications: Vec<_> = configs.iter().map(|c| c.classifications.clone()).collect();
    let extra: Vec<_> = configs.iter().map(|c| c.extra.clone()).collect();

    Config {
        classifications: merge_dict_warn_on_overwrite(
            &classifications,
            "[Merge config.classifications]",
        ),
        extra: merge_dict_warn_on_overwrite(&extra, "[Merge config extras]"),
    }
}

fn merge_attribution_to_id(scan_results: &[&ScanResults]) -> BTreeMap<OpossumPackage, String> {
    let maps: Vec<_> = scan_results
        .iter()
        .map(|sr| sr.attribution_to_id.clone())
        .collect();
    merge_dict_warn_on_overwrite(&maps, "[Merge attribution to id]")
}

fn merge_external_attribution_sources(
    scan_results: &[&ScanResults],
) -> BTreeMap<String, ExternalAttributionSource> {
    let sources: Vec<_> = scan_results
        .iter()
        .map(|sr| sr.external_attribution_sources.clone())
        .collect();
    merge_dict_warn_on_overwrite(&sources, "[Merge external attribution sources]")
}

fn merge_unassigned_attributions(scan_results: &[&ScanResults]) -> HashSet<OpossumPackage> {
    let mut all_unassigned: HashSet<OpossumPackage> = HashSet::new();
    for scan_result in scan_results {
        all_unassigned.extend(scan_result.unassigned_attributions.iter().cloned());
    }
    all_unassigned
}

fn remove_assigned_attributions(
    resources: &RootResource,
    unassigned_attributions: &HashSet<OpossumPackage>,
) -> HashSet<OpossumPackage> {
    let mut all_attributions: HashSet<OpossumPackage> = HashSet::new();
    for resource in resources.all_resources() {
        all_attributions.extend(resource.attributions.iter().cloned());
    }

    unassigned_attributions
        .difference(&all_attributions)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::entities::{ExternalAttributionSource, FrequentLicense, Resource, ResourceType};

    fn create_test_opossum(title: &str) -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: title.to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let mut scan_results = ScanResults::new(metadata);
        let mut resource = Resource::new(std::path::PathBuf::from("test.txt"));
        resource.resource_type = Some(ResourceType::File);
        let _ = scan_results.resources.add_resource(resource);

        Opossum::new(scan_results)
    }

    fn create_test_opossum_with_breakpoints(title: &str, breakpoints: Vec<String>) -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: title.to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let mut scan_results = ScanResults::new(metadata);
        scan_results.attribution_breakpoints = breakpoints;
        let mut resource = Resource::new(std::path::PathBuf::from("test.txt"));
        resource.resource_type = Some(ResourceType::File);
        let _ = scan_results.resources.add_resource(resource);

        Opossum::new(scan_results)
    }

    fn create_test_opossum_with_sources(
        title: &str,
        sources: BTreeMap<String, ExternalAttributionSource>,
    ) -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: title.to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let mut scan_results = ScanResults::new(metadata);
        scan_results.external_attribution_sources = sources;
        let mut resource = Resource::new(std::path::PathBuf::from("test.txt"));
        resource.resource_type = Some(ResourceType::File);
        let _ = scan_results.resources.add_resource(resource);

        Opossum::new(scan_results)
    }

    fn create_test_opossum_with_licenses(title: &str, licenses: Vec<FrequentLicense>) -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: title.to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let mut scan_results = ScanResults::new(metadata);
        scan_results.frequent_licenses = licenses;
        let mut resource = Resource::new(std::path::PathBuf::from("test.txt"));
        resource.resource_type = Some(ResourceType::File);
        let _ = scan_results.resources.add_resource(resource);

        Opossum::new(scan_results)
    }

    fn create_test_opossum_with_files_with_children(title: &str, files: Vec<String>) -> Opossum {
        let metadata = Metadata {
            project_id: Uuid::new_v4().to_string(),
            project_title: title.to_string(),
            file_creation_date: chrono::Utc::now().to_rfc3339(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        };

        let mut scan_results = ScanResults::new(metadata);
        scan_results.files_with_children = files;
        let mut resource = Resource::new(std::path::PathBuf::from("test.txt"));
        resource.resource_type = Some(ResourceType::File);
        let _ = scan_results.resources.add_resource(resource);

        Opossum::new(scan_results)
    }

    #[test]
    fn test_merge_opossums_requires_at_least_two() {
        let opossum = create_test_opossum("test");
        let result = merge_opossums(vec![opossum]);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_opossums_errors_with_empty_list() {
        let result = merge_opossums(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_opossums_succeeds_with_two() {
        let opossum1 = create_test_opossum("project1");
        let opossum2 = create_test_opossum("project2");
        let result = merge_opossums(vec![opossum1, opossum2]);
        assert!(result.is_ok());
        let merged = result.unwrap();
        assert!(merged
            .scan_results
            .metadata
            .project_title
            .starts_with("Merged from:"));
    }

    #[test]
    fn test_merge_metadata() {
        let o1 = create_test_opossum("project1");
        let o2 = create_test_opossum("project2");
        let scan_results: Vec<_> = vec![&o1.scan_results, &o2.scan_results];
        let merged = merge_metadata(&scan_results);
        assert!(merged.project_title.contains("project1"));
        assert!(merged.project_title.contains("project2"));
        assert_ne!(merged.project_id, o1.scan_results.metadata.project_id);
    }

    #[test]
    fn test_merge_opossums_with_empty_review_results() {
        let opossum1 = create_test_opossum("project1");
        let opossum2 = create_test_opossum("project2");
        let result = merge_opossums(vec![opossum1, opossum2]);
        assert!(result.is_ok());
        let merged = result.unwrap();
        assert!(merged.review_results.is_none());
    }

    #[test]
    fn test_merge_opossums_with_single_review_results() {
        let mut opossum1 = create_test_opossum("project1");
        let review = serde_json::json!({
            "metadata": { "projectId": "test-project" },
            "manualAttributions": {},
            "resourcesToAttributions": {}
        });
        opossum1.review_results = Some(review);

        let opossum2 = create_test_opossum("project2");

        let result = merge_opossums(vec![opossum1, opossum2]);
        assert!(result.is_ok());
        let merged = result.unwrap();
        assert!(merged.review_results.is_some());
    }

    #[test]
    fn test_merge_combines_attribution_breakpoints_correctly() {
        let opossum1 = create_test_opossum_with_breakpoints(
            "project1",
            vec!["breakpoint1".to_string(), "breakpoint2".to_string()],
        );
        let opossum2 = create_test_opossum_with_breakpoints(
            "project2",
            vec!["breakpoint2".to_string(), "breakpoint3".to_string()],
        );

        let merged = merge_opossums(vec![opossum1, opossum2]).unwrap();

        let breakpoints: HashSet<_> = merged
            .scan_results
            .attribution_breakpoints
            .into_iter()
            .collect();
        assert!(breakpoints.contains("breakpoint1"));
        assert!(breakpoints.contains("breakpoint2"));
        assert!(breakpoints.contains("breakpoint3"));
    }

    #[test]
    fn test_merge_combines_external_attribution_sources_correctly() {
        let mut sources1 = BTreeMap::new();
        sources1.insert(
            "external1".to_string(),
            ExternalAttributionSource {
                name: "external1".to_string(),
                priority: 1,
                is_relevant_for_preferred: None,
            },
        );
        sources1.insert(
            "external2".to_string(),
            ExternalAttributionSource {
                name: "external2".to_string(),
                priority: 2,
                is_relevant_for_preferred: None,
            },
        );

        let mut sources2 = BTreeMap::new();
        sources2.insert(
            "external1".to_string(),
            ExternalAttributionSource {
                name: "external1".to_string(),
                priority: 3,
                is_relevant_for_preferred: None,
            },
        );
        sources2.insert(
            "external3".to_string(),
            ExternalAttributionSource {
                name: "external3".to_string(),
                priority: 3,
                is_relevant_for_preferred: None,
            },
        );

        let opossum1 = create_test_opossum_with_sources("project1", sources1);
        let opossum2 = create_test_opossum_with_sources("project2", sources2);

        let merged = merge_opossums(vec![opossum1, opossum2]).unwrap();

        assert!(merged
            .scan_results
            .external_attribution_sources
            .contains_key("external1"));
        assert!(merged
            .scan_results
            .external_attribution_sources
            .contains_key("external2"));
        assert!(merged
            .scan_results
            .external_attribution_sources
            .contains_key("external3"));
    }

    #[test]
    fn test_merge_combines_frequent_licenses_correctly() {
        let license1 = FrequentLicense {
            short_name: "MIT".to_string(),
            full_name: "MIT License".to_string(),
            default_text: "MIT text".to_string(),
        };
        let license2 = FrequentLicense {
            short_name: "Apache-2.0".to_string(),
            full_name: "Apache License 2.0".to_string(),
            default_text: "Apache text".to_string(),
        };
        let license3 = FrequentLicense {
            short_name: "GPL-3.0".to_string(),
            full_name: "GNU General Public License v3.0".to_string(),
            default_text: "GPL text".to_string(),
        };

        let opossum1 = create_test_opossum_with_licenses("project1", vec![license1, license2.clone()]);
        let opossum2 = create_test_opossum_with_licenses("project2", vec![license2, license3]);

        let merged = merge_opossums(vec![opossum1, opossum2]).unwrap();

        let license_names: HashSet<_> = merged
            .scan_results
            .frequent_licenses
            .iter()
            .map(|l| l.short_name.as_str())
            .collect();
        assert!(license_names.contains("MIT"));
        assert!(license_names.contains("Apache-2.0"));
        assert!(license_names.contains("GPL-3.0"));
    }

    #[test]
    fn test_merge_combines_files_with_children_correctly() {
        let opossum1 = create_test_opossum_with_files_with_children(
            "project1",
            vec!["path1/".to_string(), "path2/".to_string()],
        );
        let opossum2 = create_test_opossum_with_files_with_children(
            "project2",
            vec!["path2/".to_string(), "path3/".to_string()],
        );

        let merged = merge_opossums(vec![opossum1, opossum2]).unwrap();

        let paths: HashSet<_> = merged
            .scan_results
            .files_with_children
            .into_iter()
            .collect();
        assert!(paths.contains("path1/"));
        assert!(paths.contains("path2/"));
        assert!(paths.contains("path3/"));
    }

    #[test]
    fn test_merge_three_opossums() {
        let opossum1 = create_test_opossum("project1");
        let opossum2 = create_test_opossum("project2");
        let opossum3 = create_test_opossum("project3");

        let result = merge_opossums(vec![opossum1, opossum2, opossum3]);
        assert!(result.is_ok());
        let merged = result.unwrap();
        assert!(merged.scan_results.metadata.project_title.contains("project1"));
        assert!(merged.scan_results.metadata.project_title.contains("project2"));
        assert!(merged.scan_results.metadata.project_title.contains("project3"));
    }

    #[test]
    fn test_merge_generates_new_project_id() {
        let opossum1 = create_test_opossum("project1");
        let opossum2 = create_test_opossum("project2");

        let original_id = opossum1.scan_results.metadata.project_id.clone();
        let merged = merge_opossums(vec![opossum1, opossum2]).unwrap();

        assert_ne!(merged.scan_results.metadata.project_id, original_id);
    }
}
