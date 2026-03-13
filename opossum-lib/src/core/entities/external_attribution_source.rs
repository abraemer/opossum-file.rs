use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAttributionSource {
    pub name: String,
    pub priority: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_relevant_for_preferred: Option<bool>,
}
