use crate::pbxproj::PBXObjectCollection;

use super::{PBXTarget, PBXTargetPlatform};

/// Summary of target infomration
#[derive(Debug)]
pub struct PBXTargetInfo {
    /// Target's platform
    pub platform: PBXTargetPlatform,
    /// Target's configurations
    pub configurations: Vec<String>,
}

impl PBXTargetInfo {
    /// Create new target info object
    pub fn new<'a>(target: &'a PBXTarget, objects: &'a PBXObjectCollection) -> Self {
        let mut platform = PBXTargetPlatform::default();
        let mut configurations = vec![];

        if let Some(ref bclist) = target.build_configuration_list {
            configurations.extend(
                bclist
                    .build_configurations
                    .iter()
                    .map(|c| c.name.to_string()),
            );

            if let Some(sdkroot) = bclist.extract_sdkroot_from_children(objects) {
                platform = PBXTargetPlatform::from_sdk_root(sdkroot.as_str());
            } else {
                tracing::trace!("Find SDKROOT: Trying PBXProject Objects");
                let mut sdkroots = objects
                    .projects()
                    .into_iter()
                    .flat_map(|p| {
                        p.build_configuration_list
                            .extract_sdkroot_from_children(objects)
                    })
                    .collect::<Vec<_>>();

                sdkroots.dedup();

                if sdkroots.is_empty() {
                    tracing::trace!(
                        "Find SDKROOT: using target info nor PBXPRoject data {:?}",
                        target.name
                    );
                }

                let sdkroot = &sdkroots[0];
                if sdkroots.len() > 1 {
                    tracing::trace!("Find SDKROOT: Get more then one sdkroot  {:?}", target.id);
                    tracing::trace!("Find SDKROOT Using {:?} as sdkroot", &sdkroots[0]);
                }
                platform = PBXTargetPlatform::from_sdk_root(sdkroot.as_str());
            }
        }

        Self {
            platform,
            configurations,
        }
    }
}
