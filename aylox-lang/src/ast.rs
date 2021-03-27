use crate::token::Token;
use ast_gen::ast_gen;

ast_gen!(
    "~Expr",
    [
        "Nil",
        "LiteralVal / String String, f64 Number, Nil Nil, bool Bool",
        "Assign   : Token name, Expr value",
        "Binary     : Expr left, Token operator, Expr right",
        "Grouping   : Expr expression",
        "Literal    : LiteralVal value",
        "Unary      : Token operator, Expr right",
        "Variable : Token name"
    ]
);

ast_gen!(
    "~Stmt",
    [
        "Expression : Expr expression",
        "Print      : Expr expression",
        "Var        : Token name, Expr? initializer",
        "Block      : Stmt* statements",
    ]
);
