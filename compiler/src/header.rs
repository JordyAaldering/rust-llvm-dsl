use crate::ast::*;

pub fn compile_header(fundef: Fundef) -> String {
    let mut s = String::new();

    //c_code.push_str("#include <stdint.h>\n\n");

    let ret_type = match fundef.ret_type {
        Type::I32 => "i32",
    };

    let args: Vec<String> = fundef.args.iter().map(|(ty, id)| {
        let ty_str = match ty {
            Type::I32 => "i32",
        };
        format!("{}: {}", id, ty_str)
    }).collect();

    s.push_str("unsafe extern \"C\" {\n");
    s.push_str(&format!("    fn DSL_{}({}) -> {};\n", fundef.name, args.join(", "), ret_type));
    s.push_str("}\n\n");

    // Here we have the opportunity to add checks, dispatch to different implementations, etc.
    s.push_str(&format!("fn {}({}) -> {} {{\n", fundef.name, args.join(", "), ret_type));
    s.push_str(&format!("    unsafe {{ DSL_{}({}) }}\n", fundef.name, fundef.args.iter().map(|(_, id)| id.clone()).collect::<Vec<_>>().join(", ")));
    s.push_str("}\n");

    s
}
