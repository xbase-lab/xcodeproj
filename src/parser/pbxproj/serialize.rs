#![allow(missing_docs)]
use super::object::PBXObjectKind;
use itertools::Itertools;
use std::{collections::HashMap, isize, num::ParseIntError};

use pest_consume::*;
use tap::Pipe;

/// Pest Parser to parse into [`XProj`]
#[derive(Parser)]
#[grammar = "parser/pbxproj/grammar.pest"]
pub(crate) struct PBXProjectParser;

pub(crate) type NodeResult<T> = std::result::Result<T, Error<Rule>>;
pub(crate) type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[parser]
impl PBXProjectParser {
    fn key(input: Node) -> NodeResult<String> {
        let inner = input.into_children().next().unwrap();
        let value = inner.as_str();
        value.to_string().pipe(Ok)
    }

    fn string(input: Node) -> NodeResult<PBXValue> {
        let value = input.as_str().replace("\"", "");
        // println!("string value: `{value}`");
        value.pipe(PBXValue::String).pipe(Ok)
    }

    fn kind(input: Node) -> NodeResult<PBXValue> {
        let value = PBXObjectKind::from(input.as_str());
        value.pipe(PBXValue::Kind).pipe(Ok)
    }

    fn ident(input: Node) -> NodeResult<PBXValue> {
        input.as_str().to_string().pipe(PBXValue::String).pipe(Ok)
    }

    fn uuid(input: Node) -> NodeResult<PBXValue> {
        input.as_str().to_string().pipe(PBXValue::String).pipe(Ok)
    }

    fn number(input: Node) -> NodeResult<PBXValue> {
        // TODO: identify versions as string instead of number or as ident!
        let value = input.as_str();
        if value.contains(".") {
            return Ok(PBXValue::String(value.into()));
        }
        value
            .parse()
            .map_err(|e: ParseIntError| input.error(e))
            .map(PBXValue::Number)
    }

    fn bool(input: Node) -> NodeResult<PBXValue> {
        match input.as_str() {
            "YES" => Ok(true),
            "NO" => Ok(false),
            value => input
                .error(format!("{value:?} is not parseable as boolean!"))
                .pipe(Err),
        }?
        .pipe(PBXValue::Bool)
        .pipe(Ok)
    }

    fn array(input: Node) -> NodeResult<PBXValue> {
        match_nodes!(input.into_children();
            [value(values)..] => values.collect::<Vec<PBXValue>>()
        )
        .pipe(PBXValue::Array)
        .pipe(Ok)
    }

    fn value(input: Node) -> NodeResult<PBXValue> {
        match_nodes!(input.into_children();
         [array(value)] => value,
         [object(value)] => value,
         [string(value)] => value,
         [bool(value)] => value,
         [kind(value)] => value,
         [number(value)] => value,
         [uuid(value)] => value,
         [ident(value)] => value
        )
        .pipe(Ok)
    }

    fn field(node: Node) -> NodeResult<(String, PBXValue)> {
        let (k, v) = node.into_children().collect_tuple().unwrap();
        let key = Self::key(k)?;
        let value = Self::value(v)?;

        Ok((key, value))
    }

    fn object(input: Node) -> NodeResult<PBXValue> {
        match_nodes!(input.into_children();
            [field(fields)..] => fields.collect::<HashMap<String, PBXValue>>(),
        )
        .pipe(PBXValue::Object)
        .pipe(Ok)
    }

    pub fn file(input: Node) -> NodeResult<HashMap<String, PBXValue>> {
        input
            .into_children()
            .next()
            .unwrap()
            .pipe(Self::object)?
            .try_into_object()
            .unwrap()
            .pipe(Ok)
    }
}

