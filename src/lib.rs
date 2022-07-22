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
mod scheme;
pub mod xcode;
pub use scheme::XCScheme;

/// Main presentation of XCodeProject
#[derive(Debug, Default, derive_deref_rs::Deref)]
pub struct XCodeProject {
    name: String,
    root: PathBuf,
    #[deref]
    pbxproj: PBXRootObject,
    schemes: Vec<XCScheme>,
}

impl XCodeProject {
    // /xcshareddata/xcschemes
    /// Create new XCodeProject object from xcodeproj_folder
    pub fn new<P: AsRef<Path>>(xcodeproj_folder: P) -> Result<Self> {
        let xcodeproj_folder = xcodeproj_folder.as_ref();
        let name = xcodeproj_folder
            .file_name()
            .and_then(|name| Some(name.to_str()?.split_once(".")?.0.to_string()))
            .unwrap();
        let root = xcodeproj_folder.parent().unwrap().to_path_buf();
        let mut schemes = vec![];
        let xcworkspace_folder = root.join(format!("{name}.xcworkspace"));
        let schemes_folder = xcworkspace_folder.join("xcshareddata").join("xcschemes");

        // NOTE: Should xcodeproj folder be accounted for?
        if schemes_folder.exists() {
            let mut files = std::fs::read_dir(schemes_folder)?;
            while let Some(Ok(entry)) = files.next() {
                if let Ok(xcscheme) = XCScheme::new(entry.path()) {
                    schemes.push(xcscheme);
                }
            }
        }

        let pbxproj = PBXRootObject::try_from(xcodeproj_folder.join("project.pbxproj"))?;

        Ok(Self {
            name,
            root,
            pbxproj,
            schemes,
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

    /// Get build file names with all targets
    pub fn build_file_names(&self) -> Vec<String> {
        self.build_files()
            .into_iter()
            .flat_map(|f| {
                let file = f.file.as_ref()?;
                Some(file.path.or(file.name)?.to_string())
            })
            .collect::<Vec<_>>()
    }

    /// Get XCSchemes
    pub fn schemes(&self) -> &[XCScheme] {
        self.schemes.as_ref()
    }
}
