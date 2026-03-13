use serde::{Deserialize, Serialize};

use super::source_info::SourceInfo;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpossumPackage {
    pub source: SourceInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution_confidence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_purl_appendix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_party: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_notice: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_selected: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_up: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criticality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was_preferred: Option<bool>,
}

impl OpossumPackage {
    pub fn new(source: SourceInfo) -> Self {
        Self {
            source,
            attribution_confidence: None,
            comment: None,
            package_name: None,
            package_version: None,
            package_namespace: None,
            package_type: None,
            package_purl_appendix: None,
            copyright: None,
            license_name: None,
            license_text: None,
            url: None,
            first_party: None,
            exclude_from_notice: None,
            pre_selected: None,
            follow_up: None,
            origin_id: None,
            origin_ids: None,
            criticality: None,
            classification: None,
            was_preferred: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpossumPackageBuilder {
    source: SourceInfo,
    attribution_confidence: Option<i32>,
    comment: Option<String>,
    package_name: Option<String>,
    package_version: Option<String>,
    package_namespace: Option<String>,
    package_type: Option<String>,
    package_purl_appendix: Option<String>,
    copyright: Option<String>,
    license_name: Option<String>,
    license_text: Option<String>,
    url: Option<String>,
    first_party: Option<bool>,
    exclude_from_notice: Option<bool>,
    pre_selected: Option<bool>,
    follow_up: Option<String>,
    origin_id: Option<String>,
    origin_ids: Option<Vec<String>>,
    criticality: Option<String>,
    classification: Option<i32>,
    was_preferred: Option<bool>,
}

impl OpossumPackageBuilder {
    pub fn new(source: SourceInfo) -> Self {
        Self {
            source,
            attribution_confidence: None,
            comment: None,
            package_name: None,
            package_version: None,
            package_namespace: None,
            package_type: None,
            package_purl_appendix: None,
            copyright: None,
            license_name: None,
            license_text: None,
            url: None,
            first_party: None,
            exclude_from_notice: None,
            pre_selected: None,
            follow_up: None,
            origin_id: None,
            origin_ids: None,
            criticality: None,
            classification: None,
            was_preferred: None,
        }
    }

    pub fn build(self) -> OpossumPackage {
        OpossumPackage {
            source: self.source,
            attribution_confidence: self.attribution_confidence,
            comment: self.comment,
            package_name: self.package_name,
            package_version: self.package_version,
            package_namespace: self.package_namespace,
            package_type: self.package_type,
            package_purl_appendix: self.package_purl_appendix,
            copyright: self.copyright,
            license_name: self.license_name,
            license_text: self.license_text,
            url: self.url,
            first_party: self.first_party,
            exclude_from_notice: self.exclude_from_notice,
            pre_selected: self.pre_selected,
            follow_up: self.follow_up,
            origin_id: self.origin_id,
            origin_ids: self.origin_ids,
            criticality: self.criticality,
            classification: self.classification,
            was_preferred: self.was_preferred,
        }
    }

    pub fn attribution_confidence(mut self, value: i32) -> Self {
        self.attribution_confidence = Some(value);
        self
    }

    pub fn comment(mut self, value: String) -> Self {
        self.comment = Some(value);
        self
    }

    pub fn package_name(mut self, value: String) -> Self {
        self.package_name = Some(value);
        self
    }

    pub fn package_version(mut self, value: String) -> Self {
        self.package_version = Some(value);
        self
    }

    pub fn package_namespace(mut self, value: String) -> Self {
        self.package_namespace = Some(value);
        self
    }

    pub fn package_type(mut self, value: String) -> Self {
        self.package_type = Some(value);
        self
    }

    pub fn package_purl_appendix(mut self, value: String) -> Self {
        self.package_purl_appendix = Some(value);
        self
    }

    pub fn copyright(mut self, value: String) -> Self {
        self.copyright = Some(value);
        self
    }

    pub fn license_name(mut self, value: String) -> Self {
        self.license_name = Some(value);
        self
    }

    pub fn license_text(mut self, value: String) -> Self {
        self.license_text = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }

    pub fn first_party(mut self, value: bool) -> Self {
        self.first_party = Some(value);
        self
    }

    pub fn exclude_from_notice(mut self, value: bool) -> Self {
        self.exclude_from_notice = Some(value);
        self
    }

    pub fn pre_selected(mut self, value: bool) -> Self {
        self.pre_selected = Some(value);
        self
    }

    pub fn follow_up(mut self, value: String) -> Self {
        self.follow_up = Some(value);
        self
    }

    pub fn origin_id(mut self, value: String) -> Self {
        self.origin_id = Some(value);
        self
    }

    pub fn origin_ids(mut self, value: Vec<String>) -> Self {
        self.origin_ids = Some(value);
        self
    }

    pub fn criticality(mut self, value: String) -> Self {
        self.criticality = Some(value);
        self
    }

    pub fn classification(mut self, value: i32) -> Self {
        self.classification = Some(value);
        self
    }

    pub fn was_preferred(mut self, value: bool) -> Self {
        self.was_preferred = Some(value);
        self
    }
}
