use crate::{macros::gen_hash_map_helpers, pbxproj::PBXObjectKind};
use anyhow::{bail, Result};
use derive_deref_rs::Deref;
use derive_is_enum_variant::is_enum_variant;
use enum_as_inner::EnumAsInner;
use enum_variant_macros::FromVariants;
use std::collections::HashMap;

/// Repersentation of all values that can be collected from pbxproj file.
#[derive(Debug, PartialEq, Eq, FromVariants, EnumAsInner, is_enum_variant)]
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
    /// Nothing
    Null(()),
}

impl TryFrom<PBXValue> for String {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_string()
    }
}

impl TryFrom<PBXValue> for PBXHashMap {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_object()
    }
}

impl TryFrom<PBXValue> for PBXVec {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_vec()
    }
}

impl TryFrom<PBXValue> for bool {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_bool()
    }
}

impl TryFrom<PBXValue> for isize {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_number()
    }
}

impl TryFrom<PBXValue> for PBXObjectKind {
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> Result<Self, Self::Error> {
        value.try_into_kind()
    }
}

impl<T> TryFrom<PBXValue> for HashMap<String, T>
where
    T: From<PBXValue>,
{
    type Error = anyhow::Error;
    fn try_from(value: PBXValue) -> anyhow::Result<Self> {
        Ok(value
            .try_into_object()?
            .0
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect())
    }
}

impl<T> From<HashMap<String, T>> for PBXValue
where
    T: TryFrom<PBXValue>,
    <T as TryFrom<PBXValue>>::Error: std::fmt::Debug,
    PBXValue: From<T>,
{
    fn from(v: HashMap<String, T>) -> Self {
        let inner = v
            .into_iter()
            .map(|(k, v)| (k, v.try_into().unwrap()))
            .collect();

        PBXValue::Object(PBXHashMap(inner))
    }
}

impl From<&str> for PBXValue {
    fn from(v: &str) -> Self {
        PBXValue::String(v.to_string())
    }
}

/// From/To Vec
impl<T> From<Vec<T>> for PBXValue
where
    PBXValue: From<T>,
{
    fn from(v: Vec<T>) -> Self {
        let inner = v.into_iter().map(|v| v.into()).collect();
        PBXValue::Vec(PBXVec(inner))
    }
}

impl<T> TryFrom<PBXValue> for Vec<T>
where
    T: TryFrom<PBXValue>,
    <T as TryFrom<PBXValue>>::Error: std::fmt::Debug,
{
    type Error = anyhow::Error;

    fn try_from(value: PBXValue) -> anyhow::Result<Self> {
        Ok(value
            .try_into_vec()?
            .0
            .into_iter()
            .map(|v| v.try_into().unwrap())
            .collect())
    }
}

impl<T> From<Option<T>> for PBXValue
where
    PBXValue: From<T>,
{
    fn from(t: Option<T>) -> Self {
        if let Some(v) = t {
            v.into()
        } else {
            Self::Null(())
        }
    }
}

impl PBXValue {
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

/// [`HashMap`] wrapper for [`PBXValue`] with helpers
#[derive(Default, Debug, Deref, PartialEq, Eq)]
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

gen_hash_map_helpers! {
    [string, String],
    [vec, PBXVec],
    [bool, bool],
    [number, isize],
    [kind, PBXObjectKind],
    [object, PBXHashMap]
}

/// [`Vec`] wrapper for [`PBXValue`] with helpers
#[derive(Default, Debug, Deref, PartialEq, Eq, derive_new::new)]
pub struct PBXVec(pub(crate) Vec<PBXValue>);

impl PBXVec {
    pub(crate) fn try_into_vec_strings(self) -> Result<Vec<String>> {
        let mut collector = vec![];
        for value in self.0 {
            collector.push(value.try_into_string()?);
        }
        Ok(collector)
    }

    pub(crate) fn as_vec_strings<'a>(&'a self) -> Vec<&'a String> {
        let mut collector = vec![];
        for value in self.0.iter() {
            if let Some(str) = value.as_string() {
                collector.push(str);
            }
        }
        collector
    }

    pub(crate) fn try_into_vec<T: TryFrom<T> + From<PBXValue>>(self) -> Result<Vec<T>> {
        let mut collector = vec![];
        for value in self.0 {
            collector.push(value.try_into()?);
        }
        Ok(collector)
    }
}
