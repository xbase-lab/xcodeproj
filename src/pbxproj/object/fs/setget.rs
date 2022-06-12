use super::*;
impl PBXFSReference {
    /// Check whether this fs reference is group
    pub fn is_group(&self) -> bool {
        self.kind.is_group()
    }

    /// Check whether this fs reference is group and is file group
    pub fn is_file_group(&self) -> bool {
        if let PBXFSReferenceKind::Group(ref group) = self.kind {
            group.is_file_group()
        } else {
            false
        }
    }

    /// Check whether this fs reference is group and is version group
    pub fn is_version_group(&self) -> bool {
        if let PBXFSReferenceKind::Group(ref group) = self.kind {
            group.is_version_group()
        } else {
            false
        }
    }

    /// Check whether this fs reference is group and is variant group
    pub fn is_varient_group(&self) -> bool {
        if let PBXFSReferenceKind::Group(ref group) = self.kind {
            group.is_variant_group()
        } else {
            false
        }
    }

    /// Check whether this fs reference is file
    pub fn is_file(&self) -> bool {
        self.kind.is_file()
    }

    /// Get a reference to the reference's source tree.
    #[must_use]
    pub fn source_tree(&self) -> Option<&PBXSourceTree> {
        self.source_tree.as_ref()
    }

    /// Get a reference to the reference's path.
    #[must_use]
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Get a reference to the reference's name.
    #[must_use]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Get the reference's include in index.
    #[must_use]
    pub fn include_in_index(&self) -> Option<bool> {
        self.include_in_index
    }

    /// Get the reference's uses tabs.
    #[must_use]
    pub fn uses_tabs(&self) -> Option<bool> {
        self.uses_tabs
    }

    /// Get the reference's indent width.
    #[must_use]
    pub fn indent_width(&self) -> Option<isize> {
        self.indent_width
    }

    /// Get the reference's tab width.
    #[must_use]
    pub fn tab_width(&self) -> Option<isize> {
        self.tab_width
    }

    /// Get the reference's wraps lines.
    #[must_use]
    pub fn wraps_lines(&self) -> Option<bool> {
        self.wraps_lines
    }

    /// Get a reference to the reference's kind.
    #[must_use]
    pub fn kind(&self) -> &PBXFSReferenceKind {
        &self.kind
    }

    /// Get a reference to the reference's children references.
    /// WARN: Would panic if !self.is_group()
    #[must_use]
    pub fn children_references(&self) -> &HashSet<String> {
        &self.children_references.as_ref().unwrap()
    }

    /// Get the reference's file encoding.
    #[must_use]
    pub fn file_encoding(&self) -> Option<isize> {
        self.file_encoding
    }

    /// Get a reference to the reference's explicit file type.
    #[must_use]
    pub fn explicit_file_type(&self) -> Option<&String> {
        self.explicit_file_type.as_ref()
    }

    /// Get a reference to the reference's last known file type.
    #[must_use]
    pub fn last_known_file_type(&self) -> Option<&String> {
        self.last_known_file_type.as_ref()
    }

    /// Get the reference's line ending.
    #[must_use]
    pub fn line_ending(&self) -> Option<isize> {
        self.line_ending
    }

    /// Get a reference to the reference's language specification identifier.
    #[must_use]
    pub fn language_specification_identifier(&self) -> Option<&String> {
        self.language_specification_identifier.as_ref()
    }

    /// Get a reference to the reference's xc language specification identifier.
    #[must_use]
    pub fn xc_language_specification_identifier(&self) -> Option<&String> {
        self.xc_language_specification_identifier.as_ref()
    }

    /// Get a reference to the reference's plist structure definition identifier.
    #[must_use]
    pub fn plist_structure_definition_identifier(&self) -> Option<&String> {
        self.plist_structure_definition_identifier.as_ref()
    }

    /// Get a reference to the reference's current version reference.
    #[must_use]
    pub fn current_version_reference(&self) -> Option<&String> {
        self.current_version_reference.as_ref()
    }

    /// Get a reference to the reference's version group type.
    #[must_use]
    pub fn version_group_type(&self) -> Option<&String> {
        self.version_group_type.as_ref()
    }

    /// Set the reference's source tree.
    pub fn set_source_tree(&mut self, source_tree: PBXSourceTree) {
        self.source_tree = Some(source_tree);
    }

    /// Set the reference's path.
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Set the reference's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Set the reference's include in index.
    pub fn set_include_in_index(&mut self, include_in_index: Option<bool>) {
        self.include_in_index = include_in_index;
    }

    /// Set the reference's uses tabs.
    pub fn set_uses_tabs(&mut self, uses_tabs: Option<bool>) {
        self.uses_tabs = uses_tabs;
    }

    /// Set the reference's indent width.
    pub fn set_indent_width(&mut self, indent_width: Option<isize>) {
        self.indent_width = indent_width;
    }

    /// Set the reference's tab width.
    pub fn set_tab_width(&mut self, tab_width: Option<isize>) {
        self.tab_width = tab_width;
    }

    /// Set the reference's wraps lines.
    pub fn set_wraps_lines(&mut self, wraps_lines: Option<bool>) {
        self.wraps_lines = wraps_lines;
    }

    /// Set the reference's kind.
    pub fn set_kind(&mut self, kind: PBXFSReferenceKind) {
        self.kind = kind;
    }

    /// Set the reference's children references.
    pub fn set_children_references(&mut self, children_references: HashSet<String>) {
        self.children_references = Some(children_references);
    }

    /// Set the reference's file encoding.
    pub fn set_file_encoding(&mut self, file_encoding: Option<isize>) {
        self.file_encoding = file_encoding;
    }

    /// Set the reference's explicit file type.
    pub fn set_explicit_file_type(&mut self, explicit_file_type: Option<String>) {
        self.explicit_file_type = explicit_file_type;
    }

    /// Set the reference's last known file type.
    pub fn set_last_known_file_type(&mut self, last_known_file_type: Option<String>) {
        self.last_known_file_type = last_known_file_type;
    }

    /// Set the reference's line ending.
    pub fn set_line_ending(&mut self, line_ending: Option<isize>) {
        self.line_ending = line_ending;
    }

    /// Set the reference's language specification identifier.
    pub fn set_language_specification_identifier(
        &mut self,
        language_specification_identifier: Option<String>,
    ) {
        self.language_specification_identifier = language_specification_identifier;
    }

    /// Set the reference's xc language specification identifier.
    pub fn set_xc_language_specification_identifier(
        &mut self,
        xc_language_specification_identifier: Option<String>,
    ) {
        self.xc_language_specification_identifier = xc_language_specification_identifier;
    }

    /// Set the reference's plist structure definition identifier.
    pub fn set_plist_structure_definition_identifier(
        &mut self,
        plist_structure_definition_identifier: Option<String>,
    ) {
        self.plist_structure_definition_identifier = plist_structure_definition_identifier;
    }

    /// Set the reference's current version reference.
    pub fn set_current_version_reference(&mut self, current_version_reference: Option<String>) {
        self.current_version_reference = current_version_reference;
    }

    /// Set the reference's version group type.
    pub fn set_version_group_type(&mut self, version_group_type: Option<String>) {
        self.version_group_type = version_group_type;
    }
}
