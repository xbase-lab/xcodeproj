use derive_is_enum_variant::is_enum_variant;

#[derive(Clone, Debug, PartialEq, Eq, is_enum_variant, Ord, PartialOrd)]
/// [`PBXFSReference`] abstraction kind
///
/// [`PBXFSReference`]: crate::pbxproj::PBXFSReference
pub enum PBXFSReferenceKind {
    /// File Group
    FileGroup,
    /// Version Group
    VersionGroup,
    /// Variant Group
    VariantGroup,
    /// PBXFileReference
    File,
}

impl PBXFSReferenceKind {
    /// Return string representation compatible with pbxproj
    pub fn as_isa(&self) -> &str {
        match self {
            PBXFSReferenceKind::FileGroup => "PBXFileGroup",
            PBXFSReferenceKind::VersionGroup => "XCVersionGroup",
            PBXFSReferenceKind::VariantGroup => "PBXVariantGroup",
            PBXFSReferenceKind::File => "PBXFileReference",
        }
    }
    /// Returns group if kind is FileGroup, VersionGroup, or VariantGroup,
    pub fn is_group(&self) -> bool {
        self.is_file_group() || self.is_version_group() || self.is_variant_group()
    }
}

impl Default for PBXFSReferenceKind {
    fn default() -> Self {
        Self::FileGroup
    }
}
