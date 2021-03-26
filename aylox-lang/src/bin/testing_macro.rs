use ast_gen::ast_gen;
use aylox_lang::token::Token;
use derive_new::new;

ast_gen!(
    "Expr",
    [
        "Nil",
        "LiteralVal / String String, f64 Number, Nil Nil, bool Bool",
        "Binary     : Expr left, Token operator, Expr right",
        "Grouping   : Expr expression",
        "Literal    : LiteralVal value",
        "Unary      : Token operator, Expr right"
    ]
);
fn main() {}
