use crate::pbxproj::*;

/// [`PBXObject`] A File referenced by a build phase, unique to each build phase.
#[derive(Default, Debug, derive_new::new)]
pub struct PBXBuildFile {
    /// Element settings
    pub settings: Option<PBXValue>,
    /// Platform filter attribute.
    pub platform_filter: Option<String>,
    /// Element file reference.
    file_reference: Option<String>,
    /// Product reference.
    product_reference: Option<String>,
    /// The cached build phase this build file belongs to
    build_phase_reference: Option<String>,
    objects: WeakPBXObjectCollection,
}

impl PBXBuildFile {
    /// Returns the file the build file refers to.
    pub fn get_file(&self, _data: &PBXRootObject) -> Option<&PBXObject> {
        // fileReference?.getObject()
        todo!()
    }

    /// Create new [`PBXBuildFile`] from  product_reference only
    pub fn new_from_product(
        product_reference: String,
        objects: Option<WeakPBXObjectCollection>,
    ) -> Self {
        Self {
            settings: Default::default(),
            platform_filter: Default::default(),
            file_reference: Default::default(),
            product_reference: Some(product_reference),
            build_phase_reference: Default::default(),
            objects: objects.unwrap_or_default(),
        }
    }

    /// Returns the file the build file refers to.
    pub fn set_file_reference(&mut self, reference: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.file_reference, reference)
    }

    /// Get Product using self.product_reference
    pub fn get_product(&self, _data: &PBXRootObject) -> Option<XCSwiftPackageProductDependency> {
        // productReference?.getObject()
        todo!()
    }

    /// Set Product.
    pub fn set_product_reference(&mut self, reference: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.file_reference, reference)
    }

    /// Set the pbxbuild file's build phase reference.
    pub fn set_build_phase_reference(&mut self, reference: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.build_phase_reference, reference)
    }

    /// Get a reference to the pbxbuild file's build phase reference.
    #[must_use]
    pub fn build_phase_reference(&self) -> Option<&String> {
        self.build_phase_reference.as_ref()
    }

    /// Get filename
    fn filename(&self, _data: &PBXRootObject) -> Option<String> {
        todo!()
    }

    /// Returns the type of the build phase the build file belongs to.
    fn build_phase(&self, _data: &PBXRootObject) -> Option<&PBXObject> {
        todo!()
    }
}

impl PBXObjectExt for PBXBuildFile {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            settings: value.remove_value("settings"),
            platform_filter: value.remove_string("platformFilter"),
            file_reference: value.remove_string("fileRef"),
            product_reference: value.remove_string("productRef"),
            build_phase_reference: value.remove_string("buildPhaseReference"),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
