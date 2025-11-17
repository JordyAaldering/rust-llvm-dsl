use compiler::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let src = std::fs::read_to_string(&args[0]).unwrap();
    let lexer = lexer::Lexer::new(&src);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_fundef().unwrap();

    unsafe {
        let cg = codegen::CodegenContext::new("my_module");
        cg.compile_fundef(&ast);
        llvm_sys::core::LLVMDumpModule(cg.module);
    }
}
