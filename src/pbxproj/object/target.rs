mod dependency;
mod info;
mod platform;
pub use dependency::*;

use anyhow::Result;

use crate::pbxproj::*;

pub use info::PBXTargetInfo;
pub use platform::PBXTargetPlatform;

/// `Abstraction` for building a specific targets (a library, binary, or test).
///
/// variants: `PBXAggregateTarget`, `PBXLegacyTarget`, and `PBXNativeTarget`
#[derive(Debug, derive_new::new)]
pub struct PBXTarget<'a> {
    /// ID Reference
    pub id: String,
    /// Target name.
    pub name: Option<&'a String>,
    /// Target product name.
    pub product_name: Option<&'a String>,
    /// Target product type.
    pub product_type: PBXProductType,
    /// Target build configuration list.
    pub build_configuration_list: Option<XCConfigurationList<'a>>,
    /// Target build phase references.
    pub build_phases: Vec<PBXBuildPhase<'a>>,
    /// Target build rule references.
    pub build_rules: Vec<PBXBuildRule<'a>>,
    /// Target dependency references.
    pub target_dependencies: Vec<PBXTargetDependency<'a>>,
    /// Target product reference.
    pub product: Option<PBXFSReference<'a>>,
    /// Swift package product references.
    pub package_product_dependencies: Vec<XCSwiftPackageProductDependency<'a>>,
    /// Target Kind
    pub kind: &'a PBXTargetKind,
    /// Target product install path. (relevant only for `PBXNativeTarget`)
    pub product_install_path: Option<&'a String>,
    /// Path to the build tool that is invoked (required) (relevant only for `PBXLegaecyTarget`)
    pub build_tool_path: Option<&'a String>,
    /// Build arguments to be passed to the build tool. (relevant only for `PBXLegaacyTarget`)
    pub build_arguments_string: Option<&'a String>,
    /// Whether or not to pass Xcode build settings as environment variables down to the tool when invoked (relevant only for `PBXLegaecyTarget`)
    pub pass_build_settings_in_environment: Option<bool>,
    /// The directory where the build tool will be invoked during a build
    pub build_working_directory: Option<&'a String>,
}

impl<'a> PBXTarget<'a> {
    /// Get `PBXTargetInfo`
    pub fn info(&'a self, objects: &'a PBXObjectCollection) -> PBXTargetInfo {
        PBXTargetInfo::new(self, objects)
    }
}

impl<'a> AsPBXObject<'a> for PBXTarget<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> Result<Self>
    where
        Self: Sized + 'a,
    {
        let kind = value
            .get_kind("isa")
            .and_then(|v| v.as_pbx_target())
            .unwrap();

        Ok(Self {
            id,
            name: value.get_string("name"),
            product_name: value.get_string("productName"),
            product_type: value.try_get_string("productType")?.as_str().into(),
            build_configuration_list: value
                .get_string("buildConfigurationList")
                .and_then(|key| objects.get(key)),
            build_phases: value
                .get_vec("buildPhases")
                .map(|vec| objects.get_vec(vec.as_vec_strings()))
                .unwrap_or_default(),
            build_rules: value
                .get_vec("buildRules")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            target_dependencies: value
                .get_vec("dependencies")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            product: value
                .get_string("productReference")
                .and_then(|key| objects.get(key)),
            package_product_dependencies: value
                .get_vec("packageProductDependencies")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            product_install_path: if kind.is_native() {
                value.get_string("productInstallPath")
            } else {
                None
            },
            kind,
            build_tool_path: value.get_string("buildToolPath"),
            build_arguments_string: value.get_string("buildArgumentsString"),
            pass_build_settings_in_environment: value
                .get_number("passBuildSettingsInEnvironment")
                .map(|n| n == &1),
            build_working_directory: value.get_string("buildWorkingDirectory"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::pbxproj::test_demo_file;
    macro_rules! test_samples {
        ($($name:ident),*) => {
            $(#[test]
              // #[tracing_test::traced_test]
                fn $name() {
                    let root_object = test_demo_file!($name);
                    for target in root_object.targets() {
                        let platform = target.info(&root_object);
                        println!("[{}] => {:?}: {:?}", stringify!($name), target.id, platform);
                    }

                })*
        };
    }

    test_samples![demo1, demo2, demo3, demo4, demo5, demo6, demo7, demo8, demo9, demo10, demo11];
}
