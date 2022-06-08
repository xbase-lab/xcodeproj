use crate::pbxproj::{PBXHashMap, PBXRootObject};

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

impl TryFrom<PBXHashMap> for XCBuildConfiguration {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.try_remove_string("name")?,
            build_settings: value.try_remove_object("buildSettings")?,
            base_configuration_reference: value.remove_string("base_configuration_reference"),
        })
    }
}
