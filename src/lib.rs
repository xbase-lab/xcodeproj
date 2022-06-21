#![allow(dead_code)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use anyhow::Result;
use pbxproj::PBXRootObject;
use std::path::{Path, PathBuf};

mod macros;
pub mod pbxproj;
pub mod xcode;

/// Main presentation of XCodeProject
#[derive(Debug, Default, derive_deref_rs::Deref)]
pub struct XCodeProject {
    name: String,
    root: PathBuf,
    #[deref]
    pbxproj: PBXRootObject,
}

impl XCodeProject {
    /// Create new XCodeProject object from xcodeproj_folder
    pub fn new<P: AsRef<Path>>(xcodeproj_folder: P) -> Result<Self> {
        let xcodeproj_folder = xcodeproj_folder.as_ref();
        let pbxproj_path = xcodeproj_folder.join("project.pbxproj");

        Ok(Self {
            name: xcodeproj_folder
                .file_name()
                .and_then(|name| Some(name.to_str()?.split_once(".")?.0.to_string()))
                .unwrap(),
            root: xcodeproj_folder.parent().unwrap().to_path_buf(),
            pbxproj: pbxproj_path.try_into()?,
        })
    }

    /// Get a reference to the xcode project's name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the xcode project's root.
    #[must_use]
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Get a reference to the xcode project's pbxproj.
    #[must_use]
    pub fn pbxproj(&self) -> &PBXRootObject {
        &self.pbxproj
    }
}
