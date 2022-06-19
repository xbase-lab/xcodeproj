use crate::pbxproj::*;

/// [`PBXObject`] for remote [`XCSwiftPackageProductDependency`]
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
/// [`XCSwiftPackageProductDependency`]: crate::pbxproj::XCSwiftPackageProductDependency
#[derive(Debug, derive_new::new)]
pub struct XCRemoteSwiftPackageReference<'a> {
    /// ID Reference
    pub id: String,
    /// Repository url.
    pub repository_url: Option<&'a String>,
    /// Version rules.
    pub version_requirement: Option<XCVersionRequirement>,
}

impl<'a> Eq for XCRemoteSwiftPackageReference<'a> {}

impl<'a> PartialEq for XCRemoteSwiftPackageReference<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.repository_url == other.repository_url
            && self.version_requirement == other.version_requirement
    }
}

impl<'a> XCRemoteSwiftPackageReference<'a> {
    /// It returns the name of the package reference.
    pub fn name(&self) -> Option<&str> {
        self.repository_url
            .as_ref()
            .map(|s| s.split("/").last())
            .flatten()
    }

    /// Get a reference to the xcremote swift package reference's version requirement.
    #[must_use]
    pub fn version_requirement(&self) -> Option<&XCVersionRequirement> {
        self.version_requirement.as_ref()
    }

    /// Set the xcremote swift package reference's version requirement.
    pub fn set_version_requirement(&mut self, version_requirement: Option<XCVersionRequirement>) {
        self.version_requirement = version_requirement;
    }
}

impl<'a> AsPBXObject<'a> for XCRemoteSwiftPackageReference<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        _objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            id,
            repository_url: value.get_string("repositoryURL"),
            version_requirement: value
                .get_value("requirement")
                .map(|v| v.try_into().ok())
                .flatten(),
        })
    }
}
