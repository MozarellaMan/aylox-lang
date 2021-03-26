# Aylox Programming Language 

Current version: 0.0.1


## Features

### Currently implemented

- evaluation of arithmetic expressions (`2+2`, `3 / ( 3 * 50)`)
- evaluation of boolean expressions (`2 > 5` => `false`)
- string concatenation (`"Hello " + "World!"` => `Hello World!`)
- ast code generator (in `src/bin/generate_ast.rs`)

### Planned

- everything else in the book :)

## About

This repo holds my progress in follwing the [Crafting Interpreters](https://craftinginterpreters.com/) book by Bob Nystrom. The book's first section is in Java, and second in C. I plan to do both in Rust to learn the language and because it's pretty fun!

## How to run

`cargo run`

`cargo test`

License: MIT
