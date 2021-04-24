use crate::{error::ResolverError, token::Token};
use std::{collections::HashMap, error::Error};

use crate::{ast::*, interpreter::Interpreter};

type ResolverResult = Result<(), ResolverError>;
pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_local(&mut self, expr: Expr, name: &Token) -> ResolverResult {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve_expr(&expr, self.scopes.len() - 1 - i);
            }
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> ResolverResult {
        self.visit_stmt(statement)
    }

    fn resolve_stmts(&mut self, statements: &[Stmt]) -> ResolverResult {
        for stmt in statements.iter() {
            self.resolve_stmt(stmt)?
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> ResolverResult {
        self.visit_expr(expr)
    }

    fn resolve_function(&mut self, function: &Function) -> ResolverResult {
        self.begin_scope();
        function.params.iter().for_each(|param| {
            self.declare(param);
            self.define(param);
        });
        self.resolve_stmts(&function.body)?;
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }
    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}

impl StmtVisitor<ResolverResult> for Resolver {
    fn visit_expression(&mut self, expression: &Expression) -> ResolverResult {
        self.resolve_stmt(&Stmt::Expression(expression.clone()))
    }

    fn visit_function(&mut self, function: &Function) -> ResolverResult {
        self.declare(&function.name);
        self.define(&function.name);

        self.resolve_function(&function)
    }

    fn visit_if_(&mut self, if_: &If_) -> ResolverResult {
        self.resolve_expr(&if_.condition)?;
        self.resolve_stmt(&if_.then_branch)?;
        if let Some(else_branch) = &if_.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print(&mut self, print: &Print) -> ResolverResult {
        self.resolve_expr(&print.expression)
    }

    fn visit_while_(&mut self, while_: &While_) -> ResolverResult {
        self.resolve_expr(&while_.condition)?;
        self.resolve_stmt(&while_.body)
    }

    fn visit_return_(&mut self, return_: &Return_) -> ResolverResult {
        if let Some(return_value) = &return_.value {
            self.resolve_expr(return_value)?;
        }
        Ok(())
    }

    fn visit_var(&mut self, var: &Var) -> ResolverResult {
        self.declare(&var.name);
        if let Some(init) = &var.initializer {
            self.resolve_expr(init)?;
        }
        self.define(&var.name);
        Ok(())
    }

    fn visit_block(&mut self, block: &Block) -> ResolverResult {
        self.begin_scope();
        self.resolve_stmts(&block.statements)?;
        self.end_scope();
        Ok(())
    }
}

impl ExprVisitor<ResolverResult> for Resolver {
    fn visit_assign(&mut self, assign: &Assign) -> ResolverResult {
        self.resolve_expr(&assign.value)?;
        self.resolve_local(Expr::Assign(assign.clone()), &assign.name)
    }

    fn visit_binary(&mut self, binary: &Binary) -> ResolverResult {
        self.resolve_expr(&binary.left)?;
        self.resolve_expr(&binary.right)
    }

    fn visit_call(&mut self, call: &Call) -> ResolverResult {
        self.resolve_expr(&call.callee)?;
        for arg in call.arguments.iter() {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> ResolverResult {
        self.resolve_expr(&grouping.expression)
    }

    fn visit_literal(&mut self, _literal: &Literal) -> ResolverResult {
        Ok(())
    }

    fn visit_logical(&mut self, logical: &Logical) -> ResolverResult {
        self.resolve_expr(&logical.left)?;
        self.resolve_expr(&logical.right)
    }

    fn visit_unary(&mut self, unary: &Unary) -> ResolverResult {
        self.resolve_expr(&unary.right)
    }

    fn visit_variable(&mut self, variable: &Variable) -> ResolverResult {
        if let Some(scope) = self.scopes.last() {
            if let Some(initialized) = scope.get(&variable.name.lexeme) {
                if !initialized {
                    return Err(ResolverError::ReadInOwnInitializer {
                        lexeme: variable.name.lexeme.clone(),
                        line: variable.name.line,
                    });
                }
            }
        }
        self.resolve_local(Expr::Variable(variable.clone()), &variable.name)
    }
}
