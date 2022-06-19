use crate::pbxproj::*;

/// [`PBXObject`] referencing other targets through content proxies.
#[derive(Debug, derive_new::new)]
pub struct PBXTargetDependency<'a> {
    /// ID Reference
    pub id: String,
    /// Target name.
    pub name: Option<&'a String>,
    /// Platform filter attribute.
    pub platform_filter: Option<&'a String>,
    /// Target
    pub target: Option<PBXTarget<'a>>,
    /// Target proxy
    pub target_proxy: Option<PBXContainerItemProxy<'a>>,
    /// Product reference.
    pub product: Option<XCSwiftPackageProductDependency<'a>>,
}

impl<'a> AsPBXObject<'a> for PBXTargetDependency<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            id,
            name: value.get_string("name"),
            platform_filter: value.get_string("platformFilter"),
            target: value.get_string("target").and_then(|key| objects.get(key)),
            target_proxy: value
                .get_string("targetProxy")
                .and_then(|key| objects.get(key)),
            product: value
                .get_string("productRef")
                .and_then(|key| objects.get(key)),
        })
    }
}
