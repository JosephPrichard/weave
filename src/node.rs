// Joseph Prichard
// 3/15/2024
// Abstract syntax tree for the programming language

pub enum Node {
    DefFunc(DefFuncNode),
    DefStruct(DefStructNode),
    Block(Vec<Node>),
    Cond(CondNode),
    Guard(GuardNode),
    While(WhileNode),
    For(ForNode),
    Assign(String, Expr),
    Return(Expr),
    Func(FuncNode),
    Struct(StructNode),
    Array(Vec<Node>),
}

pub enum Expr {
    Constant(Constant),
    Variable(String),
    BinArith(ArithExpr),
    BinBool(BoolExpr),
    Unary(Unop, Box<Expr>),
    CallFunc(FuncNode),
}

pub enum Type {
    Int,
    Float,
    Bool,
    Char,
    String,
    Array(Box<Type>),
    Struct(String),
}

pub struct DefFuncNode {
    pub iden: String,
    pub args: Vec<(String, Type)>,
    pub body: Box<Node>
}

pub struct DefStructNode {
    pub iden: String,
    pub fields: Vec<(String, Type)>
}

pub struct CondNode {
    pub cond: BoolExpr,
    pub this: Box<Node>,
    pub that: Box<Node>
}

pub struct GuardNode {
    pub cond: BoolExpr,
    pub this: Expr,
}

pub struct WhileNode {
    pub cond: Expr,
    pub body: Box<Node>,
}

pub struct ForNode {
    pub element: String,
    pub index: Option<String>,
    pub coll: Expr,
}

pub struct FuncNode {
    pub iden: String,
    pub args: Vec<Expr>
}

pub struct StructNode {
    pub iden: String,
    pub fields: Vec<(String, Expr)>
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Int(i32),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

pub enum ArithOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum BoolOp {
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

pub struct ArithExpr {
    pub op: ArithOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

pub struct BoolExpr {
    pub op: BoolOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}