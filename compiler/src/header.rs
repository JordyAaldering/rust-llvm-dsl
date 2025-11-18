use crate::ast::*;

pub fn compile_header(fundef: Fundef) -> String {
    let ret_type = match fundef.ret_type {
        Type::I32 => "i32",
    };

    let args: Vec<String> = fundef.args.iter().map(|(ty, id)| {
        let ty_str = match ty {
            Type::I32 => "i32",
        };
        format!("{}: {}", id, ty_str)
    }).collect();

    format!("unsafe extern \"C\" {{\n    fn {}({}) -> {};\n}}\n", fundef.name, args.join(", "), ret_type)
}
