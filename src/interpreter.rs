// Joseph Prichard
// 3/15/2024
// Implementation of an ast walker for the interpreter

use crate::node::{Constant, Expr, Unop, ArithOp, BoolOp, CondNode, ArithExpr, BoolExpr, FuncNode, Node};

pub enum RunErr {
    Type(&'static str),
    Undefined(String),
}

impl RunErr {
    fn undefined(iden: &str) -> RunErr {
        RunErr::Undefined(format!("Undefined variable {}", iden))
    }
}

type StackFrame = Vec<(String, Constant)>;

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

    pub fn write(&mut self, iden: &str, constant: Constant) -> Result<(), RunErr> {
        let frame = self.top();
        for pair in frame {
            if iden == pair.0 {
                pair.1 = constant;
                return Ok(())
            }
        }
        Err(RunErr::undefined(iden))
    }

    pub fn read(&mut self, iden: &str) -> Result<&Constant, RunErr> {
        let frame = self.top();
        for pair in frame {
            if iden == pair.0 {
                return Ok(&pair.1)
            }
        }
        Err(RunErr::undefined(iden))
    }
}

pub type ExprResult = Result<Constant, RunErr>;

pub fn eval_expr(expr: &Expr) -> ExprResult {
    match expr {
        Expr::Constant(constant) => Ok(constant.clone()),
        Expr::Variable(v) => panic!("Variable access not yet implemented"),
        Expr::BinArith(arith) => eval_arith_expr(arith),
        Expr::BinBool(bool) => Ok(Constant::Bool(eval_bool_expr(bool)?)),
        Expr::Unary(op, expr) => eval_unary_expr(op, expr),
        Expr::CallFunc(func) => eval_func(func)
    }
}

pub fn eval_arith_expr(expr: &ArithExpr) -> ExprResult {
    let lhs = eval_expr(expr.lhs.as_ref())?;
    let rhs = eval_expr(expr.rhs.as_ref())?;
    match expr.op {
        ArithOp::Add => eval_add(&lhs, &rhs),
        ArithOp::Multiply => eval_multiply(&lhs, &rhs),
        ArithOp::Subtract => eval_subtract(&lhs, &rhs),
        ArithOp::Divide => eval_divide(&lhs, &rhs),
    }
}

pub fn eval_bool_expr(expr: &BoolExpr) -> Result<bool, RunErr> {
    let lhs = eval_expr(expr.lhs.as_ref())?;
    let rhs = eval_expr(expr.rhs.as_ref())?;
    match expr.op {
        BoolOp::Eq => Ok(lhs == rhs),
        BoolOp::Neq => Ok(lhs != rhs),
        BoolOp::Leq => Ok(lhs <= rhs),
        BoolOp::Geq => Ok(lhs >= rhs),
        BoolOp::Lt => Ok(lhs < rhs),
        BoolOp::Gt => Ok(lhs > rhs)
    }
}

pub fn eval_add(lhs: &Constant, rhs: &Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs),  Constant::Int(rhs)) => Ok(Constant::Int(lhs + rhs)),
        (Constant::Float(lhs),  Constant::Float(rhs)) => Ok(Constant::Float(lhs + rhs)),
        (Constant::String(lhs), Constant::String(rhs)) => {
            let mut s_new = lhs.to_owned();
            s_new.push_str(&rhs);
            Ok(Constant::String(s_new))
        }
        _ => Err(RunErr::Type("Add operator must be applied to 2 ints, floats, or strings"))
    }
}

pub fn eval_multiply(lhs: &Constant, rhs: &Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs), Constant::Int(rhs)) => Ok(Constant::Int(lhs * rhs)),
        (Constant::Float(lhs), Constant::Float(rhs)) => Ok(Constant::Float(lhs * rhs)),
        (Constant::String(lhs), Constant::Int(rhs)) => {
            let mut s_new = String::new();
            for _ in 0..*rhs {
                s_new.push_str(lhs)
            }
            Ok(Constant::String(s_new))
        }
        _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints, 2 floats, or between a string and an int"))
    }
}

pub fn eval_subtract(lhs: &Constant, rhs: &Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs), Constant::Int(rhs)) => Ok(Constant::Int(lhs - rhs)),
        (Constant::Float(lhs), Constant::Float(rhs)) => Ok(Constant::Float(lhs - rhs)),
        _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints or 2 floats"))
    }
}

pub fn eval_divide(lhs: &Constant, rhs: &Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs), Constant::Int(rhs)) => Ok(Constant::Int(lhs / rhs)),
        (Constant::Float(lhs), Constant::Float(rhs)) => Ok(Constant::Float(lhs / rhs)),
        _ => Err(RunErr::Type("Divide operator must be applied to 2 ints or 2 floats"))
    }
}

pub fn eval_unary_expr(op: &Unop, expr: &Expr) -> ExprResult {
    match op {
        Unop::Not => match eval_expr(expr)? {
            Constant::Bool(b) => Ok(Constant::Bool(b)),
            _ => Err(RunErr::Type("Not operator must be applied to a bool"))
        }
        Unop::Minus => match eval_expr(expr)? {
            Constant::Int(n) => Ok(Constant::Int(-n)),
            Constant::Float(n) => Ok(Constant::Float(-n)),
            _ => Err(RunErr::Type("Unary minus must be applied to an int or a float"))
        }
    }
}

pub fn eval_func(func: &FuncNode) -> Result<Constant, RunErr> {
    let mut results = vec![];
    for arg in func.args.iter() {
        match eval_expr(&arg) {
            Ok(result) => results.push(result),
            Err(err) => {
                return Err(err)
            }
        }
    }
    panic!("Function call not yet implemented")
}

pub fn eval_cond(expr: &CondNode) -> Result<(), RunErr> {
    if eval_bool_expr(&expr.cond)? {
        eval_node(&expr.this)
    } else {
        eval_node(&expr.that)
    }
}

pub fn eval_node(node: &Node) -> Result<(), RunErr> {
    match node {
        _ => {}
    }
    Ok(())
}