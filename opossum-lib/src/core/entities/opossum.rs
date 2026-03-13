use serde::{Deserialize, Serialize};

use super::scan_results::ScanResults;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opossum {
    pub scan_results: ScanResults,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_results: Option<serde_json::Value>,
}

impl Opossum {
    pub fn new(scan_results: ScanResults) -> Self {
        Self {
            scan_results,
            review_results: None,
        }
    }
}
