use anyhow::{bail, Result};

use crate::pbxproj::PBXObjectKind;
use derive_deref_rs::Deref;
use std::collections::HashMap;

#[derive(Default, Debug, Deref, PartialEq, Eq)]
/// [`HashMap`] wrapper for [`PBXValue`] with helpers
pub struct PBXHashMap(pub(crate) HashMap<String, PBXValue>);

impl PBXHashMap {
    /// ...
    pub fn new(inner: HashMap<String, PBXValue>) -> Self {
        Self(inner)
    }
    /// Get Value from map
    pub fn get_value(&self, key: &str) -> Option<&PBXValue> {
        self.0.get(key)
    }

    /// Try Get Value from map or error
    pub fn try_get_value(&self, key: &str) -> Result<&PBXValue> {
        self.0
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("{key} is not found!"))
    }

    /// Remove value from map
    pub fn remove_value(&mut self, key: &str) -> Option<PBXValue> {
        self.try_remove_value(key).ok()
    }

    /// Try remove value from map
    pub fn try_remove_value(&mut self, key: &str) -> Result<PBXValue> {
        self.0
            .remove(key)
            .ok_or_else(|| anyhow::anyhow!("{key} is not found!"))
    }
}

#[derive(Default, Debug, Deref, PartialEq, Eq)]
/// [`Vec`] wrapper for [`PBXValue`] with helpers
pub struct PBXVec(Vec<PBXValue>);

impl PBXVec {
    pub(crate) fn new(inner: Vec<PBXValue>) -> Self {
        Self(inner)
    }

    pub(crate) fn try_into_vec_strings(self) -> Result<Vec<String>> {
        let mut collector = vec![];
        for value in self.0 {
            collector.push(value.try_into_string()?);
        }
        Ok(collector)
    }
}

macro_rules! gen_hash_map_helpers {
    ($([$key:ident, $output:ident]),*) => {
        paste::paste! {
            impl PBXHashMap {
                $(
                    #[doc = "Get " $key " if value is of type " $output]
                    pub fn [<get_ $key>](&self, key: &str) -> Option<&$output> {
                        self.0.get(key)?.[<as_ $key>]()
                    }

                    #[doc = "Try get " $key " of type " $output]
                    pub fn [<try_get_ $key>](&self, key: &str) -> Result<&$output> {
                        let value = self.try_get_value(key)?;
                        value.[<as_ $key>]().ok_or_else(|| anyhow::anyhow!("expected value to be {}, got {:?}", stringify!($key), value))
                    }

                    #[doc = "remove " $key " of type " $output]
                    pub fn [<remove_ $key>](&mut self, key: &str) -> Option<$output> {
                        self.remove_value(key).map(|v| v.[<try_into_ $key>]().ok()).flatten()
                    }

                    #[doc = "Try remove " $key " of type " $output]
                    pub fn [<try_remove_ $key>](&mut self, key: &str) -> Result<$output> {
                        let value = self.try_remove_value(key)?;
                        value.[<try_into_ $key>]()
                    }

                )*
            }
        }
    };
}

gen_hash_map_helpers! {
    [string, String],
    [vec, PBXVec],
    [bool, bool],
    [number, isize],
    [kind, PBXObjectKind],
    [object, PBXHashMap]
}

/// Repersentation of all values that can be collected from pbxproj file.
#[derive(Debug, PartialEq, Eq)]
pub enum PBXValue {
    /// Normal String value. NOTE: This may literal string!
    String(String),
    /// Object value represented as [`HashMap`]
    Object(PBXHashMap),
    /// Vec of [`PBXValue`]
    Vec(PBXVec),
    /// Number
    Number(isize),
    /// Boolean representation of YES, NO
    Bool(bool),
    /// ObjectKind
    Kind(PBXObjectKind),
}

impl PBXValue {
    /// Returns `true` if the value is [`String`].
    ///
    /// [`String`]: PBXValue::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `Ok(T)` if the value is [`String`].
    ///
    /// [`String`]: PBXValue::String
    pub fn try_into_string(self) -> Result<String> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            bail!("expected string got {self:#?}")
        }
    }

    /// Returns `Some(T)` if the value is [`String`].
    ///
    /// [`String`]: PBXValue::String
    pub fn as_string(&self) -> Option<&String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Object`].
    ///
    /// [`Object`]: PBXValue::Object
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(..))
    }

    /// Returns `Ok(T)` if the value is [`Object`].
    ///
    /// [`Object`]: PBXValue::Object
    pub fn try_into_object(self) -> Result<PBXHashMap> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            bail!("expected object got {self:#?}")
        }
    }

    /// Returns `Some(T)` if the value is [`Object`].
    ///
    /// [`Object`]: PBXValue::Object
    pub fn as_object(&self) -> Option<&PBXHashMap> {
        if let Self::Object(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Vec`].
    ///
    /// [`Vec`]: PBXValue::Vec
    #[must_use]
    pub fn is_vec(&self) -> bool {
        matches!(self, Self::Vec(..))
    }

    /// Returns `Some(T)` if the value is [`Vec`].
    ///
    /// [`Vec`]: PBXValue::Vec
    pub fn as_vec(&self) -> Option<&PBXVec> {
        if let Self::Vec(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(T)` if the value is [`Vec`].
    ///
    /// [`Vec`]: PBXValue::Vec
    pub fn try_into_vec(self) -> Result<PBXVec> {
        if let Self::Vec(v) = self {
            Ok(v)
        } else {
            bail!("expected Vec got {self:#?}")
        }
    }

    /// Returns `true` if the value is [`Number`].
    ///
    /// [`Number`]: PBXValue::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `Ok(T)` if the value is [`Number`].
    ///
    /// [`Number`]: PBXValue::Number
    pub fn try_into_number(self) -> Result<isize> {
        if let Self::Number(v) = self {
            Ok(v)
        } else {
            bail!("expected number got {self:#?}")
        }
    }

    /// Returns `Some(T)` if the value is [`Number`].
    ///
    /// [`Number`]: PBXValue::Number
    pub fn as_number(&self) -> Option<&isize> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Bool`].
    ///
    /// [`Bool`]: PBXValue::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `Ok(T)` if the value is [`Bool`].
    ///
    /// [`Bool`]: PBXValue::Bool
    pub fn try_into_bool(self) -> Result<bool> {
        if let Self::Bool(v) = self {
            Ok(v)
        } else {
            bail!("expected bool got {self:#?}")
        }
    }

    /// Returns `Some(T)` if the value is [`Bool`].
    ///
    /// [`Bool`]: PBXValue::Bool
    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Kind`].
    ///
    /// [`Kind`]: PBXValue::Kind
    #[must_use]
    pub fn is_kind(&self) -> bool {
        matches!(self, Self::Kind(..))
    }

    /// Returns `Some(T)` if the value is [`Kind`].
    ///
    /// [`Kind`]: PBXValue::Kind
    pub fn as_kind(&self) -> Option<&PBXObjectKind> {
        if let Self::Kind(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(T)` if the value is [`Kind`].
    ///
    /// [`Kind`]: PBXValue::Kind
    pub fn try_into_kind(self) -> Result<PBXObjectKind> {
        if let Self::Kind(v) = self {
            Ok(v)
        } else {
            bail!("expected kind got {self:#?}")
        }
    }
}
