use super::*;
use derive_deref_rs::Deref;

/// [`PBXObject`] specifying [`PBXTarget`] for shell scripts or only specifying dependencies.
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXAggregateTarget {
    #[deref]
    pub(crate) inner: PBXTarget,
}

impl PBXObjectExt for PBXAggregateTarget {
    fn from_hashmap(value: PBXHashMap, objects: Weak<RefCell<PBXObjectCollection>>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            inner: PBXObjectExt::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
