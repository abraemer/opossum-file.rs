use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

use purl::PackageURL;

use crate::core::entities::{
    ExternalAttributionSource, Metadata, Opossum, OpossumPackage, OpossumPackageBuilder, Resource,
    ResourceType, RootResource, ScanResults, SourceInfo,
};
use crate::core::services::InputReader;
use crate::error::OpossumError;

use super::entities::{
    DependencyModel, FileModel, FileTypeModel, HeaderModel, LicenseReferenceModel, MatchModel,
    PackageDataModel, ScancodeModel,
};

const SCANCODE_SOURCE_NAME: &str = "SC";
const SCANCODE_SOURCE_NAME_PACKAGE: &str = "SC-P";
const SCANCODE_SOURCE_NAME_DEPENDENCY: &str = "SC-D";
const SCANCODE_PRIORITY: i32 = 50;
const SCANCODE_PACKAGE_PRIORITY: i32 = 30;
const SCANCODE_DEPENDENCY_PRIORITY: i32 = 40;

pub struct ScanCodeFileReader {
    content: String,
}

impl ScanCodeFileReader {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn from_file(path: &std::path::Path) -> Result<Self, OpossumError> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::new(content))
    }
}

impl InputReader for ScanCodeFileReader {
    fn read(&self) -> Result<Opossum, OpossumError> {
        let scancode_data: ScancodeModel = serde_json::from_str(&self.content)?;
        convert_to_opossum(scancode_data).map_err(OpossumError::ParseError)
    }
}

fn convert_to_opossum(scancode_data: ScancodeModel) -> Result<Opossum, String> {
    let scancode_header = extract_scancode_header(&scancode_data)?;

    Ok(Opossum::new(ScanResults {
        metadata: generate_metadata(scancode_header),
        resources: extract_opossum_resources(&scancode_data)?,
        external_attribution_sources: get_external_attribution_sources(),
        ..ScanResults::new(Metadata {
            project_id: String::new(),
            file_creation_date: String::new(),
            project_title: String::new(),
            project_version: None,
            expected_release_date: None,
            build_date: None,
            extra: BTreeMap::new(),
        })
    }))
}

fn get_external_attribution_sources() -> BTreeMap<String, ExternalAttributionSource> {
    let mut sources = BTreeMap::new();
    sources.insert(
        SCANCODE_SOURCE_NAME.to_string(),
        ExternalAttributionSource {
            name: "ScanCode".to_string(),
            priority: SCANCODE_PRIORITY,
            is_relevant_for_preferred: None,
        },
    );
    sources.insert(
        SCANCODE_SOURCE_NAME_PACKAGE.to_string(),
        ExternalAttributionSource {
            name: "ScanCode Package".to_string(),
            priority: SCANCODE_PACKAGE_PRIORITY,
            is_relevant_for_preferred: None,
        },
    );
    sources.insert(
        SCANCODE_SOURCE_NAME_DEPENDENCY.to_string(),
        ExternalAttributionSource {
            name: "ScanCode Dependency".to_string(),
            priority: SCANCODE_DEPENDENCY_PRIORITY,
            is_relevant_for_preferred: None,
        },
    );
    sources
}

fn generate_metadata(scancode_header: &HeaderModel) -> Metadata {
    Metadata {
        project_id: uuid::Uuid::new_v4().to_string(),
        file_creation_date: scancode_header.end_timestamp.clone(),
        project_title: "ScanCode file".to_string(),
        build_date: Some(chrono::Utc::now().to_rfc3339()),
        project_version: None,
        expected_release_date: None,
        extra: BTreeMap::new(),
    }
}

fn extract_scancode_header(scancode_data: &ScancodeModel) -> Result<&HeaderModel, String> {
    if scancode_data.headers.len() != 1 {
        return Err("Headers of ScanCode file are invalid.".to_string());
    }
    Ok(&scancode_data.headers[0])
}

