use crate::pbxproj::{PBXHashMap, PBXRootObject};

use super::PBXFileSourceTree;

/// `Abstraction` of file and group elements
///
/// Used by: [`PBXFileReference`] and [`PBXGroup`]
///
/// [`PBXFileReference`]: crate::pbxproj::PBXFileReference
/// [`PBXGroup`]: crate::pbxproj::PBXGroup
#[derive(Debug, Default, derive_new::new)]
/// This element is an abstract parent for file and group elements.
pub struct PBXFileElement {
    /// Element source tree.
    pub source_tree: Option<PBXFileSourceTree>,
    /// Element path.
    pub path: Option<String>,
    /// Element name.
    pub name: Option<String>,
    /// Element include in index.
    pub include_in_index: Option<bool>,
    /// Element uses tabs.
    pub uses_tabs: Option<bool>,
    /// Element indent width.
    pub indent_width: Option<isize>,
    /// Element tab width.
    pub tab_width: Option<isize>,
    /// Element wraps lines.
    pub wraps_lines: Option<bool>,
    /// Element parent.
    pub(crate) parent_reference: Option<String>,
}

impl TryFrom<PBXHashMap> for PBXFileElement {
    type Error = anyhow::Error;

    fn try_from(mut value: PBXHashMap) -> Result<Self, Self::Error> {
        Ok(Self {
            source_tree: value.remove_string("sourceTree").map(|s| s.into()),
            path: value.remove_string("path"),
            name: value.remove_string("name"),
            include_in_index: value.remove_number("includeInIndex").map(|v| v == 1),
            uses_tabs: value.remove_number("usesTabs").map(|v| v == 1),
            indent_width: value.remove_number("indentWidth"),
            tab_width: value.remove_number("tabWidth"),
            wraps_lines: value.remove_number("wrapsLines").map(|v| v == 1),
            parent_reference: None,
        })
    }
}

impl PBXFileElement {
    /// Get a reference to the pbxfile element's parent reference.
    #[must_use]
    pub fn parent(&self, _data: PBXRootObject) -> Option<&PBXFileElement> {
        todo!()
        // self.parent_reference.as_ref()
    }

    /// Set the pbxfile element's parent reference.
    pub fn set_parent_reference(&mut self, parent_reference: Option<String>) {
        self.parent_reference = parent_reference;
    }

    /// Set the pbxfile element's source tree.
    pub fn set_source_tree(&mut self, source_tree: Option<PBXFileSourceTree>) {
        self.source_tree = source_tree;
    }

    /// Get a reference to the pbxfile element's source tree.
    #[must_use]
    pub fn source_tree(&self) -> Option<&PBXFileSourceTree> {
        self.source_tree.as_ref()
    }

    /// Set the pbxfile element's path.
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Get a reference to the pbxfile element's path.
    #[must_use]
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Set the pbxfile element's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Get a reference to the pbxfile element's name.
    #[must_use]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Set the pbxfile element's include in index.
    pub fn set_include_in_index(&mut self, include_in_index: Option<bool>) {
        self.include_in_index = include_in_index;
    }

    /// Get the pbxfile element's include in index.
    #[must_use]
    pub fn include_in_index(&self) -> Option<bool> {
        self.include_in_index
    }

    /// Set the pbxfile element's uses tabs.
    pub fn set_uses_tabs(&mut self, uses_tabs: Option<bool>) {
        self.uses_tabs = uses_tabs;
    }

    /// Get the pbxfile element's uses tabs.
    #[must_use]
    pub fn uses_tabs(&self) -> Option<bool> {
        self.uses_tabs
    }

    /// Set the pbxfile element's indent width.
    pub fn set_indent_width(&mut self, indent_width: Option<isize>) {
        self.indent_width = indent_width;
    }

    /// Get the pbxfile element's indent width.
    #[must_use]
    pub fn indent_width(&self) -> Option<isize> {
        self.indent_width
    }

    /// Set the pbxfile element's tab width.
    pub fn set_tab_width(&mut self, tab_width: Option<isize>) {
        self.tab_width = tab_width;
    }

    /// Get the pbxfile element's tab width.
    #[must_use]
    pub fn tab_width(&self) -> Option<isize> {
        self.tab_width
    }

    /// Set the pbxfile element's wraps lines.
    pub fn set_wraps_lines(&mut self, wraps_lines: Option<bool>) {
        self.wraps_lines = wraps_lines;
    }

    /// Get the pbxfile element's wraps lines.
    #[must_use]
    pub fn wraps_lines(&self) -> Option<bool> {
        self.wraps_lines
    }

    /// Get a reference to the pbxfile element's parent reference.
    #[must_use]
    pub fn parent_reference(&self) -> Option<&String> {
        self.parent_reference.as_ref()
    }
}
