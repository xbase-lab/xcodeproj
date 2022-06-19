mod file;
mod kind;
mod rule;
mod script;

pub use file::*;
pub use kind::*;
pub use rule::*;
pub use script::*;

use crate::pbxproj::*;

/// `Abstraction` of build phase variants.
#[derive(Debug, derive_new::new)]
pub struct PBXBuildPhase<'a> {
    /// ID Reference
    pub id: String,
    /// Element build action mask.
    pub build_action_mask: isize,
    /// References to build files.
    pub files: Vec<PBXBuildFile<'a>>,
    /// Paths to the input file lists.
    pub input_file_list_paths: Option<Vec<&'a String>>,
    /// Paths to the output file lists.
    pub output_file_list_paths: Option<Vec<&'a String>>,
    /// Element run only for deployment post processing value.
    pub run_only_for_deployment_postprocessing: bool,
    /// Build Phase Kind
    pub kind: PBXBuildPhaseKind,
    /// inner (Some if PBXBuildPhase is PBXShellScriptBuildPhase)
    pub inner: Option<PBXShellScriptBuildPhase<'a>>,
}

impl<'a> PBXBuildPhase<'a> {
    const DEFAULT_BUILD_ACTION_MASK: isize = 2_147_483_647;
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

impl<'a> AsPBXObject<'a> for PBXBuildPhase<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        let kind = value
            .try_get_kind("isa")?
            .as_pbx_build_phase()
            .unwrap()
            .clone();

        Ok(Self {
            id,
            build_action_mask: value
                .try_get_number("buildActionMask")
                .map(|v| v.clone())
                .unwrap_or_else(|_| Self::DEFAULT_BUILD_ACTION_MASK),
            files: value
                .get_vec("files")
                .and_then(|vec| {
                    Some(
                        vec.as_vec_strings()
                            .iter()
                            .flat_map(|&k| objects.get(k))
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap_or_default(),
            input_file_list_paths: value
                .get_vec("inputFileListPaths")
                .map(|v| v.as_vec_strings()),
            output_file_list_paths: value
                .get_vec("outputFileListPaths")
                .map(|v| v.as_vec_strings()),
            run_only_for_deployment_postprocessing: value
                .get_number("runOnlyForDeploymentPostprocessing")
                .map(|v| v == &1)
                .unwrap_or_default(),
            inner: if kind.is_run_script() {
                Some(AsPBXObject::as_pbx_object(
                    Default::default(),
                    value,
                    objects,
                )?)
            } else {
                None
            },
            kind,
        })
    }
}
