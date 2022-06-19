use crate::pbxproj::*;

/// [`PBXObject`] A File referenced by a build phase, unique to each build phase.
#[derive(Default, Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXBuildFile<'a> {
    /// ID Reference
    pub id: String,
    /// Element settings
    pub settings: Option<&'a PBXValue>,
    /// Platform filter attribute.
    pub platform_filter: Option<&'a String>,
    /// Element file reference.
    #[deref]
    pub file: Option<PBXFSReference<'a>>,
    /// Product reference.
    pub product: Option<XCSwiftPackageProductDependency<'a>>,
    /// The cached build phase this build file belongs to
    pub build_phase: Option<PBXBuildPhase<'a>>,
}

impl<'a> AsPBXObject<'a> for PBXBuildFile<'a> {
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
            settings: value.get_value("settings"),
            platform_filter: value.get_string("platformFilter"),
            file: value.get_string("fileRef").and_then(|k| objects.get(k)),
            product: value.get_string("productRef").and_then(|k| objects.get(k)),
            build_phase: value
                .get_string("buildPhaseReference")
                .and_then(|k| objects.get(k)),
        })
    }
}
