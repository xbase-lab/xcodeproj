/// Helper Specifying source trees for files
///
/// Corresponds to the "Location" dropdown in Xcode's File Inspector
#[derive(PartialEq, Eq, Debug)]
pub enum PBXSourceTree {
    /// No source tree
    None,
    /// Absolute source tree
    Absolute,
    /// Group source tree
    Group,
    /// Root tree
    SourceRoot,
    /// Products Directory source tree
    BuildProductsDir,
    /// SDK root source tree
    SdkRoot,
    /// Developer Directory source tree
    DeveloperDir,
    /// Custom source tree
    Custom(String),
}

impl Default for PBXSourceTree {
    fn default() -> Self {
        PBXSourceTree::None
    }
}

impl ToString for PBXSourceTree {
    fn to_string(&self) -> String {
        match self {
            Self::None => "",
            Self::Absolute => "<absolute>",
            Self::Group => "<group>",
            Self::SourceRoot => "SOURCE_ROOT",
            Self::BuildProductsDir => "BUILT_PRODUCTS_DIR",
            Self::SdkRoot => "SDKROOT",
            Self::DeveloperDir => "DEVELOPER_DIR",
            Self::Custom(s) => s,
        }
        .into()
    }
}
impl From<String> for PBXSourceTree {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for PBXSourceTree {
    fn from(s: &str) -> Self {
        match s {
            "" => Self::None,
            "<absolute>" => Self::Absolute,
            "<group>" => Self::Group,
            "SOURCE_ROOT" => Self::SourceRoot,
            "BUILT_PRODUCTS_DIR" => Self::BuildProductsDir,
            "SDKROOT" => Self::SdkRoot,
            "DEVELOPER_DIR" => Self::DeveloperDir,
            s => Self::Custom(s.into()),
        }
    }
}
