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
        let val = var.initializer.as_ref();
        if let Some(val) = val {
            let val = self.interpret_expr(val)?;
            self.environment.borrow_mut().define(
                var.name.lexeme.clone(),
                Some(Expr::Literal(Literal { value: val })),
            );
        } else {
            self.environment
                .borrow_mut()
                .define(var.name.lexeme.clone(), None);
        }
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), RuntimeError> {
        let new_env = Environment::with_enclosing(self.environment.clone());
        self.interpret_block(&block.statements, new_env)
    }

    fn visit_if_(&mut self, if_: &If_) -> Result<(), RuntimeError> {
        if is_truthy(&self.visit_expr(&if_.condition)?) {
            self.visit_stmt(&if_.then_branch)?;
        }
        if let Some(else_branch) = &if_.else_branch {
            self.visit_stmt(else_branch)?;
        }
        Err(RuntimeError::ControlFlowError)
    }

    fn visit_while_(&mut self, while_: &While_) -> Result<(), RuntimeError> {
        while is_truthy(&self.visit_expr(&while_.condition)?) {
            self.visit_stmt(&while_.body)?;
        }
        Ok(())
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
                        line: binary.operator.line,
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
                        line: binary.operator.line,
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
                        line: binary.operator.line,
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
                    line: binary.operator.line,
                }),
            },
            TokenType::Greater => {
                if let (LiteralVal::Number(x), LiteralVal::Number(y)) = (left, right) {
                    Ok(LiteralVal::Bool(x > y))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
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
                        line: binary.operator.line,
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
                        line: binary.operator.line,
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
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::BangEqual => Ok(LiteralVal::Bool(left != right)),
            TokenType::EqualEqual => Ok(LiteralVal::Bool(left == right)),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: binary.operator.lexeme.clone(),
                expression: Expr::Binary(binary.clone()),
                line: binary.operator.line,
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
                        line: unary.operator.line,
                    })
                }
            }
            TokenType::Bang => Ok(LiteralVal::Bool(!is_truthy(&right))),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: unary.operator.lexeme.clone(),
                expression: Expr::Unary(unary.clone()),
                line: unary.operator.line,
            }),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> Result<LiteralVal, RuntimeError> {
        let val = self.environment.borrow().get(&variable.name)?;
        let val = val.as_ref().as_ref().unwrap();
        self.interpret_expr(&val)
    }

    fn visit_assign(&mut self, assign: &Assign) -> Result<LiteralVal, RuntimeError> {
        let val = Expr::Literal(Literal {
            value: self.visit_expr(&assign.value)?,
        });
        self.environment
            .borrow_mut()
            .assign(&assign.name, Some(val.clone()))?;
        self.visit_expr(&val)
    }

    fn visit_logical(&mut self, logical: &Logical) -> Result<LiteralVal, RuntimeError> {
        let left = self.visit_expr(&logical.left)?;

        if logical.operator._type == TokenType::Or {
            if is_truthy(&left) {
                return Ok(left);
            }
        } else if !is_truthy(&left) {
            return Ok(left);
        }

        self.visit_expr(&logical.right)
    }
}

fn is_truthy(literal: &LiteralVal) -> bool {
    match literal {
        LiteralVal::Nil(_) => false,
        LiteralVal::Bool(boolean) => *boolean,
        _ => true,
    }
}
