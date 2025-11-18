use std::{ffi::CString, ptr};

use llvm_sys::core::LLVMPrintModuleToFile;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod codegen;
pub mod c_codegen;

pub fn parse(infile: &str) -> ast::Fundef {
    let src = std::fs::read_to_string(infile).unwrap();
    let lexer = lexer::Lexer::new(&src);
    let mut parser = parser::Parser::new(lexer);
    parser.parse_fundef().unwrap()
}

pub fn compile(ast: ast::Fundef, outfile: &str) {
    unsafe {
        let cg = codegen::CodegenContext::new("my_module");
        cg.compile_fundef(&ast);
        //llvm_sys::core::LLVMDumpModule(cg.module);
        let err = ptr::null_mut();
        LLVMPrintModuleToFile(cg.module, CString::new(outfile).unwrap().as_ptr(), err);
    }
}

pub fn compile_c(ast: ast::Fundef, outfile: &str) {
    let res = c_codegen::compile_fundef(ast);
    std::fs::write(outfile, res).unwrap();
}
