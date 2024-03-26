// Joseph Prichard
// 3/15/2024
// Abstract syntax tree for the programming language

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Array(Box<TypeNode>),
    Fn(Vec<TypeNode>, Option<Box<TypeNode>>),
    Iden(String),
}

#[derive(Debug, PartialEq)]
pub struct DefFuncNode {
    pub iden: String,
    pub args: Vec<(String, TypeNode)>,
    pub ret: Option<TypeNode>,
    pub body: Vec<Node>
}

#[derive(Debug, PartialEq)]
pub struct DefStructNode {
    pub iden: String,
    pub fields: Vec<(String, TypeNode)>
}

#[derive(Debug, PartialEq)]
pub struct DefTypeAliasNode {
    pub iden: String,
    pub type_node: TypeNode
}

#[derive(Debug, PartialEq)]
pub struct ImportNode {
    pub iden: String
}

#[derive(Debug, PartialEq)]
pub struct IfNode {
    pub cond: Box<Node>,
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct GuardNode {
    pub cond: Box<Node>,
    pub this: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct WhileNode {
    pub cond: Box<Node>,
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ForNode {
    pub element: String,
    pub index: Option<String>,
    pub collection: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct FuncNode {
    pub iden: String,
    pub args: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct StructNode {
    pub iden: String,
    pub fields: Vec<(String, TypeNode)>,
}

#[derive(Debug, PartialEq)]
pub struct LambdaNode {
    pub args: Vec<(String, Option<TypeNode>)>,
    pub body: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct BinopNode {
    pub op: Bop,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct UnopNode {
    pub op: Uop,
    pub expr: Box<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
