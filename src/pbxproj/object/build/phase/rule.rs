use anyhow::Result;

use crate::pbxproj::*;

/// [`PBXObject`] specifying how to transform input file(s) to an output file(s).
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXBuildRule<'a> {
    /// ID Reference
    pub id: String,
    /// Element compiler spec.
    pub compiler_spec: Option<&'a String>,
    /// Element file patterns.
    pub file_patterns: Option<&'a String>,
    /// Element file type.
    pub file_type: Option<&'a String>,
    /// Element is editable.
    pub is_editable: Option<bool>,
    /// Element name.
    pub name: Option<&'a String>,
    /// Element output files.
    pub output_files: Option<Vec<&'a String>>,
    /// Element input files.
    pub input_files: Option<Vec<&'a String>>,
    /// Element output files compiler flags.
    pub output_files_compiler_flags: Option<Vec<&'a String>>,
    /// Element script.
    pub script: Option<&'a String>,
    /// Element run once per architecture.
    pub run_once_per_architecture: Option<bool>,
}

impl<'a> AsPBXObject<'a> for PBXBuildRule<'a> {
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        _objects: &'a PBXObjectCollection,
    ) -> Result<Self>
    where
        Self: Sized + 'a,
    {
        Ok(Self {
            id,
            compiler_spec: value.get_string("compilerSpec"),
            file_patterns: value.get_string("filePatterns"),
            file_type: value.get_string("fileType"),
            is_editable: value.get_number("isEditable").map(|n| n == &1),
            name: value.get_string("name"),
            output_files: value.get_vec("outputFiles").map(|v| v.as_vec_strings()),
            input_files: value.get_vec("inputFiles").map(|v| v.as_vec_strings()),
            output_files_compiler_flags: value
                .get_vec("outputFilesCompilerFlags")
                .map(|v| v.as_vec_strings()),
            script: value.get_string("script"),
            run_once_per_architecture: value.get_number("runOncePerArchitecture").map(|n| n == &1),
        })
    }
}
