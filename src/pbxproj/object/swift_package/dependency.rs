use crate::pbxproj::*;
use std::collections::HashMap;

/// [`PBXObject`] represents swift package dependency
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCSwiftPackageProductDependency {
    /// Product name.
    pub product_name: String,
    /// Package reference.
    package_reference: Option<String>,
    objects: WeakPBXObjectCollection,
}

impl XCSwiftPackageProductDependency {
    /// Package the product dependency refers to.
    pub fn get_package(
        &self,
        _root: &HashMap<String, PBXObject>,
    ) -> Option<XCRemoteSwiftPackageReference> {
        // root.get(root).map(|o| o.try_into());
        None
    }

    /// Get a reference to the xcswift package product dependency's package reference.
    #[must_use]
    pub fn package_reference(&self) -> Option<&String> {
        self.package_reference.as_ref()
    }

    /// Set the xcswift package product dependency's package reference.
    pub fn set_package_reference(&mut self, package_reference: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.package_reference, package_reference)
    }
}

impl PBXObjectExt for XCSwiftPackageProductDependency {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            product_name: value.try_remove_string("productName")?,
            package_reference: value.remove_string("package"),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
