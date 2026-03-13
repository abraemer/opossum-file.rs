use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RootResource {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub children: BTreeMap<String, Resource>,
}

impl RootResource {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
        }
    }

    pub fn add_resource(&mut self, mut resource: Resource) -> Result<(), String> {
        if resource.path.is_absolute() {
            let parts: Vec<_> = resource.path.iter().skip(1).collect();
            resource.path = parts.iter().collect();
        }

        let parts: Vec<_> = resource.path.iter().collect();
        if parts.is_empty() {
            return Err(format!(
                "Every resource needs a filepath. Got: {:?}",
                resource
            ));
        }

        let first = parts[0].to_string_lossy().into_owned();

        if !self.children.contains_key(&first) {
            self.children.insert(
                first.clone(),
                Resource::new(std::path::PathBuf::from(&first)),
            );
        }

        if let Some(child) = self.children.get_mut(&first) {
            child.add_resource(resource)?;
        }

        Ok(())
    }

    pub fn all_resources(&self) -> impl Iterator<Item = &Resource> {
        AllResourcesIter::new(self)
    }
}

struct AllResourcesIter<'a> {
    stack: Vec<&'a Resource>,
}

impl<'a> AllResourcesIter<'a> {
    fn new(root: &'a RootResource) -> Self {
        Self {
            stack: root.children.values().collect(),
        }
    }
}

impl<'a> Iterator for AllResourcesIter<'a> {
    type Item = &'a Resource;

    fn next(&mut self) -> Option<Self::Item> {
        let resource = self.stack.pop()?;
        self.stack.extend(resource.children.values());
        Some(resource)
    }
}
