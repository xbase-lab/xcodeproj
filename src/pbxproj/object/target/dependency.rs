use super::PBXTarget;
use crate::pbxproj::PBXContainerItemProxy;
use crate::pbxproj::PBXHashMap;
use crate::pbxproj::PBXObject;
use crate::pbxproj::PBXRootObject;
use crate::pbxproj::XCSwiftPackageProductDependency;

/// [`PBXObject`] referencing other targets through content proxies.
#[derive(Debug, derive_new::new)]
pub struct PBXTargetDependency {
    /// Target name.
    pub name: Option<String>,
    /// Platform filter attribute.
    /// Introduced in: Xcode 11
    pub platform_filter: Option<String>,
    /// Target reference.
    target_reference: Option<String>,
    /// Target proxy reference.
    target_proxy_reference: Option<String>,
    /// Product reference.
    product_reference: Option<String>,
}

impl PBXTargetDependency {
    /// Target.
    pub fn target(&self, _data: &PBXRootObject) -> Option<PBXObject> {
        // targetReference?.getObject()
        todo!()
    }

    /// Target.
    pub fn set_target(&mut self) -> Option<PBXTarget> {
        todo!()
    }
    /// Target proxy.
    pub fn target_proxy(&self) -> Option<PBXContainerItemProxy> {
        // targetProxyReference?.getObject()
        todo!()
    }
    /// Target proxy.
    pub fn set_target_proxy(&mut self) -> Option<PBXContainerItemProxy> {
        todo!()
    }

    /// Product.
    pub fn set_product(&mut self) -> Option<XCSwiftPackageProductDependency> {
        todo!()
    }

    /// Product.
    pub fn product(&self) -> Option<XCSwiftPackageProductDependency> {
        // productReference?.getObject()
        todo!()
    }
}

impl TryFrom<PBXHashMap> for PBXTargetDependency {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.remove_string("name"),
            platform_filter: value.remove_string("platformFilter"),
            target_reference: value.remove_string("target"),
            target_proxy_reference: value.remove_string("targetProxy"),
            product_reference: value.remove_string("productRef"),
        })
    }
}
