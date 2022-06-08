use crate::pbxproj::{PBXHashMap, PBXValue};
use anyhow::bail;
use std::collections::HashMap;
use tap::Pipe;

/// [`XCRemoteSwiftPackageReference`] version rules.
///
/// [`XCRemoteSwiftPackageReference`]: crate::pbxproj::XCRemoteSwiftPackageReference
#[derive(Debug, PartialEq, Eq)]
pub enum XCVersionRequirement {
    /// Version can be bumped up to the next major version.
    UpToNextMajorVersion(String),
    /// Version can be bumped up to the next minor version.
    UpToNextMinorVersion(String),
    /// Version is constrained between a given range.
    Range(String, String),
    /// The package version needs to be the given version.
    Exact(String),
    /// Branch to use a specific branch of the git repository.
    Branch(String),
    /// Revision to use an specific revision of the git repository.
    Revision(String),
}

impl TryFrom<PBXValue> for XCVersionRequirement {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        let mut map = value.try_into_object()?;
        let key = map.try_remove_string("kind")?;
        match key.as_str() {
            "bracnh" => Self::Branch(map.try_remove_string(&key)?),
            "revision" => Self::Revision(map.try_remove_string(&key)?),
            "exactVersion" => Self::Exact(map.try_remove_string("version")?),
            "versionRange" => {
                let min = map.try_remove_string("minimumVersion")?;
                let max = map.try_remove_string("maximumVersion")?;
                Self::Range(min, max)
            }
            "upToNextMinorVersion" => {
                let min = map.try_remove_string("minimumVersion")?;
                Self::UpToNextMinorVersion(min)
            }
            "upToNextMajorVersion" => {
                let max = map.try_remove_string("maximumVersion")?;
                Self::UpToNextMajorVersion(max)
            }
            k => bail!("Unkown kind {k}"),
        }
        .pipe(Ok)
    }
}

impl From<XCVersionRequirement> for PBXValue {
    fn from(value: XCVersionRequirement) -> Self {
        let mut collect = HashMap::default();
        match value {
            XCVersionRequirement::UpToNextMajorVersion(v) => {
                collect.insert("kind".to_string(), "upToNextMajorVersion".into());
                collect.insert("maximumVersion".to_string(), v.into());
            }
            XCVersionRequirement::UpToNextMinorVersion(v) => {
                collect.insert("kind".to_string(), "upToNextMinorVersion".into());
                collect.insert("minimumVersion".to_string(), v.into());
            }
            XCVersionRequirement::Range(s, e) => {
                collect.insert("kind".to_string(), "versionRange".into());
                collect.insert("minimumVersion".to_string(), e.into());
                collect.insert("maximumVersion".to_string(), s.into());
            }
            XCVersionRequirement::Exact(v) => {
                collect.insert("kind".to_string(), "exactVersion".into());
                collect.insert("version".to_string(), v.into());
            }
            XCVersionRequirement::Branch(v) => {
                collect.insert("kind".to_string(), "branch".into());
                collect.insert("branch".to_string(), v.into());
            }
            XCVersionRequirement::Revision(v) => {
                collect.insert("kind".to_string(), "revision".into());
                collect.insert("revision".to_string(), v.into());
            }
        }

        PBXValue::Object(PBXHashMap::new(collect))
    }
}
