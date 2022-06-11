use super::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

/// An alias for weak reference of [`PBXObjectCollection`]
pub type WeakPBXObjectCollection = Weak<RefCell<PBXObjectCollection>>;

/// [`PBXObject`] storage with convenient helper methods
#[derive(Default, Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXObjectCollection(pub(crate) HashMap<String, PBXObject>);

/// TODO: make collections a HashSet of PBXObject with identifier included?
impl PBXObjectCollection {
    pub(crate) fn set_inner(&mut self, map: HashMap<String, PBXObject>) {
        self.0 = map;
    }

    /// Add new object. same as insert but it auto create id and returns it
    pub fn push<O: Into<PBXObject>>(&mut self, object: O) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.insert(id.clone(), object.into());
        id
    }

    /// Get PBXTarget by reference
    pub fn get_target<'a>(&'a self, reference: &str) -> Option<Rc<RefCell<PBXTarget>>> {
        self.get(reference)?.as_pbx_target().map(|r| r.clone())
    }

    /// Get PBXBuildPhase by reference
    pub fn get_build_phase<'a>(&'a self, reference: &str) -> Option<Rc<RefCell<PBXBuildPhase>>> {
        self.get(reference)?.as_pbx_build_phase().map(|r| r.clone())
    }

    /// Get PBXBuildFile by reference
    pub fn get_build_file<'a>(&'a self, reference: &str) -> Option<Rc<RefCell<PBXBuildFile>>> {
        self.get(reference)?.as_pbx_build_file().map(|r| r.clone())
    }

    /// Get all PBXBuildPhase
    pub fn build_phases<'a>(&'a self) -> Vec<(String, Rc<RefCell<PBXBuildPhase>>)> {
        self.iter()
            .filter(|o| o.1.is_pbx_build_phase())
            .map(|(k, o)| (k.clone(), o.as_pbx_build_phase().unwrap().clone()))
            .collect()
    }

    /// Get all PBXBuildFile
    pub fn build_files<'a>(&'a self) -> Vec<(String, Rc<RefCell<PBXBuildFile>>)> {
        self.iter()
            .filter(|o| o.1.is_pbx_build_file())
            .map(|(k, o)| (k.clone(), o.as_pbx_build_file().unwrap().clone()))
            .collect()
    }

    /// Get All XCSwiftPackageProductDependency Objects
    pub fn swift_package_product_dependencies<'a>(
        &'a self,
    ) -> Vec<(String, Rc<RefCell<XCSwiftPackageProductDependency>>)> {
        self.iter()
            .map(|(k, v)| {
                Some((
                    k.clone(),
                    v.as_xc_swift_package_product_dependency()?.clone(),
                ))
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Get All XCRemoteSwiftPackageReference Objects
    pub fn swift_package_references<'a>(
        &'a self,
    ) -> Vec<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)> {
        self.iter()
            .map(|(k, v)| Some((k.clone(), v.as_xc_remote_swift_package_reference()?.clone())))
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Get XCSwiftPackageProductDependency by reference
    pub fn get_swift_package_product_dependency<'a>(
        &'a self,
        object_reference: &str,
    ) -> Option<Rc<RefCell<XCSwiftPackageProductDependency>>> {
        self.get(object_reference)?
            .as_xc_swift_package_product_dependency()
            .map(|r| r.clone())
    }

    /// Get XCSwiftPackageProductDependency by reference
    pub fn get_swift_package_reference<'a>(
        &'a self,
        object_reference: &str,
    ) -> Option<Rc<RefCell<XCRemoteSwiftPackageReference>>> {
        self.get(object_reference)?
            .as_xc_remote_swift_package_reference()
            .map(|r| r.clone())
    }

    /// Get PBXTarget from a vec of references
    pub fn get_targets_from_references<'a>(
        &'a self,
        references: &Vec<String>,
    ) -> Vec<(String, Rc<RefCell<PBXTarget>>)> {
        references
            .iter()
            .map(|id| Some((id.clone(), self.get_target(id)?)))
            .flatten()
            .collect()
    }

    /// Get PBXTarget by the target name
    pub fn get_target_by_name<'a>(
        &'a self,
        target_name: &'a str,
    ) -> Option<(String, Rc<RefCell<PBXTarget>>)> {
        self.iter()
            .find(|(_, o)| {
                if let Some(target) = o.as_pbx_target() {
                    if let Some(name) = target.borrow().name.as_ref() {
                        name == target_name
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .map(|(key, o)| (key.clone(), o.as_pbx_target().unwrap().clone()))
    }

    /// Get XCRemoteSwiftPackageReference from a vec of references
    pub fn get_swift_package_reference_from_references<'a>(
        &'a self,
        references: &Vec<String>,
    ) -> Vec<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)> {
        references
            .iter()
            .map(|reference| {
                Some((
                    reference.clone(),
                    self.get_swift_package_reference(reference)?,
                ))
            })
            .flatten()
            .collect()
    }

    /// Get PBXBuildPhase from a vec of references
    pub fn get_build_phases_from_reference<'a>(
        &'a self,
        references: &Vec<String>,
    ) -> Vec<(String, Rc<RefCell<PBXBuildPhase>>)> {
        references
            .iter()
            .map(|reference| Some((reference.clone(), self.get_build_phase(reference)?)))
            .flatten()
            .collect()
    }

    /// Get XCSwiftPackageProductDependency form a given target reference
    pub fn get_product_dependency_from_target_reference<'a>(
        &'a self,
        target_reference: &str,
    ) -> Option<(String, Rc<RefCell<XCSwiftPackageProductDependency>>)> {
        self.swift_package_product_dependencies()
            .into_iter()
            .find(|(_, p)| {
                p.borrow()
                    .package_reference()
                    .map(|v| v == target_reference)
                    .unwrap_or_default()
            })
    }
}
