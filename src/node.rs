// Joseph Prichard
// 3/15/2024
// Abstract syntax tree for the programming language

pub enum Node {
    DefFunc(DefFuncNode),
    DefStruct(DefStructNode),
    DefTypeAlias(DefTypeAliasNode),
    Import(ImportNode),
    Constant(Const),
    Variable(String),
    Binop(BinopNode),
    Unop(UnopNode),
    CallFunc(FuncNode),
    If(IfNode),
    Else(Vec<Node>),
    Guard(GuardNode),
    While(WhileNode),
    For(ForNode),
    Assign(String, Box<Node>),
    Return(Box<Node>),
    Break,
    Continue,
    Func(FuncNode),
    Struct(StructNode),
    Array(Vec<Node>),
    Tuple(Vec<Node>),
    Range(i32, i32),
    Lambda(LambdaNode)
}
pub enum TypeNode {
    Array(Box<TypeNode>),
    Fn(Vec<TypeNode>, Option<Box<TypeNode>>),
    Iden(String),
}

pub struct DefFuncNode {
    pub iden: String,
    pub args: Vec<(String, TypeNode)>,
    pub ret: Option<TypeNode>,
    pub body: Vec<Node>
}

pub struct DefStructNode {
    pub iden: String,
    pub fields: Vec<(String, TypeNode)>
}

pub struct DefTypeAliasNode {
    pub iden: String,
    pub type_node: TypeNode
}

pub struct ImportNode {
    pub iden: String
}

pub struct IfNode {
    pub cond: Box<Node>,
    pub body: Vec<Node>,
}

pub struct GuardNode {
    pub cond: Box<Node>,
    pub this: Box<Node>,
}

pub struct WhileNode {
    pub cond: Box<Node>,
    pub body: Vec<Node>,
}

pub struct ForNode {
    pub element: String,
    pub index: Option<String>,
    pub collection: Box<Node>,
}

pub struct FuncNode {
    pub iden: String,
    pub args: Vec<Node>
}

pub struct StructNode {
    pub iden: String,
    pub fields: Vec<(String, TypeNode)>
}

pub struct LambdaNode {
    pub args: Vec<(String, Option<TypeNode>)>,
    pub body: Box<Node>
}

pub struct BinopNode {
    pub op: Bop,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

pub struct UnopNode {
    pub op: Uop,
    pub expr: Box<Node>,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Const {
    Int(i32),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Bop {
    Plus,
    Exp,
    Minus,
    Multiply,
    Divide,
    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Uop {
    Not,
    Minus
}
