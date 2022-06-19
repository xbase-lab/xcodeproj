use crate::pbxproj::*;
use std::collections::HashMap;

/// [`PBXObject`] producing a binary content (application or library).
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXProject<'a> {
    /// ID Reference
    pub id: String,
    /// Project name
    pub name: Option<&'a String>,
    /// A string representation of the XcodeCompatibilityVersion.
    pub compatibility_version: &'a String,
    /// The region of development.
    pub development_region: Option<&'a String>,
    /// Whether file encodings have been scanned.
    pub has_scanned_for_encodings: &'a isize,
    /// The known regions for localized files.
    pub known_regions: Vec<&'a String>,
    /// The relative path of the project.
    pub project_dir_path: &'a String,
    /// The relative root paths of the project.
    pub project_roots: Vec<&'a String>,
    /// Project main or root file group
    pub main_group: PBXFSReference<'a>,
    /// Project attributes.
    pub attributes: &'a PBXHashMap,
    /// Project's Targets attributes by target reference key
    pub target_attributes: HashMap<&'a String, &'a PBXHashMap>,
    /// Project's Package references.
    pub packages: Vec<XCRemoteSwiftPackageReference<'a>>,
    /// Project's Build configuration list
    pub build_configuration_list: XCConfigurationList<'a>,
    /// Project's targets
    pub targets: Vec<PBXTarget<'a>>,
    // The object is a reference to a PBXGroup element.
    // products_group_reference: Option<String>,
}

impl<'a> AsPBXObject<'a> for PBXProject<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> anyhow::Result<Self>
    where
        Self: Sized + 'a,
    {
        let project_roots =
            if let Some(roots) = value.get_vec("projectRoots").map(|v| v.as_vec_strings()) {
                roots
            } else if let Some(root) = value.get_string("projectRoot") {
                vec![root]
            } else {
                vec![]
            };

        let attributes = value.try_get_object("attributes")?;
        Ok(Self {
            id,
            name: value.get_string("name"),
            compatibility_version: value.try_get_string("compatibilityVersion")?,
            development_region: value.get_string("developmentRegion"),

            has_scanned_for_encodings: value.try_get_number("hasScannedForEncodings")?,
            known_regions: value.try_get_vec("knownRegions")?.as_vec_strings(),
            project_dir_path: value.try_get_string("projectDirPath")?,
            project_roots,
            target_attributes: attributes
                .get_object("TargetAttributes")
                .map(|v| {
                    v.iter()
                        .map(|(k, value)| Some((k, value.as_object()?)))
                        .flatten()
                        .collect::<HashMap<_, _>>()
                })
                .unwrap_or_default(),
            attributes,
            packages: value
                .get_vec("packageReferences")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            build_configuration_list: value
                .try_get_string("buildConfigurationList")
                .and_then(|key| objects.try_get(key))?,
            targets: value
                .get_vec("targets")
                .map(|v| objects.get_vec(v.as_vec_strings()))
                .unwrap_or_default(),
            main_group: value
                .try_get_string("mainGroup")
                .and_then(|key| objects.try_get(key))?,
            // products_group_reference: value
            //     .get_value("productRefGroup")
            //     .map(|v| v.try_into().ok())
            //     .flatten(),
        })
    }
}
impl<'a> PBXProject<'a> {
    /// Returns the attributes of a given target.
    #[must_use]
    pub fn get_attributes_for_target_reference(
        &self,
        target_reference: &String,
    ) -> Option<&&PBXHashMap> {
        self.target_attributes.get(target_reference)
    }
}

// impl PBXProject {
//     /// Adds a remote swift package
//     pub fn add_swift_package(
//         &mut self,
//         repository_url: String,
//         product_name: String,
//         version_requirement: XCVersionRequirement,
//         target_name: String,
//     ) -> Result<Rc<RefCell<XCRemoteSwiftPackageReference>>> {
//         let objects = self
//             .objects
//             .upgrade()
//             .ok_or_else(|| anyhow::anyhow!("PBXObjectCollection is released"))?;
//         let mut objects = objects.borrow_mut();

//         // Get target reference for given target name
//         let (_, target) = match objects.get_target_by_name(&target_name) {
//             Some((reference, value)) => (reference, value),
//             None => bail!("No target found with {target_name:?}"),
//         };

//         // Add swift package reference
//         let (package_reference, package) = self.add_swift_package_reference(
//             &mut objects,
//             &product_name,
//             version_requirement,
//             repository_url,
//         )?;

