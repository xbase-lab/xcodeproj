use crate::pbxproj::PBXObjectKind;
use std::collections::HashMap;

/// Repersentation of all values that can be collected from pbxproj file.
#[derive(Debug, PartialEq, Eq)]
pub enum PBXValue {
    /// Normal String value. NOTE: This may literal string!
    String(String),
    /// Object value represented as [`HashMap`]
    Object(HashMap<String, Self>),
    /// Array of [`PBXValue`]
    Array(Vec<Self>),
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
    pub fn try_into_string(self) -> Result<String, Self> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            Err(self)
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
    pub fn try_into_object(self) -> Result<HashMap<String, Self>, Self> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Some(T)` if the value is [`Object`].
    ///
    /// [`Object`]: PBXValue::Object
    pub fn as_object(&self) -> Option<&HashMap<String, Self>> {
        if let Self::Object(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Array`].
    ///
    /// [`Array`]: PBXValue::Array
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }

    /// Returns `Some(T)` if the value is [`Array`].
    ///
    /// [`Array`]: PBXValue::Array
    pub fn as_array(&self) -> Option<&Vec<Self>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(T)` if the value is [`Array`].
    ///
    /// [`Array`]: PBXValue::Array
    pub fn try_into_array(self) -> Result<Vec<Self>, Self> {
        if let Self::Array(v) = self {
            Ok(v)
        } else {
            Err(self)
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
    pub fn try_into_number(self) -> Result<isize, Self> {
        if let Self::Number(v) = self {
            Ok(v)
        } else {
            Err(self)
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
    pub fn try_into_bool(self) -> Result<bool, Self> {
        if let Self::Bool(v) = self {
            Ok(v)
        } else {
            Err(self)
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
    pub fn try_into_kind(self) -> Result<PBXObjectKind, Self> {
        if let Self::Kind(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
