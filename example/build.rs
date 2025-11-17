use std::process::Command;
use std::path::Path;

use compiler::compile;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dst_ll = Path::new(&out_dir).join("simple.ll");
    let dst_o = Path::new(&out_dir).join("simple.o");

    println!("cargo:rerun-if-changed=src/simple.dsl");

    compile("src/simple.dsl", dst_ll.to_str().unwrap());

    // 2. Convert LLVM IR → object file using llvm-as + llc
    Command::new("llvm-as")
        .args([dst_ll.to_str().unwrap()])
        .status()
        .expect("failed to assemble LLVM");

    let bc_path = dst_ll.with_extension("bc");

    Command::new("llc")
        .args([bc_path.to_str().unwrap(), "-filetype=obj", "-o", dst_o.to_str().unwrap()])
        .status()
        .expect("failed to generate object file");

    // 3. Tell Rust to link it
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-arg={}/simple.o", out_dir);
}
