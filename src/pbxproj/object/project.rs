use anyhow::{bail, Result};
use tap::Pipe;

use crate::pbxproj::*;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

/// [`PBXObject`] producing a binary content (application or library).
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXProject {
    /// Project name
    pub name: Option<String>,
    /// A string representation of the XcodeCompatibilityVersion.
    pub compatibility_version: String,
    /// The region of development.
    pub development_region: Option<String>,
    /// Whether file encodings have been scanned.
    pub has_scanned_for_encodings: isize,
    /// The known regions for localized files.
    pub known_regions: Vec<String>,
    /// The relative path of the project.
    pub project_dir_path: String,
    /// The relative root paths of the project.
    pub project_roots: Vec<String>,
    /// Project attributes.
    /// Target attributes will be merged into this
    pub attributes: PBXHashMap,
    /// Target attribute references.
    target_attribute_references: Option<PBXHashMap>,
    /// Package references.
    package_references: Option<Vec<String>>,
    /// Build configuration list reference.
    build_configuration_list_reference: String,
    /// The objects are a reference to a PBXTarget element.
    target_references: Vec<String>,
    /// Project references.
    project_references: Option<Vec<HashMap<String, String>>>,
    /// The object is a reference to a PBXGroup element.
    products_group_reference: Option<String>,
    /// The object is a reference to a PBXGroup element.
    main_group_reference: String,
    /// root
    objects: WeakPBXObjectCollection,
}

