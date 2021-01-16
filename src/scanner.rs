
use crate::{error::AyloxError, token::{KEYWORDS, Token, TokenType, Tokens}};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Tokens,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> &Tokens {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            },
            '/' => {
                if self.match_next('/') {
                    // comment goes till end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.multi_line_comment()
                }
                else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _other => {
                if _other.is_ascii_digit() {
                    self.number()
                } else if _other.is_ascii_alphabetic() || _other == '_' {
                    self.identifier()
                } else {
                println!(
                    "{}",
                    AyloxError::Syntax {
                        line: self.line,
                        found: _other.into()
                    }
                );
                }
            }
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }    
        }

        let value = self.source.get(self.start..self.current).expect("could not find number");
        self.add_token_literal(TokenType::Number(value.parse::<f64>().expect("could not read number from string")))
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = self.source.get(self.start..self.current).expect("could not read rest of token");
        let token_type = KEYWORDS.get(text);
        if let Some(token_type) = token_type {
            self.add_token(token_type.to_owned())
        } else {
            self.add_token_literal(TokenType::Identifier(text.to_owned()))
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("{}", AyloxError::UnterminatedString);
            return;
        }

        // the closing "
        self.advance();

        // trim surrounding quotes
        let value = self.source.get(self.start + 1..self.current-1).expect("could not trim quotes");
        self.add_token_literal(TokenType::String(value.to_owned()))
    }

    fn multi_line_comment(&mut self) {
        while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return;
        }
        // the closing * and /
        self.advance();
        self.advance();
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current].into()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        return self.source.as_bytes()[self.current + 1].into()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.as_bytes()[self.current] as char != expected { return false }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1].into()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source.get(self.start..self.current).expect("could not read rest of token");
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn add_token_literal(&mut self, token_type: TokenType) {
        let text = self.source.get(self.start..self.current).expect("could not read rest of token");
        match token_type {
            TokenType::String(literal) => {
                self.tokens
                    .push(Token::new(TokenType::String(literal), text, self.line))
            }
            TokenType::Identifier(literal) => {
                self.tokens
                    .push(Token::new(TokenType::Identifier(literal), text, self.line))
            }
            TokenType::Number(literal) => {
                self.tokens
                    .push(Token::new(TokenType::Number(literal), text, self.line))
            }
            _ => {}
        }
    }
}