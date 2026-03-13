use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrequentLicense {
    pub full_name: String,
    pub short_name: String,
    pub default_text: String,
}
