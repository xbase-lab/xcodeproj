use derive_is_enum_variant::is_enum_variant;
use enum_as_inner::EnumAsInner;

/// Representation of all Target kinds
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, is_enum_variant)]
pub enum PBXTargetKind {
    /// A build target that produces a binary content (application or library).
    Native,
    /// A build target that according to Xcode is an "External Build System".
    Legacy,
    /// A build target that aggregates several others.
    Aggregate,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumAsInner)]
/// Pbxproj object kinds
pub enum PBXObjectKind {
    /// A Kind representing:
    /// - PBXAggregateTarget: A build target that aggregates several others.
    /// - PBXLegacyTarget: A build target that according to Xcode is an "External Build System".
    /// - PBXNativeTarget: A  build target that produces a binary content (application or library).
    PBXTarget(PBXTargetKind),
    /// A Kind for defining build configurations
    XCBuildConfiguration,
    /// A Kind indicating a file reference that is used in a BuildPhase (either as an include or resource).
    PBXBuildFile,
    /// A Kind representing Build Rule
    PBXBuildRule,
    /// A Kind representing ['BuildConfiguration'] list
    XCConfigurationList,
    /// A Kind representing Decoration for a target element
    PBXContainerItemProxy,
    /// A Kind representing the copy file build phase
    PBXCopyFilesBuildPhase,
    /// A Kind representing to track every external file referenced by the project: source files,
    /// resource files, libraries, generated application files, and so on.
    PBXFileReference,
    /// A Kind representing a framework link build phase
    PBXFrameworksBuildPhase,
    /// A Kind representing group files
    PBXGroup,
    /// A Kind representing the header link build phase
    PBXHeadersBuildPhase,
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXProject,
    /// A Kind representing an abstract parent for specialized targets.
    XCRemoteSwiftPackageReference,
    /// A Kind representing the resources copy build phase
    PBXResourcesBuildPhase,
    /// A Kind representing the Build Carbon Resources build phase
    PBXRezBuildPhase,
    /// A Kind representing shell script build phase.
    PBXShellScriptBuildPhase,
    /// A Kind representing the sources compilation build phase.
    PBXSourcesBuildPhase,
    /// A Kind representing an abstract parent for specialized targets.
    XCSwiftPackageProductDependency,
    /// A Kind representing a reference to other targets through content proxies.
    PBXTargetDependency,
    /// UnknownPBXObjectKind
    Unknown(String),
    /// a Kind representing a reference localized resources.
    PBXVariantGroup,
    /// Kind representing  Group that contains multiple files references to the different versions
    /// of a resource. Used to contain the different versions of a xcdatamodel
    XCVersionGroup,
}

impl PBXObjectKind {
    /// Try get inner PBXTarget
    pub fn try_into_pbxtarget(self) -> Result<PBXTargetKind, Self> {
        if let Self::PBXTarget(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl From<&str> for PBXObjectKind {
    fn from(s: &str) -> Self {
        match s {
            "PBXBuildFile" => Self::PBXBuildFile,
            "PBXFileReference" => Self::PBXFileReference,
            "PBXLegacyTarget" => Self::PBXTarget(PBXTargetKind::Legacy),
            "PBXNativeTarget" => Self::PBXTarget(PBXTargetKind::Native),
            "PBXAggregateTarget" => Self::PBXTarget(PBXTargetKind::Aggregate),
            "PBXProject" => Self::PBXProject,
            "PBXGroup" => Self::PBXGroup,
            "PBXHeadersBuildPhase" => Self::PBXHeadersBuildPhase,
            "PBXFrameworksBuildPhase" => Self::PBXFrameworksBuildPhase,
            "XCConfigurationList" => Self::XCConfigurationList,
            "PBXResourcesBuildPhase" => Self::PBXResourcesBuildPhase,
            "PBXShellScriptBuildPhase" => Self::PBXShellScriptBuildPhase,
            "PBXSourcesBuildPhase" => Self::PBXSourcesBuildPhase,
            "PBXTargetDependency" => Self::PBXTargetDependency,
            "PBXVariantGroup" => Self::PBXVariantGroup,
            "XCBuildConfiguration" => Self::XCBuildConfiguration,
            "PBXCopyFilesBuildPhase" => Self::PBXCopyFilesBuildPhase,
            "PBXContainerItemProxy" => Self::PBXContainerItemProxy,
            "XCVersionGroup" => Self::XCVersionGroup,
            "PBXRezBuildPhase" => Self::PBXRezBuildPhase,
            "PBXBuildRule" => Self::PBXBuildRule,
            "XCRemoteSwiftPackageReference" => Self::XCRemoteSwiftPackageReference,
            "XCSwiftPackageProductDependency" => Self::XCSwiftPackageProductDependency,
            str => Self::Unknown(str.to_string()),
        }
    }
}

impl ToString for PBXObjectKind {
    fn to_string(&self) -> String {
        match self {
            Self::PBXBuildFile => "PBXBuildFile",
            Self::PBXFileReference => "PBXFileReference",
            Self::PBXProject => "PBXProject",
            Self::PBXGroup => "PBXGroup",
            Self::PBXHeadersBuildPhase => "PBXHeadersBuildPhase",
            Self::PBXFrameworksBuildPhase => "PBXFrameworksBuildPhase",
            Self::XCConfigurationList => "XCConfigurationList",
            Self::PBXResourcesBuildPhase => "PBXResourcesBuildPhase",
            Self::PBXShellScriptBuildPhase => "PBXShellScriptBuildPhase",
            Self::PBXSourcesBuildPhase => "PBXSourcesBuildPhase",
            Self::PBXTargetDependency => "PBXTargetDependency",
            Self::PBXVariantGroup => "PBXVariantGroup",
            Self::XCBuildConfiguration => "XCBuildConfiguration",
            Self::PBXCopyFilesBuildPhase => "PBXCopyFilesBuildPhase",
            Self::PBXContainerItemProxy => "PBXContainerItemProxy",
            Self::XCVersionGroup => "XCVersionGroup",
            Self::PBXRezBuildPhase => "PBXRezBuildPhase",
            Self::PBXBuildRule => "PBXBuildRule",
            Self::XCRemoteSwiftPackageReference => "XCRemoteSwiftPackageReference",
            Self::XCSwiftPackageProductDependency => "XCSwiftPackageProductDependency",
            Self::PBXTarget(kind) => match kind {
                PBXTargetKind::Native => "PBXNativeTarget",
                PBXTargetKind::Legacy => "PBXLegacyTarget",
                PBXTargetKind::Aggregate => "PBXAggregateTarget",
            },
            Self::Unknown(str) => str,
        }
        .into()
    }
}
