use super::*;
use crate::pbxproj::PBXValue;
use derive_is_enum_variant::is_enum_variant;
use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;
use enum_variant_macros::FromVariants;

/// PBX Object Representation
#[enum_dispatch]
#[derive(Debug, FromVariants, EnumAsInner, is_enum_variant)]
pub enum PBXObject {
    /// A Target backed by shell scripts or nothing (only specifying dependencies).
    PBXAggregateTarget,
    /// A Kind for defining build configurations
    XCBuildConfiguration,
    /// File referenced by a build phase, unique to each build phase.
    PBXBuildFile,
    /// Specification of input transform to an output file(s).
    PBXBuildRule,
    /// List of build configurations.
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
    /// a Kind representing a reference localized resources.
    PBXVariantGroup,
    /// Kind representing  Group that contains multiple files references to the different versions
    /// of a resource. Used to contain the different versions of a xcdatamodel
    XCVersionGroup,
}

impl PBXObject {
    /// Whether the object is a PBXTarget
    pub fn is_pbx_target(&self) -> bool {
        self.is_pbx_aggregate_target() || self.is_pbx_native_target() || self.is_pbx_legacy_target()
    }
}

macro_rules! kind_to_object {
    ($kind:ident, $value:ident, [$($variant:ident),*]) => {
        match $kind {
            $(PBXObjectKind::$variant => PBXObject::from($variant::try_from($value)?),)*
                kind => anyhow::bail!("{kind:?} isn't supported")
        }
    };
}

impl TryFrom<PBXValue> for PBXObject {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        let map = value.try_into_object()?;
        let kind = if let Some(kind) = map.get_kind("isa") {
            kind
        } else {
            anyhow::bail!("isa field isn't defined: {map:#?}");
        };
        Ok(kind_to_object!(
            kind,
            map,
            [
                PBXAggregateTarget,
                XCBuildConfiguration,
                PBXBuildFile,
                PBXBuildRule,
                XCConfigurationList,
                PBXContainerItemProxy,
                PBXCopyFilesBuildPhase,
                PBXFileReference,
                PBXFrameworksBuildPhase,
                PBXGroup,
                PBXHeadersBuildPhase,
                PBXLegacyTarget,
                PBXNativeTarget,
                PBXProject,
                XCRemoteSwiftPackageReference,
                PBXResourcesBuildPhase,
                PBXRezBuildPhase,
                PBXShellScriptBuildPhase,
                PBXSourcesBuildPhase,
                XCSwiftPackageProductDependency,
                PBXTargetDependency,
                PBXVariantGroup,
                XCVersionGroup
            ]
        ))
    }
}

#[cfg(test)]
macro_rules! get_objects {
    ($path:expr) => {{
        use super::*;
        use crate::pbxproj::pest::PBXProjectParser;
        use std::collections::HashMap;

        let file = PBXProjectParser::try_parse_from_file($path);
        if file.is_err() {
            println!("Error: {:#?}", file.as_ref().unwrap_err())
        }

        assert!(file.is_ok());
        file.unwrap()
            .try_remove_object("objects")
            .unwrap()
            .0
            .into_iter()
            .map(|(k, v)| (k, PBXObject::try_from(v).unwrap()))
            .collect::<HashMap<_, _>>()
    }};
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    macro_rules! test_samples {
        ($($name:ident),*) => {
            $(#[test]
                fn $name() {
                    let (root, name) = (env!("CARGO_MANIFEST_DIR"), stringify!($name));
                    get_objects!(format!("{root}/tests/samples/{name}.pbxproj"));
                })*
        };
    }

    test_samples![demo1, demo2, demo3, demo4, demo5, demo6, demo7, demo8, demo9];

    #[test]
    fn test_object_creation() {
        use crate::pbxproj::pest::PBXProjectParser;
        let mut file =
            PBXProjectParser::try_from_str(include_str!("../../../tests/samples/demo1.pbxproj"))
                .unwrap();
        let objects = file
            .try_remove_object("objects")
            .unwrap()
            .0
            .into_iter()
            .map(|(_, v)| PBXObject::try_from(v).unwrap())
            .collect::<Vec<_>>();

        println!("{objects:#?}")
    }
}
