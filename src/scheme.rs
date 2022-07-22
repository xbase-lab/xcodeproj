use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// XCode Scheme representation
#[derive(Debug, Deserialize, Serialize)]
pub struct XCScheme {
    /// Scheme name (reflects scheme file name)
    #[serde(default)]
    pub name: String,
    /// ...
    pub last_upgrade_version: Option<String>,
    /// ...
    pub version: Option<String>,
    /// ...
    pub was_created_for_app_extension: Option<bool>,
}

impl XCScheme {
    /// Read and parse *.xcscheme content
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(&path)?;
        let mut scheme = serde_xml_rs::from_str::<Self>(&content)?;
        scheme.name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .split(".")
            .next()
            .unwrap_or_default()
            .into();
        Ok(scheme)
    }
}

#[cfg(test)]
macro_rules! test_demo_file {
    ($name:expr) => {{
        let (root, name) = (env!("CARGO_MANIFEST_DIR"), stringify!($name));
        let path = format!("{root}/tests/schemes/{name}.xcscheme");
        let scheme = super::XCScheme::new(path);
        if scheme.is_err() {
            eprintln!("Error: {:#?}", scheme.as_ref().unwrap_err())
        }
        assert!(scheme.is_ok());
        scheme.unwrap()
    }};
}

#[cfg(test)]
mod tests {
    macro_rules! test_samples {
        ($($name:ident),*) => {
            $(#[test]
                fn $name() {
                    let scheme = test_demo_file!($name);
                    println!("{scheme:?}")
                })*
        };
    }

    test_samples![demo1];
}
