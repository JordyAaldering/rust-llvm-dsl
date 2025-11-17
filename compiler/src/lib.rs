use std::{ffi::CString, ptr};

use llvm_sys::core::LLVMPrintModuleToFile;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod codegen;

pub fn compile(infile: &str, outfile: &str) {
    let src = std::fs::read_to_string(infile).unwrap();
    let lexer = lexer::Lexer::new(&src);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_fundef().unwrap();

    unsafe {
        let cg = codegen::CodegenContext::new("my_module");
        cg.compile_fundef(&ast);
        //llvm_sys::core::LLVMDumpModule(cg.module);
        let err = ptr::null_mut();
        LLVMPrintModuleToFile(cg.module, CString::new(outfile).unwrap().as_ptr(), err);
    }
}
