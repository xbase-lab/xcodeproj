use super::XCVersionRequirement;
use crate::pbxproj::PBXHashMap;

/// [`PBXObject`] for remote [`XCSwiftPackageProductDependency`]
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
/// [`XCSwiftPackageProductDependency`]: crate::pbxproj::XCSwiftPackageProductDependency
#[derive(Debug, derive_new::new)]
pub struct XCRemoteSwiftPackageReference {
    /// Repository url.
    pub repository_url: Option<String>,
    /// Version rules.
    pub version_requirement: Option<XCVersionRequirement>,
}

impl XCRemoteSwiftPackageReference {
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
}

impl TryFrom<PBXHashMap> for XCRemoteSwiftPackageReference {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            repository_url: value
                .remove_value("repository_url")
                .map(|v| v.try_into().ok())
                .flatten(),
            version_requirement: value
                .remove_value("requirement")
                .map(|v| v.try_into().ok())
                .flatten(),
        })
    }
}
