use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::pbxproj::{PBXHashMap, PBXValue};
use anyhow::Result;
use derive_is_enum_variant::is_enum_variant;
use enum_as_inner::EnumAsInner;
use enum_variant_macros::FromVariants;

/// PBX Object Representation
#[derive(Clone, Debug, FromVariants, EnumAsInner, is_enum_variant)]
pub enum PBXObject {
    /// A Target backed by shell scripts or nothing (only specifying dependencies).
    PBXAggregateTarget(Rc<RefCell<PBXAggregateTarget>>),
    /// A Kind for defining build configurations
    XCBuildConfiguration(Rc<RefCell<XCBuildConfiguration>>),
    /// File referenced by a build phase, unique to each build phase.
    PBXBuildFile(Rc<RefCell<PBXBuildFile>>),
    /// Specification of input transform to an output file(s).
    PBXBuildRule(Rc<RefCell<PBXBuildRule>>),
    /// List of build configurations.
    XCConfigurationList(Rc<RefCell<XCConfigurationList>>),
    /// A Kind representing Decoration for a target element
    PBXContainerItemProxy(Rc<RefCell<PBXContainerItemProxy>>),
    /// A Kind representing the copy file build phase
    PBXCopyFilesBuildPhase(Rc<RefCell<PBXCopyFilesBuildPhase>>),
    /// A Kind representing to track every external file referenced by the project: source files,
    /// resource files, libraries, generated application files, and so on.
    PBXFileReference(Rc<RefCell<PBXFileReference>>),
    /// A Kind representing a framework link build phase
    PBXFrameworksBuildPhase(Rc<RefCell<PBXFrameworksBuildPhase>>),
    /// A Kind representing group files
    PBXGroup(Rc<RefCell<PBXGroup>>),
    /// A Kind representing the header link build phase
    PBXHeadersBuildPhase(Rc<RefCell<PBXHeadersBuildPhase>>),
    /// A Kind representing a build target that according to Xcode is an "External Build System".
    PBXLegacyTarget(Rc<RefCell<PBXLegacyTarget>>),
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXNativeTarget(Rc<RefCell<PBXNativeTarget>>),
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXProject(Rc<RefCell<PBXProject>>),
    /// A Kind representing an abstract parent for specialized targets.
    XCRemoteSwiftPackageReference(Rc<RefCell<XCRemoteSwiftPackageReference>>),
    /// A Kind representing the resources copy build phase
    PBXResourcesBuildPhase(Rc<RefCell<PBXResourcesBuildPhase>>),
    /// A Kind representing the Build Carbon Resources build phase
    PBXRezBuildPhase(Rc<RefCell<PBXRezBuildPhase>>),
    /// A Kind representing shell script build phase.
    PBXShellScriptBuildPhase(Rc<RefCell<PBXShellScriptBuildPhase>>),
    /// A Kind representing the sources compilation build phase.
    PBXSourcesBuildPhase(Rc<RefCell<PBXSourcesBuildPhase>>),
    /// A Kind representing an abstract parent for specialized targets.
    XCSwiftPackageProductDependency(Rc<RefCell<XCSwiftPackageProductDependency>>),
    /// A Kind representing a reference to other targets through content proxies.
    PBXTargetDependency(Rc<RefCell<PBXTargetDependency>>),
    /// a Kind representing a reference localized resources.
    PBXVariantGroup(Rc<RefCell<PBXVariantGroup>>),
    /// Kind representing  Group that contains multiple files references to the different versions
    /// of a resource. Used to contain the different versions of a xcdatamodel
    XCVersionGroup(Rc<RefCell<XCVersionGroup>>),
}

impl PBXObject {
    /// Create new [`PBXObject`]
    pub fn new(value: PBXValue, objects: WeakPBXObjectCollection) -> Result<Self> {
        macro_rules! kind_to_object {
    ($kind:ident, $objects:ident, $value:ident, [$($variant:ident),*]) => {
        match $kind {
            $(PBXObjectKind::$variant => PBXObject::$variant(Rc::new(RefCell::new(PBXObjectExt::from_hashmap($value, $objects)?))),)*
                kind => anyhow::bail!("{kind:?} isn't supported")
        }
    };
}

        let map = value.try_into_object()?;
        let kind = if let Some(kind) = map.get_kind("isa") {
            kind
        } else {
            anyhow::bail!("isa field isn't defined: {map:#?}");
        };
        Ok(kind_to_object!(
            kind,
            objects,
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

    /// Whether the object is a PBXTarget
    pub fn is_pbx_target(&self) -> bool {
        self.is_pbx_aggregate_target() || self.is_pbx_native_target() || self.is_pbx_legacy_target()
    }
}

/// Process [`PBXObject`]
pub trait PBXObjectExt {
    /// Create from [`PBXHashMap`]
    fn from_hashmap(value: PBXHashMap, objects: WeakPBXObjectCollection) -> Result<Self>
    where
        Self: Sized;
    /// generate [`PBXHashMap`]
    fn to_hashmap(&self) -> PBXHashMap;
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
        let ref_object = Rc::new(RefCell::new(Default::default()));

        assert!(file.is_ok());
        file.unwrap()
            .try_remove_object("objects")
            .unwrap()
            .0
            .into_iter()
            .map(|(k, v)| (k, PBXObject::new(v, Rc::downgrade(&ref_object)).unwrap()))
            .collect::<HashMap<_, _>>()
    }};
}

#[cfg(test)]
mod parse_tests {
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
        let objects = get_objects!(format!(
            "{}/tests/samples/demo1.pbxproj",
            env!("CARGO_MANIFEST_DIR")
        ));

        println!("{objects:#?}")
    }
}
