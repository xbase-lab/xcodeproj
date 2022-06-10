use super::{PBXObject, PBXTarget, XCRemoteSwiftPackageReference};
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

    /// Get XCRemoteSwiftPackageReference from a vec of references
    pub fn get_packages_from_references<'a>(
        &'a self,
        references: &Vec<String>,
    ) -> Vec<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)> {
        references
            .iter()
            .map(|id| {
                let package = self.get(id)?.as_xc_remote_swift_package_reference()?;
                Some((id.clone(), package.clone()))
            })
            .flatten()
            .collect()
    }

    /// Get PBXTarget from a vec of references
    pub fn get_targets_from_references<'a>(
        &'a self,
        references: &Vec<String>,
    ) -> Vec<(String, Rc<RefCell<PBXTarget>>)> {
        references
            .iter()
            .map(|id| {
                let target = self.get_target(id)?;
                Some((id.clone(), target.clone()))
            })
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
}
