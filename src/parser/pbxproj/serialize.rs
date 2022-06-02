#![allow(dead_code)]
use std::{collections::HashMap, num::ParseIntError};

use itertools::Itertools;
use pest_consume::*;
use tap::Pipe;

#[derive(Parser)]
#[grammar = "parser/pbxproj/grammar.pest"]
pub(crate) struct XProjParser;

#[derive(Debug, PartialEq, Eq)]
pub enum XValue {
    UUID(String),
    Object(HashMap<String, Self>),
    Array(Vec<Self>),
    String(String),
    Number(u32),
    Bool(bool),
    Kind(String),
}

impl XValue {
    fn try_into_object(self) -> std::result::Result<HashMap<String, Self>, Self> {
        if let Self::Object(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error<Rule>>;
pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[parser]
impl XProjParser {
    fn key(input: Node) -> Result<String> {
        let inner = input.into_children().next().unwrap();
        let value = inner.as_str();
        value.to_string().pipe(Ok)
    }

    fn string(input: Node) -> Result<XValue> {
        let value = input.as_str().replace("\"", "");
        // println!("string value: `{value}`");
        value.pipe(XValue::String).pipe(Ok)
    }

    fn kind(input: Node) -> Result<XValue> {
        input.as_str().to_string().pipe(XValue::Kind).pipe(Ok)
    }

    fn ident(input: Node) -> Result<XValue> {
        input.as_str().to_string().pipe(XValue::String).pipe(Ok)
    }

    fn uuid(input: Node) -> Result<XValue> {
        input.as_str().to_string().pipe(XValue::UUID).pipe(Ok)
    }

    fn number(input: Node) -> Result<XValue> {
        // TODO: identify versions as string instead of number or as ident!
        let value = input.as_str();
        if value.contains(".") {
            return Ok(XValue::String(value.into()));
        }
        value
            .parse()
            .map_err(|e: ParseIntError| input.error(e))
            .map(XValue::Number)
    }

    fn bool(input: Node) -> Result<XValue> {
        match input.as_str() {
            "YES" => Ok(true),
            "NO" => Ok(false),
            value => input
                .error(format!("{value:?} is not parseable as boolean!"))
                .pipe(Err),
        }?
        .pipe(XValue::Bool)
        .pipe(Ok)
    }

    fn array(input: Node) -> Result<XValue> {
        match_nodes!(input.into_children();
            [value(values)..] => values.collect::<Vec<XValue>>()
        )
        .pipe(XValue::Array)
        .pipe(Ok)
    }

    fn value(input: Node) -> Result<XValue> {
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

    fn field(node: Node) -> Result<(String, XValue)> {
        let (k, v) = node.into_children().collect_tuple().unwrap();
        let key = Self::key(k)?;
        let value = Self::value(v)?;

        Ok((key, value))
    }

    fn object(input: Node) -> Result<XValue> {
        match_nodes!(input.into_children();
            [field(fields)..] => fields.collect::<HashMap<String, XValue>>(),
        )
        .pipe(XValue::Object)
        .pipe(Ok)
    }

    fn file(input: Node) -> Result<XValue> {
        input.into_children().next().unwrap().pipe(Self::object)
    }
}

#[cfg(test)]
macro_rules! test_file {
    ($path:expr) => {{
        use $crate::parser::*;

        let demo = std::fs::read_to_string($path).unwrap();
        let file = XProjParser::parse(Rule::file, &demo);
        if file.is_err() {
            println!("Error: {:#?}", file.as_ref().unwrap_err())
        }
        assert!(file.is_ok());
        file.unwrap()
    }};
}

#[cfg(test)]
mod parse_tests {
    use pest::Parser;
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
        let (key, value) = XProjParser::parse(Rule::field, str)
            .map(|n| XProjParser::field(n.single().unwrap()))
            .unwrap()
            .unwrap();

        assert_eq!(key, "0EC07ACE89150EC90442393B");
        assert!(matches!(value, XValue::Object(_)));

        let object = value.try_into_object().unwrap();
        assert_eq!(
            object.get("isa"),
            Some(&XValue::Kind("PBXBuildFile".into()))
        );
        assert_eq!(
            object["fileRef"],
            XValue::UUID("F2E640B5C2B85914F6801498".into())
        );
    }

    #[test]
    #[ignore = "reason"]
    fn test_consume() {
        let demo = include_str!("../../tests/samples/demo2.pbxproj");
        let inputs = XProjParser::parse(Rule::file, demo).unwrap();
        let input = inputs.single().unwrap();
        let object = XProjParser::file(input).unwrap();
        println!("{object:#?}");
    }
}
