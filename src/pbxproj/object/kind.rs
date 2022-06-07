#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Pbxproj object kinds
pub enum PBXObjectKind {
    /// A Kind representing a build target that aggregates several others.
    PBXAggregateTarget,
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
    /// A Kind representing a build target that according to Xcode is an "External Build System".
    PBXLegacyTarget,
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXNativeTarget,
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXProject,
    /// A Kind representing a proxy for another object which might belong to another project
    /// contained in the same workspace of the document. This class is referenced by
    /// PBXTargetDependency.
    PBXReferenceProxy,
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
    /// Returns `true` if the object kind is [`BuildFile`].
    ///
    /// [`BuildFile`]:PBXObjectKind::PBXBuildFile
    #[must_use]
    pub fn is_build_file(&self) -> bool {
        matches!(self, Self::PBXBuildFile)
    }

    /// Returns `true` if the object kind is [`FileReference`].
    ///
    /// [`FileReference`]:PBXObjectKind::PBXFileReference
    #[must_use]
    pub fn is_file_reference(&self) -> bool {
        matches!(self, Self::PBXFileReference)
    }

    /// Returns `true` if the object kind is [`LegacyTarget`].
    ///
    /// [`LegacyTarget`]:PBXObjectKind::PBXLegacyTarget
    #[must_use]
    pub fn is_legacy_target(&self) -> bool {
        matches!(self, Self::PBXLegacyTarget)
    }

    /// Returns `true` if the object kind is [`NativeTarget`].
    ///
    /// [`NativeTarget`]:PBXObjectKind::PBXNativeTarget
    #[must_use]
    pub fn is_native_target(&self) -> bool {
        matches!(self, Self::PBXNativeTarget)
    }

    /// Returns `true` if the object kind is [`AggregateTarget`].
    ///
    /// [`AggregateTarget`]:PBXObjectKind::PBXAggregateTarget
    #[must_use]
    pub fn is_aggregate_target(&self) -> bool {
        matches!(self, Self::PBXAggregateTarget)
    }

    /// Returns `true` if the object kind is [`Project`].
    ///
    /// [`Project`]:PBXObjectKind::PBXProject
    #[must_use]
    pub fn is_project(&self) -> bool {
        matches!(self, Self::PBXProject)
    }

    /// Returns `true` if the object kind is [`Group`].
    ///
    /// [`Group`]:PBXObjectKind::PBXGroup
    #[must_use]
    pub fn is_group(&self) -> bool {
        matches!(self, Self::PBXGroup)
    }

    /// Returns `true` if the object kind is [`HeadersBuildPhase`].
    ///
    /// [`HeadersBuildPhase`]:PBXObjectKind::PBXHeadersBuildPhase
    #[must_use]
    pub fn is_headers_build_phase(&self) -> bool {
        matches!(self, Self::PBXHeadersBuildPhase)
    }

    /// Returns `true` if the object kind is [`FrameworksBuildPhase`].
    ///
    /// [`FrameworksBuildPhase`]:PBXObjectKind::PBXFrameworksBuildPhase
    #[must_use]
    pub fn is_frameworks_build_phase(&self) -> bool {
        matches!(self, Self::PBXFrameworksBuildPhase)
    }

    /// Returns `true` if the object kind is [`ConfigurationList`].
    ///
    /// [`ConfigurationList`]:PBXObjectKind::XCConfigurationList
    #[must_use]
    pub fn is_configuration_list(&self) -> bool {
        matches!(self, Self::XCConfigurationList)
    }

    /// Returns `true` if the object kind is [`ResourcesBuildPhase`].
    ///
    /// [`ResourcesBuildPhase`]:PBXObjectKind::PBXResourcesBuildPhase
    #[must_use]
    pub fn is_resources_build_phase(&self) -> bool {
        matches!(self, Self::PBXResourcesBuildPhase)
    }

    /// Returns `true` if the object kind is [`ShellScriptBuildPhase`].
    ///
    /// [`ShellScriptBuildPhase`]:PBXObjectKind::PBXShellScriptBuildPhase
    #[must_use]
    pub fn is_shell_script_build_phase(&self) -> bool {
        matches!(self, Self::PBXShellScriptBuildPhase)
    }

    /// Returns `true` if the object kind is [`SourcesBuildPhase`].
    ///
    /// [`SourcesBuildPhase`]:PBXObjectKind::PBXSourcesBuildPhase
    #[must_use]
    pub fn is_sources_build_phase(&self) -> bool {
        matches!(self, Self::PBXSourcesBuildPhase)
    }

    /// Returns `true` if the object kind is [`TargetDependency`].
    ///
    /// [`TargetDependency`]:PBXObjectKind::PBXTargetDependency
    #[must_use]
    pub fn is_target_dependency(&self) -> bool {
        matches!(self, Self::PBXTargetDependency)
    }

