use super::{PBXObject, PBXObjectExt, XCRemoteSwiftPackageReference};
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
    project_references: Vec<HashMap<String, String>>,
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
    /// TODO: Wrap target
    #[must_use]
    pub fn targets(&self) -> Vec<PBXObject> {
        let objects = if let Some(objects) = self.objects.upgrade() {
            objects
        } else {
            return vec![];
        };
        let objects = objects.borrow();
        let mut targets = vec![];
        for id in self.target_references.iter() {
            if let Some(object) = objects.get(id) {
                targets.push(object.clone())
            }
        }
        targets
    }

    /// Returns the attributes of a given target.
    #[must_use]
    fn get_attributes_for_target_reference(&self, target_reference: &str) -> Option<&PBXHashMap> {
        self.target_attribute_references
            .as_ref()?
            .get(target_reference)?
            .as_object()
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
    println!("Packages: {:#?}", packages);
    println!("Targets: {:#?}", targets);
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
            project_references: value
                .remove_vec("projectReferences")
                .map(|v| {
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
                })
                .unwrap_or_default(),
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
