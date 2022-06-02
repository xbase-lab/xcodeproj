#[derive(pest_derive::Parser)]
#[grammar = "parser/pbxproj.pest"]

pub(crate) struct PBXProjParser;

#[test]
fn test_demo1() {
    use pest::Parser;
    let demo = include_str!("../../tests/samples/demo1.pbxproj");
    let file = PBXProjParser::parse(Rule::file, demo);
    file.unwrap();

#[cfg(test)]
macro_rules! test_file {
    ($path:literal) => {{
        let demo = include_str!($path);
        let file = PBXProjParser::parse(Rule::file, demo);
        if file.is_err() {
            println!("Error: {:#?}", file.as_ref().unwrap_err())
        }
        assert!(file.is_ok());
    }};
}

#[test]
fn test_demo1() {
    test_file!("../../tests/samples/demo1.pbxproj");
}

#[test]
fn parse_uuid_key() {
    let value =
        "0EC07ACE89150EC90442393B = {isa = PBXBuildFile; fileRef = F2E640B5C2B85914F6801498; };";
    let node = PBXProjParser::parse(Rule::field, value).unwrap();
    println!("{:#?}", PBXProjParser::field(node.single().unwrap()));
}

#[test]
fn test_demo2() {
    test_file!("../../tests/samples/demo2.pbxproj")
}

#[test]
fn test_demo3() {
    test_file!("../../tests/samples/demo3.pbxproj")
}

#[test]
fn test_demo4() {
    test_file!("../../tests/samples/demo4.pbxproj")
}

#[test]
fn test_demo5() {
    test_file!("../../tests/samples/demo5.pbxproj")
}

#[test]
fn test_demo6() {
    test_file!("../../tests/samples/demo6.pbxproj")
}

#[test]
fn test_demo7() {
    test_file!("../../tests/samples/demo7.pbxproj")
}

#[test]
fn test_demo8() {
    test_file!("../../tests/samples/demo8.pbxproj")
}

#[test]
fn test_demo9() {
    test_file!("../../tests/samples/demo9.pbxproj")
}
