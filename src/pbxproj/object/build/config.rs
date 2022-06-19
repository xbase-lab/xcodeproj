use crate::pbxproj::*;

/// [`PBXObject`] specifying build configurations
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCBuildConfiguration<'a> {
    /// ID Reference
    pub id: String,
    /// The configuration name.
    pub name: &'a String,
    /// A map of build settings.
    pub build_settings: &'a PBXHashMap,
    /// Base xcconfig file.
    pub base_configuration: Option<PBXFSReference<'a>>,
}

impl<'a> AsPBXObject<'a> for XCBuildConfiguration<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            id,
            name: value.try_get_string("name")?,
            build_settings: value.try_get_object("buildSettings")?,
            base_configuration: value
                .get_value("baseConfigurationReference")
                .and_then(|v| v.as_string())
                .and_then(|key| objects.get(key)),
        })
    }
}
