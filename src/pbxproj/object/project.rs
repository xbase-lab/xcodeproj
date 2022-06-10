use tap::Pipe;

use crate::pbxproj::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    pub fn packages<'a>(&self) -> Option<Vec<Rc<RefCell<XCRemoteSwiftPackageReference>>>> {
        let package_references = self.package_references.as_ref()?;
        let objects = self.objects.upgrade()?;
        let objects = objects.borrow();
        let mut packages = vec![];

        for id in package_references.iter() {
            if let Some(object) = objects
                .get(id)
                .map(|o| o.as_xc_remote_swift_package_reference())
                .flatten()
            {
                packages.push(object.clone())
            }
        }

        Some(packages)
    }

    /// Get targets for given reference
    #[must_use]
    pub fn targets(&self) -> Vec<Rc<RefCell<PBXTarget>>> {
        let objects = if let Some(objects) = self.objects.upgrade() {
            objects
        } else {
            return vec![];
        };
        let objects = objects.borrow();
        let mut targets = vec![];
        for id in self.target_references.iter() {
            if let Some(traget) = objects.get(id).map(|v| v.as_pbx_target()).flatten() {
                targets.push(traget.clone())
            }
        }
        targets
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
            .expect("objects weak is valid")
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
}

#[test]
fn test_collections() {
    use crate::pbxproj::pest::PBXProjectParser;
    use crate::pbxproj::PBXRootObject;
    let file = PBXProjectParser::try_from_str(include_str!("../../../tests/samples/demo1.pbxproj"))
        .unwrap();
    let root = PBXRootObject::try_from(file).unwrap();
    let inner = root.borrow();
    let project = inner
        .iter()
        .find(|s| s.1.is_pbx_project())
        .map(|(_, o)| o.as_pbx_project().unwrap().clone())
        .unwrap();

    let project = project.borrow();
    let packages = project.packages();
    let targets = project.targets();
    let build_configuration_list = project.build_configuration_list();
    let projects = project.projects();
    let main_group = project.main_group();
    let products_group = project.products_group();

    println!("Project: {:#?}", project);
    println!("Packages: {:#?}", packages);
    println!("Targets: {:#?}", targets);
    println!("build_configuration_list: {:#?}", build_configuration_list);
    println!("projects: {:#?}", projects);
    println!("main_group: {:#?}", main_group);
    println!("products_group: {:#?}", products_group);
}

impl PBXObjectExt for PBXProject {
    fn from_hashmap(
        mut value: PBXHashMap,
        objects: WeakPBXObjectCollection,
    ) -> anyhow::Result<Self> {
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
