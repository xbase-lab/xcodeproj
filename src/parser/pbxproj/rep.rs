use super::{PBXHashMap, PBXValue};
use std::collections::HashMap;

/// Result of Parsing *.pbxproj
#[derive(Debug)]
pub struct PBXProjectData {
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

impl PBXProjectData {
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

    /// Create new PBXProject with required fields
    /// Use PBXProject::try_from(String/&str/Path/PathBuf) instead to parse and create object
    pub fn new(
        archive_version: u8,
        object_version: u8,
        classes: PBXHashMap,
        objects: PBXHashMap,
        root_object_reference: String,
    ) -> Self {
        Self {
            archive_version,
            object_version,
            classes,
            objects,
            root_object_reference,
        }
    }
}

#[test]
#[ignore = "check_output"]
fn test_parse() {
    let test_content = include_str!("../../../tests/samples/demo1.pbxproj");
    let project = PBXProjectData::try_from(test_content).unwrap();
    println!("{project:#?}");
}

#[test]
fn test_extract_string() {
    let test_content = include_str!("../../../tests/samples/demo1.pbxproj");
    let project = PBXProjectData::try_from(test_content).unwrap();
    // let development_region = project.extract_string("development_region");
    // assert_eq!(Some(&String::from("en")), development_region);
}
#[test]
fn test_extract_value() {
    let test_content = include_str!("../../../tests/samples/demo2.pbxproj");
    let project = PBXProjectData::try_from(test_content).unwrap();
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
