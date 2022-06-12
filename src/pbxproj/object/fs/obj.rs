use crate::pbxproj::PBXHashMap;
use anyhow::Result;

use super::*;
impl PBXObjectExt for PBXFSReference {
    fn from_hashmap(mut value: PBXHashMap, objects: WeakPBXObjectCollection) -> Result<Self>
    where
        Self: Sized,
    {
        let kind = value
            .try_remove_kind("isa")?
            .try_into_fs_reference_kind()
            .unwrap();
        Ok(Self {
            name: value.remove_string("name"),
            path: value.remove_string("path"),
            kind,
            source_tree: value.remove_string("sourceTree").map(|s| s.into()),
            include_in_index: value.remove_number("includeInIndex").map(|v| v == 1),
            uses_tabs: value.remove_number("usesTabs").map(|v| v == 1),
            indent_width: value.remove_number("indentWidth"),
            tab_width: value.remove_number("tabWidth"),
            wraps_lines: value.remove_number("wrapsLines").map(|v| v == 1),
            current_version_reference: value.remove_string("currentVersion"),
            children_references: value
                .remove_vec("children")
                .map(|v| v.try_into_vec_strings().ok().map(|v| HashSet::from_iter(v)))
                .flatten(),
            parent_reference: None,
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
            objects,
            version_group_type: value.remove_string("versioGroupType"),
        })
    }

    fn to_hashmap(&self) -> PBXHashMap {
        todo!()
    }
}
