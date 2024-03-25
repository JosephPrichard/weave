// Joseph Prichard
// 3/15/2024
// Implementation of an ast walker for the interpreter

use crate::node::{Const, Uop, FuncNode, Node, UnopNode, BinopNode, Bop};

pub enum RunErr {
    Type(&'static str),
    Undefined(String),
}

impl RunErr {
    fn undefined(iden: &str) -> RunErr {
        RunErr::Undefined(format!("Undefined variable {}", iden))
    }
}

type StackFrame = Vec<(String, Const)>;

pub struct Environment {
    frames: Vec<StackFrame>
}

impl Environment {
    pub fn push(&mut self) {
        self.frames.push(vec![])
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }

    pub fn top(&mut self) -> &mut StackFrame {
        if self.frames.len() == 0 {
            self.push()
        }
        let len = self.frames.len();
        &mut self.frames[len - 1]
    }

    pub fn write(&mut self, iden: &str, constant: Const) -> Result<(), RunErr> {
        let frame = self.top();
        for pair in frame {
            if iden == pair.0 {
                pair.1 = constant;
                return Ok(())
            }
        }
        Err(RunErr::undefined(iden))
    }

    pub fn read(&mut self, iden: &str) -> Result<&Const, RunErr> {
        let frame = self.top();
        for pair in frame {
            if iden == pair.0 {
                return Ok(&pair.1)
            }
        }
        Err(RunErr::undefined(iden))
    }
}

pub type ExprResult = Result<Const, RunErr>;

pub fn eval_node(node: &Node) -> ExprResult {
    match node {
        Node::Constant(constant) => Ok(constant.clone()),
        Node::Variable(v) => panic!("Variable access not yet implemented"),
        Node::Binop(node) => eval_binary_expr(node),
        Node::Unop(node) => eval_unary_expr(node),
        Node::CallFunc(node) => eval_func(node),
        _ => panic!("Not yet implemented")
    }
}

pub fn eval_binary_expr(node: &BinopNode) -> ExprResult {
    let lhs = eval_node(node.lhs.as_ref())?;
    let rhs = eval_node(node.rhs.as_ref())?;
    match node.op {
        Bop::Plus => match (lhs, rhs) {
            (Const::Int(lhs), Const::Int(rhs)) => Ok(Const::Int(lhs + rhs)),
            (Const::Float(lhs), Const::Float(rhs)) => Ok(Const::Float(lhs + rhs)),
            (Const::String(lhs), Const::String(rhs)) => {
                let mut s_new = lhs.to_owned();
                s_new.push_str(&rhs);
                Ok(Const::String(s_new))
            }
            _ => Err(RunErr::Type("Add operator must be applied to 2 ints, floats, or strings"))
        },
        Bop::Multiply => match (lhs, rhs) {
            (Const::Int(lhs), Const::Int(rhs)) => Ok(Const::Int(lhs * rhs)),
            (Const::Float(lhs), Const::Float(rhs)) => Ok(Const::Float(lhs * rhs)),
            (Const::String(lhs), Const::Int(rhs)) => {
                let mut s_new = String::new();
                for _ in 0..rhs {
                    s_new.push_str(&lhs)
                }
                Ok(Const::String(s_new))
            }
            _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints, 2 floats, or between a string and an int"))
        },
        Bop::Minus => match (lhs, rhs) {
            (Const::Int(lhs), Const::Int(rhs)) => Ok(Const::Int(lhs - rhs)),
            (Const::Float(lhs), Const::Float(rhs)) => Ok(Const::Float(lhs - rhs)),
            _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints or 2 floats"))
        },
        Bop::Divide => match (lhs, rhs) {
            (Const::Int(lhs), Const::Int(rhs)) => Ok(Const::Int(lhs / rhs)),
            (Const::Float(lhs), Const::Float(rhs)) => Ok(Const::Float(lhs / rhs)),
            _ => Err(RunErr::Type("Divide operator must be applied to 2 ints or 2 floats"))
        },
        Bop::Exp => match (lhs, rhs) {
            (Const::Int(lhs), Const::Int(rhs)) => {
                if rhs < 0 {
                    Err(RunErr::Type("Exponent operator rhs must be a positive int"))
                } else {
                    Ok(Const::Int(lhs.pow(rhs as u32)))
                }
            },
            _ => Err(RunErr::Type("Exponent operator must be applied to an int and a positive int"))
        },
        Bop::Eq => Ok(Const::Bool(lhs == rhs)),
        Bop::Neq => Ok(Const::Bool(lhs != rhs)),
        Bop::Leq => Ok(Const::Bool(lhs <= rhs)),
        Bop::Geq => Ok(Const::Bool(lhs >= rhs)),
        Bop::Lt => Ok(Const::Bool(lhs < rhs)),
        Bop::Gt => Ok(Const::Bool(lhs > rhs)),
        Bop::And => match (lhs, rhs) {
            (Const::Bool(lhs), Const::Bool(rhs)) => Ok(Const::Bool(lhs && rhs)),
            _ => Err(RunErr::Type("And operator must be applied to 2 bools"))
        },
        Bop::Or => match (lhs, rhs) {
            (Const::Bool(lhs), Const::Bool(rhs)) => Ok(Const::Bool(lhs && rhs)),
            _ => Err(RunErr::Type("Or operator must be applied to 2 bools"))
        },
    }
}
pub fn eval_unary_expr(node: &UnopNode) -> ExprResult {
    match node.op {
        Uop::Not => match eval_node(node.expr.as_ref())? {
            Const::Bool(b) => Ok(Const::Bool(b)),
            _ => Err(RunErr::Type("Not operator must be applied to a bool"))
        }
        Uop::Minus => match eval_node(node.expr.as_ref())? {
            Const::Int(n) => Ok(Const::Int(-n)),
            Const::Float(n) => Ok(Const::Float(-n)),
            _ => Err(RunErr::Type("Unary minus must be applied to an int or a float"))
        }
    }
}

pub fn eval_func(func: &FuncNode) -> Result<Const, RunErr> {
    let mut results = vec![];
    for arg in func.args.iter() {
        match eval_node(&arg) {
            Ok(result) => results.push(result),
            Err(err) => {
                return Err(err)
            }
        }
    }
    panic!("Function call not yet implemented")
}