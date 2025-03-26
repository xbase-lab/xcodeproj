// use md5::Digest;
// use rand::distributions::Alphanumeric;
// use rand::{thread_rng, Rng};
use crate::pbxproj::*;
use anyhow::Result;
use std::collections::HashMap;

/// [`PBXObject`] storage with convenient helper methods
#[derive(Default, Debug, derive_new::new, derive_deref_rs::Deref)]
pub struct PBXObjectCollection(pub(crate) HashMap<String, PBXHashMap>);

/// Get PBXObject from PBXHashMap and PBXObjectCollection
pub trait AsPBXObject<'a> {
    /// create a pbx object out of given value
    fn as_pbx_object(
        id: String,
        value: &'a PBXHashMap,
        objects: &'a PBXObjectCollection,
    ) -> Result<Self>
    where
        Self: Sized + 'a;
}

impl PBXObjectCollection {
    /// Get T from collection
    pub fn get<'a, T, S>(&'a self, key: S) -> Option<T>
    where
        T: AsPBXObject<'a> + 'a,
        S: AsRef<str>,
    {
        self.0.get(key.as_ref()).and_then(|value| {
            AsPBXObject::as_pbx_object(key.as_ref().to_string(), value, self).ok()
        })
    }

    /// Get T from collection
    pub fn try_get<'a, T, S>(&'a self, key: S) -> Result<T>
    where
        T: AsPBXObject<'a> + 'a,
        S: AsRef<str> + std::fmt::Debug,
    {
        self.get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("{key:?} doesn't exists!"))
    }

    /// Get PBXObject a vector of type T
    pub fn get_vec<'a, T, I, S>(&'a self, keys: I) -> Vec<T>
    where
        T: AsPBXObject<'a> + 'a,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<str>,
    {
        keys.into_iter()
            .flat_map(|key| self.get(key.as_ref()))
            .collect::<Vec<_>>()
    }

    /// Get vector by vector of T by predict
    pub fn get_vec_by<'a, T: AsPBXObject<'a> + 'a>(
        &'a self,
        predict: impl Fn(&(&String, &PBXHashMap)) -> bool,
    ) -> Vec<T> {
        self.iter()
            .filter(predict)
            .flat_map(|(k, _)| self.get(k))
            .collect::<Vec<_>>()
    }

    /// Get all PBXTarget
    pub fn targets<'a>(&'a self) -> Vec<PBXTarget<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_pbx_target())
                .unwrap_or_default()
        })
    }

    /// Get all PBXProject
    pub fn projects<'a>(&'a self) -> Vec<PBXProject<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_pbx_project())
                .unwrap_or_default()
        })
    }

    /// Get all build phases
    pub fn build_phases<'a>(&'a self) -> Vec<PBXBuildPhase<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_pbx_build_phase())
                .unwrap_or_default()
        })
    }

    /// Get all build phases
    pub fn build_configurations<'a>(&'a self) -> Vec<XCBuildConfiguration<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_xc_build_configuration())
                .unwrap_or_default()
        })
    }

    /// Get all build phases
    pub fn build_files<'a>(&'a self) -> Vec<PBXBuildFile<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_pbx_build_file())
                .unwrap_or_default()
        })
    }

    /// Get all build phases
    pub fn build_rules<'a>(&'a self) -> Vec<PBXBuildRule<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_pbx_build_rule())
                .unwrap_or_default()
        })
    }

    /// Get all source code files
    pub fn files<'a>(&'a self) -> Vec<PBXFSReference<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| {
                    k.is_pbx_fsreference()
                        && k.as_pbxfs_reference()
                            .map(|r| r.is_file())
                            .unwrap_or_default()
                })
                .unwrap_or_default()
        })
    }

    /// Get all groups
    pub fn groups<'a>(&'a self) -> Vec<PBXFSReference<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| {
                    k.is_pbx_fsreference()
                        && k.as_pbxfs_reference()
                            .map(|r| r.is_group())
                            .unwrap_or_default()
                })
                .unwrap_or_default()
        })
    }

    /// Get All XCSwiftPackageProductDependency Objects
    pub fn swift_package_product_dependencies<'a>(
        &'a self,
    ) -> Vec<XCSwiftPackageProductDependency<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_xc_swift_package_product_dependency())
                .unwrap_or_default()
        })
    }

    /// Get All XCRemoteSwiftPackageReference Objects
    pub fn swift_package_references<'a>(&'a self) -> Vec<XCRemoteSwiftPackageReference<'a>> {
        self.get_vec_by(|(_, v)| {
            v.get_kind("isa")
                .map(|k| k.is_xc_remote_swift_package_reference())
                .unwrap_or_default()
        })
    }

    /// Get PBXTarget
    pub fn get_target<'a>(&'a self, key: &str) -> Option<PBXTarget<'a>> {
        self.get(key)
    }

    /// Get PBXBuildPhase
    pub fn get_build_phase<'a>(&'a self, key: &str) -> Option<PBXBuildPhase<'a>> {
        self.get(key)
    }

    /// Get PBXBuildFile
    pub fn get_build_file<'a>(&'a self, key: &str) -> Option<PBXBuildFile<'a>> {
        self.get(key)
    }

    /// Get PBXBuildRule
    pub fn get_build_rule<'a>(&'a self, key: &str) -> Option<PBXBuildRule<'a>> {
        self.get(key)
    }

    /// Get PBXProject
    pub fn get_project<'a>(&'a self, key: &str) -> Option<PBXProject<'a>> {
        self.get(key)
    }

    /// Get all files
    pub fn get_file<'a>(&'a self, key: &str) -> Option<PBXFSReference<'a>> {
        let fs_ref = self.get::<PBXFSReference, _>(key)?;
        if fs_ref.is_file() {
            Some(fs_ref)
        } else {
            None
        }
    }

    /// Get all files
    pub fn get_group<'a>(&'a self, key: &str) -> Option<PBXFSReference<'a>> {
        let fs_ref = self.get::<PBXFSReference, _>(key)?;
        if fs_ref.is_group() {
            Some(fs_ref)
        } else {
            None
        }
    }

    /// Get fs object
    pub fn get_fs_object<'a>(&'a self, key: &str) -> Option<PBXFSReference<'a>> {
        self.get(key)
    }

    /// Get PBXGroup with by name or path
    pub fn get_group_by_name_or_path<'a, S: AsRef<str>>(
        &'a self,
        name_or_path: S,
    ) -> Option<PBXFSReference<'a>> {
        let name = name_or_path.as_ref();
        self.groups().into_iter().find(|o| {
            if let Some(n) = o.name {
                return n == name;
            } else if let Some(p) = o.path {
                return p == name;
            } else {
                false
            }
        })
    }

    /// Get build configurations shearing a given baseConfiguration id
    pub fn get_build_configurations_by_base_id<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Vec<XCBuildConfiguration> {
        let key = id.as_ref();
        self.get_vec_by(move |(_, v)| {
            v.get_kind("isa")
                .map(|o| o.is_xc_build_configuration())
                .unwrap_or_default()
                && v.get_string("baseConfigurationReference")
                    .map(|s| s.as_str())
                    == Some(key)
        })
    }

    /// Get XCSwiftPackageProductDependency by reference
    pub fn get_swift_package_product_dependency<'a>(
        &'a self,
        key: &str,
    ) -> Option<XCSwiftPackageProductDependency<'a>> {
        self.get(key)
    }

    /// Get XCSwiftPackageProductDependency by reference
    pub fn get_swift_package_reference<'a>(
        &'a self,
        key: &str,
    ) -> Option<XCRemoteSwiftPackageReference<'a>> {
        self.get(key)
    }

    /// Get PBXTarget by the target name
    pub fn get_target_by_name<'a>(&'a self, name: &'a str) -> Option<PBXTarget<'a>> {
        self.targets().into_iter().find(|target| {
            if let Some(target_name) = target.name {
                target_name == name
            } else {
                false
            }
        })
    }
}
