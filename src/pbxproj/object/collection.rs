use super::PBXObject;
use std::{cell::RefCell, collections::HashMap, rc::Weak};

/// An alias for weak reference of [`PBXObjectCollection`]
pub type WeakPBXObjectCollection = Weak<RefCell<PBXObjectCollection>>;

/// [`PBXObject`] storage with convenient helper methods
#[derive(Default, Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXObjectCollection(pub(crate) HashMap<String, PBXObject>);

impl PBXObjectCollection {
    /// Add new object. same as insert but it auto create id and returns it
    pub fn push<O: Into<PBXObject>>(&mut self, object: O) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.insert(id.clone(), object.into());
        id
    }

    pub(crate) fn set_inner(&mut self, map: HashMap<String, PBXObject>) {
        self.0 = map;
    }

    // /// Get PBXTarget by the target name
    // pub fn get_target_by_name<'a>(
    //     &'a self,
    //     target_name: &'a str,
    // ) -> Option<(&'a String, Ref<'a, PBXObject>)> {
    //     self.0
    //         .iter()
    //         .find(|(_, &o)| {
    //             let target = match *o.borrow() {
    //                 PBXObject::PBXAggregateTarget(ref v) => &v.inner,
    //                 PBXObject::PBXLegacyTarget(ref v) => &v.inner,
    //                 PBXObject::PBXNativeTarget(ref v) => &v.inner,
    //                 _ => return false,
    //             };
    //             if let Some(name) = target.name.as_ref() {
    //                 name == target_name
    //             } else {
    //                 false
    //             }
    //         })
    //         .map(|(key, o)| (key, o.borrow()))
    // }

    // /// Get PBXTarget by reference
    // pub fn get_target(&self, reference: &str) -> Option<&PBXObject> {
    //     if let Some(object) = self.get(reference) {
    //         if object.borrow().is_pbx_target() {
    //             return Some(Rc::downgrade(object));
    //         }
    //         None
    //     } else {
    //         None
    //     }
    // }

    // /// Get mutable PBXTarget by reference
    // pub fn get_target_mut(&mut self, reference: &str) -> Option<&mut PBXObject> {
    //     if let Some(object) = self.get_mut(reference) {
    //         if object.is_pbx_target() {
    //             return Some(object);
    //         }
    //         None
    //     } else {
    //         None
    //     }
    // }
}
