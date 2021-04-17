use std::{fmt::Display, rc::Rc};

use crate::{error::RuntimeException, functions::Callable, token::Token};
use ast_gen::ast_gen;

ast_gen!(
    "~Expr",
    [
        "Nil",
        "Value      / String String, f64 Number, Nil Nil, bool Bool",
        "Assign     : Token name, Expr value",
        "Binary     : Expr left, Token operator, Expr right",
        "Call       : Expr callee, Token paren, Expr* arguments",
        "Grouping   : Expr expression",
        "Literal    : Value value",
        "Logical    : Expr left, Token operator, Expr right",
        "Unary      : Token operator, Expr right",
        "Variable   : Token name"
    ]
);

ast_gen!(
    "~Stmt",
    [
        "Expression : Expr expression",
        "Function   : Token name, Token* params, Stmt* body",
        "If_        : Expr condition, Stmt then_branch, Stmt? else_branch",
        "Print      : Expr expression",
        "While_     : Expr condition, Stmt body",
        "Return_    : Token keyword, Expr? value",
        "Var        : Token name, Expr? initializer",
        "Block      : Stmt* statements",
    ]
);

pub type AloxObjResult = Result<AloxObject, RuntimeException>;
pub type ValueResult = Result<Value, RuntimeException>;

#[derive(Clone, Debug)]
pub enum AloxObject {
    Value(Value),
    Function(Rc<dyn Callable>),
    Expr(Expr),
}

impl AloxObject {
    pub fn to_value(self) -> ValueResult {
        if let AloxObject::Value(val) = self {
            Ok(val)
        } else {
            Err(RuntimeException::ValueMissing {
                line: None,
                lexeme: None,
            })
        }
    }

    pub fn to_function(self, callee: &Call) -> Result<Rc<dyn Callable>, RuntimeException> {
        if let AloxObject::Function(func) = self {
            Ok(func)
        } else {
            Err(RuntimeException::ExpectedFunction {
                line: callee.paren.line,
                lexeme: callee.paren.lexeme.clone(),
            })
        }
    }

    pub fn to_value_with_info(self, line: usize, lexeme: &str) -> ValueResult {
        if let AloxObject::Value(val) = self {
            Ok(val)
        } else {
            dbg!(&self);
            Err(RuntimeException::ValueMissing {
                line: Some(line),
                lexeme: Some(lexeme.to_string()),
            })
        }
    }
}
pub enum FunctionKind {
    Function,
    Method,
}

impl Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionKind::Function => write!(f, "function"),
            FunctionKind::Method => write!(f, "method"),
        }
    }
}
