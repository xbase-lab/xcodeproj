mod file;
mod kind;
mod rule;
mod script;

pub use file::*;
pub use kind::*;
pub use rule::*;
pub use script::*;

use crate::pbxproj::*;
use std::collections::HashSet;

/// `Abstraction` of build phase variants.
#[derive(Debug, derive_new::new)]
pub struct PBXBuildPhase {
    /// Element build action mask.
    pub build_action_mask: isize,
    /// References to build files.
    pub file_references: Option<HashSet<String>>,
    /// Paths to the input file lists.
    pub input_file_list_paths: Option<Vec<String>>,
    /// Paths to the output file lists.
    pub output_file_list_paths: Option<Vec<String>>,
    /// Element run only for deployment post processing value.
    pub run_only_for_deployment_postprocessing: bool,
    // ----
    kind: PBXBuildPhaseKind,
    inner: Option<PBXShellScriptBuildPhase>,
    objects: WeakPBXObjectCollection,
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
    pub fn add_file_reference(&mut self, reference: String) {
        let mut file_references = self.file_references.take().unwrap_or_default();
        file_references.insert(reference);
        self.file_references = file_references.into();
    }
}

impl PBXObjectExt for PBXBuildPhase {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let kind = value
            .try_remove_kind("isa")?
            .try_into_build_phase_kind()
            .unwrap();

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
            objects,
            inner: if kind.is_run_script() {
                Some(PBXObjectExt::from_hashmap(value, Default::default())?)
            } else {
                None
            },
            kind,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}

impl PBXBuildPhase {
    /// Get inner script representation
    pub fn get_inner(&self) -> Option<&PBXShellScriptBuildPhase> {
        self.inner.as_ref()
    }

    /// Get mutable inner script representation
    pub fn get_inner_mut(&mut self) -> Option<&mut PBXShellScriptBuildPhase> {
        self.inner.as_mut()
    }

    /// Whether build phase is PBXSourcesBuildPhase
    pub fn is_sources(&self) -> bool {
        self.kind.is_sources()
    }

    /// Whether build phase is PBXFrameworksBuildPhase
    pub fn is_frameworks(&self) -> bool {
        self.kind.is_frameworks()
    }

    /// Whether build phase is PBXResourcesBuildPhase
    pub fn is_resources(&self) -> bool {
        self.kind.is_resources()
    }

    /// Whether build phase is PBXCopyFilesBuildPhase
    pub fn is_copy_files(&self) -> bool {
        self.kind.is_copy_files()
    }

    /// Whether build phase is PBXShellScriptBuildPhase
    pub fn is_run_script(&self) -> bool {
        self.kind.is_run_script()
    }

    /// Whether build phase is PBXHeaderBuildPhase
    pub fn is_headers(&self) -> bool {
        self.kind.is_headers()
    }

    /// Whether build phase is PBXRezBuildPhase
    pub fn is_carbon_resources(&self) -> bool {
        self.kind.is_carbon_resources()
    }
}
