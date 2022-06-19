#![allow(dead_code)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use anyhow::Result;
use pbxproj::{PBXFSReference, PBXObjectCollection, PBXProject, PBXRootObject};
use std::path::{Path, PathBuf};

mod macros;
pub mod pbxproj;
pub mod xcode;

/// Main presentation of XCodeProject
pub struct XCodeProject {
    root: PathBuf,
    pbxproj: PBXRootObject,
}

impl XCodeProject {
    /// Create new XCodeProject object
    pub fn new<P: AsRef<Path>>(xcodeproj_folder: P) -> Result<Self> {
        let xcodeproj_folder = xcodeproj_folder.as_ref();
        let pbxproj_path = xcodeproj_folder.join("project.pbxproj");

        Ok(Self {
            root: xcodeproj_folder.parent().unwrap().to_path_buf(),
            pbxproj: pbxproj_path.try_into()?,
        })
    }

    /// Get archive version
    pub fn archive_version(&self) -> u8 {
        self.pbxproj.archive_version()
    }

    /// Get pbxproj object version
    pub fn object_version(&self) -> u8 {
        self.pbxproj.object_version()
    }

    /// Get root project of pbxproj
    pub fn root_project(&self) -> PBXProject {
        self.pbxproj.root_project()
    }

    /// Get root group of pbxproj
    pub fn root_group(&self) -> PBXFSReference {
        self.pbxproj.root_group()
    }

    /// Get pbxproj objects
    pub fn objects(&self) -> &PBXObjectCollection {
        self.pbxproj.objects()
    }

    /// Get mutable reference of pbxproj objects
    pub fn objects_mut(&mut self) -> &mut PBXObjectCollection {
        self.pbxproj.objects_mut()
    }
}
