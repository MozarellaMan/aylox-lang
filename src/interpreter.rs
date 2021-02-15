use crate::{ast::LiteralVal::*, ast::*, error::RuntimeError, token::TokenType};

pub struct Interpreter;
impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Result<LiteralVal, RuntimeError> {
        self.visit_expr(expr)
    }
}
impl Visitor<Result<LiteralVal, RuntimeError>> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary) -> Result<LiteralVal, RuntimeError> {
        let left = self.visit_expr(&binary.left)?;
        let right = self.visit_expr(&binary.right)?;

        match binary.operator._type {
            TokenType::Minus => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Number(x - y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Slash => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Number(x / y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Star => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Number(x * y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Plus => match (left, right) {
                (Number(x), Number(y)) => Ok(Number(x + y)),
                (String(x), String(y)) => Ok(String(format!("{}{}", x, y))),
                _ => Err(RuntimeError::InvalidOperand {
                    lexeme: binary.operator.lexeme.clone(),
                    expected: "Numbers OR Strings".to_string(),
                }),
            },
            TokenType::Greater => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Bool(x > y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::GreaterEqual => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Bool(x >= y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::Less => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Bool(x < y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::LessEqual => {
                if let (Number(x), Number(y)) = (left, right) {
                    Ok(Bool(x <= y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                    })
                }
            }
            TokenType::BangEqual => Ok(Bool(!is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Bool(is_equal(&left, &right))),
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
                if let Number(num) = right {
                    Ok(Number(-num))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: unary.operator.lexeme.clone(),
                        expected: "number".to_string(),
                    })
                }
            }
            TokenType::Bang => Ok(Bool(!is_truthy(right))),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: unary.operator.lexeme.clone(),
                expression: Expr::Unary(unary.clone()),
            }),
        }
    }
}

fn is_equal(left: &LiteralVal, right: &LiteralVal) -> bool {
    left == right
}

fn is_truthy(literal: LiteralVal) -> bool {
    match literal {
        LiteralVal::Nil(_) => false,
        Bool(boolean) => boolean,
        _ => true,
    }
}
