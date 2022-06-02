#[derive(pest_derive::Parser)]
#[grammar = "parser/pbxproj.pest"]
pub(crate) struct PBXProjParser;

#[cfg(test)]
macro_rules! test_file {
    ($path:expr) => {{
        use $crate::parser::*;

        let demo = std::fs::read_to_string($path).unwrap();
        let file = PBXProjParser::parse(Rule::file, &demo);
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
