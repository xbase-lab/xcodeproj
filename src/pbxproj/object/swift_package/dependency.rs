use crate::pbxproj::*;

/// [`PBXObject`] represents swift package dependency
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCSwiftPackageProductDependency<'a> {
    /// ID Reference
    pub id: String,
    /// Product name.
    pub product_name: &'a String,
    /// Package reference.
    pub package: Option<XCRemoteSwiftPackageReference<'a>>,
}

impl<'a> AsPBXObject<'a> for XCSwiftPackageProductDependency<'a> {
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
            product_name: value.try_get_string("productName")?,
            package: value.get_string("package").and_then(|key| objects.get(key)),
        })
    }
}