impl PBXProject {
    /// Get packages that the project uses
    #[must_use]
    pub fn packages<'a>(
        &self,
    ) -> Option<Vec<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)>> {
        let package_references = self.package_references.as_ref()?;
        let objects = self.objects.upgrade()?;
        let objects = objects.borrow();
        Some(objects.get_swift_package_reference_from_references(package_references))
    }

    /// Get targets for given reference
    #[must_use]
    pub fn targets(&self) -> Vec<(String, Rc<RefCell<PBXTarget>>)> {
        let target_references = self.target_references.as_ref();
        if let Some(objects) = self.objects.upgrade() {
            let objects = objects.borrow();
            objects.get_targets_from_references(target_references)
        } else {
            vec![]
        }
    }

    /// Returns the attributes of a given target.
    #[must_use]
    pub fn get_attributes_for_target_reference(
        &self,
        target_reference: &str,
    ) -> Option<&PBXHashMap> {
        self.target_attribute_references
            .as_ref()?
            .get(target_reference)?
            .as_object()
    }

    /// Returns the attributes of a given target.
    #[must_use]
    pub fn get_attributes_for_target_reference_mut(
        &mut self,
        target_reference: &str,
    ) -> Option<&mut PBXHashMap> {
        self.target_attribute_references
            .as_mut()?
            .get_mut(target_reference)?
            .as_object_mut()
    }

    /// Git build configuration list
    pub fn build_configuration_list(&self) -> Option<Rc<RefCell<XCConfigurationList>>> {
        self.objects
            .upgrade()?
            .borrow()
            .get(&self.build_configuration_list_reference)?
            .as_xc_configuration_list()?
            .clone()
            .pipe(Some)
    }

    /// Get project projects
    pub fn projects(&self) -> Option<Vec<HashMap<&String, PBXObject>>> {
        self.project_references
            .as_ref()?
            .iter()
            .map(|v| {
                v.iter()
                    .map(|(k, v)| Some((k, self.objects.upgrade()?.borrow().get(v)?.clone())))
                    .flatten()
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>()
            .pipe(Some)
    }

    /// Get Project main group.
    pub fn main_group(&self) -> Rc<RefCell<PBXGroup>> {
        self.objects
            .upgrade()
            .expect("objects weak is invalid")
            .borrow()
            .get(&self.main_group_reference)
            .expect("PBXProject should contain mainGroup")
            .as_pbx_group()
            .expect("given reference point to PBXGroup")
            .clone()
    }

    /// Products Group
    pub fn products_group(&self) -> Option<Rc<RefCell<PBXGroup>>> {
        let products_group = self.products_group_reference.as_ref()?;
        self.objects
            .upgrade()?
            .borrow()
            .get(products_group)?
            .as_pbx_group()?
            .clone()
            .pipe(Some)
    }

    /// Get attributes for a given target reference
    pub fn get_target_attributes(&mut self, target_reference: &str) -> Option<&PBXHashMap> {
        let target_attributes = self.target_attribute_references.as_mut()?;
        target_attributes.get(target_reference)?.as_object()
    }

    /// Sets the attributes for the given target.
    pub fn set_target_attributes(&mut self, attributes: PBXHashMap, target_reference: &str) {
        let target_attributes = self
            .target_attribute_references
            .get_or_insert(Default::default());
        target_attributes.insert(target_reference.into(), attributes.into());
    }

    /// Remove attributes for a given target reference
    pub fn remove_target_attributes(&mut self, target_reference: &str) -> Option<PBXHashMap> {
        if let Some(target_attributes) = self.target_attribute_references.as_mut() {
            target_attributes
                .remove(target_reference)?
                .into_object()
                .ok()
        } else {
            None
        }
    }

    /// Removes the all the target attributes
    pub fn clear_all_target_attributes(&mut self) {
        if let Some(target_attributes) = self.target_attribute_references.as_mut() {
            target_attributes.clear();
        }
    }

    /// Adds a remote swift package
    pub fn add_swift_package(
        &mut self,
        repository_url: String,
        product_name: String,
        version_requirement: XCVersionRequirement,
        target_name: String,
    ) -> Result<Rc<RefCell<XCRemoteSwiftPackageReference>>> {
        let objects = self
            .objects
            .upgrade()
            .ok_or_else(|| anyhow::anyhow!("PBXObjectCollection is released"))?;
        let mut objects = objects.borrow_mut();

        // Get target reference for given target name
        let (_, target) = match objects.get_target_by_name(&target_name) {
            Some((reference, value)) => (reference, value),
            None => bail!("No target found with {target_name:?}"),
        };

        // Add swift package reference
        let (package_reference, package) = self.add_swift_package_reference(
            &mut objects,
            &product_name,
            version_requirement,
            repository_url,
        )?;

        let mut target = target.borrow_mut();

        // Add swift package product dependency
        let (product_reference, _) = self.add_swift_package_product(
            &mut target,
            &mut objects,
            product_name,
            package_reference,
        )?;

        // Build file
        let build_file =
            PBXBuildFile::new_from_swift_product(product_reference, self.objects.clone())
                .pipe(|v| Rc::new(RefCell::new(v)));

        let build_file_reference = objects.push(build_file.clone());

        // Link the product
        let (_, frameworks_build_phase) = objects
            .get_build_phases_from_reference(&target.build_phase_references)
            .into_iter()
            .find(|(_, b)| b.borrow().is_frameworks())
            .ok_or_else(|| anyhow::anyhow!("frameworks Build Phase Not Found for {target_name}"))?;

        frameworks_build_phase
            .borrow_mut()
            .add_file_reference(build_file_reference);

        Ok(package)
    }

    fn add_swift_package_product(
        &mut self,
        target: &mut RefMut<PBXTarget>,
        objects: &mut RefMut<PBXObjectCollection>,
        product_name: String,
        package_reference: String,
    ) -> Result<(String, Rc<RefCell<XCSwiftPackageProductDependency>>)> {
        let product_details = if let Some(product) =
            objects.get_product_dependency_from_target_reference(&package_reference)
        {
            product
        } else {
            let value = XCSwiftPackageProductDependency::new(
                product_name,
                package_reference.into(),
                self.objects.clone(),
            )
            .pipe(|v| Rc::new(RefCell::new(v)));
            let reference = objects.push(value.clone());
            (reference, value)
        };

        target.insert_package_product_dependency(product_details.0.clone());

        Ok(product_details)
    }

    /// Create swift package reference if it doesn't already exists, update version_requirement and
    /// return. This function would error if version_requirement is identical to current existing
    /// one.
    fn add_swift_package_reference(
        &mut self,
        objects: &mut RefMut<PBXObjectCollection>,
        product_name: &str,
        version_requirement: XCVersionRequirement,
        repository_url: String,
    ) -> Result<(String, Rc<RefCell<XCRemoteSwiftPackageReference>>)> {
        // self's swift package references
        let package_references = self.package_references.get_or_insert(Default::default());

        // Try finding swift package where repository_url == given repository_url.
        let package_details = objects
            .get_swift_package_reference_from_references(&package_references)
            .into_iter()
            .find(|(_, v)| {
                if let Some(url) = &v.borrow().repository_url {
                    repository_url.eq(url)
                } else {
                    false
                }
            });

        match package_details {
            // Package reference with given url already exists
            Some((reference, value)) => {
                let package = value.clone();
                let mut package = package.borrow_mut();
                let identical_version_requirement = package
                    .version_requirement
                    .as_ref()
                    .map(|v| version_requirement.eq(v))
                    .unwrap_or_default();

                if identical_version_requirement {
                    bail!("{product_name:?} with {version_requirement:?} is added already, reference key: {reference:?}")
                } else {
                    package.set_version_requirement(version_requirement.into());
                    Ok((reference, value))
                }
            }
            // Package reference with given url is new
            None => {
                let value = XCRemoteSwiftPackageReference::new(
                    repository_url.into(),
                    version_requirement.into(),
                    self.objects.clone(),
                )
                .pipe(|v| Rc::new(RefCell::new(v)));

                let reference = objects.push(value.clone());

                // Insert package reference
                package_references.push(reference.clone());

                Ok((reference, value))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    fn get_project(demo_name: &str) -> (PBXRootObject, Rc<RefCell<PBXProject>>) {
        use crate::pbxproj::pest::PBXProjectParser;
        let root = env!("CARGO_MANIFEST_DIR");
        let file = PBXProjectParser::try_parse_from_file(format!(
            "{root}/tests/samples/{demo_name}.pbxproj"
        ))
        .unwrap();
        let objects = PBXRootObject::try_from(file).unwrap();
        let inner = objects.clone();
        let inner = inner.borrow();
        let project = inner
            .iter()
            .find(|s| s.1.is_pbx_project())
            .map(|(_, o)| o.as_pbx_project().unwrap().clone())
            .unwrap();
        // root must live to next scope
        (objects, project)
    }

    #[test]
    fn test_packages() {
        let (_objects, project) = get_project("demo1");
        let project = project.borrow();
        let mut packages = project.packages().unwrap();
        let package = packages.remove(0).1;
        let package = package.borrow();
        assert_eq!(
            package.repository_url,
            Some("https://github.com/apple/swift-log.git".into()),
        )
    }

    #[test]
    fn test_targets() {
        let (_objects, project) = get_project("demo2");
        let project = project.borrow();
        let mut targets = project.targets();
        let target = targets.remove(0).1;
        let target = target.borrow();
        assert_eq!(
            target.name.as_ref().unwrap(),
            "backbase-showcase-mobile-ios"
        )
    }

    #[test]
    fn test_build_configuration_list() {
        let (_objects, project) = get_project("demo3");
        let project = project.borrow();
        let build_configuration_list = project.build_configuration_list().unwrap();
        let build_configuration_list = build_configuration_list.borrow();
        assert_eq!(
            build_configuration_list.default_configuration_name,
            Some("Release".into())
        )
    }

    #[test]
    fn test_main_group() {
        let (_objects, project) = get_project("demo4");
        let project = project.borrow();
        let main_group = project.main_group();
        let main_group = main_group.borrow();
        assert_eq!(
            main_group.children_references(),
            &HashSet::from([
                "4FE00ECD1A97227F00D83062".to_string(),
                "4FF5E5001AA60ED3003996B4".to_string(),
                "4FE00EC01A97227F00D83062".to_string(),
                "4F303ED81A978C6100A83368".to_string(),
                "4FE00EBF1A97227F00D83062".to_string()
            ])
        );
        println!("{main_group:#?}")
    }

    #[test]
    fn test_products_group() {
        let (_objects, project) = get_project("demo7");
        let project = project.borrow();
        let main_group = project.products_group().unwrap();
        let main_group = main_group.borrow();
        assert_eq!(
            main_group.children_references(),
            &HashSet::from(["53C4ED31159E740C0019285D".to_string()])
        );
        println!("{main_group:#?}")
    }

    #[test]
    fn test_add_swift_package_duplication() {
        let (_objects, project) = get_project("demo1");
        let mut project = project.borrow_mut();
        let err = project
            .add_swift_package(
                "https://github.com/apple/swift-log.git".into(),
                "Logging".into(),
                XCVersionRequirement::Exact("1.4.2".into()),
                "Wordle".into(),
            )
            .unwrap_err();
        assert!(err.to_string().contains("added already"));
    }

    #[test]
    fn test_add_swift_package_with_new_version() {
        let (objects, project) = get_project("demo1");
        let mut project = project.borrow_mut();
        let new_package = project
            .add_swift_package(
                "https://github.com/apple/swift-log.git".into(),
                "Logging".into(),
                XCVersionRequirement::Exact("1.4.3".into()),
                "Wordle".into(),
            )
            .unwrap();
        assert_eq!(
            new_package,
            project.packages().unwrap().first().unwrap().1,
            "new package should be added to project"
        );
        let objects = objects.borrow();
        assert_eq!(
            new_package,
            objects.swift_package_references().first().unwrap().1,
            "new package should be added in object collection"
        );
    }
}

impl PBXObjectExt for PBXProject {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> Result<Self> {
        let project_roots = if let Some(roots) = value
            .remove_vec("projectRoots")
            .map(|v| v.try_into_vec_strings().ok())
            .flatten()
        {
            roots
        } else if let Some(root) = value.remove_string("projectRoot") {
            vec![root]
        } else {
            vec![]
        };

        Ok(Self {
            name: value.remove_string("name"),
            compatibility_version: value.try_remove_value("compatibilityVersion")?.try_into()?,
            development_region: value
                .try_remove_value("developmentRegion")
                .map(|v| v.try_into().ok())
                .ok()
                .flatten(),
            has_scanned_for_encodings: value
                .try_remove_value("hasScannedForEncodings")?
                .try_into()?,
            known_regions: value.try_remove_value("knownRegions")?.try_into()?,
            project_dir_path: value.try_remove_value("projectDirPath")?.try_into()?,
            project_roots,
            attributes: value.try_remove_value("attributes")?.try_into()?,
            target_attribute_references: value
                .remove_value("TargetAttributes")
                .map(|v| v.try_into().ok())
                .flatten(),
            package_references: value
                .remove_value("packageReferences")
                .map(|v| v.try_into().ok())
                .flatten(),
            build_configuration_list_reference: value
                .try_remove_value("buildConfigurationList")?
                .try_into()?,
            target_references: value.try_remove_value("targets")?.try_into()?,
            project_references: value.remove_vec("projectReferences").map(|v| {
                v.0.into_iter()
                    .map(|v| v.try_into_object())
                    .flatten()
                    .map(|v| {
                        v.0.into_iter()
                            .map(|(k, v)| anyhow::Ok((k, v.try_into_string()?)))
                            .flatten()
                            .collect()
                    })
                    .collect::<Vec<_>>()
            }),
            main_group_reference: value.try_remove_value("mainGroup")?.try_into()?,
            products_group_reference: value
                .remove_value("productRefGroup")
                .map(|v| v.try_into().ok())
                .flatten(),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
