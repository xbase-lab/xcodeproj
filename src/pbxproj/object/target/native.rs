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

impl TryFrom<PBXHashMap> for PBXNativeTarget {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            product_install_path: value.remove_string("productInstallPath"),
            inner: PBXTarget::try_from(value)?,
        })
    }
}
