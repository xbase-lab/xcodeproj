mod full_path;
mod kind;
mod obj;
mod setget;
mod source_tree;

use crate::xcode::xcode_file_type;

use super::*;
use std::{
    cell::RefCell,
    collections::HashSet,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

use anyhow::{bail, Result};
pub use kind::*;
pub use source_tree::*;
use tap::Pipe;

/// Abstraction over `PBXFileReference`, `PBXGroup`, `PBXVariantGroup`, and `XCVersionGroup`
#[derive(Debug, Default)]
pub struct PBXFSReference {
    /// Element source tree.
    source_tree: Option<PBXSourceTree>,
    /// Element path.
    path: Option<String>,
    /// Element name.
    name: Option<String>,
    /// Element include in index.
    include_in_index: Option<bool>,
    /// Element uses tabs.
    uses_tabs: Option<bool>,
    /// Element indent width.
    indent_width: Option<isize>,
    /// Element tab width.
    tab_width: Option<isize>,
    /// Element wraps lines.
    wraps_lines: Option<bool>,
    /// Element parent.
    kind: PBXFSReferenceKind,
    /// Group children references (only relevant to PBX*Group)
    children_references: Option<HashSet<String>>,
    /// Text encoding of file content (only relevant to PBXFileReference)
    file_encoding: Option<isize>,
    /// User-specified file type. use `last_known_file_type` instead. (only relevant to PBXFileReference)
    explicit_file_type: Option<String>,
    /// Derived file type. For a file named "foo.swift" this value would be "sourcecode.swift" (only relevant to PBXFileReference)
    last_known_file_type: Option<String>,
    /// Line ending type for the file (only relevant to PBXFileReference)
    line_ending: Option<isize>,
    /// Legacy programming language identifier (only relevant to PBXFileReference)
    language_specification_identifier: Option<String>,
    /// Programming language identifier (only relevant to PBXFileReference)
    xc_language_specification_identifier: Option<String>,
    /// Plist organizational family identifier (only relevant to PBXFileReference)
    plist_structure_definition_identifier: Option<String>,
    /// Current version. (only relevant for XCVersionGroup)
    current_version_reference: Option<String>,
    /// Version group type. (only relevant for XCVersionGroup)
    version_group_type: Option<String>,

    parent: Weak<RefCell<Self>>,
    pub(crate) objects: WeakPBXObjectCollection,
}

impl PBXFSReference {
    /// Get Group children.
    /// WARN: This will return empty if self is of type file
    pub fn children(&self) -> Vec<Rc<RefCell<PBXFSReference>>> {
        if self.is_file() || self.children_references.is_none() {
            return vec![];
        }
        let objects = self.objects.upgrade().expect("Objects to valid reference");
        let objects = objects.borrow();
        self.children_references
            .as_ref()
            .unwrap()
            .iter()
            .map(|r| Some(objects.get(r)?.as_pbxfs_reference()?.clone()))
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Get group from children with given name
    ///
    /// NOTE: This will return None if self is file
    pub fn get_subgroup(&self, name: &str) -> Option<Rc<RefCell<PBXFSReference>>> {
        if self.is_file() {
            return None;
        }

        self.children()
            .into_iter()
            .filter(|v| v.borrow().is_group())
            .find(|v| {
                let group = v.borrow();
                if let Some(group_path) = group.path() {
                    group_path.eq(name)
                } else if let Some(group_name) = group.name() {
                    group_name.eq(name)
                } else {
                    false
                }
            })
    }

    pub(crate) fn assign_parent_to_children(&self, this: Weak<RefCell<Self>>) {
        if self.is_group() {
            self.children().into_iter().for_each(|o| {
                let mut fs_reference = o.borrow_mut();
                fs_reference.parent = this.clone();
                fs_reference.assign_parent_to_children(Rc::downgrade(&o))
            });
        }
    }

    /// Set the pbxfsreference's parent.
    pub fn set_parent(&mut self, parent: Weak<RefCell<Self>>) {
        self.parent = parent;
    }

    /// Get a reference to the pbxfsreference's parent.
    #[must_use]
    pub fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        self.parent.upgrade()
    }

    /// Get File from the group
    ///
    /// NOTE: This will return None if self is file
    pub fn get_file<S: AsRef<str>>(&self, name: S) -> Option<Rc<RefCell<PBXFSReference>>> {
        let name = name.as_ref();
        self.children().into_iter().find(|o| {
            if !o.borrow().is_file() {
                return false;
            }
            let file = o.borrow();

            if let Some(n) = file.name() {
                n == name
            } else if let Some(p) = file.path() {
                p == name
            } else {
                false
            }
        })
    }

    /// Add file to a group
    ///
    /// NOTE: This will return None if self is file
    pub fn add_file<P: AsRef<Path>>(
        &mut self,
        file_path: P,
        source_root: P,
        source_tree: Option<PBXSourceTree>,
    ) -> Result<Rc<RefCell<PBXFSReference>>> {
        let (file_path, source_root) = (file_path.as_ref(), source_root.as_ref());

        // if !file_path.exists() {
        //     bail!("Trying to add non-existing file {file_path:?}")
        // }

        let group_path = self.full_path(source_root)?;
        let objects = self
            .objects
            .upgrade()
            .ok_or_else(|| anyhow::anyhow!("objects already released!"))?;
        let mut objects = objects.borrow_mut();

        // TODO(fs): ensure we are not adding a duplication
        //
        // NOTE: This function error because self is already borrowed mutably
        //
        // if let Some((file_reference, existing_file)) =
        //     objects.files().into_iter().find(|(_, file_reference)| {
        //         let existing_file_ref = file_reference.borrow();
        //         let existing_file_path = if let Some(path) = existing_file_ref.path() {
        //             PathBuf::from(path)
        //         } else {
        //             return file_path
        //                 == existing_file_ref
        //                     .full_path(&source_root)
        //                     .unwrap_or_default();
        //         };
        //         if existing_file_path.components().last() == file_path.components().last() {
        //             file_path == existing_file_ref.full_path(source_root).unwrap_or_default()
        //         } else {
        //             false
        //         }
        //     })
        // {
        //     if !self
        //         .children_references
        //         .as_ref()
        //         .map(|r| r.contains(&file_reference))
        //         .unwrap_or_default()
        //     {
        //         // TODO: file exists but doesn't exists in self.
        //     }
        //     return Ok(existing_file);
        // }

        let source_tree = source_tree.unwrap_or_else(|| PBXSourceTree::Group);
        let path: Option<PathBuf> = match &source_tree {
            PBXSourceTree::Group => Some(file_path.strip_prefix(group_path)?.to_path_buf()),
            PBXSourceTree::SourceRoot => Some(file_path.strip_prefix(source_root)?.to_path_buf()),
            PBXSourceTree::Absolute | PBXSourceTree::SdkRoot | PBXSourceTree::DeveloperDir => {
                Some(file_path.to_path_buf())
            }
            _ => None,
        };

        let mut file_reference = PBXFSReference::default();
        file_reference.set_source_tree(source_tree);
        file_reference.set_name(
            file_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string()),
        );

        if let Some(path) = path {
            let path = path.to_string_lossy().to_string().into();
            file_reference.set_path(path);
        }

        let file_extension = file_path.extension().unwrap_or_default().to_string_lossy();
        let file_extension = xcode_file_type(file_extension);

        file_reference.set_explicit_file_type(file_extension.clone());
        file_reference.set_last_known_file_type(file_extension);
        file_reference.set_kind(PBXFSReferenceKind::File);

        let file_reference = Rc::new(RefCell::new(file_reference));
        let reference = objects.push(file_reference.clone());

        let children_references = self.children_references.get_or_insert(Default::default());

        if !children_references.contains(&reference) {
            children_references.insert(reference);
        };

        Ok(file_reference)
    }
}

