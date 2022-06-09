use crate::pbxproj::*;

/// [`PBXObject`] poitning to an external file referenced by the project
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_deref_rs::Deref, derive_new::new)]
pub struct PBXFileReference {
    /// Text encoding of file content
    pub file_encoding: Option<isize>,
    /// User-specified file type. Typically this is not set and you want to use `lastKnownFileType` instead.
    pub explicit_file_type: Option<String>,
    /// Derived file type. For a file named "foo.swift" this value would be "sourcecode.swift"
    pub last_known_file_type: Option<String>,
    /// Line ending type for the file
    pub line_ending: Option<isize>,
    /// Legacy programming language identifier
    pub language_specification_identifier: Option<String>,
    /// Programming language identifier
    pub xc_language_specification_identifier: Option<String>,
    /// Plist organizational family identifier
    pub plist_structure_definition_identifier: Option<String>,
    #[deref]
    inner: PBXFileElement,
}

impl PBXObjectExt for PBXFileReference {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            file_encoding: value.remove_number("fileEncoding"),
            explicit_file_type: value.remove_string("explicitFileType"),
            last_known_file_type: value.remove_string("lastKnownFileType"),
            line_ending: value.remove_number("lineEnding"),
            language_specification_identifier: value
                .remove_string("languageSpecificationIdentifier"),
            xc_language_specification_identifier: value
                .remove_string("xcLanguageSpecificationIdentifier"),
            plist_structure_definition_identifier: value
                .remove_string("xcLanguageSpecificationIdentifier"),
            inner: PBXFileElement::from_hashmap(value, objects)?,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
