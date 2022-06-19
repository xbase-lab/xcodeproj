use crate::pbxproj::*;

/// Part of [`PBXBuildPhase`] when [`PBXBuildPhaseKind`] is [`RunScript`]
///
/// [`RunScript`]: crate::pbxproj::PBXBuildPhaseKind::RunScript
#[derive(Debug, derive_new::new)]
pub struct PBXShellScriptBuildPhase<'a> {
    /// Build phase name.
    pub name: Option<&'a String>,
    /// Input paths
    pub input_paths: Vec<&'a String>,
    /// Output paths
    pub output_paths: Vec<&'a String>,
    /// Path to the shell.
    pub shell_path: Option<&'a String>,
    /// Shell script.
    pub shell_script: Option<&'a String>,
    /// Show environment variables in the logs.
    pub show_env_vars_in_log: bool,
    /// Force script to run in all incremental builds.
    pub always_out_of_date: bool,
    /// Path to the discovery .d dependency file
    pub dependency_file: Option<&'a String>,
}

impl<'a> AsPBXObject<'a> for PBXShellScriptBuildPhase<'a> {
    fn as_pbx_object(
        _id: String,
        value: &'a PBXHashMap,
        _objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            name: value.get_string("name"),
            input_paths: value.try_get_vec("inputPaths")?.as_vec_strings(),
            output_paths: value.try_get_vec("outputPaths")?.as_vec_strings(),
            shell_path: value.get_string("shellPath"),
            shell_script: value.get_string("shellScript"),
            show_env_vars_in_log: value
                .get_number("runOnlyForDeploymentPostprocessing")
                .map(|v| v == &1)
                .unwrap_or_else(|| true),
            always_out_of_date: value
                .get_number("alwaysOutOfDate")
                .map(|v| v == &1)
                .unwrap_or_else(|| false),
            dependency_file: value.get_string("dependencyFile"),
        })
    }
}
