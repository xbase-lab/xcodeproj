#[derive(pest_derive::Parser)]
#[grammar = "parser/pbxproj.pest"]

pub(crate) struct PBXProjParser;

#[test]
fn test_demo1() {
    use pest::Parser;
    let demo = include_str!("../../tests/samples/demo1.pbxproj");
    let file = PBXProjParser::parse(Rule::file, demo);
    file.unwrap();
}

#[test]
fn test_demo2() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo2.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo3() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo3.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo4() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo4.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo5() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo5.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo6() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo6.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo7() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo7.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo8() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo8.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}

#[test]
fn test_demo9() {
    use pest::Parser;
    let content = include_str!("../../tests/samples/demo9.pbxproj");
    PBXProjParser::parse(Rule::file, content).unwrap();
}
