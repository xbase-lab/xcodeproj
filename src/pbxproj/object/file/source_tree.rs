/// Specifies source trees for files
/// Corresponds to the "Location" dropdown in Xcode's File Inspector
#[derive(PartialEq, Eq, Debug)]
pub enum PBXFileSourceTree {
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

impl ToString for PBXFileSourceTree {
    fn to_string(&self) -> String {
        match self {
            PBXFileSourceTree::None => "",
            PBXFileSourceTree::Absolute => "<absolute>",
            PBXFileSourceTree::Group => "<group>",
            PBXFileSourceTree::SourceRoot => "SOURCE_ROOT",
            PBXFileSourceTree::BuildProductsDir => "BUILT_PRODUCTS_DIR",
            PBXFileSourceTree::SdkRoot => "SDKROOT",
            PBXFileSourceTree::DeveloperDir => "DEVELOPER_DIR",
            PBXFileSourceTree::Custom(s) => s,
        }
        .into()
    }
}
impl From<String> for PBXFileSourceTree {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for PBXFileSourceTree {
    fn from(s: &str) -> Self {
        match s {
            "" => PBXFileSourceTree::None,
            "<absolute>" => PBXFileSourceTree::Absolute,
            "<group>" => PBXFileSourceTree::Group,
            "SOURCE_ROOT" => PBXFileSourceTree::SourceRoot,
            "BUILT_PRODUCTS_DIR" => PBXFileSourceTree::BuildProductsDir,
            "SDKROOT" => PBXFileSourceTree::SdkRoot,
            "DEVELOPER_DIR" => PBXFileSourceTree::DeveloperDir,
            s => PBXFileSourceTree::Custom(s.into()),
        }
    }
}
