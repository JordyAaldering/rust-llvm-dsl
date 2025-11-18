use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let c_path = Path::new(&out_dir).join("simple.c");
    let h_path = Path::new(&out_dir).join("simple.rs");

    let ast = compiler::parse("src/simple.dsl");
    compiler::compile_c(ast.clone(), c_path.to_str().unwrap());

    cc::Build::new()
        .file(&c_path)
        .compile("simple");

    compiler::compile_header(ast, h_path.to_str().unwrap());

    println!("cargo:rerun-if-changed=src/simple.dsl");
}