//         let mut target = target.borrow_mut();

//         // Add swift package product dependency
//         let (product_reference, _) = self.add_swift_package_product(
//             &mut target,
//             &mut objects,
//             product_name,
//             package_reference,
//         )?;

//         // Build file
//         let build_file =
//             PBXBuildFile::new_from_swift_product(product_reference, self.objects.clone())
//                 .pipe(|v| Rc::new(RefCell::new(v)));

//         let build_file_reference = objects.push(build_file.clone());

//         // Link the product
//         let (_, frameworks_build_phase) = objects
//             .get_build_phases_from_reference(&target.build_phase_references)
//             .into_iter()
//             .find(|(_, b)| b.borrow().is_frameworks())
//             .ok_or_else(|| anyhow::anyhow!("frameworks Build Phase Not Found for {target_name}"))?;

//         frameworks_build_phase
//             .borrow_mut()
//             .add_file_reference(build_file_reference);

//         Ok(package)
//     }

//     fn add_swift_package_product(
//         &mut self,
//         target: &mut RefMut<PBXTarget>,
//         objects: &mut RefMut<PBXObjectCollection>,
//         product_name: String,
//         package_reference: String,
//     ) -> Result<(String, Rc<RefCell<XCSwiftPackageProductDependency>>)> {
//         let product_details = if let Some(product) =
//             objects.get_product_dependency_from_target_reference(&package_reference)
//         {
//             product
//         } else {
//             let value = XCSwiftPackageProductDependency::new(
//                 product_name,
//                 package_reference.into(),
//                 self.objects.clone(),
//             )
//             .pipe(|v| Rc::new(RefCell::new(v)));
//             let reference = objects.push(value.clone());
//             (reference, value)
//         };

//         target.insert_package_product_dependency(product_details.0.clone());

//         Ok(product_details)
//     }

//     /// Create swift package reference if it doesn't already exists, update version_requirement and
//     /// return. This function would error if version_requirement is identical to current existing
//     /// one.
//     fn add_swift_package_reference(
//         &mut self,
//         objects: &mut RefMut<PBXObjectCollection>,
//         product_name: &str,
//         version_requirement: XCVersionRequirement,
//         repository_url: String,
//     ) -> Result<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)> {
//         // self's swift package references
//         let package_references = self.package_references.get_or_insert(Default::default());

//         // Try finding swift package where repository_url == given repository_url.
//         let package_details = objects
//             .get_swift_package_reference_from_references(&package_references)
//             .into_iter()
//             .find(|(_, v)| {
//                 if let Some(url) = &v.borrow().repository_url {
//                     repository_url.eq(url)
//                 } else {
//                     false
//                 }
//             });

//         match package_details {
//             // Package reference with given url already exists
//             Some((reference, value)) => {
//                 let package = value.clone();
//                 let mut package = package.borrow_mut();
//                 let identical_version_requirement = package
//                     .version_requirement
//                     .as_ref()
//                     .map(|v| version_requirement.eq(v))
//                     .unwrap_or_default();

//                 if identical_version_requirement {
//                     bail!("{product_name:?} with {version_requirement:?} is added already, reference key: {reference:?}")
//                 } else {
//                     package.set_version_requirement(version_requirement.into());
//                     Ok((reference, value))
//                 }
//             }
//             // Package reference with given url is new
//             None => {
//                 let value = XCRemoteSwiftPackageReference::new(
//                     repository_url.into(),
//                     version_requirement.into(),
//                     self.objects.clone(),
//                 )
//                 .pipe(|v| Rc::new(RefCell::new(v)));

//                 let reference = objects.push(value.clone());

//                 // Insert package reference
//                 package_references.push(reference.clone());

//                 Ok((reference, value))
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashSet;

//     use super::*;
//     fn get_project(demo_name: &str) -> (PBXRootObject, Rc<RefCell<PBXProject>>) {
//         use crate::pbxproj::pest::PBXProjectParser;
//         let root = env!("CARGO_MANIFEST_DIR");
//         let file = PBXProjectParser::try_parse_from_file(format!(
//             "{root}/tests/samples/{demo_name}.pbxproj"
//         ))
//         .unwrap();
//         let objects = PBXRootObject::try_from(file).unwrap();
//         let inner = objects.objects();
//         let project = inner
//             .iter()
//             .find(|s| s.1.is_pbx_project())
//             .map(|(_, o)| o.as_pbx_project().unwrap().clone())
//             .unwrap();
//         drop(inner);
//         // root must live to next scope
//         (objects, project)
//     }

