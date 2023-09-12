#![allow(unused)]

use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
}

#[derive(Debug, Deserialize)]
pub struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Var {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Binary(Binary),
    Bool(Bool),
    If(If),
    Let(Let),
    Var(Var),
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    Tuple(Box<Value>, Box<Value>),
    Void,
}

pub type Context = HashMap<String, Value>;

pub fn eval(term: Term, context: &mut Context) -> Value {
    match term {
        Term::Int(int) => Value::Int(int.value),
        Term::Str(str) => Value::Str(str.value),
        Term::Bool(bool) => Value::Bool(bool.value),
        Term::Var(v) => match context.get(&v.text) {
            Some(v) => v.clone(),
            None => panic!("Error"),
        },
        Term::Print(print) => {
            let value = eval(*print.value, context);
            match value {
                Value::Int(i) => println!("{i}"),
                Value::Str(s) => println!("{s}"),
                Value::Bool(b) => println!("{b}"),
                _ => panic!("Error"),
            };
            Value::Void
        }
        Term::If(elem) => {
            let condition = eval(*elem.condition, context);

            match condition {
                Value::Bool(true) => eval(*elem.then, context),
                Value::Bool(false) => eval(*elem.otherwise, context),
                _ => panic!("Error"),
            }
        }
        Term::Let(elem) => {
            let value = eval(*elem.value, context);
            context.insert(elem.name.text, value);
            eval(*elem.next, context)
        }
        Term::Binary(binary) => {
            let lhs = eval(*binary.lhs, context);
            let rhs = eval(*binary.rhs, context);

            match binary.op {
                BinaryOp::Add => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    (Value::Str(a), Value::Int(b)) => Value::Str(format!("{a}{b}")),
                    (Value::Int(a), Value::Str(b)) => Value::Str(format!("{a}{b}")),
                    (Value::Str(a), Value::Str(b)) => Value::Str(format!("{a}{b}")),
                    _ => panic!("Error"),
                },
                BinaryOp::Sub => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                    _ => panic!("Error"),
                },
                BinaryOp::Mul => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                    _ => panic!("Error"),
                },
                BinaryOp::Div => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                    _ => panic!("Error"),
                },
                BinaryOp::Rem => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                    _ => panic!("Error"),
                },
                BinaryOp::Eq => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
                    (Value::Str(a), Value::Str(b)) => Value::Bool(a == b),
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                    _ => panic!("Error"),
                },
                BinaryOp::Neq => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a != b),
                    (Value::Str(a), Value::Str(b)) => Value::Bool(a != b),
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                    _ => panic!("Error"),
                },
                BinaryOp::Lt => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                    _ => panic!("Error"),
                },
                BinaryOp::Gt => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                    _ => panic!("Error"),
                },
                BinaryOp::Lte => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
                    _ => panic!("Error"),
                },
                BinaryOp::Gte => match (lhs, rhs) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
                    _ => panic!("Error"),
                },
                BinaryOp::And => match (lhs, rhs) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                    _ => panic!("Error"),
                },
                BinaryOp::Or => match (lhs, rhs) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                    _ => panic!("Error"),
                },
            }
        }
    }
}

pub fn main() {
    let json = fs::read_to_string("./files/test.json").unwrap();
    let program = serde_json::from_str::<File>(&json).unwrap();
    let mut context = HashMap::new();
    eval(program.expression, &mut context);
}
