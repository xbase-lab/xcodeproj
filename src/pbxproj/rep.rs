use super::{PBXHashMap, PBXObject, PBXObjectCollection};
use anyhow::Result;
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};
use tap::Pipe;

/// `Main` Representation of project.pbxproj file
#[derive(Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXRootObject {
    /// archiveVersion
    archive_version: u8,
    /// objectVersion
    object_version: u8,
    /// classes
    classes: PBXHashMap,
    /// Objects
    #[deref]
    pub(crate) objects: Rc<RefCell<PBXObjectCollection>>,
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
        let refcell = Rc::new(RefCell::new(PBXObjectCollection::default()));
        let objects = map
            .try_remove_object("objects")?
            .0
            .into_iter()
            .map(|(k, v)| anyhow::Ok((k, PBXObject::new(v, Rc::downgrade(&refcell))?)))
            .flatten()
            .collect::<HashMap<_, _>>();

        refcell.borrow_mut().set_inner(objects);

        Ok(Self {
            archive_version,
            object_version,
            classes,
            objects: refcell,
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