fn extract_opossum_resources(scancode_data: &ScancodeModel) -> Result<RootResource, String> {
    let license_references: BTreeMap<String, &LicenseReferenceModel> = scancode_data
        .license_references
        .as_ref()
        .map(|refs| {
            refs.iter()
                .map(|r| (r.spdx_license_key.clone(), r))
                .collect()
        })
        .unwrap_or_default();

    let mut resources = RootResource::new();

    for file in &scancode_data.files {
        let resource = Resource::new(PathBuf::from(&file.path))
            .with_type(convert_resource_type(file.file_type))
            .with_attributions(get_attribution_info(file, &license_references)?);
        resources.add_resource(resource)?;
    }

    Ok(resources)
}

fn convert_resource_type(file_type: FileTypeModel) -> ResourceType {
    match file_type {
        FileTypeModel::File => ResourceType::File,
        FileTypeModel::Directory => ResourceType::Directory,
    }
}

fn get_attribution_info(
    file: &FileModel,
    license_references: &BTreeMap<String, &LicenseReferenceModel>,
) -> Result<Vec<OpossumPackage>, String> {
    let mut attribution_infos =
        create_attributions_from_license_detections(file, license_references)?;

    if let Some(package_data) = &file.package_data {
        for package in package_data {
            let package_attribution = create_package_attribution(package, license_references)?;
            let parent_name = package_attribution.package_name.clone();
            attribution_infos.push(package_attribution);

            if let Some(dependencies) = &package.dependencies {
                for dependency in dependencies {
                    let dependency_attribution =
                        create_dependency_attribution(dependency, parent_name.as_deref())?;
                    attribution_infos.push(dependency_attribution);
                }
            }
        }
    }

    Ok(attribution_infos)
}

fn create_attributions_from_license_detections(
    file: &FileModel,
    license_references: &BTreeMap<String, &LicenseReferenceModel>,
) -> Result<Vec<OpossumPackage>, String> {
    let purl_data = file
        .for_packages
        .as_ref()
        .and_then(|pkgs| pkgs.first())
        .map(|p| extract_purl_data(Some(p)));

    let copyright = extract_copyrights(file);
    let comment = create_base_comment(file);

    let license_detections = file.license_detections.as_ref();

    let has_purl_data = purl_data.as_ref().is_some_and(|d| d.is_some());

    if license_detections.is_none_or(|d| d.is_empty())
        && (!copyright.is_empty() || has_purl_data || !comment.is_empty())
    {
        let source_info = SourceInfo {
            name: SCANCODE_SOURCE_NAME.to_string(),
            document_confidence: Some(50),
            additional_name: None,
        };
        let full_comment = format!("{}\nNo license information.", comment);

        let mut builder = OpossumPackageBuilder::new(source_info)
            .copyright(copyright)
            .comment(full_comment);

        if let Some(data) = purl_data {
            builder = apply_purl_data(builder, &data);
        }

        return Ok(vec![builder.build()]);
    }

    let mut attribution_infos = Vec::new();

    if let Some(detections) = license_detections {
        for license_detection in detections {
            let license_name = &license_detection.license_expression_spdx;

            let matches = if !license_detection.matches.is_empty() {
                &license_detection.matches
            } else if let Some(ref_matches) = &license_detection.reference_matches {
                ref_matches
            } else {
                &license_detection.matches
            };

            let max_score = matches.iter().map(|m| m.score).fold(0.0_f64, f64::max);

            let source_info = SourceInfo {
                name: SCANCODE_SOURCE_NAME.to_string(),
                document_confidence: Some(max_score as i32),
                additional_name: None,
            };

            let reference = license_references.get(license_name);
            let text = reference.and_then(|r| r.text.clone());

            let license_data: Vec<String> = matches.iter().map(format_license_match).collect();
            let license_comment = format!("Detected License(s):\n{}", license_data.join("\n"));
            let full_comment = format!("{}\n{}", comment, license_comment);

            let mut builder = OpossumPackageBuilder::new(source_info)
                .license_name(license_name.clone())
                .attribution_confidence(max_score as i32)
                .copyright(copyright.clone())
                .comment(full_comment);

            if let Some(t) = text {
                builder = builder.license_text(t);
            }

            if let Some(ref data) = purl_data {
                builder = apply_purl_data(builder, data);
            }

            attribution_infos.push(builder.build());
        }
    }

    Ok(attribution_infos)
}

