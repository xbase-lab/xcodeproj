mod dependency;
use crate::pbxproj::WeakPBXObjectCollection;
use std::{cell::RefCell, rc::Weak};

use anyhow::Result;
pub use dependency::*;
mod aggregated;
pub use aggregated::*;
mod legacy;
pub use legacy::*;
mod native;
pub use native::*;

use super::{product_type::PBXProductType, PBXObjectCollection, PBXObjectExt};
use crate::pbxproj::{PBXHashMap, PBXRootObject};

/// `Abstraction` for building a specific targets (a library, binary, or test).
///
/// used in [`PBXAggregateTarget`], [`PBXNativeTarget`], [`PBXLegacyTarget`]
#[derive(Debug, derive_new::new)]
pub struct PBXTarget {
    /// Target name.
    pub name: Option<String>,
    /// Target product name.
    pub product_name: Option<String>,
    /// Target product type.
    pub product_type: PBXProductType,
    /// Target build configuration list.
    pub(crate) build_configuration_list_reference: Option<String>,
    /// Target build phase references.
    pub(crate) build_phase_references: Vec<String>,
    /// Target build rule references.
    pub(crate) build_rule_references: Vec<String>,
    /// Target dependency references.
    pub(crate) target_dependency_references: Vec<String>,
    /// Target product reference.
    pub(crate) product_reference: Option<String>,
    /// Swift package product references.
    pub(crate) package_product_dependency_references: Vec<String>,
    objects: WeakPBXObjectCollection,
}

impl PBXTarget {
    /// Get build configuration list from data for current target
    pub fn get_build_configuration_list(&self, _data: &PBXRootObject) -> () {}

    /// Set build configuration list reference
    pub fn set_build_configuration_list(&mut self, reference: Option<String>) {
        self.build_configuration_list_reference = reference;
    }

    /// Get build phases from data for current target
    pub fn get_build_phases(&self, _data: &PBXRootObject) -> () {}

    /// Set the target's build phase references.
    pub fn set_build_phases(&mut self, references: Vec<String>) {
        self.build_phase_references = references;
    }

    /// Get build phases from data for current target
    pub fn get_build_rules(&self, _data: &PBXRootObject) -> () {}

    /// Set the target's build rule references.
    pub fn set_build_rule_references(&mut self, build_rule_references: Vec<String>) {
        self.build_rule_references = build_rule_references;
    }

    /// Get target dependences from data for current target
    pub fn get_target_dependences(&self, _data: &PBXRootObject) -> () {}

    /// Set the target's dependency references.
    pub fn set_target_dependency_references(&mut self, target_dependency_references: Vec<String>) {
        self.target_dependency_references = target_dependency_references;
    }

    /// Set the target's package product dependency references.
    pub fn set_package_product_dependency_references(
        &mut self,
        package_product_dependency_references: Vec<String>,
    ) {
        self.package_product_dependency_references = package_product_dependency_references;
    }

    /// Set the target's product reference.
    pub fn set_product_reference(&mut self, product_reference: Option<String>) {
        self.product_reference = product_reference;
    }
}

impl PBXObjectExt for PBXTarget {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            name: value.remove_string("name"),
            product_name: value.remove_string("productName"),
            product_type: value
                .try_remove_string("productType")
                .unwrap_or_default()
                .into(),
            build_configuration_list_reference: value.remove_string("buildConfigurationList"),
            build_phase_references: value
                .try_remove_vec("buildPhases")?
                .try_into_vec_strings()?,
            build_rule_references: value
                .remove_vec("buildRules")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten()
                .unwrap_or_default(),
            target_dependency_references: value
                .try_remove_vec("dependencies")?
                .try_into_vec_strings()?,
            product_reference: value.remove_string("productReference"),
            package_product_dependency_references: value
                .remove_vec("packageProductDependencies")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten()
                .unwrap_or_default(),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