/// Repersentation of all values that can be collected from pbxproj file.
#[derive(Debug, PartialEq, Eq)]
pub enum PBXValue {
    /// Normal String value. NOTE: This may literal string!
    String(String),
    /// Object value represented as [`HashMap`]
    Object(HashMap<String, Self>),
    /// Array of [`XValue`]
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
    /// [`String`]: Value::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `Ok(T)` if the value is [`String`].
    ///
    /// [`String`]: Value::String
    pub fn try_into_string(self) -> Result<String, Self> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Some(T)` if the value is [`String`].
    ///
    /// [`String`]: Value::String
    pub fn as_string(&self) -> Option<&String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Object`].
    ///
    /// [`Object`]: Value::Object
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(..))
    }

    /// Returns `Ok(T)` if the value is [`Object`].
    ///
    /// [`Object`]: Value::Object
    pub fn try_into_object(self) -> Result<HashMap<String, Self>, Self> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Some(T)` if the value is [`Object`].
    ///
    /// [`Object`]: Value::Object
    pub fn as_object(&self) -> Option<&HashMap<String, Self>> {
        if let Self::Object(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Array`].
    ///
    /// [`Array`]: Value::Array
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }

    /// Returns `Some(T)` if the value is [`Array`].
    ///
    /// [`Array`]: Value::Array
    pub fn as_array(&self) -> Option<&Vec<Self>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(T)` if the value is [`Array`].
    ///
    /// [`Array`]: Value::Array
    pub fn try_into_array(self) -> Result<Vec<Self>, Self> {
        if let Self::Array(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the value is [`Number`].
    ///
    /// [`Number`]: Value::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `Ok(T)` if the value is [`Number`].
    ///
    /// [`Number`]: Value::Number
    pub fn try_into_number(self) -> Result<isize, Self> {
        if let Self::Number(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Some(T)` if the value is [`Number`].
    ///
    /// [`Number`]: Value::Number
    pub fn as_number(&self) -> Option<&isize> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Bool`].
    ///
    /// [`Bool`]: Value::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `Ok(T)` if the value is [`Bool`].
    ///
    /// [`Bool`]: Value::Bool
    pub fn try_into_bool(self) -> Result<bool, Self> {
        if let Self::Bool(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `Some(T)` if the value is [`Bool`].
    ///
    /// [`Bool`]: Value::Bool
    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the value is [`Kind`].
    ///
    /// [`Kind`]: Value::Kind
    #[must_use]
    pub fn is_kind(&self) -> bool {
        matches!(self, Self::Kind(..))
    }

    /// Returns `Some(T)` if the value is [`Kind`].
    ///
    /// [`Kind`]: Value::Kind
    pub fn as_kind(&self) -> Option<&PBXObjectKind> {
        if let Self::Kind(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `Ok(T)` if the value is [`Kind`].
    ///
    /// [`Bool`]: Value::Kind
    pub fn try_into_kind(self) -> Result<PBXObjectKind, Self> {
        if let Self::Kind(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[cfg(test)]
macro_rules! test_file {
    ($path:expr) => {{
        use super::*;

        let demo = std::fs::read_to_string($path).unwrap();
        let file = PBXProjectParser::parse(Rule::file, &demo);
        if file.is_err() {
            println!("Error: {:#?}", file.as_ref().unwrap_err())
        }
        assert!(file.is_ok());
        file.unwrap()
    }};
}

#[cfg(test)]
mod parse_tests {
    macro_rules! test_samples {
        ($($name:ident),*) => {
            $(#[test]
                fn $name() {
                    let (root, name) = (env!("CARGO_MANIFEST_DIR"), stringify!($name));
                    test_file!(format!("{root}/tests/samples/{name}.pbxproj"));
                })*
        };
    }

    test_samples![demo1, demo2, demo3, demo4, demo5, demo6, demo7, demo8, demo9];
}

#[cfg(test)]
mod consume {
    use super::*;
    use pest_consume::Parser;

    #[test]
    fn parse_key_pair() {
        let str =
            "0EC07ACE89150EC90442393B = {isa = PBXBuildFile; fileRef = F2E640B5C2B85914F6801498; };";
        let (key, value) = PBXProjectParser::parse(Rule::field, str)
            .map(|n| PBXProjectParser::field(n.single().unwrap()))
            .unwrap()
            .unwrap();

        assert_eq!(key, "0EC07ACE89150EC90442393B");
        assert!(matches!(value, PBXValue::Object(_)));

        let object = value.try_into_object().unwrap();
        assert_eq!(
            object.get("isa"),
            Some(&PBXValue::Kind("PBXBuildFile".into()))
        );
        assert_eq!(
            object["fileRef"],
            PBXValue::String("F2E640B5C2B85914F6801498".into())
        );
    }

    #[test]
    #[ignore = "reason"]
    fn test_consume() {
        let demo = include_str!("../../../tests/samples/demo2.pbxproj");
        let inputs = PBXProjectParser::parse(Rule::file, demo).unwrap();
        let input = inputs.single().unwrap();
        let object = PBXProjectParser::file(input).unwrap();
        println!("{object:#?}");
    }
}
