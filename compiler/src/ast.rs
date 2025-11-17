#[derive(Clone, Debug)]
pub struct Fundef {
    pub name: String,
    pub args: Vec<(Type, String)>,
    pub ret_type: Type,
    pub body: Expr,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Var(String),
    I32(i32),
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub l: Box<Expr>,
    pub r: Box<Expr>,
    pub op: Bop,
}

#[derive(Copy, Clone, Debug)]
pub enum Bop {
    Add,
    Sub,
}

#[derive(Copy, Clone, Debug)]
pub enum Type {
    I32,
}
