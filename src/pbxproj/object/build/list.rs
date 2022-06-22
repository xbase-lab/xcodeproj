use crate::pbxproj::*;

/// [`PBXObject`] aggregating a list of [`XCBuildConfiguration`] references
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct XCConfigurationList<'a> {
    /// ID Reference
    pub id: String,
    /// Element build configurations.
    pub build_configurations: Vec<XCBuildConfiguration<'a>>,
    /// Element default configuration is visible.
    pub default_configuration_is_visible: bool,
    /// Element default configuration name
    pub default_configuration_name: Option<&'a String>,
}
impl<'a> AsPBXObject<'a> for XCConfigurationList<'a> {
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
            build_configurations: value
                .get_vec("buildConfigurations")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            default_configuration_is_visible: value
                .try_get_number("defaultConfigurationIsVisible")?
                == &1,
            default_configuration_name: value.get_string("defaultConfigurationName"),
        })
    }
}
impl<'a> XCConfigurationList<'a> {
    /// Extract SDKROOT from build configurations
    pub fn extract_sdkroot_from_children(&self, objects: &PBXObjectCollection) -> Option<String> {
        let mut sdkroots = self
            .build_configurations
            .iter()
            .flat_map(|b| b.build_settings.get_string("SDKROOT"))
            .collect::<Vec<&String>>();

        sdkroots.dedup();

        if sdkroots.is_empty() {
            tracing::trace!(
                "no sdkroot found in build_configuration_list: {:?}",
                self.id
            );

            // sdkroot isn't defined in current build settings.
            // Here, we need to find all build configurations sharing
            // the same base configuration id
            self.build_configurations
                .iter()
                .flat_map(|b| Some(b.base_configuration.as_ref()?.id.as_str()))
                .flat_map(|id| objects.get_build_configurations_by_base_id(id))
                .flat_map(|b| b.build_settings.get_string("SDKROOT"))
                .for_each(|root| sdkroots.push(root));

            // Means base configuration no defined
            if sdkroots.is_empty() {
                tracing::trace!(
                    "Find SDKROOT: No base configuration in all config_list's configuration",
                );
                return None;
            }
        } else if sdkroots.len() > 1 {
            tracing::trace!("Find SDKROOT: Get more then one sdkroot  {:?}", self.id);
            tracing::trace!("Find SDKROOT Using {:?} as sdkroot", &sdkroots[0]);
        }

        Some(sdkroots[0].into())
    }
}

// impl XCConfigurationList {
//     /// Build configurations
//     pub fn set_build_configuration_references(&mut self, references: Vec<String>) -> Vec<String> {
//         let old = std::mem::replace(&mut self.build_configuration_references, references);
//         old
//     }

//     /// Build configurations
//     // pub fn get_build_configurations<'a>(
//     //     &'a self,
//     //     data: &'a PBXRootObject,
//     // ) -> Vec<&'a XCBuildConfiguration> {
//     //     self.build_configuration_references
//     //         .iter()
//     //         .map(|r| Some(data.get(r)?.borrow().as_xc_build_configuration()?))
//     //         .flatten()
//     //         .collect()
//     // }

//     /// Returns the build configuration with the given name (if it exists)
//     // pub fn get_configuration_by_name<'a>(
//     //     &'a self,
//     //     data: &'a PBXRootObject,
//     //     name: &'a str,
//     // ) -> Option<&'a XCBuildConfiguration> {
//     //     self.get_build_configurations(data)
//     //         .into_iter()
//     //         .find(|o| &o.name == name)
//     // }

//     /// Adds the default configurations, debug and release
//     // pub fn add_default_configurations(&mut self, data: &mut PBXRootObject) {
//     //     let mut configurations = vec![];
//     //     let debug = XCBuildConfiguration::new("Debug".into(), Default::default(), None);
//     //     let debug_id = data.push(debug);

//     //     configurations.push(debug_id);

//     //     let release = XCBuildConfiguration::new("Release".into(), Default::default(), None);
//     //     let release_id = data.push(release);

//     //     configurations.push(release_id);

//     //     self.build_configuration_references.extend(configurations);
//     // }

//     /// Returns the object with the given configuration list (project or target)
//     pub fn object_with_configuration_list(&self, _data: &PBXRootObject) -> Option<&PBXObject> {
//         // projects, Native target, aggregateTargets, legacyTargets build_configuration_list_reference

//         // data.iter().find(|o| {
//         //     match o {
//         //         PBXObject::PBXProject(p) => p
//         //     }
//         // });
//         todo!()
//     }
// }
