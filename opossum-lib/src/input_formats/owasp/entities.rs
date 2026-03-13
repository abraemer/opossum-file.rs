use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwaspDependencyReportModel {
    #[serde(rename = "reportSchema")]
    pub report_schema: String,
    #[serde(rename = "scanInfo")]
    pub scan_info: ScanInfoModel,
    #[serde(rename = "projectInfo")]
    pub project_info: ProjectInfoModel,
    pub dependencies: Vec<DependencyModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInfoModel {
    #[serde(rename = "engineVersion")]
    pub engine_version: String,
    #[serde(rename = "dataSource")]
    pub data_source: Vec<DataSourceModel>,
    #[serde(default, rename = "analysisExceptions")]
    pub analysis_exceptions: Option<Vec<AnalysisExceptionModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisExceptionModel {
    pub exception: ExceptionModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionModel {
    pub message: String,
    #[serde(default)]
    pub stack_trace: Option<Vec<String>>,
    pub cause: Option<Box<ExceptionModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceModel {
    pub name: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfoModel {
    pub name: String,
    #[serde(rename = "reportDate")]
    pub report_date: String,
    #[serde(default, rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(default, rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(default, rename = "applicationVersion")]
    pub application_version: Option<String>,
    #[serde(default)]
    pub credits: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyModel {
    #[serde(rename = "isVirtual")]
    pub is_virtual: bool,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default, rename = "projectReferences")]
    pub project_references: Option<Vec<String>>,
    #[serde(default, rename = "includedBy")]
    pub included_by: Option<Vec<IncludedByModel>>,
    #[serde(default, rename = "relatedDependencies")]
    pub related_dependencies: Option<Vec<RelatedDependencyModel>>,
    #[serde(rename = "evidenceCollected")]
    pub evidence_collected: EvidenceCollectedModel,
    #[serde(default)]
    pub packages: Option<Vec<PackageModel>>,
    #[serde(default, rename = "vulnerabilityIds")]
    pub vulnerability_ids: Option<Vec<VulnerabilityIdModel>>,
    #[serde(default, rename = "suppressedVulnerabilityIds")]
    pub suppressed_vulnerability_ids: Option<Vec<VulnerabilityIdModel>>,
    #[serde(default)]
    pub vulnerabilities: Option<Vec<VulnerabilityModel>>,
    #[serde(default, rename = "suppressedVulnerabilities")]
    pub suppressed_vulnerabilities: Option<Vec<VulnerabilityModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssV2Model {
    pub score: f64,
    #[serde(rename = "accessVector")]
    pub access_vector: String,
    #[serde(rename = "accessComplexity")]
    pub access_complexity: String,
    pub authenticationr: String,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: String,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: String,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: String,
    pub severity: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, rename = "exploitabilityScore")]
    pub exploitability_score: Option<String>,
    #[serde(default, rename = "impactScore")]
    pub impact_score: Option<String>,
    #[serde(default, rename = "acInsufInfo")]
    pub ac_insuf_info: Option<String>,
    #[serde(default, rename = "obtainAllPrivilege")]
    pub obtain_all_privilege: Option<String>,
    #[serde(default, rename = "obtainUserPrivilege")]
    pub obtain_user_privilege: Option<String>,
    #[serde(default, rename = "obtainOtherPrivilege")]
    pub obtain_other_privilege: Option<String>,
    #[serde(default, rename = "userInteractionRequired")]
    pub user_interaction_required: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssV3Model {
    #[serde(rename = "baseScore")]
    pub base_score: f64,
    #[serde(rename = "attackVector")]
    pub attack_vector: String,
    #[serde(rename = "attackComplexity")]
    pub attack_complexity: String,
    #[serde(rename = "privilegesRequired")]
    pub privileges_required: String,
    #[serde(rename = "userInteraction")]
    pub user_interaction: String,
    pub scope: String,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: String,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: String,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: String,
    #[serde(rename = "baseSeverity")]
    pub base_severity: String,
    #[serde(default, rename = "exploitabilityScore")]
    pub exploitability_score: Option<String>,
    #[serde(default, rename = "impactScore")]
    pub impact_score: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssV4Model {
    #[serde(default, rename = "vectorString")]
    pub vector_string: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default, rename = "attackVector")]
    pub attack_vector: Option<String>,
    #[serde(default, rename = "attackComplexity")]
    pub attack_complexity: Option<String>,
    #[serde(default, rename = "attackRequirements")]
    pub attack_requirements: Option<String>,
    #[serde(default, rename = "privilegesRequired")]
    pub privileges_required: Option<String>,
    #[serde(default, rename = "userInteraction")]
    pub user_interaction: Option<String>,
    #[serde(default, rename = "vulnerableSystemConfidentiality")]
    pub vulnerable_system_confidentiality: Option<String>,
    #[serde(default, rename = "vulnerableSystemIntegrity")]
    pub vulnerable_system_integrity: Option<String>,
    #[serde(default, rename = "vulnerableSystemAvailability")]
    pub vulnerable_system_availability: Option<String>,
    #[serde(default, rename = "subsequentSystemConfidentiality")]
    pub subsequent_system_confidentiality: Option<String>,
    #[serde(default, rename = "subsequentSystemIntegrity")]
    pub subsequent_system_integrity: Option<String>,
    #[serde(default, rename = "subsequentSystemAvailability")]
    pub subsequent_system_availability: Option<String>,
    #[serde(default, rename = "exploitMaturity")]
    pub exploit_maturity: Option<String>,
    #[serde(default, rename = "confidentialityRequirements")]
    pub confidentiality_requirements: Option<String>,
    #[serde(default, rename = "integrityRequirements")]
    pub integrity_requirements: Option<String>,
    #[serde(default, rename = "availabilityRequirements")]
    pub availability_requirements: Option<String>,
    #[serde(default, rename = "modifiedAttackVector")]
    pub modified_attack_vector: Option<String>,
    #[serde(default, rename = "modifiedAttackComplexity")]
    pub modified_attack_complexity: Option<String>,
    #[serde(default, rename = "modifiedAttackRequirements")]
    pub modified_attack_requirements: Option<String>,
    #[serde(default, rename = "modifiedPrivilegesRequired")]
    pub modified_privileges_required: Option<String>,
    #[serde(default, rename = "modifiedUserInteraction")]
    pub modified_user_interaction: Option<String>,
    #[serde(default, rename = "modifiedVulnerableSystemConfidentiality")]
    pub modified_vulnerable_system_confidentiality: Option<String>,
    #[serde(default, rename = "modifiedVulnerableSystemIntegrity")]
    pub modified_vulnerable_system_integrity: Option<String>,
    #[serde(default, rename = "modifiedVulnerableSystemAvailability")]
    pub modified_vulnerable_system_availability: Option<String>,
    #[serde(default, rename = "modifiedSubsequentSystemConfidentiality")]
    pub modified_subsequent_system_confidentiality: Option<String>,
    #[serde(default, rename = "modifiedSubsequentSystemIntegrity")]
    pub modified_subsequent_system_integrity: Option<String>,
    #[serde(default, rename = "modifiedSubsequentSystemAvailability")]
    pub modified_subsequent_system_availability: Option<String>,
    #[serde(default)]
    pub safety: Option<String>,
    #[serde(default)]
    pub automatable: Option<String>,
    #[serde(default)]
    pub recovery: Option<String>,
    #[serde(default, rename = "valueDensity")]
    pub value_density: Option<String>,
    #[serde(default, rename = "vulnerabilityResponseEffort")]
    pub vulnerability_response_effort: Option<String>,
    #[serde(default, rename = "providerUrgency")]
    pub provider_urgency: Option<String>,
    #[serde(default, rename = "baseScore")]
    pub base_score: Option<f64>,
    #[serde(default, rename = "baseSeverity")]
    pub base_severity: Option<String>,
    #[serde(default, rename = "threatScore")]
    pub threat_score: Option<f64>,
    #[serde(default, rename = "threatSeverity")]
    pub threat_severity: Option<String>,
    #[serde(default, rename = "environmentalScore")]
    pub environmental_score: Option<f64>,
    #[serde(default, rename = "environmentalSeverity")]
    pub environmental_severity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityModel {
    pub source: String,
    pub name: String,
    #[serde(default)]
    pub cvssv2: Option<CvssV2Model>,
    #[serde(default)]
    pub cvssv3: Option<CvssV3Model>,
    #[serde(default)]
    pub cvssv4: Option<CvssV4Model>,
    #[serde(default)]
    pub cwes: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub references: Option<Vec<ReferenceModel>>,
    #[serde(default, rename = "vulnerableSoftware")]
    pub vulnerable_software: Option<Vec<VulnerableSoftwareModel>>,
    #[serde(
        default,
        rename = "unscored",
        deserialize_with = "deserialize_string_or_bool"
    )]
    pub unscored: Option<bool>,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default, rename = "knownExploitedVulnerability")]
    pub known_exploited_vulnerability: Option<KnownExploitedVulnerabilityModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerableSoftwareModel {
    pub software: SoftwareModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareModel {
    pub id: String,
    #[serde(
        default,
        rename = "vulnerabilityIdMatched",
        deserialize_with = "deserialize_string_or_bool"
    )]
    pub vulnerability_id_matched: Option<bool>,
    #[serde(default, rename = "versionStartIncluding")]
    pub version_start_including: Option<String>,
    #[serde(default, rename = "versionStartExcluding")]
    pub version_start_excluding: Option<String>,
    #[serde(default, rename = "versionEndIncluding")]
    pub version_end_including: Option<String>,
    #[serde(default, rename = "versionEndExcluding")]
    pub version_end_excluding: Option<String>,
    #[serde(default)]
    pub vulnerable: Option<String>,
}

fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use std::str::FromStr;

    let opt: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(serde_json::Value::Bool(b)) => Ok(Some(b)),
        Some(serde_json::Value::String(s)) => {
            bool::from_str(&s).map(Some).map_err(D::Error::custom)
        }
        Some(other) => Err(D::Error::custom(format!(
            "expected bool or string, got {:?}",
            other
        ))),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceModel {
    pub source: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownExploitedVulnerabilityModel {
    #[serde(default, rename = "VendorProject")]
    pub vendor_project: Option<String>,
    #[serde(default, rename = "Product")]
    pub product: Option<String>,
    #[serde(default, rename = "Name")]
    pub name: Option<String>,
    #[serde(default, rename = "DateAdded")]
    pub date_added: Option<String>,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "RequiredAction")]
    pub required_action: Option<String>,
    #[serde(default, rename = "DueDate")]
    pub due_date: Option<String>,
    #[serde(default, rename = "Notes")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityIdModel {
    pub id: String,
    #[serde(default)]
    pub confidence: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludedByModel {
    pub reference: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedDependencyModel {
    #[serde(rename = "isVirtual")]
    pub is_virtual: bool,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(default)]
    pub md5: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default, rename = "packageIds")]
    pub package_ids: Option<Vec<PackageIdModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageIdModel {
    pub id: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCollectedModel {
    #[serde(default, rename = "productEvidence")]
    pub product_evidence: Vec<EvidenceModel>,
    #[serde(default, rename = "versionEvidence")]
    pub version_evidence: Vec<EvidenceModel>,
    #[serde(default, rename = "vendorEvidence")]
    pub vendor_evidence: Vec<EvidenceModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceModel {
    #[serde(rename = "type")]
    pub type_: String,
    pub confidence: String,
    pub source: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageModel {
    pub id: String,
    #[serde(default)]
    pub confidence: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}
