#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Types
    TypeI32,
    // Symbols
    Fn,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Arrow,
    // Arithmetic operators
    Add,
    Sub,
    // Primary
    Var(String),
    I32(i32),
    // Error
    Unexpected(char),
}

pub struct Lexer<'source> {
    /// The input program as a string.
    src: &'source str,
    /// Index of the current character in the source string.
    current: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { src: source, current: 0 }
    }

    /// Return the next character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.src.chars().nth(self.current)
    }

    /// Return the next character and consume it.
    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.src.chars().nth(self.current) {
            self.current += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Check whether the next character is equal to the expected character.
    /// Consumes the next character if it matches.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek_char().is_some_and(|c| c == expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn match_str(&mut self, expected: &str) -> bool {
        if self.src[self.current..].starts_with(expected) {
            self.current += expected.len();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.current += 1;
                },
                _ => break,
            }
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        self.skip_whitespace();

        // Keywords
        let token = if self.match_str("i32") {
            TypeI32
        } else if self.match_str("fn") {
            Fn
        } else {
            match self.next_char()? {
                // Symbols
                '(' => LParen,
                ')' => RParen,
                '{' => LBrace,
                '}' => RBrace,
                ',' => Comma,
                '-' if self.match_char('>') => Arrow,
                // Arithmetic operators
                '+' => Add,
                '-' => Sub,
                // Primary
                c if c.is_ascii_digit() => {
                    // Include this initial character as well.
                    let start = self.current - 1;

                    while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                        self.current += 1;
                    }

                    let end = self.current;
                    I32(self.src[start..end].parse().unwrap())
                }
                c if c.is_ascii_alphabetic() => {
                    // Include this initial character as well.
                    let start = self.current - 1;

                    while self.peek_char().is_some_and(|c| c.is_ascii_alphanumeric()) {
                        self.current += 1;
                    }

                    let end = self.current;
                    Var(self.src[start..end].to_string())
                }
                // Error
                c => Unexpected(c),
            }
        };

        Some(token)
    }
}
