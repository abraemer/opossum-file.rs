use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

use super::opossum_package::OpossumPackage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributions: Vec<OpossumPackage>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub children: BTreeMap<String, Resource>,
}

impl Resource {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            resource_type: None,
            attributions: Vec::new(),
            children: BTreeMap::new(),
        }
    }

    pub fn with_type(mut self, resource_type: ResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    pub fn with_attributions(mut self, attributions: Vec<OpossumPackage>) -> Self {
        self.attributions = attributions;
        self
    }

    pub fn add_resource(&mut self, resource: Resource) -> Result<(), String> {
        if !resource.path.starts_with(&self.path) {
            return Err(format!(
                "The path {:?} is not a child of this node at {:?}",
                resource.path, self.path
            ));
        }

        let remaining: PathBuf = resource
            .path
            .strip_prefix(&self.path)
            .map_err(|e| e.to_string())?
            .to_path_buf();

        self.add_resource_internal(resource, remaining);
        Ok(())
    }

    fn add_resource_internal(&mut self, resource: Resource, remaining: PathBuf) {
        let parts: Vec<&std::ffi::OsStr> = remaining.iter().collect();

        if parts.is_empty() {
            self.update(resource);
            return;
        }

        let next = parts[0].to_string_lossy().into_owned();
        let rest: PathBuf = parts[1..].iter().collect();

        if !self.children.contains_key(&next) {
            self.children
                .insert(next.clone(), Resource::new(self.path.join(&next)));
        }

        if let Some(child) = self.children.get_mut(&next) {
            child.add_resource_internal(resource, rest);
        }
    }

    fn update(&mut self, other: Resource) {
        if self.path != other.path {
            return;
        }

        if self.resource_type.is_none() {
            self.resource_type = other.resource_type;
        }

        self.attributions.extend(other.attributions);

        for (key, child) in other.children {
            if let Some(existing) = self.children.get_mut(&key) {
                existing.update(child);
            } else {
                self.children.insert(key, child);
            }
        }
    }
}
