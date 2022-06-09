use crate::pbxproj::*;

/// [`PBXObject`] aggregating a list of [`XCBuildConfiguration`] references
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCConfigurationList {
    /// Element build configurations.
    build_configuration_references: Vec<String>,
    /// Element default configuration is visible.
    pub default_configuration_is_visible: bool,
    /// Element default configuration name
    pub default_configuration_name: Option<String>,
    objects: WeakPBXObjectCollection,
}

impl XCConfigurationList {
    /// Build configurations
    pub fn set_build_configuration_references(&mut self, references: Vec<String>) -> Vec<String> {
        let old = std::mem::replace(&mut self.build_configuration_references, references);
        old
    }

    /// Build configurations
    // pub fn get_build_configurations<'a>(
    //     &'a self,
    //     data: &'a PBXRootObject,
    // ) -> Vec<&'a XCBuildConfiguration> {
    //     self.build_configuration_references
    //         .iter()
    //         .map(|r| Some(data.get(r)?.borrow().as_xc_build_configuration()?))
    //         .flatten()
    //         .collect()
    // }

    /// Returns the build configuration with the given name (if it exists)
    // pub fn get_configuration_by_name<'a>(
    //     &'a self,
    //     data: &'a PBXRootObject,
    //     name: &'a str,
    // ) -> Option<&'a XCBuildConfiguration> {
    //     self.get_build_configurations(data)
    //         .into_iter()
    //         .find(|o| &o.name == name)
    // }

    /// Adds the default configurations, debug and release
    // pub fn add_default_configurations(&mut self, data: &mut PBXRootObject) {
    //     let mut configurations = vec![];
    //     let debug = XCBuildConfiguration::new("Debug".into(), Default::default(), None);
    //     let debug_id = data.push(debug);

    //     configurations.push(debug_id);

    //     let release = XCBuildConfiguration::new("Release".into(), Default::default(), None);
    //     let release_id = data.push(release);

    //     configurations.push(release_id);

    //     self.build_configuration_references.extend(configurations);
    // }

    /// Returns the object with the given configuration list (project or target)
    pub fn object_with_configuration_list(&self, _data: &PBXRootObject) -> Option<&PBXObject> {
        // projects, Native target, aggregateTargets, legacyTargets build_configuration_list_reference

        // data.iter().find(|o| {
        //     match o {
        //         PBXObject::PBXProject(p) => p
        //     }
        // });
        todo!()
    }
}

impl PBXObjectExt for XCConfigurationList {
    fn from_hashmap(
        mut value: PBXHashMap,
        objects: crate::pbxproj::WeakPBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            build_configuration_references: value
                .try_remove_vec("buildConfigurations")?
                .try_into_vec_strings()?,
            default_configuration_is_visible: value
                .try_remove_number("defaultConfigurationIsVisible")?
                == 1,
            default_configuration_name: value.remove_string("defaultConfigurationName"),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
