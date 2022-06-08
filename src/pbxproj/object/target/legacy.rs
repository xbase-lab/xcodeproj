use super::*;
use derive_deref_rs::Deref;

/// [`PBXObject`] specifying [`PBXTarget`]  and representing an External Build System
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXLegacyTarget {
    /// Path to the build tool that is invoked (required)
    pub build_tool_path: Option<String>,
    /// Build arguments to be passed to the build tool.
    pub build_arguments_string: Option<String>,
    /// Whether or not to pass Xcode build settings as environment variables down to the tool when invoked
    pub pass_build_settings_in_environment: u8,
    /// The directory where the build tool will be invoked during a build
    pub build_working_directory: Option<String>,
    #[deref]
    inner: PBXTarget,
}

impl TryFrom<PBXHashMap> for PBXLegacyTarget {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            build_tool_path: value.remove_string("buildToolPath"),
            build_arguments_string: value.remove_string("buildArgumentsString"),
            pass_build_settings_in_environment: value
                .remove_number("passBuildSettingsInEnvironment")
                .unwrap_or_default() as u8,
            build_working_directory: value.remove_string("buildWorkingDirectory"),
            inner: PBXTarget::try_from(value)?,
        })
    }
}
