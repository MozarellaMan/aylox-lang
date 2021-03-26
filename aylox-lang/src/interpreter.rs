use crate::{ast::*, error::RuntimeError, token::TokenType};

pub struct Interpreter;
impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Result<LiteralVal, RuntimeError> {
        self.visit_expr(expr)
    }
}
impl ExprVisitor<Result<LiteralVal, RuntimeError>> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary) -> Result<LiteralVal, RuntimeError> {
        let left = self.visit_expr(&binary.left)?;
        let right = self.visit_expr(&binary.right)?;

        match binary.operator._type {
            TokenType::Minus => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Number(x - y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Slash => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Number(x / y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Star => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Number(x * y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Plus => match (left, right) {
                (LiteralVal::Number(x), LiteralVal::Number(y)) => Ok(LiteralVal::Number(x + y)),
                (LiteralVal::String(x), LiteralVal::String(y)) => {
                    Ok(LiteralVal::String(format!("{}{}", x, y)))
                }
                _ => Err(RuntimeError::InvalidOperand {
                    lexeme: binary.operator.lexeme.clone(),
                    expected: "Numbers OR Strings".to_string(),
                }),
            },
            TokenType::Greater => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Bool(x > y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::GreaterEqual => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Bool(x >= y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Less => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Bool(x < y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::LessEqual => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Bool(x <= y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::BangEqual => Ok(LiteralVal::Bool(left != right)),
            TokenType::EqualEqual => Ok(LiteralVal::Bool(left == right)),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: binary.operator.lexeme.clone(),
                expression: Expr::Binary(binary.clone()),
            }),
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<LiteralVal, RuntimeError> {
        self.visit_expr(&grouping.expression)
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<LiteralVal, RuntimeError> {
        Ok(literal.value.clone())
    }

    fn visit_unary(&mut self, unary: &Unary) -> Result<LiteralVal, RuntimeError> {
        let right = self.visit_expr(&unary.right)?;
        match unary.operator._type {
            TokenType::Minus => {
                if let LiteralVal::Number(num) = right {
                    Ok(LiteralVal::Number(-num))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: unary.operator.lexeme.clone(),
                        expected: "number".to_string(),
                    })
                }
            }
            TokenType::Bang => Ok(LiteralVal::Bool(!is_truthy(right))),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: unary.operator.lexeme.clone(),
                expression: Expr::Unary(unary.clone()),
            }),
        }
    }
}

fn is_truthy(literal: LiteralVal) -> bool {
    match literal {
        LiteralVal::Nil(_) => false,
        LiteralVal::Bool(boolean) => boolean,
        _ => true,
    }
}
