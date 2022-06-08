use crate::pbxproj::{PBXHashMap, PBXValue};

/// Item Proxy type used in [`PBXContainerItemProxy`]
#[derive(Debug)]
pub enum PBXProxyType {
    /// Native Target
    NativeTarget,
    /// Reference
    Reference,
    /// Other
    Other(u8),
}

impl TryFrom<PBXValue> for PBXProxyType {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        Ok(match value.try_into_number()? {
            1 => Self::NativeTarget,
            2 => Self::Reference,
            o => Self::Other(o as u8),
        })
    }
}

/// [`PBXObject`] that reference another object used by [`PBXTargetDependency`]
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
/// [`PBXTargetDependency`]: crate::pbxproj::PBXTargetDependency
#[derive(Debug, derive_new::new)]
pub struct PBXContainerItemProxy {
    /// The object is a reference to a PBXProject, if proxy is for the object located in current .xcodeproj, otherwise PBXFileReference.
    container_portal_reference: String,
    /// Element proxy type.
    pub proxy_type: Option<PBXProxyType>,
    /// Element remote global ID reference. ID of the proxied object.
    remote_global_id_reference: Option<String>,
    /// Element remote info.
    pub remote_info: Option<String>,
}

impl TryFrom<PBXHashMap> for PBXContainerItemProxy {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            container_portal_reference: value.try_remove_string("containerPortal")?,
            proxy_type: value
                .remove_value("proxyType")
                .map(|v| v.try_into().ok())
                .flatten(),
            remote_global_id_reference: value.remove_string("remoteGlobalIdString"),
            remote_info: value.remove_string("remoteInfo"),
        })
    }
}
