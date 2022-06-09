use super::PBXObject;
use std::collections::HashMap;

/// [`PBXObject`] storage with convenient helper methods
#[derive(Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXObjectCollection(HashMap<String, PBXObject>);

impl PBXObjectCollection {
    /// Add new object. same as insert but it auto create id and returns it
    pub fn push<O: Into<PBXObject>>(&mut self, object: O) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.insert(id.clone(), object.into());
        id
    }

    /// Get PBXTarget by the target name
    pub fn get_target_by_name(&self, target_name: &str) -> Option<(&String, &PBXObject)> {
        self.0.iter().find(|(_, o)| {
            let target = match o {
                PBXObject::PBXAggregateTarget(ref v) => &v.inner,
                PBXObject::PBXLegacyTarget(ref v) => &v.inner,
                PBXObject::PBXNativeTarget(ref v) => &v.inner,
                _ => return false,
            };
            if let Some(name) = target.name.as_ref() {
                name == target_name
            } else {
                false
            }
        })
    }

    /// Get PBXTarget by reference
    pub fn get_target(&self, reference: &str) -> Option<&PBXObject> {
        if let Some(object) = self.get(reference) {
            if object.is_pbx_target() {
                return Some(object);
            }
            None
        } else {
            None
        }
    }

    /// Get mutable PBXTarget by reference
    pub fn get_target_mut(&mut self, reference: &str) -> Option<&mut PBXObject> {
        if let Some(object) = self.get_mut(reference) {
            if object.is_pbx_target() {
                return Some(object);
            }
            None
        } else {
            None
        }
    }
}
