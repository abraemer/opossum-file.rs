use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_confidence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
}

impl SourceInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            document_confidence: Some(0),
            additional_name: None,
        }
    }
}
