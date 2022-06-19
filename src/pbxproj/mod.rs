//! pbxproj file serialize and deserializer
mod object;
mod value;

pub(crate) mod pest;
pub use object::*;
pub use value::*;

use anyhow::Result;
use std::path::{Path, PathBuf};
use tap::Pipe;

/// `Main` Representation of project.pbxproj file
#[derive(Default, Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXRootObject {
    /// archiveVersion
    archive_version: u8,
    /// objectVersion
    object_version: u8,
    /// classes
    classes: PBXHashMap,
    /// Objects
    #[deref]
    objects: PBXObjectCollection,
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

    /// Get Root PBXProject
    pub fn root_project(&self) -> PBXProject {
        self.objects
            .projects()
            .into_iter()
            .find(|o| o.id == self.root_object_reference())
            .unwrap()
    }

    /// Get root group
    pub fn root_group(&self) -> PBXFSReference {
        self.root_project().main_group
    }

    /// Get a reference to the pbxroot object's objects.
    #[must_use]
    pub fn objects(&self) -> &PBXObjectCollection {
        &self.objects
    }

    /// Get a mutable reference to the pbxroot object's objects.
    #[must_use]
    pub fn objects_mut(&mut self) -> &mut PBXObjectCollection {
        &mut self.objects
    }
}

impl TryFrom<PBXHashMap> for PBXRootObject {
    type Error = anyhow::Error;
    fn try_from(mut map: PBXHashMap) -> Result<Self> {
        let archive_version = map.try_remove_number("archiveVersion")? as u8;
        let object_version = map.try_remove_number("objectVersion")? as u8;
        let classes = map.try_remove_object("classes").unwrap_or_default();
        let root_object_reference = map.try_remove_string("rootObject")?;
        let objects = PBXObjectCollection(
            map.try_remove_object("objects")?
                .0
                .into_iter()
                .map(|(k, v)| (k, v.try_into_object().unwrap()))
                .collect(),
        );

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
fn test_demo1_representation() {
    let test_content = include_str!("../../tests/samples/demo1.pbxproj");
    let project = PBXRootObject::try_from(test_content).unwrap();
    let targets = project.targets();

    assert_eq!(1, targets.len());
    assert_eq!(&PBXTargetKind::Native, targets[0].kind);
    assert_eq!(Some(&String::from("Wordle")), targets[0].product_name);
    assert_eq!(Some(&String::from("Wordle")), targets[0].name);
    assert_eq!(PBXProductType::Application, targets[0].product_type);
    assert_eq!(None, targets[0].build_tool_path);
    assert_eq!(None, targets[0].build_arguments_string);
    assert_eq!(None, targets[0].build_working_directory);
    assert_eq!(None, targets[0].pass_build_settings_in_environment);
    assert_eq!(3, targets[0].build_phases.len());
    assert_eq!(
        vec![
            (&PBXBuildPhaseKind::Sources, 12),   // 12
            (&PBXBuildPhaseKind::Resources, 3),  // 3
            (&PBXBuildPhaseKind::Frameworks, 1)  // 1
        ],
        targets[0]
            .build_phases
            .iter()
            .map(|phase| (&phase.kind, phase.files.len()))
            .collect::<Vec<_>>()
    );

    assert_eq!(1, project.projects().len());

    let root_group = project.root_group();
    assert_eq!(17, project.files().len());
    println!("{:#?}", root_group.children);
    assert_eq!(3, root_group.children.len());
    assert_eq!(None, root_group.name);
    assert_eq!(None, root_group.path);
}

#[cfg(test)]
macro_rules! test_demo_file {
    ($name:expr) => {{
        let (root, name) = (env!("CARGO_MANIFEST_DIR"), stringify!($name));
        let path = format!("{root}/tests/samples/{name}.pbxproj");
        let file = crate::pbxproj::PBXRootObject::try_from(std::path::PathBuf::from(path));
        if file.is_err() {
            println!("Error: {:#?}", file.as_ref().unwrap_err())
        }
        assert!(file.is_ok());
        file.unwrap()
    }};
}

#[cfg(test)]
mod tests {
    macro_rules! test_samples {
        ($($name:ident),*) => {
            $(#[test]
                fn $name() {
                    test_demo_file!($name);
                })*
        };
    }

    test_samples![demo1, demo2, demo3, demo4, demo5, demo6, demo7, demo8, demo9];
}
