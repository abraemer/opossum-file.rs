use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub project_id: String,
    pub file_creation_date: String,
    pub project_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_release_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
