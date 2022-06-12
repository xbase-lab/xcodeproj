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
    /// Abstraction over `PBXAggregateTarget`, `PBXLegacyTarget`, and `PBXNativeTarget`
    PBXTarget(Rc<RefCell<PBXTarget>>),
    /// Abstraction over Build phases
    PBXBuildPhase(Rc<RefCell<PBXBuildPhase>>),
    /// Abstraction over PBXFileReference and PBX*Group
    PBXFSReference(Rc<RefCell<PBXFSReference>>),
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
    /// A Kind representing a build target that produces a binary content (application or library).
    PBXProject(Rc<RefCell<PBXProject>>),
    /// A Kind representing an abstract parent for specialized targets.
    XCRemoteSwiftPackageReference(Rc<RefCell<XCRemoteSwiftPackageReference>>),
    /// A Kind representing an abstract parent for specialized targets.
    XCSwiftPackageProductDependency(Rc<RefCell<XCSwiftPackageProductDependency>>),
    /// A Kind representing a reference to other targets through content proxies.
    PBXTargetDependency(Rc<RefCell<PBXTargetDependency>>),
}

impl PBXObject {
    /// Create new [`PBXObject`]
    pub fn new(value: PBXValue, objects: WeakPBXObjectCollection) -> Result<Self> {
        macro_rules! into {
            ($var:ident, $map:ident, $objects:ident) => {
                PBXObject::$var(Rc::new(RefCell::new(PBXObjectExt::from_hashmap(
                    $map, $objects,
                )?)))
            };
        }

        let map = value.try_into_object()?;
        let kind = if let Some(kind) = map.get_kind("isa") {
            kind
        } else {
            anyhow::bail!("isa field isn't defined: {map:#?}");
        };

        Ok(match kind {
            PBXObjectKind::XCBuildConfiguration => into!(XCBuildConfiguration, map, objects),
            PBXObjectKind::PBXBuildFile => into!(PBXBuildFile, map, objects),
            PBXObjectKind::PBXBuildRule => into!(PBXBuildRule, map, objects),
            PBXObjectKind::XCConfigurationList => into!(XCConfigurationList, map, objects),
            PBXObjectKind::PBXContainerItemProxy => into!(PBXContainerItemProxy, map, objects),
            PBXObjectKind::PBXProject => into!(PBXProject, map, objects),
            PBXObjectKind::XCRemoteSwiftPackageReference => {
                into!(XCRemoteSwiftPackageReference, map, objects)
            }
            PBXObjectKind::XCSwiftPackageProductDependency => {
                into!(XCSwiftPackageProductDependency, map, objects)
            }
            PBXObjectKind::PBXTargetDependency => into!(PBXTargetDependency, map, objects),
            PBXObjectKind::PBXFSReference(_) => into!(PBXFSReference, map, objects),

            PBXObjectKind::PBXTarget(_) => into!(PBXTarget, map, objects),
            PBXObjectKind::PBXBuildPhase(_) => into!(PBXBuildPhase, map, objects),
            kind => anyhow::bail!("{kind:?} isn't supported"),
        })
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