fn apply_purl_data(mut builder: OpossumPackageBuilder, data: &PurlData) -> OpossumPackageBuilder {
    if let Some(name) = &data.package_name {
        builder = builder.package_name(name.clone());
    }
    if let Some(version) = &data.package_version {
        builder = builder.package_version(version.clone());
    }
    if let Some(namespace) = &data.package_namespace {
        builder = builder.package_namespace(namespace.clone());
    }
    if let Some(ptype) = &data.package_type {
        builder = builder.package_type(ptype.clone());
    }
    if let Some(appendix) = &data.package_purl_appendix {
        builder = builder.package_purl_appendix(appendix.clone());
    }
    builder
}

fn format_license_match(match_model: &MatchModel) -> String {
    let start_line = match_model.start_line;
    let end_line = match_model.end_line;
    let line_str = if start_line == end_line {
        format!("line {}", start_line)
    } else {
        format!("lines {}-{}", start_line, end_line)
    };
    let license = &match_model.license_expression_spdx;
    let additional_information = match_model
        .matched_text
        .as_ref()
        .map(|t| format!(":\n{}", t))
        .unwrap_or_default();
    format!(
        "Matched {} in {}{}",
        license, line_str, additional_information
    )
}

#[derive(Debug, Clone, Default)]
struct PurlData {
    package_name: Option<String>,
    package_version: Option<String>,
    package_namespace: Option<String>,
    package_type: Option<String>,
    package_purl_appendix: Option<String>,
}

impl PurlData {
    fn is_some(&self) -> bool {
        self.package_name.is_some()
            || self.package_version.is_some()
            || self.package_namespace.is_some()
            || self.package_type.is_some()
            || self.package_purl_appendix.is_some()
    }
}

