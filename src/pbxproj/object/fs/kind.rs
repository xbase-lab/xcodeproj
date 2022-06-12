use derive_is_enum_variant::is_enum_variant;

#[derive(Debug, PartialEq, Eq, is_enum_variant, Ord, PartialOrd)]
/// [`PBXFSKind`] group abstraction kind
pub enum PBXGroupKind {
    /// PBXGroup
    FileGroup,
    /// XCVersionGroup
    VersionGroup,
    /// XCVersionGroup
    VariantGroup,
}

impl PBXGroupKind {
    /// Return string representation of [`PBXGroupKind`]
    pub fn as_isa(&self) -> &str {
        match self {
            PBXGroupKind::FileGroup => "PBXFileGroup",
            PBXGroupKind::VersionGroup => "XCVersionGroup",
            PBXGroupKind::VariantGroup => "PBXVariantGroup",
        }
    }
}

impl Default for PBXGroupKind {
    fn default() -> Self {
        Self::FileGroup
    }
}

#[derive(Debug, PartialEq, Eq, is_enum_variant, Ord, PartialOrd)]
/// [`PBXFSReference`] abstraction kind
pub enum PBXFSReferenceKind {
    /// Group
    Group(PBXGroupKind),
    /// PBXFileReference
    File,
}

impl PBXFSReferenceKind {
    /// Return string representation of [`PBXFSKind`]
    pub fn as_isa(&self) -> &str {
        match self {
            PBXFSReferenceKind::Group(group_kind) => group_kind.as_isa(),
            PBXFSReferenceKind::File => "PBXFileReference",
        }
    }
}

impl Default for PBXFSReferenceKind {
    fn default() -> Self {
        Self::Group(PBXGroupKind::default())
    }
}
