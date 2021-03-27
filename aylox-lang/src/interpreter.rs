use std::{cell::RefCell, mem, rc::Rc};

use crate::{
    ast::*, ast_printer::AstPrinter, environment::Environment, error::RuntimeError,
    token::TokenType,
};

pub struct Interpreter {
    printer: AstPrinter,
    environment: Rc<RefCell<Environment>>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            printer: AstPrinter,
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn interpret_expr(&mut self, expr: &Expr) -> Result<LiteralVal, RuntimeError> {
        self.visit_expr(expr)
    }

    fn interpret_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let previous = mem::replace(&mut self.environment, Rc::new(RefCell::new(environment)));
        let result = self.interpret(statements);
        self.environment = previous;
        result
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<(), RuntimeError> {
        self.interpret_expr(&expression.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, print: &Print) -> Result<(), RuntimeError> {
        let value = self.interpret_expr(&print.expression)?;
        println!("{}", self.printer.print(&Expr::Literal(Literal { value })));
        Ok(())
    }

    fn visit_var(&mut self, var: &Var) -> Result<(), RuntimeError> {
        let value =
            self.interpret_expr(&var.initializer.as_ref().unwrap_or(&Expr::Literal(Literal {
                value: LiteralVal::Nil(Nil),
            })))?;

        self.environment.borrow_mut().define(
            var.name.lexeme.clone(),
            Rc::new(Expr::Literal(Literal { value })),
        );
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), RuntimeError> {
        let new_env = Environment::with_enclosing(self.environment.clone());
        self.interpret_block(&block.statements, new_env)
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

    fn visit_variable(&mut self, variable: &Variable) -> Result<LiteralVal, RuntimeError> {
        let val = self.environment.borrow().get(&variable.name)?;
        self.interpret_expr(&val)
    }

    fn visit_assign(&mut self, assign: &Assign) -> Result<LiteralVal, RuntimeError> {
        let val = Expr::Literal(Literal {
            value: self.visit_expr(&assign.value)?,
        });
        self.environment
            .borrow_mut()
            .assign(&assign.name, Rc::new(val.clone()))?;
        self.visit_expr(&val)
    }
}

fn is_truthy(literal: LiteralVal) -> bool {
    match literal {
        LiteralVal::Nil(_) => false,
        LiteralVal::Bool(boolean) => boolean,
        _ => true,
    }
}
