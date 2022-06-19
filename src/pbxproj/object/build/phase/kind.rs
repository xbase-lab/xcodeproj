use derive_is_enum_variant::is_enum_variant;
use tap::Pipe;

/// Enum that encapsulates all kind of build phases available in Xcode.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, is_enum_variant)]
pub enum PBXBuildPhaseKind {
    /// Sources
    Sources,
    /// Frameworks
    Frameworks,
    /// Resources
    Resources,
    /// CopyFiles
    CopyFiles,
    /// Run Script
    RunScript,
    /// Headers
    Headers,
    /// Build Legacy Carbon Resources
    CarbonResources,
}

impl PBXBuildPhaseKind {
    /// Return string representation of PBXBuildPhaseKind
    pub fn as_isa(&self) -> &str {
        match self {
            Self::Headers => "PBXHeadersBuildPhase",
            Self::Frameworks => "PBXFrameworksBuildPhase",
            Self::Resources => "PBXResourcesBuildPhase",
            Self::RunScript => "PBXShellScriptBuildPhase",
            Self::Sources => "PBXSourcesBuildPhase",
            Self::CopyFiles => "PBXCopyFilesBuildPhase",
            Self::CarbonResources => "PBXRezBuildPhase",
        }
    }
}

impl ToString for PBXBuildPhaseKind {
    fn to_string(&self) -> String {
        match self {
            Self::Sources => "Sources",
            Self::Frameworks => "Frameworks",
            Self::Resources => "Resources",
            Self::CopyFiles => "CopyFiles",
            Self::RunScript => "Run Script",
            Self::Headers => "Headers",
            Self::CarbonResources => "Rez",
        }
        .into()
    }
}

impl TryFrom<&str> for PBXBuildPhaseKind {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Sources" => Self::Sources,
            "Frameworks" => Self::Frameworks,
            "Resources" => Self::Resources,
            "CopyFiles" => Self::CopyFiles,
            "Run Script" => Self::RunScript,
            "Headers" => Self::Headers,
            "Rez" => Self::CarbonResources,
            str => anyhow::bail!("Unable to generate BuildPhase from '{str}'"),
        }
        .pipe(Ok)
    }
}

impl TryFrom<String> for PBXBuildPhaseKind {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PBXBuildPhaseKind::try_from(value.as_str())
    }
}
