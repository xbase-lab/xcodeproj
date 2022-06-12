use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use phf_codegen::Map;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let ref mut f = File::create(Path::new(&out_dir).join("file_types.rs")).unwrap();
    let mut map: Map<&str> = phf_codegen::Map::new();
    let txt = include_str!("./res/file_type.txt");

    txt.lines().for_each(|line| {
        let (key, value) = line.split_once(": ").unwrap();
        map.entry(key, &format!("\"{value}\""));
    });

    let map = map.build();
    let base = "static XCODE_FILE_TYPES: phf::Map<&'static str, &'static str>";

    write!(f, "{base} = \n{map};\n",).unwrap();
}
