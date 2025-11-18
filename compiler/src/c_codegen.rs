use crate::ast::*;

pub fn compile_fundef(fundef: Fundef) -> String {
    let mut c_code = String::new();

    // Function signature
    let ret_type = match fundef.ret_type {
        Type::I32 => "int",
    };

    let args: Vec<String> = fundef.args.iter().map(|(ty, name)| {
        let ty_str = match ty {
            Type::I32 => "int",
        };
        format!("{} {}", ty_str, name)
    }).collect();

    c_code.push_str(&format!("{} {}({}) {{\n", ret_type, fundef.name, args.join(", ")));

    c_code.push_str(&format!("    return {};\n", compile_expr(fundef.body)));

    c_code.push_str("}\n");

    c_code
}

fn compile_expr(expr: Expr) -> String {
    match expr {
        Expr::Binary(binary) => {
            let l = compile_expr(*binary.l);
            let r = compile_expr(*binary.r);
            let op = match binary.op {
                Bop::Add => "+",
                Bop::Sub => "-",
            };
            format!("({} {} {})", l, op, r)
        }
        Expr::Var(s) => s,
        Expr::I32(n) => n.to_string(),
    }
}
