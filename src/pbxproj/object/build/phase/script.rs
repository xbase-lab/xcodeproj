use crate::pbxproj::*;

/// Part of [`PBXBuildPhase`] when [`PBXBuildPhaseKind`] is [`RunScript`]
///
/// [`RunScript`]: crate::pbxproj::PBXBuildPhaseKind::RunScript
#[derive(Debug, derive_new::new)]
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
}

impl PBXObjectExt for PBXShellScriptBuildPhase {
    fn from_hashmap(
        mut value: PBXHashMap,
        _objects: WeakPBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
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
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