fn extract_purl_data(purl_str: Option<&String>) -> PurlData {
    let purl_str = match purl_str {
        Some(s) => s,
        None => return PurlData::default(),
    };

    let purl = match PackageURL::from_str(purl_str.as_str()) {
        Ok(p) => p,
        Err(_) => return PurlData::default(),
    };

    let mut data = PurlData::default();

    let qualifiers_str = if purl.qualifiers.is_empty() {
        String::new()
    } else {
        let pairs: Vec<String> = purl
            .qualifiers
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        pairs.join("&")
    };

    let mut parts = Vec::new();
    if !qualifiers_str.is_empty() {
        parts.push(qualifiers_str);
    }
    if let Some(subpath) = &purl.subpath
        && !subpath.is_empty()
    {
        parts.push(subpath.clone());
    }

    data.package_name = Some(purl.name.clone());
    data.package_version = purl.version.clone();
    data.package_namespace = purl.namespace.clone();
    data.package_type = Some(purl.r#type.clone());
    data.package_purl_appendix = if parts.is_empty() {
        None
    } else {
        Some(parts.join("#"))
    };

    data
}

fn create_package_attribution(
    package: &PackageDataModel,
    license_references: &BTreeMap<String, &LicenseReferenceModel>,
) -> Result<OpossumPackage, String> {
    let mut purl_data = extract_purl_data(package.purl.as_ref());

    if purl_data.package_name.is_none() {
        purl_data.package_name = package.name.clone();
    }
    if purl_data.package_type.is_none() {
        purl_data.package_type = package.package_type.clone();
    }
    if purl_data.package_namespace.is_none() {
        purl_data.package_namespace = package.namespace.clone();
    }
    if purl_data.package_version.is_none() {
        purl_data.package_version = package.version.clone();
    }

    let url = package
        .homepage_url
        .as_ref()
        .or(package.repository_homepage_url.as_ref())
        .or(package.download_url.as_ref())
        .or(package.code_view_url.as_ref())
        .or(package.vcs_url.as_ref())
        .cloned();

    let attribution_confidence = package.license_detections.as_ref().and_then(|detections| {
        let all_scores: Vec<f64> = detections
            .iter()
            .flat_map(|d| {
                let matches = if !d.matches.is_empty() {
                    &d.matches
                } else if let Some(ref_matches) = &d.reference_matches {
                    ref_matches
                } else {
                    &d.matches
                };
                matches.iter().map(|m| m.score)
            })
            .collect();
        all_scores.iter().copied().fold(None, |max, score| {
            Some(max.map_or(score, |m: f64| m.max(score)))
        })
    });

    let license_name = package
        .declared_license_expression_spdx
        .as_ref()
        .or(package.other_license_expression_spdx.as_ref())
        .cloned();

    let text = license_name
        .as_ref()
        .and_then(|ln| license_references.get(ln).and_then(|r| r.text.clone()));

    let mut comment_parts = vec![
        "== ScanCode ==".to_string(),
        "Created from package detection".to_string(),
    ];
    if let Some(ptype) = &package.package_type {
        comment_parts.push(format!("Type: {}", ptype));
    }
    if let Some(desc) = &package.description {
        comment_parts.push(format!("Description:\n{}", desc));
    }
    if let Some(notice) = &package.notice_text {
        comment_parts.push(format!("Notice:\n{}", notice));
    }

    let copyright = package
        .copyright
        .as_ref()
        .or(package.holder.as_ref())
        .cloned();

    let mut builder = OpossumPackageBuilder::new(SourceInfo {
        name: SCANCODE_SOURCE_NAME_PACKAGE.to_string(),
        document_confidence: None,
        additional_name: None,
    })
    .comment(comment_parts.join("\n"))
    .copyright(copyright.unwrap_or_default())
    .license_name(license_name.unwrap_or_default());

    if let Some(conf) = attribution_confidence {
        builder = builder.attribution_confidence(conf as i32);
    }
    if let Some(t) = text {
        builder = builder.license_text(t);
    }
    if let Some(u) = url {
        builder = builder.url(u);
    }
    builder = apply_purl_data(builder, &purl_data);

    Ok(builder.build())
}

fn create_dependency_attribution(
    dependency: &DependencyModel,
    parent: Option<&str>,
) -> Result<OpossumPackage, String> {
    let purl_data = extract_purl_data(dependency.purl.as_ref());

    let mut comment_parts = vec!["== ScanCode ==".to_string()];
    if let Some(p) = parent {
        comment_parts.push(format!("Dependency of {}", p));
    } else {
        comment_parts.push("Detected as dependency".to_string());
    }
    if let Some(scope) = &dependency.scope {
        comment_parts.push(format!("Scope: {}", scope));
    }

    let mut builder = OpossumPackageBuilder::new(SourceInfo {
        name: SCANCODE_SOURCE_NAME_DEPENDENCY.to_string(),
        document_confidence: Some(50),
        additional_name: None,
    })
    .comment(comment_parts.join("\n"));

    builder = apply_purl_data(builder, &purl_data);

    Ok(builder.build())
}

fn extract_copyrights(file: &FileModel) -> String {
    file.copyrights
        .as_ref()
        .map(|c| {
            c.iter()
                .map(|cr| cr.copyright.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn create_base_comment(file: &FileModel) -> String {
    let mut parts = vec!["== ScanCode ==".to_string()];

    if file.size.unwrap_or(0) == 0 {
        parts.push("File is empty.".to_string());
    }
    if file.is_binary {
        parts.push("File is binary.".to_string());
    }
    if file.is_archive {
        parts.push("File is an archive.".to_string());
    }
    if let Some(urls) = &file.urls
        && !urls.is_empty()
    {
        let url_data: Vec<String> = urls
            .iter()
            .map(|url| format!("Line {}: {}", url.start_line.unwrap_or(0), url.url))
            .collect();
        parts.push(format!("URLs:\n{}", url_data.join("\n")));
    }

    parts.join("\n")
}
