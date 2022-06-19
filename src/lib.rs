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
#[derive(Debug, derive_deref_rs::Deref)]
pub struct XCodeProject {
    root: PathBuf,
    #[deref]
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
}
