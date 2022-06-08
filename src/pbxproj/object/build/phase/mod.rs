mod file;
mod kind;
mod rule;

pub use file::*;
pub use kind::*;
pub use rule::*;

use crate::pbxproj::PBXHashMap;
use derive_deref_rs::Deref;
use std::collections::HashSet;

/// `Abstraction` of build phase variants.
#[derive(Debug, derive_new::new)]
pub struct PBXBuildPhase {
    /// Element build action mask.
    pub build_action_mask: isize,
    /// References to build files.
    file_references: Option<HashSet<String>>,
    /// Paths to the input file lists.
    pub input_file_list_paths: Option<Vec<String>>,
    /// Paths to the output file lists.
    pub output_file_list_paths: Option<Vec<String>>,
    /// Element run only for deployment post processing value.
    pub run_only_for_deployment_postprocessing: bool,
}

impl PBXBuildPhase {
    const DEFAULT_BUILD_ACTION_MASK: isize = 2_147_483_647;

    /// Get Build files that the build phase include
    pub fn files(&self) -> Option<Vec<&PBXBuildFile>> {
        // objects from file_references?.objects()
        todo!()
    }

    /// set file references.
    pub fn set_file_references(
        &mut self,
        references: Option<HashSet<String>>,
    ) -> Option<HashSet<String>> {
        std::mem::replace(&mut self.file_references, references)
    }

    /// Add file_reference
    fn add_file_reference(&mut self, reference: String) {
        let mut file_references = self.file_references.take().unwrap_or_default();
        file_references.insert(reference);
        self.file_references = file_references.into();
    }
}

impl TryFrom<PBXHashMap> for PBXBuildPhase {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            build_action_mask: value
                .try_remove_number("buildActionMask")
                .unwrap_or_else(|_| Self::DEFAULT_BUILD_ACTION_MASK),
            file_references: value
                .remove_vec("files")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten()
                .map(|v| HashSet::from_iter(v)),
            input_file_list_paths: value
                .remove_vec("inputFileListPaths")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten(),
            output_file_list_paths: value
                .remove_vec("outputFileListPaths")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten(),
            run_only_for_deployment_postprocessing: value
                .remove_number("runOnlyForDeploymentPostprocessing")
                .map(|v| v == 1)
                .unwrap_or_default(),
        })
    }
}

/// [`PBXObject`] specifying [`PBXBuildPhase`] for copy file build phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref)]
pub struct PBXCopyFilesBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXCopyFilesBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::CopyFiles;
    /// Create Copy files Build Phase
    pub fn new(inner: PBXBuildPhase) -> Self {
        Self { inner }
    }
}

impl TryFrom<PBXHashMap> for PBXCopyFilesBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// [`PBXObject`] specifying [`PBXBuildPhase`] for frameworks linking phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXFrameworksBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXFrameworksBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::Frameworks;

    /// Return [`Self::KIND`]
    pub fn kind(&self) -> PBXBuildPhaseKind {
        return Self::KIND;
    }
}

impl TryFrom<PBXHashMap> for PBXFrameworksBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}
/// [`PBXObject`] specifying [`PBXBuildPhase`] for header linking phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXHeadersBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXHeadersBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::Headers;
}

impl TryFrom<PBXHashMap> for PBXHeadersBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

#[derive(Debug, Deref, derive_new::new)]
/// [`PBXObject`] specifying [`PBXBuildPhase`] for resouces linking phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
pub struct PBXResourcesBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXResourcesBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::Resources;
}

impl TryFrom<PBXHashMap> for PBXResourcesBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// [`PBXObject`] specifying [`PBXBuildPhase`] for Carbon Resources phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXRezBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXRezBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::CarbonResources;
}

impl TryFrom<PBXHashMap> for PBXRezBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// [`PBXObject`] specifying [`PBXBuildPhase`] for compilation phase
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXSourcesBuildPhase {
    inner: PBXBuildPhase,
}

impl PBXSourcesBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::Sources;
}

impl TryFrom<PBXHashMap> for PBXSourcesBuildPhase {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// [`PBXObject`] specifying [`PBXBuildPhase`] for processing scripts
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXShellScriptBuildPhase {
    /// Build phase name.
    pub name: Option<String>,
    /// Input paths
    pub input_paths: Vec<String>,
    /// Output paths
    pub output_paths: Vec<String>,
    /// Path to the shell.
    pub shell_path: Option<String>,
    /// Shell script.
    pub shell_script: Option<String>,
    /// Show environment variables in the logs.
    pub show_env_vars_in_log: bool,
    /// Force script to run in all incremental builds.
    pub always_out_of_date: bool,
    /// Path to the discovery .d dependency file
    pub dependency_file: Option<String>,
    #[deref]
    inner: PBXBuildPhase,
}

impl PBXShellScriptBuildPhase {
    /// Static reference representing build phase kind
    pub const KIND: PBXBuildPhaseKind = PBXBuildPhaseKind::RunScript;
}

impl TryFrom<PBXHashMap> for PBXShellScriptBuildPhase {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.remove_string("name"),
            input_paths: value.try_remove_vec("inputPaths")?.try_into_vec_strings()?,
            output_paths: value
                .try_remove_vec("outputPaths")?
                .try_into_vec_strings()?,
            shell_path: value.remove_string("shellPath"),
            shell_script: value.remove_string("shellScript"),
            show_env_vars_in_log: value
                .remove_number("runOnlyForDeploymentPostprocessing")
                .map(|v| v == 1)
                .unwrap_or_else(|| true),
            always_out_of_date: value
                .remove_number("alwaysOutOfDate")
                .map(|v| v == 1)
                .unwrap_or_else(|| false),
            dependency_file: value.remove_string("dependencyFile"),
            inner: TryFrom::try_from(value)?,
        })
    }
}