    /// Returns `true` if the object kind is [`VariantGroup`].
    ///
    /// [`VariantGroup`]:PBXObjectKind::PBXVariantGroup
    #[must_use]
    pub fn is_variant_group(&self) -> bool {
        matches!(self, Self::PBXVariantGroup)
    }

    /// Returns `true` if the object kind is [`BuildConfiguration`].
    ///
    /// [`BuildConfiguration`]:PBXObjectKind::XCBuildConfiguration
    #[must_use]
    pub fn is_build_configuration(&self) -> bool {
        matches!(self, Self::XCBuildConfiguration)
    }

    /// Returns `true` if the object kind is [`CopyFilesBuildPhase`].
    ///
    /// [`CopyFilesBuildPhase`]:PBXObjectKind::PBXCopyFilesBuildPhase
    #[must_use]
    pub fn is_copy_files_build_phase(&self) -> bool {
        matches!(self, Self::PBXCopyFilesBuildPhase)
    }

    /// Returns `true` if the object kind is [`ContainerItemProxy`].
    ///
    /// [`ContainerItemProxy`]:PBXObjectKind::PBXContainerItemProxy
    #[must_use]
    pub fn is_container_item_proxy(&self) -> bool {
        matches!(self, Self::PBXContainerItemProxy)
    }

    /// Returns `true` if the object kind is [`ReferenceProxy`].
    ///
    /// [`ReferenceProxy`]:PBXObjectKind::PBXReferenceProxy
    #[must_use]
    pub fn is_reference_proxy(&self) -> bool {
        matches!(self, Self::PBXReferenceProxy)
    }

    /// Returns `true` if the object kind is [`VersionGroup`].
    ///
    /// [`VersionGroup`]:PBXObjectKind::XCVersionGroup
    #[must_use]
    pub fn is_version_group(&self) -> bool {
        matches!(self, Self::XCVersionGroup)
    }

    /// Returns `true` if the object kind is [`RezBuildPhase`].
    ///
    /// [`RezBuildPhase`]:PBXObjectKind::PBXRezBuildPhase
    #[must_use]
    pub fn is_rez_build_phase(&self) -> bool {
        matches!(self, Self::PBXRezBuildPhase)
    }

    /// Returns `true` if the object kind is [`BuildRule`].
    ///
    /// [`BuildRule`]:PBXObjectKind::PBXBuildRule
    #[must_use]
    pub fn is_build_rule(&self) -> bool {
        matches!(self, Self::PBXBuildFile)
    }

    /// Returns `true` if the object kind is [`RemoteSwiftPackageReference`].
    ///
    /// [`RemoteSwiftPackageReference`]:PBXObjectKind::XCRemoteSwiftPackageReference
    #[must_use]
    pub fn is_remote_swift_package_reference(&self) -> bool {
        matches!(self, Self::XCRemoteSwiftPackageReference)
    }

    /// Returns `true` if the object kind is [`SwiftPackageProductDependency`].
    ///
    /// [`SwiftPackageProductDependency`]:PBXObjectKind::XCSwiftPackageProductDependency
    #[must_use]
    pub fn is_swift_package_product_dependency(&self) -> bool {
        matches!(self, Self::XCSwiftPackageProductDependency)
    }

    /// Returns `true` if the object kind is [`Unknown`].
    ///
    /// [`Unknown`]:PBXObjectKind::Unknown
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(..))
    }

    /// Returns `Ok` if the object kind is [`Unknown`].
    ///
    /// [`Unknown`]:PBXObjectKind::Unknown
    pub fn try_into_unknown(self) -> Result<String, Self> {
        if let Self::Unknown(v) = self {
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
            "PBXLegacyTarget" => Self::PBXLegacyTarget,
            "PBXNativeTarget" => Self::PBXNativeTarget,
            "PBXAggregateTarget" => Self::PBXAggregateTarget,
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
            "PBXReferenceProxy" => Self::PBXReferenceProxy,
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
            Self::PBXLegacyTarget => "PBXLegacyTarget",
            Self::PBXNativeTarget => "PBXNativeTarget",
            Self::PBXAggregateTarget => "PBXAggregateTarget",
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
            Self::PBXReferenceProxy => "PBXReferenceProxy",
            Self::XCVersionGroup => "XCVersionGroup",
            Self::PBXRezBuildPhase => "PBXRezBuildPhase",
            Self::PBXBuildRule => "PBXBuildRule",
            Self::XCRemoteSwiftPackageReference => "XCRemoteSwiftPackageReference",
            Self::XCSwiftPackageProductDependency => "XCSwiftPackageProductDependency",
            Self::Unknown(str) => str,
        }
        .into()
    }
}