impl Eq for PBXFSReference {}
impl PartialEq for PBXFSReference {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.source_tree == other.source_tree
            && self.path == other.path
            && self.name == other.name
            && self.children_references == other.children_references
            && self.current_version_reference == other.current_version_reference
            && self.version_group_type == other.version_group_type
            && self.include_in_index == other.include_in_index
            && self.uses_tabs == other.uses_tabs
            && self.indent_width == other.indent_width
            && self.tab_width == other.tab_width
            && self.wraps_lines == other.wraps_lines
            && self.file_encoding == other.file_encoding
            && self.explicit_file_type == other.explicit_file_type
            && self.last_known_file_type == other.last_known_file_type
            && self.line_ending == other.line_ending
            && self.language_specification_identifier == other.language_specification_identifier
            && self.xc_language_specification_identifier
                == other.xc_language_specification_identifier
            && self.plist_structure_definition_identifier
                == other.plist_structure_definition_identifier
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn get_parent() {
        use crate::pbxproj::test_demo_file;
        let project = test_demo_file!(demo1);
        let main_group = project
            .objects()
            .projects()
            .first()
            .unwrap()
            .1
            .borrow()
            .main_group();

        let main_group = main_group.borrow();
        let source_group = main_group.get_subgroup("Source").unwrap();
        let source_group = source_group.borrow();
        let parent = source_group.parent();

        assert_eq!(
            parent.unwrap().borrow().children_references(),
            main_group.children_references()
        )
    }

    #[test]
    fn get_file() {
        use crate::pbxproj::test_demo_file;
        let project = test_demo_file!(demo1);
        let source_group = project
            .objects()
            .get_group_by_name_or_path("Source")
            .unwrap()
            .1;
        let source_group = source_group.borrow();
        let file = source_group.get_file("Log.swift");
        assert!(file.is_some())
    }

    #[test]
    fn add_file_full_path() {
        use crate::pbxproj::test_demo_file;
        let root = PathBuf::from("/path/to/project");
        let project = test_demo_file!(demo1);
        let source_group = project
            .objects()
            .get_group_by_name_or_path("Source")
            .unwrap()
            .1;
        let mut source_group = source_group.borrow_mut();
        let file = source_group
            .add_file(
                root.join("Source").join("MyFile.swift").as_path(),
                root.as_path(),
                None,
            )
            .unwrap();

        assert_eq!(file.borrow().name(), Some(&String::from("MyFile.swift")));
        assert_eq!(file.borrow().path(), Some(&String::from("MyFile.swift")));

        drop(file);
        drop(source_group);

        let file = project.objects().files().into_iter().find(|(_, o)| {
            o.borrow()
                .path()
                .map(|n| n == "MyFile.swift")
                .unwrap_or_default()
        });

        assert!(file.is_some());
    }
}
