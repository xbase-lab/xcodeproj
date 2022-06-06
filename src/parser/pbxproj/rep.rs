use super::serialize::PBXValue;
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use tap::Pipe;

/// Result of Parsing *.pbxproj
#[derive(Debug)]
pub struct PBXProject {
    /// archiveVersion
    archive_version: u8,
    /// objectVersion
    object_version: u8,
    /// classes
    classes: HashMap<String, PBXValue>,
    /// Objects
    objects: HashMap<String, HashMap<String, PBXValue>>,
    /// rootObjectReference
    root_object_reference: String,
}

impl PBXProject {
    /// Create new PBXProject with required fields
    /// Use PBXProject::try_from(String/&str/Path/PathBuf) instead to parse and create object
    pub fn new(
        archive_version: u8,
        object_version: u8,
        classes: HashMap<String, PBXValue>,
        objects: HashMap<String, HashMap<String, PBXValue>>,
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

    /// Extract a string value using key from objects in iterative matter
    pub fn extract_string<S: AsRef<str>>(&self, key: S) -> Option<&String> {
        for (_, object) in self.objects.iter() {
            for (field_key, field_value) in object {
                if field_key == key.as_ref() {
                    return field_value.as_string();
                } else if field_value.is_object() {
                    if let Some(value) = self._extract_string(key.as_ref(), field_value) {
                        return Some(value);
                    }
                }
            }
        }
        None
    }

    fn _extract_string<'a>(&'a self, key: &str, value: &'a PBXValue) -> Option<&'a String> {
        if let PBXValue::Object(object) = value {
            for (field_key, field_value) in object {
                if field_key == key {
                    return field_value.as_string();
                } else if field_value.is_object() {
                    if let Some(value) = self._extract_string(key, field_value) {
                        return Some(value);
                    }
                }
            }
        }

        None
    }

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
    pub fn classes(&self) -> &HashMap<String, PBXValue> {
        &self.classes
    }

    /// Get a reference to the pbxproject's root object reference.
    #[must_use]
    pub fn root_object_reference(&self) -> &str {
        self.root_object_reference.as_ref()
    }
}

impl TryFrom<String> for PBXProject {
    type Error = anyhow::Error;
    fn try_from(content: String) -> anyhow::Result<Self> {
        PBXProject::try_from(content.as_str())
    }
}

impl TryFrom<&str> for PBXProject {
    type Error = anyhow::Error;
    fn try_from(content: &str) -> anyhow::Result<Self> {
        use crate::parser::pbxproj::serialize::*;
        use pest_consume::Parser;

        let nodes = PBXProjectParser::parse(Rule::file, content).context("Parse content")?;
        let node = nodes.single().context("nodes to single")?;
        let mut object = PBXProjectParser::file(node)?;

        let archive_version = object
            .remove("archive_version")
            .ok_or_else(|| anyhow::anyhow!("archiveVersion is not found"))?
            .try_into_number()
            .unwrap() as u8;
        let object_version = object
            .remove("object_version")
            .ok_or_else(|| anyhow::anyhow!("archiveVersion is not found"))?
            .try_into_number()
            .unwrap() as u8;
        let classes = object
            .remove("classes")
            .unwrap_or(PBXValue::Object(HashMap::new()))
            .try_into_object()
            .unwrap();
        let root_object_reference = object
            .remove("root_object")
            .ok_or_else(|| anyhow::anyhow!("rootObject key is not found"))?
            .try_into_string()
            .unwrap();
        let objects = object
            .remove("objects")
            .ok_or_else(|| anyhow::anyhow!("objects key is not found"))?
            .try_into_object()
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.try_into_object().unwrap()))
            .collect();

        Ok(Self {
            archive_version,
            object_version,
            classes,
            objects,
            root_object_reference,
        })
    }
}

impl TryFrom<&Path> for PBXProject {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self> {
        read_to_string(&value)
            .map_err(|e| anyhow::anyhow!("PBXProject from path {value:?}: {e}"))?
            .pipe(TryFrom::try_from)
    }
}

impl TryFrom<PathBuf> for PBXProject {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self> {
        Self::try_from(value.as_path())
    }
}

#[test]
fn test_parse() {
    let test_content = include_str!("../../../tests/samples/demo1.pbxproj");
    let project = PBXProject::try_from(test_content).unwrap();
    println!("{project:#?}");
}

#[test]
fn test_extract_string() {
    let test_content = include_str!("../../../tests/samples/demo1.pbxproj");
    let project = PBXProject::try_from(test_content).unwrap();
    let development_region = project.extract_string("development_region");
    assert_eq!(Some(&String::from("en")), development_region);
}
