use crate::ast::*;
pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, expression: &Expr) -> String {
        self.visit_expr(expression)
    }
}
impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Binary) -> String {
        parenthesize(
            self,
            &binary.operator.lexeme,
            &[&binary.left, &binary.right],
        )
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        parenthesize(self, "group", &[&grouping.expression])
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        match &literal.value {
            Value::String(val) => val.clone(),
            Value::Number(val) => val.to_string(),
            Value::Nil(_) => "Nil".to_owned(),
            Value::Bool(val) => val.to_string(),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        parenthesize(self, &unary.operator.lexeme, &[&unary.right])
    }

    fn visit_variable(&mut self, variable: &Variable) -> String {
        format!("var {}", variable.name.lexeme)
    }

    fn visit_assign(&mut self, _assign: &Assign) -> String {
        todo!()
    }

    fn visit_logical(&mut self, _logical: &Logical) -> String {
        todo!()
    }

    fn visit_call(&mut self, _call: &Call) -> String {
        todo!()
    }
}

fn parenthesize(visitor: &mut AstPrinter, operator: &str, expressions: Expressions) -> String {
    let mut builder = String::new();
    builder.push('(');
    builder.push_str(operator);
    for expr in expressions.iter() {
        builder.push(' ');
        let next = visitor.visit_expr(expr);
        builder.push_str(&next);
    }
    builder.push(')');
    builder
}
