mod kind;
mod obj;
mod setget;
mod source_tree;

use super::*;
use std::{
    cell::RefCell,
    collections::HashSet,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

use anyhow::Result;
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

    /// Returns a file path to current fs reference using source root.
    pub fn full_path<P: AsRef<Path>>(&self, source_root: P) -> Result<Option<PathBuf>> {
        let source_root = source_root.as_ref();

        let path = || {
            self.path()
                .ok_or_else(|| anyhow::anyhow!("Expected path to be set in file element!!"))
        };

        fn get_parts(path: &String) -> Vec<&str> {
            if path.contains("/") {
                path.split("/").collect()
            } else {
                vec![path]
            }
        }

        match self.source_tree() {
            Some(PBXSourceTree::Absolute) => path()?.pipe(PathBuf::from).pipe(Some),
            Some(PBXSourceTree::SourceRoot) => {
                let mut root = source_root.to_path_buf();
                root.extend(get_parts(path()?));
                Some(root)
            }
            Some(PBXSourceTree::Group) => {
                let mut group_path: Option<PathBuf>;

                if let Some(parent) = self.parent() {
                    println!("Using parent path");
                    group_path = parent.borrow().full_path(&source_root)?;
                    if let Some(ref mut g) = group_path {
                        if let Some(path) = self.path() {
                            g.extend(get_parts(path))
                        }
                    }
                } else {
                    let objects = self
                        .objects
                        .upgrade()
                        .ok_or_else(|| anyhow::anyhow!("objects is released already!"))?;

                    let objects = objects.borrow();

                    if objects
                        .projects()
                        .into_iter()
                        .find(|(_, p)| &*p.borrow().main_group().borrow() == self)
                        .is_some()
                    {
                        if let Some(path) = self.path() {
                            let mut root = source_root.to_path_buf();
                            root.extend(get_parts(path));
                            println!("Joining {source_root:?} with {path:?}");
                            return Ok(Some(root));
                        } else {
                            println!("Self is main group and return source_root as is!");
                            return Ok(Some(source_root.to_path_buf()));
                        }
                    }

                    println!("Falling back to search through all groups");

                    // Fallback if parent is nil and it's not root element
                    let group = objects
                        .groups()
                        .into_iter()
                        .find(|(_, o)| {
                            o.borrow()
                                .children()
                                .into_iter()
                                .any(|o| &*o.borrow() == self)
                        })
                        .map(|(_, o)| o)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "Invalid group path {source_root:?} with {:?}",
                                self.path()
                            )
                        })?;

                    group_path = group.borrow().full_path(source_root)?;
                }
                group_path
            }
            _ => None,
        }
        .pipe(Ok)
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

#[test]
fn test_parent() {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_root_full_path() {
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

        let root = PathBuf::from("/path/to/project");
        let main_group = main_group.borrow();
        let main_group_full_path = main_group.full_path(&root);
        assert_eq!(main_group_full_path.unwrap().unwrap(), root);
    }

    #[test]
    fn get_subgroup_full_path() {
        let root = PathBuf::from("/path/to/project");
        let project = crate::pbxproj::test_demo_file!(demo1);

        let source_group = project
            .objects()
            .groups()
            .into_iter()
            .find(|(_, o)| o.borrow().path().map(|p| p == "Source").unwrap_or_default())
            .map(|(_, o)| o.clone())
            .unwrap();

        let source_group = source_group.borrow();
        let source_group_full_path = source_group.full_path(&root);
        assert_eq!(
            source_group_full_path.unwrap().unwrap(),
            root.join("Source")
        );
    }

    #[test]
    fn get_file_full_path() {
        let root = PathBuf::from("/path/to/project");
        let project = crate::pbxproj::test_demo_file!(demo1);

        let mut expected_file_path = root.clone();
        expected_file_path.extend(&["Source", "Views", "GuessView.swift"]);

        let file = project
            .objects()
            .get_fs_references(|fs_reference| {
                fs_reference
                    .path()
                    .map(|name| name == "GuessView.swift")
                    .unwrap_or_default()
            })
            .first()
            .map(|(_, o)| o.clone())
            .unwrap();

        let file = file.borrow();

        assert_eq!(file.full_path(root).unwrap().unwrap(), expected_file_path)
    }
}
