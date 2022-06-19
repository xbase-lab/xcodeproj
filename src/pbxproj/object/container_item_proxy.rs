use crate::pbxproj::*;

/// [`PBXObject`] that reference another object used by [`PBXTargetDependency`]
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
/// [`PBXTargetDependency`]: crate::pbxproj::PBXTargetDependency
#[derive(Debug, derive_new::new)]
pub struct PBXContainerItemProxy<'a> {
    /// ID Reference
    pub id: String,
    /// The object is a reference to a PBXProject, if proxy is for the object located in current .xcodeproj, otherwise PBXFileReference.
    pub container_portal_reference: &'a String,
    /// Element proxy type.
    pub proxy_type: Option<PBXProxyType>,
    /// Element remote global ID reference. ID of the proxied object.
    pub remote_global_id_reference: Option<&'a String>,
    /// Element remote info.
    pub remote_info: Option<&'a String>,
}

impl<'a> AsPBXObject<'a> for PBXContainerItemProxy<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        _objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            id,
            container_portal_reference: value.try_get_string("containerPortal")?,
            proxy_type: value
                .get_value("proxyType")
                .map(|v| v.try_into().ok())
                .flatten(),
            remote_global_id_reference: value.get_string("remoteGlobalIdString"),
            remote_info: value.get_string("remoteInfo"),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
/// [`PBXContainerItemProxy`] Type
pub enum PBXProxyType {
    /// Native Target
    NativeTarget,
    /// Reference
    Reference,
    /// Other
    Other(u8),
}

impl TryFrom<&PBXValue> for PBXProxyType {
    type Error = anyhow::Error;

    fn try_from(value: &PBXValue) -> Result<Self, Self::Error> {
        Ok(
            match value
                .as_number()
                .ok_or_else(|| anyhow::anyhow!("No pbx product type foudn"))?
            {
                1 => Self::NativeTarget,
                2 => Self::Reference,
                o => Self::Other(*o as u8),
            },
        )
    }
}

