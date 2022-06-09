use crate::pbxproj::{PBXHashMap, PBXObjectCollection, PBXObjectExt, PBXRootObject};
use std::{cell::RefCell, rc::Weak};

/// [`PBXObject`] specifying build configurations
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCBuildConfiguration {
    /// The configuration name.
    pub name: String,
    /// A map of build settings.
    pub build_settings: PBXHashMap,
    /// Base xcconfig file reference.
    base_configuration_reference: Option<String>,
    objects: Weak<RefCell<PBXObjectCollection>>,
}

impl XCBuildConfiguration {
    /// GGet Base xcconfig file reference.
    pub fn base_configuration(&self, _data: PBXRootObject) -> Option<()> {
        todo!()
    }

    /// Base xcconfig file reference.
    pub fn set_base_configuration(&mut self, reference: Option<String>) -> Option<String> {
        let old = self.base_configuration_reference.take();
        self.base_configuration_reference = reference;
        old
    }
}

impl PBXObjectExt for XCBuildConfiguration {
    fn from_hashmap(
        mut value: PBXHashMap,
        objects: Weak<RefCell<PBXObjectCollection>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            name: value.try_remove_string("name")?,
            build_settings: value.try_remove_object("buildSettings")?,
            base_configuration_reference: value.remove_string("base_configuration_reference"),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
