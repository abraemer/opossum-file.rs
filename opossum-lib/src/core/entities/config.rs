use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub classifications: BTreeMap<i32, String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
