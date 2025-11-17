use std::{collections::HashMap, ffi::CString};

use crate::ast::*;

use llvm_sys::{core::*, prelude::*};

pub struct CodegenContext {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
}

impl CodegenContext {
    pub fn new(module_name: &str) -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module_name_c = CString::new(module_name).unwrap();
            let module = LLVMModuleCreateWithNameInContext(module_name_c.as_ptr(), context);
            let builder = LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
            }
        }
    }

    pub fn compile_fundef(&self, f: &Fundef) -> LLVMValueRef {
        unsafe {
            // 1. Function type
            let arg_types: Vec<LLVMTypeRef> =
                f.args.iter().map(|(ty, _)| self.llvm_type(ty)).collect();

            let fn_type = LLVMFunctionType(
                self.llvm_type(&f.ret_type),
                arg_types.as_ptr() as *mut _,
                arg_types.len() as u32,
                0,
            );

            // 2. Declare the function
            let function = LLVMAddFunction(
                self.module,
                f.name.as_ptr() as *const _,
                fn_type,
            );

            // 3. Create entry block
            let entry = LLVMAppendBasicBlockInContext(
                self.context,
                function,
                "entry\0".as_ptr() as *const _
            );

            LLVMPositionBuilderAtEnd(self.builder, entry);

            // 4. Build variable map (arguments)
            let mut vars = HashMap::new();
            for (i, (_, name)) in f.args.iter().enumerate() {
                let param = LLVMGetParam(function, i as u32);
                vars.insert(name.clone(), param);
            }

            // 5. Compile the function body
            let ret_val = self.compile_expr(&f.body, &mut vars);

            // 6. Emit `ret`
            LLVMBuildRet(self.builder, ret_val);

            function
        }
    }

    pub fn compile_expr(
        &self,
        expr: &Expr,
        vars: &mut HashMap<String, LLVMValueRef>,
    ) -> LLVMValueRef {
        match expr {
            Expr::I32(n) => self.build_i32(*n),

            Expr::Var(name) => vars[name],

            Expr::Binary(bin) => {
                let l = self.compile_expr(&bin.l, vars);
                let r = self.compile_expr(&bin.r, vars);

                unsafe {
                    match bin.op {
                        Bop::Add => LLVMBuildAdd(
                            self.builder, l, r,
                            "addtmp\0".as_ptr() as *const _,
                        ),
                        Bop::Sub => LLVMBuildSub(
                            self.builder, l, r,
                            "subtmp\0".as_ptr() as *const _,
                        ),
                    }
                }
            }
        }
    }

    fn build_i32(&self, v: i32) -> LLVMValueRef {
        unsafe {
            LLVMConstInt(self.u32_type(), v as u64, 0)
        }
    }

    fn llvm_type(&self, t: &Type) -> LLVMTypeRef {
        match t {
            Type::I32 => self.u32_type(),
        }
    }

    pub fn u32_type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt32TypeInContext(self.context) }
    }
}