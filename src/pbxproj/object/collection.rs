use super::PBXObject;
use std::collections::HashMap;

/// [`PBXObject`] storage with convenient helper methods
#[derive(Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXObjectCollection(HashMap<String, PBXObject>);

impl PBXObjectCollection {
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
}
