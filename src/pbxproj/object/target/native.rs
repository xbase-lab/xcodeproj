use super::*;
use derive_deref_rs::Deref;

/// [`PBXObject`] specifying [`PBXTarget`] producing a binary content (application or library).
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXNativeTarget {
    /// Target product install path.
    pub product_install_path: Option<String>,
    #[deref]
    pub(crate) inner: PBXTarget,
}

impl PBXNativeTarget {
    /// Set the pbxnative target's product install path.
    pub fn set_product_install_path(&mut self, product_install_path: Option<String>) {
        self.product_install_path = product_install_path;
    }
}

impl PBXObjectExt for PBXNativeTarget {
    fn from_hashmap(
        mut value: PBXHashMap,
        objects: Weak<RefCell<PBXObjectCollection>>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            product_install_path: value.remove_string("productInstallPath"),
            inner: PBXObjectExt::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
