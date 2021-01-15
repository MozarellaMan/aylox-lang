use aylox_lang::{prompt::run_prompt, run_file};
use clap::{App, Arg, SubCommand};
fn main() {
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

    if let Some("repl") = matches.subcommand_name() { run_prompt() }
    if let Some(filepath) = matches.value_of("script") {
        //println!("Script-\n{}", file)
        if let Err(err) = run_file(filepath) {
            println!("{}", err)
        }
    } else {
        run_prompt()
    }
}
