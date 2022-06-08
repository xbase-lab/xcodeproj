use std::collections::HashMap;

use crate::pbxproj::{PBXHashMap, PBXObject};

use super::XCRemoteSwiftPackageReference;

/// [`PBXObject`] represents swift package dependency
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCSwiftPackageProductDependency {
    /// Product name.
    pub product_name: String,
    /// Package reference.
    package_reference: Option<String>,
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
    /// Package the product dependency refers to.
    pub fn set_package_reference(&mut self) -> Option<String> {
        todo!()
    }
}

impl TryFrom<PBXHashMap> for XCSwiftPackageProductDependency {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            product_name: value.try_remove_string("productName")?,
            package_reference: value.remove_string("package"),
        })
    }
}
