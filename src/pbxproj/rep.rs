use super::PBXHashMap;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tap::Pipe;

/// Result of Parsing *.pbxproj
#[derive(Debug, derive_new::new)]
pub struct PBXRootObject {
    /// archiveVersion
    archive_version: u8,
    /// objectVersion
    object_version: u8,
    /// classes
    classes: PBXHashMap,
    /// Objects
    objects: PBXHashMap,
    /// rootObjectReference
    root_object_reference: String,
}

impl PBXRootObject {
    /// Get the pbxproject's archive version.
    #[must_use]
    pub fn archive_version(&self) -> u8 {
        self.archive_version
    }

    /// Get the pbxproject's object version.
    #[must_use]
    pub fn object_version(&self) -> u8 {
        self.object_version
    }

    /// Get a reference to the pbxproject's classes.
    #[must_use]
    pub fn classes(&self) -> &PBXHashMap {
        &self.classes
    }

    /// Get a reference to the pbxproject's root object reference.
    #[must_use]
    pub fn root_object_reference(&self) -> &str {
        self.root_object_reference.as_ref()
    }
}

impl TryFrom<PBXHashMap> for PBXRootObject {
    type Error = anyhow::Error;
    fn try_from(mut map: PBXHashMap) -> Result<Self> {
        let archive_version = map.try_remove_number("archiveVersion")? as u8;
        let object_version = map.try_remove_number("objectVersion")? as u8;
        let classes = map.try_remove_object("classes").unwrap_or_default();
        let root_object_reference = map.try_remove_string("rootObject")?;
        let objects = map.try_remove_object("objects")?;

        Ok(Self {
            archive_version,
            object_version,
            classes,
            objects,
            root_object_reference,
        })
    }
}

impl TryFrom<&str> for PBXRootObject {
    type Error = anyhow::Error;
    fn try_from(content: &str) -> Result<Self> {
        use crate::pbxproj::pest::PBXProjectParser;

        PBXProjectParser::try_from_str(content)?.pipe(Self::try_from)
    }
}

impl TryFrom<String> for PBXRootObject {
    type Error = anyhow::Error;
    fn try_from(content: String) -> Result<Self> {
        Self::try_from(content.as_str())
    }
}

impl TryFrom<&Path> for PBXRootObject {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self> {
        std::fs::read_to_string(&value)
            .map_err(|e| anyhow::anyhow!("PBXProjectData from path {value:?}: {e}"))?
            .pipe(TryFrom::try_from)
    }
}

impl TryFrom<PathBuf> for PBXRootObject {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self> {
        Self::try_from(value.as_path())
    }
}
#[test]
#[ignore = "check_output"]
fn test_parse() {
    let test_content = include_str!("../../tests/samples/demo1.pbxproj");
    let project = PBXRootObject::try_from(test_content).unwrap();
    println!("{project:#?}");
}

#[test]
fn test_extract_string() {
    let test_content = include_str!("../../tests/samples/demo1.pbxproj");
    let project = PBXRootObject::try_from(test_content).unwrap();
    // let development_region = project.extract_string("development_region");
    // assert_eq!(Some(&String::from("en")), development_region);
}
#[test]
fn test_extract_value() {
    let test_content = include_str!("../../tests/samples/demo2.pbxproj");
    let project = PBXRootObject::try_from(test_content).unwrap();
    // let has_scanned_for_encodings = project.extract_value("has_scanned_for_encodings");
    // let targets = project.extract_value("targets");
    // assert_eq!(Some(&PBXValue::Number(0)), has_scanned_for_encodings);
    // assert_eq!(
    //     Some(&PBXValue::Array(vec![
    //         PBXValue::String("A0D495491ADE8368000B98EC".into()),
    //         PBXValue::String("8EF0E26B1B340CF900CF1FCC".into())
    //     ])),
    //     targets
    // )
}
