use std::path::Path;

use compiler::{parse, compile_c};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let c_path = Path::new(&out_dir).join("simple.c");

    let ast = parse("src/simple.dsl");
    compile_c(ast, c_path.to_str().unwrap());

    cc::Build::new()
        .file(&c_path)
        .compile("simple");

    println!("cargo:rerun-if-changed=src/simple.dsl");
}
