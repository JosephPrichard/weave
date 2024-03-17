// Joseph Prichard
// 3/15/2024
// Abstract syntax tree for the programming language

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
}

pub enum Value {
    Constant(Constant),
    Variable(String),
}

pub enum Bop {
    Add,
    Subtract,
    Multiply,
    Divide,
    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,
}

pub enum Unop {
    Not,
    Minus
}

pub struct Func {
    pub iden: String,
    pub args: Vec<Expr>
}

pub struct BinaryExpr {
    pub op: Bop,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

pub enum Expr {
    Value(Value),
    BinaryOp(BinaryExpr),
    UnaryOp(Unop, Box<Expr>),
    Func(Func),
}

pub enum Statement {
    Assign(String, Expr),
    Func(Func),
    Return(Expr),
}

type Body = Vec<Statement>;

pub enum Ast {
    Block(Body),
    IfElse(BinaryExpr, Body, Body),
    IfReturn(BinaryExpr, Expr),
    While(Expr, Body),
    DefineFunc(String, Vec<String>, Body),
}

