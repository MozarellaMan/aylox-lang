use ast_printer::AstPrinter;
use aylox_lang::{
    ast::*,
    ast_printer,
    prompt::run_prompt,
    run_file,
    token::{Token, TokenType},
};
use clap::{App, Arg, SubCommand};
fn main() {
    let expr = Expr::Binary(Binary::new(
        Box::new(Expr::Unary(Unary::new(
            Token::new(TokenType::Minus, "-", 1),
            Box::new(Expr::Literal(Literal::new(LiteralVal::Number(123f64)))),
        ))),
        Token::new(TokenType::Star, "*", 1),
        Box::new(Expr::Grouping(Grouping::new(Box::new(Expr::Literal(
            Literal::new(LiteralVal::Number(45.67)),
        ))))),
    ));
    let mut printer = AstPrinter;
    println!("{}", printer.print(&expr));
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let matches = App::new("Aylox")
        .version(VERSION)
        .author("Ayomide B. <ayoeze@hotmail.com>")
        .about("A Lox programming language")
        .arg(
            Arg::with_name("script")
                .short("s")
                .long("script")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets an input script file to run"),
        )
        .subcommand(SubCommand::with_name("repl").about("a REPL"))
        .get_matches();

    if let Some("repl") = matches.subcommand_name() {
        run_prompt()
    }
    if let Some(filepath) = matches.value_of("script") {
        //println!("Script-\n{}", file)
        if let Err(err) = run_file(filepath) {
            println!("Can't open file: {}", err)
        }
    } else {
        run_prompt()
    }
}
