use std::{cell::RefCell, mem, rc::Rc};

use crate::{
    ast::*, ast_printer::AstPrinter, environment::Environment, error::RuntimeError,
    functions::AloxFunction, native_functions::Clock, token::TokenType,
};

pub struct Interpreter {
    printer: AstPrinter,
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}
impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        globals.define("clock", Some(AloxObject::Function(Rc::new(Clock))));
        Self {
            printer: AstPrinter,
            environment: Rc::new(RefCell::new(Environment::new())),
            globals: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn interpret_expr(&mut self, expr: &Expr) -> AloxObjResult {
        self.visit_expr(expr)
    }

    pub fn interpret_block(
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
        let value = self.interpret_expr(&print.expression)?.to_value()?;
        println!("{}", self.printer.print(&Expr::Literal(Literal { value })));
        Ok(())
    }

    fn visit_var(&mut self, var: &Var) -> Result<(), RuntimeError> {
        let val = var.initializer.as_ref();
        if let Some(val) = val {
            let val = self
                .interpret_expr(val)?
                .to_value_with_info(var.name.line, &var.name.lexeme)?;
            self.environment
                .borrow_mut()
                .define(&var.name.lexeme, Some(AloxObject::Value(val)));
        } else {
            self.environment.borrow_mut().define(&var.name.lexeme, None);
        }
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> Result<(), RuntimeError> {
        let new_env = Environment::with_enclosing(self.environment.clone());
        self.interpret_block(&block.statements, new_env)
    }

    fn visit_if_(&mut self, if_: &If_) -> Result<(), RuntimeError> {
        if is_truthy(&self.visit_expr(&if_.condition)?.to_value()?) {
            self.visit_stmt(&if_.then_branch)?;
        }
        if let Some(else_branch) = &if_.else_branch {
            self.visit_stmt(else_branch)?;
        }
        Err(RuntimeError::ControlFlowError)
    }

    fn visit_while_(&mut self, while_: &While_) -> Result<(), RuntimeError> {
        while is_truthy(&self.visit_expr(&while_.condition)?.to_value()?) {
            self.visit_stmt(&while_.body)?;
        }
        Ok(())
    }

    fn visit_function(&mut self, function: &Function) -> Result<(), RuntimeError> {
        let alox_function = AloxFunction::new(function.clone());
        self.environment.borrow_mut().define(
            &function.name.lexeme,
            Some(AloxObject::Function(Rc::new(alox_function))),
        );
        Ok(())
    }
}

impl ExprVisitor<AloxObjResult> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary) -> AloxObjResult {
        let left = self
            .visit_expr(&binary.left)?
            .to_value_with_info(binary.operator.line, &binary.operator.lexeme)?;
        let right = self
            .visit_expr(&binary.right)?
            .to_value_with_info(binary.operator.line, &binary.operator.lexeme)?;

        match binary.operator._type {
            TokenType::Minus => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Number(x - y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::Slash => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Number(x / y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::Star => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Number(x * y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::Plus => match (left, right) {
                (Value::Number(x), Value::Number(y)) => Ok(AloxObject::Value(Value::Number(x + y))),
                (Value::String(x), Value::String(y)) => {
                    Ok(AloxObject::Value(Value::String(format!("{}{}", x, y))))
                }
                _ => Err(RuntimeError::InvalidOperand {
                    lexeme: binary.operator.lexeme.clone(),
                    expected: "Numbers OR Strings".to_string(),
                    line: binary.operator.line,
                }),
            },
            TokenType::Greater => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Bool(x > y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::GreaterEqual => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Bool(x >= y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::Less => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Bool(x < y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::LessEqual => {
                if let (Value::Number(x), Value::Number(y)) = (left, right) {
                    Ok(AloxObject::Value(Value::Bool(x <= y)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: binary.operator.lexeme.clone(),
                        expected: "Number".to_string(),
                        line: binary.operator.line,
                    })
                }
            }
            TokenType::BangEqual => Ok(AloxObject::Value(Value::Bool(left != right))),
            TokenType::EqualEqual => Ok(AloxObject::Value(Value::Bool(left == right))),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: binary.operator.lexeme.clone(),
                expression: Expr::Binary(binary.clone()),
                line: binary.operator.line,
            }),
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> AloxObjResult {
        self.visit_expr(&grouping.expression)
    }

    fn visit_literal(&mut self, literal: &Literal) -> AloxObjResult {
        Ok(AloxObject::Value(literal.value.clone()))
    }

    fn visit_unary(&mut self, unary: &Unary) -> AloxObjResult {
        let right = self
            .visit_expr(&unary.right)?
            .to_value_with_info(unary.operator.line, &unary.operator.lexeme)?;
        match unary.operator._type {
            TokenType::Minus => {
                if let Value::Number(num) = right {
                    Ok(AloxObject::Value(Value::Number(-num)))
                } else {
                    Err(RuntimeError::InvalidOperand {
                        lexeme: unary.operator.lexeme.clone(),
                        expected: "number".to_string(),
                        line: unary.operator.line,
                    })
                }
            }
            TokenType::Bang => Ok(AloxObject::Value(Value::Bool(!is_truthy(&right)))),
            _ => Err(RuntimeError::InvalidOperator {
                lexeme: unary.operator.lexeme.clone(),
                expression: Expr::Unary(unary.clone()),
                line: unary.operator.line,
            }),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> AloxObjResult {
        let val = self.environment.borrow().get(&variable.name)?;
        let val = val.as_ref().as_ref().unwrap();
        Ok(val.clone())
    }

    fn visit_assign(&mut self, assign: &Assign) -> AloxObjResult {
        let val = self
            .visit_expr(&assign.value)?
            .to_value_with_info(assign.name.line, &assign.name.lexeme)?;
        self.environment
            .borrow_mut()
            .assign(&assign.name, Some(AloxObject::Value(val.clone())))?;
        Ok(AloxObject::Value(val))
    }

    fn visit_logical(&mut self, logical: &Logical) -> AloxObjResult {
        let left = self
            .visit_expr(&logical.left)?
            .to_value_with_info(logical.operator.line, &logical.operator.lexeme)?;

        if logical.operator._type == TokenType::Or {
            if is_truthy(&left) {
                return Ok(AloxObject::Value(left));
            }
        } else if !is_truthy(&left) {
            return Ok(AloxObject::Value(left));
        }

        self.visit_expr(&logical.right)
    }

    fn visit_call(&mut self, call: &Call) -> AloxObjResult {
        let function = self.visit_expr(&call.callee)?.to_function(call)?;
        let mut arguments = vec![];
        for arg in call.arguments.iter() {
            arguments.push(self.visit_expr(arg)?);
        }

        function.call_mut(self, &arguments)
    }
}

fn is_truthy(literal: &Value) -> bool {
    match literal {
        Value::Nil(_) => false,
        Value::Bool(boolean) => *boolean,
        _ => true,
    }
}
