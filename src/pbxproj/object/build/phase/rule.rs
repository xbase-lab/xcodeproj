use crate::pbxproj::*;

/// [`PBXObject`] specifying how to transform input file(s) to an output file(s).
///
/// [`PBXObject`]: crate::pbxproj::PBXObject
#[derive(Debug, derive_new::new)]
pub struct PBXBuildRule {
    /// Element compiler spec.
    pub compiler_spec: Option<String>,
    /// Element file patterns.
    pub file_patterns: Option<String>,
    /// Element file type.
    pub file_type: Option<String>,
    /// Element is editable.
    pub is_editable: Option<bool>,
    /// Element name.
    pub name: Option<String>,
    /// Element output files.
    pub output_files: Option<Vec<String>>,
    /// Element input files.
    pub input_files: Option<Vec<String>>,
    /// Element output files compiler flags.
    pub output_files_compiler_flags: Option<Vec<String>>,
    /// Element script.
    pub script: Option<String>,
    /// Element run once per architecture.
    pub run_once_per_architecture: Option<bool>,
    objects: WeakPBXObjectCollection,
}

impl PBXObjectExt for PBXBuildRule {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            compiler_spec: value.remove_string("compilerSpec"),
            file_patterns: value.remove_string("filePatterns"),
            file_type: value.remove_string("fileType"),
            is_editable: value.remove_number("isEditable").map(|n| n == 1),
            name: value.remove_string("name"),
            output_files: value
                .remove_vec("outputFiles")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten(),
            input_files: value
                .remove_vec("inputFiles")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten(),
            output_files_compiler_flags: value
                .remove_vec("outputFilesCompilerFlags")
                .map(|v| v.try_into_vec_strings().ok())
                .flatten(),
            script: value.remove_string("script"),
            run_once_per_architecture: value
                .remove_number("runOncePerArchitecture")
                .map(|n| n == 1),
            objects,
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
