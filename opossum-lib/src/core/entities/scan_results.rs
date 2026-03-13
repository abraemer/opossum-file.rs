use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

use super::base_urls_for_sources::BaseUrlsForSources;
use super::config::Config;
use super::external_attribution_source::ExternalAttributionSource;
use super::frequent_license::FrequentLicense;
use super::metadata::Metadata;
use super::opossum_package::OpossumPackage;
use super::root_resource::RootResource;

pub type OpossumPackageIdentifier = String;
pub type ResourcePath = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResults {
    pub metadata: Metadata,
    #[serde(default)]
    pub resources: RootResource,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribution_breakpoints: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub external_attribution_sources: BTreeMap<String, ExternalAttributionSource>,
    #[serde(default)]
    pub config: Config,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frequent_licenses: Vec<FrequentLicense>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files_with_children: Vec<String>,
    #[serde(default)]
    pub base_urls_for_sources: BaseUrlsForSources,
    #[serde(skip)]
    pub attribution_to_id: BTreeMap<OpossumPackage, String>,
    #[serde(skip)]
    pub unassigned_attributions: HashSet<OpossumPackage>,
}

impl ScanResults {
    pub fn new(metadata: Metadata) -> Self {
        Self {
            metadata,
            resources: RootResource::new(),
            attribution_breakpoints: Vec::new(),
            external_attribution_sources: BTreeMap::new(),
            config: Config::default(),
            frequent_licenses: Vec::new(),
            files_with_children: Vec::new(),
            base_urls_for_sources: BaseUrlsForSources::default(),
            attribution_to_id: BTreeMap::new(),
            unassigned_attributions: HashSet::new(),
        }
    }

    pub fn get_or_create_attribution_id(&mut self, attribution: &OpossumPackage) -> String {
        if let Some(id) = self.attribution_to_id.get(attribution) {
            id.clone()
        } else {
            let id = Uuid::new_v4().to_string();
            self.attribution_to_id
                .insert(attribution.clone(), id.clone());
            id
        }
    }
}