//     #[test]
//     fn get_packages() {
//         let (_objects, project) = get_project("demo1");
//         let project = project.borrow();
//         let mut packages = project.packages().unwrap();
//         let package = packages.remove(0).1;
//         let package = package.borrow();
//         assert_eq!(
//             package.repository_url,
//             Some("https://github.com/apple/swift-log.git".into()),
//         )
//     }

//     #[test]
//     fn get_targets() {
//         let (_objects, project) = get_project("demo2");
//         let project = project.borrow();
//         let mut targets = project.targets();
//         let target = targets.remove(0).1;
//         let target = target.borrow();
//         assert_eq!(
//             target.name.as_ref().unwrap(),
//             "backbase-showcase-mobile-ios"
//         )
//     }

//     #[test]
//     fn get_build_configuration_list() {
//         let (_objects, project) = get_project("demo3");
//         let project = project.borrow();
//         let build_configuration_list = project.build_configuration_list().unwrap();
//         let build_configuration_list = build_configuration_list.borrow();
//         assert_eq!(
//             build_configuration_list.default_configuration_name,
//             Some("Release".into())
//         )
//     }

//     #[test]
//     fn get_main_group() {
//         let (_objects, project) = get_project("demo4");
//         let project = project.borrow();
//         let main_group = project.main_group();
//         let main_group = main_group.borrow();
//         assert_eq!(
//             main_group.children_references(),
//             &HashSet::from([
//                 "4FE00ECD1A97227F00D83062".to_string(),
//                 "4FF5E5001AA60ED3003996B4".to_string(),
//                 "4FE00EC01A97227F00D83062".to_string(),
//                 "4F303ED81A978C6100A83368".to_string(),
//                 "4FE00EBF1A97227F00D83062".to_string()
//             ])
//         );
//         println!("{main_group:#?}")
//     }

//     #[test]
//     fn get_products_group() {
//         let (_objects, project) = get_project("demo7");
//         let project = project.borrow();
//         let main_group = project.products_group().unwrap();
//         let main_group = main_group.borrow();
//         assert_eq!(
//             main_group.children_references(),
//             &HashSet::from(["53C4ED31159E740C0019285D".to_string()])
//         );
//         println!("{main_group:#?}")
//     }

//     #[test]
//     fn add_swift_package_duplication() {
//         let (_objects, project) = get_project("demo1");
//         let mut project = project.borrow_mut();
//         let err = project
//             .add_swift_package(
//                 "https://github.com/apple/swift-log.git".into(),
//                 "Logging".into(),
//                 XCVersionRequirement::Exact("1.4.2".into()),
//                 "Wordle".into(),
//             )
//             .unwrap_err();
//         assert!(err.to_string().contains("added already"));
//     }

//     #[test]
//     fn add_swift_package_with_new_version() {
//         let (root, project) = get_project("demo1");
//         let mut project = project.borrow_mut();
//         let new_package = project
//             .add_swift_package(
//                 "https://github.com/apple/swift-log.git".into(),
//                 "Logging".into(),
//                 XCVersionRequirement::Exact("1.4.3".into()),
//                 "Wordle".into(),
//             )
//             .unwrap();
//         assert_eq!(
//             new_package,
//             project.packages().unwrap().first().unwrap().1,
//             "new package should be added to project"
//         );
//         println!("{:#?}", root.objects().build_files());
//         assert_eq!(
//             new_package,
//             root.objects().swift_package_references().first().unwrap().1,
//             "new package should be added in object collection"
//         );
//     }

//     #[test]
//     fn add_swift_package_new_package() {
//         let (root, project) = get_project("demo1");
//         let mut project = project.borrow_mut();
//         let new_package = project
//             .add_swift_package(
//                 "url".into(),
//                 "Log".into(),
//                 XCVersionRequirement::Exact("1.4.3".into()),
//                 "Wordle".into(),
//             )
//             .unwrap();
//         assert_eq!(
//             new_package,
//             project.packages().unwrap()[1].1,
//             "new package should be added to project"
//         );
//         let has_new_package = root.objects().swift_package_references().iter().any(|v| {
//             v.1.borrow()
//                 .repository_url
//                 .eq(&new_package.borrow().repository_url)
//         });
//         assert!(
//             has_new_package,
//             "new package should be added in object collection"
//         );
//     }
// }
