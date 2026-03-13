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
    use crate::core::entities::{Resource, ResourceType};

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

    #[test]
    fn test_merge_opossums_requires_at_least_two() {
        let opossum = create_test_opossum("test");
        let result = merge_opossums(vec![opossum]);
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
}
