// Joseph Prichard
// 3/15/2024
// Implementation of an ast walker for the interpreter

use crate::syntaxtree::{Bop, BinaryExpr, Constant, Expr, Func, Unop, Value};

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
    stack_frames: Vec<StackFrame>
}

impl Environment {
    pub fn push(&mut self) {
        self.stack_frames.push(vec![])
    }

    pub fn pop(&mut self) {
        self.stack_frames.pop();
    }

    pub fn top(&mut self) -> &mut StackFrame {
        if self.stack_frames.len() == 0 {
            self.push()
        }
        let len = self.stack_frames.len();
        &mut self.stack_frames[len - 1]
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
        Expr::Value(value) => eval_value(value),
        Expr::BinaryOp(arith) => eval_binary_expr(arith),
        Expr::UnaryOp(op, expr) => eval_unary_expr(op, expr),
        Expr::Func(func) => eval_func(func)
    }
}

pub fn eval_value(value: &Value) -> ExprResult {
    match value {
        Value::Constant(constant) => Ok(constant.clone()),
        Value::Variable(v) => panic!("Variable access not yet implemented")
    }
}

pub fn eval_binary_expr(expr: &BinaryExpr) -> ExprResult {
    let lhs = eval_expr(expr.lhs.as_ref())?;
    let rhs = eval_expr(expr.rhs.as_ref())?;
    match expr.op {
        Bop::Add => eval_add_expr(lhs, rhs),
        Bop::Multiply => eval_multiply_expr(lhs, rhs),
        Bop::Subtract => eval_subtract_expr(lhs, rhs),
        Bop::Divide => eval_divide_expr(lhs, rhs),
        Bop::Eq => Ok(Constant::Bool(lhs == rhs)),
        Bop::Neq => Ok(Constant::Bool(lhs != rhs)),
        Bop::Leq => Ok(Constant::Bool(lhs <= rhs)),
        Bop::Geq => Ok(Constant::Bool(lhs >= rhs)),
        Bop::Lt => Ok(Constant::Bool(lhs < rhs)),
        Bop::Gt => Ok(Constant::Bool(lhs > rhs))
    }
}

pub fn eval_add_expr(lhs: Constant, rhs: Constant) -> ExprResult {
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

pub fn eval_multiply_expr(lhs: Constant, rhs: Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs), Constant::Int(rhs)) => Ok(Constant::Int(lhs * rhs)),
        (Constant::Float(lhs), Constant::Float(rhs)) => Ok(Constant::Float(lhs * rhs)),
        (Constant::String(lhs), Constant::Int(rhs)) => {
            let mut s_new = String::new();
            for _ in 0..rhs {
                s_new.push_str(&lhs)
            }
            Ok(Constant::String(s_new))
        }
        _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints, 2 floats, or between a string and an int"))
    }
}

pub fn eval_subtract_expr(lhs: Constant, rhs: Constant) -> ExprResult {
    match (lhs, rhs) {
        (Constant::Int(lhs), Constant::Int(rhs)) => Ok(Constant::Int(lhs - rhs)),
        (Constant::Float(lhs), Constant::Float(rhs)) => Ok(Constant::Float(lhs - rhs)),
        _ => Err(RunErr::Type("Subtract operator must be applied to 2 ints or 2 floats"))
    }
}

pub fn eval_divide_expr(lhs: Constant, rhs: Constant) -> ExprResult {
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

pub fn eval_func(func: &Func) -> Result<Constant, RunErr> {
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

