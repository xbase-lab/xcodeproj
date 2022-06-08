use super::*;
use derive_deref_rs::Deref;

/// [`PBXObject`] specifying [`PBXTarget`] for shell scripts or only specifying dependencies.
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXAggregateTarget {
    #[deref]
    inner: PBXTarget,
}

impl TryFrom<PBXHashMap> for PBXAggregateTarget {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: PBXTarget::try_from(value)?,
        })
    }
}
