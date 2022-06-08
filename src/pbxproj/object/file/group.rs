use derive_deref_rs::Deref;

use super::{PBXFileElement, PBXFileSourceTree};
use crate::pbxproj::PBXFileReference;
use crate::pbxproj::PBXHashMap;
use crate::pbxproj::PBXObject;
use crate::pbxproj::PBXRootObject;
use std::collections::HashSet;
use std::path::Path;

/// [`PBXObject`] representing a collection of files in Xcode's VF hierarchy.
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXGroup {
    /// Group children references.
    children_references: HashSet<String>,
    inner: PBXFileElement,
}

impl PBXGroup {
    /// Group children.
    pub fn set_children_references(&mut self, references: HashSet<String>) -> HashSet<String> {
        let old = std::mem::replace(&mut self.children_references, references);
        old
    }

    /// Group children.
    pub fn children<'a>(&'a self, data: &'a PBXRootObject) -> Vec<&'a PBXObject> {
        self.children_references
            .iter()
            .map(|r| data.get(r))
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl TryFrom<PBXHashMap> for PBXGroup {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            children_references: HashSet::from_iter(
                value.try_remove_vec("children")?.try_into_vec_strings()?,
            ),
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// Group Adding option variants
pub enum PBXGroupAddingOption {
    /// Group without a folder
    WithoutFolder,
}

impl PBXGroup {
    /// Creates a group with the given name and returns it.
    ///
    /// - Parameters:
    ///   - groupName: group name.
    ///   - options: creation options.
    /// - Returns: created groups.
    fn add_group(
        &mut self,
        _data: &mut PBXRootObject,
        _group_name: String,
        _options: Vec<PBXGroupAddingOption>,
    ) -> Vec<&PBXGroup> {
        // {
        //         let objects = try self.objects()
        //         return groupName.components(separatedBy: "/").reduce(into: [PBXGroup]()) { groups, name in
        //             let group = groups.last ?? self
        //             let newGroup = PBXGroup(children: [], sourceTree: .group, name: name, path: options.contains(.withoutFolder) ? nil : name)
        //             newGroup.parent = group
        //             group.childrenReferences.append(newGroup.reference)
        //             objects.add(object: newGroup)
        //             groups.append(newGroup)
        //         }
        //     }
        todo!()
    }

    /// Creates a variant group with the given name and returns it.
    ///
    /// - Parameters:
    ///   - groupName: group name.
    /// - Returns: created groups.
    fn add_variant_group(
        &mut self,
        _data: &mut PBXRootObject,
        _group_name: String,
    ) -> Vec<PBXVariantGroup> {
        // {
        //         let objects = try self.objects()
        //         return groupName.components(separatedBy: "/").reduce(into: [PBXVariantGroup]()) { groups, name in
        //             let group = groups.last ?? self
        //             let newGroup = PBXVariantGroup(children: [], sourceTree: .group, name: name)
        //             newGroup.parent = self
        //             group.childrenReferences.append(newGroup.reference)
        //             objects.add(object: newGroup)
        //             groups.append(newGroup)
        //         }
        //     }
        todo!()
    }

    /// Adds file at the give path to the project or returns existing file and its reference.
    ///
    /// - Parameters:
    ///   - filePath: path to the file.
    ///   - sourceTree: file sourceTree, default is `.group`.
    ///   - sourceRoot: path to project's source root.
    ///   - override: flag to enable overriding of existing file references, default is `true`.
    ///   - validatePresence: flag to validate the existence of the file in the file system, default is `true`.
    /// - Returns: new or existing file and its reference.
    fn add_file<P: AsRef<Path>>(
        &mut self,
        _data: &mut PBXRootObject,
        _file_path: P,
        _source_tree: PBXFileSourceTree,
        _source_root: P,
        _roverride: bool,
        _validate_presence: bool,
    ) -> PBXFileReference {
        // {
        //         let projectObjects = try objects()
        //         if validatePresence, !filePath.exists {
        //             throw XcodeprojEditingError.unexistingFile(filePath)
        //         }
        //         let groupPath = try fullPath(sourceRoot: sourceRoot)
        //         if override, let existingFileReference = try projectObjects.fileReferences.first(where: {
        //             // Optimization: compare lastComponent before fullPath compare
        //             guard let fileRefPath = $0.value.path else {
        //                 return try filePath == $0.value.fullPath(sourceRoot: sourceRoot)
        //             }
        //             let fileRefLastPathComponent = fileRefPath.split(separator: "/").last!
        //             if filePath.lastComponent == fileRefLastPathComponent {
        //                 return try filePath == $0.value.fullPath(sourceRoot: sourceRoot)
        //             }
        //             return false
        //         }) {
        //             if !childrenReferences.contains(existingFileReference.key) {
        //                 existingFileReference.value.path = groupPath.flatMap { filePath.relative(to: $0) }?.string
        //                 childrenReferences.append(existingFileReference.key)
        //             }
        //             return existingFileReference.value
        //         }
        //         let path: String?
        //         switch sourceTree {
        //         case .group:
        //             path = groupPath.map { filePath.relative(to: $0) }?.string
        //         case .sourceRoot:
        //             path = filePath.relative(to: sourceRoot).string
        //         case .absolute,
        //              .sdkRoot,
        //              .developerDir:
        //             path = filePath.string
        //         default:
        //             path = nil
        //         }
        //         let fileReference = PBXFileReference(
        //             sourceTree: sourceTree,
        //             name: filePath.lastComponent,
        //             explicitFileType: filePath.extension.flatMap(Xcode.filetype),
        //             lastKnownFileType: filePath.extension.flatMap(Xcode.filetype),
        //             path: path
        //         )
        //         projectObjects.add(object: fileReference)
        //         fileReference.parent = self
        //         if !childrenReferences.contains(fileReference.reference) {
        //             childrenReferences.append(fileReference.reference)
        //         }
        //         return fileReference
        //     }
        todo!()
    }
}

/// [`PBXObject`] specifying [`PBXGroup`] representing localized resources
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct PBXVariantGroup {
    inner: PBXGroup,
}

impl TryFrom<PBXHashMap> for PBXVariantGroup {
    type Error = anyhow::Error;

    fn try_from(value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: TryFrom::try_from(value)?,
        })
    }
}

/// [`PBXObject`] specifying [`PBXGroup`] containing different versions of a resource
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, Deref, derive_new::new)]
pub struct XCVersionGroup {
    /// Current version.
    current_version_reference: Option<String>,
    /// Version group type.
    pub version_group_type: Option<String>,
    #[deref]
    inner: PBXGroup,
}

impl XCVersionGroup {
    /// Returns the current version file reference.
    pub fn current_version(&self, _store: &PBXRootObject) -> Option<PBXFileReference> {
        // currentVersionReference?.getObject()
        todo!()
    }

    /// Set current version reference
    pub fn set_current_version_reference(&mut self, value: Option<String>) -> Option<String> {
        let old = std::mem::replace(&mut self.current_version_reference, value);
        old
    }
}

impl TryFrom<PBXHashMap> for XCVersionGroup {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            current_version_reference: value.remove_string("current_version"),
            version_group_type: value.remove_string("version_group_type"),
            inner: TryFrom::try_from(value)?,
        })
    }
}
