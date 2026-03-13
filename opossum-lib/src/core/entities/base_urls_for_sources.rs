use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseUrlsForSources {
    #[serde(flatten)]
    pub urls: BTreeMap<String, String>,
}
