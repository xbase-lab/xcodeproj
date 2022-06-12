//! Xcode related types and helpers
include!(concat!(env!("OUT_DIR"), "/file_types.rs"));

/// Translate file extsnion to xcode file type
pub fn xcode_file_type<S: AsRef<str>>(extension: S) -> Option<String> {
    XCODE_FILE_TYPES
        .get(extension.as_ref())
        .map(|s| s.to_string())
}
