use crate::pbxproj::*;
use derive_deref_rs::Deref;
use std::collections::HashSet;

/// [`PBXObject`] representing a collection of files in Xcode's VF hierarchy.
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXGroup {
    /// Group children references.
    children_references: HashSet<String>,
    inner: PBXFileElement,
}

impl PBXGroup {
    /// Group children.
    pub fn set_children_references(&mut self, references: HashSet<String>) -> HashSet<String> {
        let old = std::mem::replace(&mut self.children_references, references);
        old
    }

    // /// Group children.
    // pub fn children<'a>(&'a self, data: &'a PBXRootObject) -> Vec<Ref<'a, PBXObject>> {
    //     self.children_references
    //         .iter()
    //         .map(|r| Some(data.get(r)?.borrow()))
    //         .flatten()
    //         .collect::<Vec<_>>()
    // }

    /// Get a reference to the pbxgroup's children references.
    #[must_use]
    pub fn children_references(&self) -> &HashSet<String> {
        &self.children_references
    }
}

impl PBXObjectExt for PBXGroup {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            children_references: HashSet::from_iter(
                value.try_remove_vec("children")?.try_into_vec_strings()?,
            ),
            inner: PBXObjectExt::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}

/// Group Adding option variants
pub enum PBXGroupAddingOption {
    /// Group without a folder
    WithoutFolder,
}

/// [`PBXObject`] specifying [`PBXGroup`] representing localized resources
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXVariantGroup {
    inner: PBXGroup,
}

impl PBXObjectExt for PBXVariantGroup {
    fn from_hashmap(value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            inner: PBXObjectExt::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}

/// [`PBXObject`] specifying [`PBXGroup`] containing different versions of a resource
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct XCVersionGroup {
    /// Current version.
    current_version_reference: Option<String>,
    /// Version group type.
    pub version_group_type: Option<String>,
    #[deref]
    inner: PBXGroup,
}

impl XCVersionGroup {
    /// Returns the current version file reference.
    pub fn current_version(&self, _store: &PBXRootObject) -> Option<PBXFileReference> {
        // currentVersionReference?.getObject()
        todo!()
    }

    /// Set current version reference
    pub fn set_current_version_reference(&mut self, value: Option<String>) -> Option<String> {
        let old = std::mem::replace(&mut self.current_version_reference, value);
        old
    }
}

impl PBXObjectExt for XCVersionGroup {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            current_version_reference: value.remove_string("current_version"),
            version_group_type: value.remove_string("version_group_type"),
            inner: PBXObjectExt::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
