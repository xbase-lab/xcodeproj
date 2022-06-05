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
            .remove("archiveVersion")
            .ok_or_else(|| anyhow::anyhow!("archiveVersion is not found"))?
            .try_into_number()
            .unwrap() as u8;
        let object_version = object
            .remove("objectVersion")
            .ok_or_else(|| anyhow::anyhow!("archiveVersion is not found"))?
            .try_into_number()
            .unwrap() as u8;
        let classes = object
            .remove("classes")
            .unwrap_or(PBXValue::Object(HashMap::new()))
            .try_into_object()
            .unwrap();
        let root_object_reference = object
            .remove("rootObject")
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
fn test_from_string() {
    let test_content = include_str!("../../../tests/samples/demo1.pbxproj");
    let project = PBXProject::try_from(test_content).unwrap();
    println!("{project:#?}");
}
