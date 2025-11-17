use std::iter::Peekable;

use crate::{ast::*, lexer::*};

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self { lexer: lexer.peekable() }
    }

    pub fn parse_fundef(&mut self) -> Option<Fundef> {
        self.expect(Token::Fn)?;

        match self.lexer.next()? {
            Token::Var(s) => {
                let args = self.parse_fargs()?;

                self.expect(Token::Arrow)?;
                let ret_type = self.parse_type()?;
                self.expect(Token::LBrace)?;
                let body = self.parse_expr()?;
                self.expect(Token::RBrace)?;

                Some(Fundef {
                    name: s,
                    args,
                    ret_type,
                    body,
                })
            }
            _ => None
        }
    }

    fn parse_fargs(&mut self) -> Option<Vec<(Type, String)>> {
        self.expect(Token::LParen)?;

        let mut args = Vec::new();

        if self.matches(Token::RParen).is_none() {
            args.push(self.parse_farg()?);

            while self.matches(Token::Comma).is_some() {
                args.push(self.parse_farg()?);
            }

            self.expect(Token::RParen)?;
        }

        Some(args)
    }

    fn parse_farg(&mut self) -> Option<(Type, String)> {
        let t = self.parse_type()?;
        let s = self.parse_id()?;
        Some((t, s))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let l = match self.lexer.next()? {
            Token::Var(s) => {
                Expr::Var(s)
            },
            _ => return None,
        };

        if let Some(op) = self.parse_bop() {
            let r = self.parse_expr()?;
            let binary = Binary {
                l: Box::new(l),
                r: Box::new(r),
                op,
            };
            Some(Expr::Binary(binary))
        } else {
            Some(l)
        }
    }

    fn parse_bop(&mut self) -> Option<Bop> {
        match self.lexer.peek()? {
            Token::Add => {
                self.lexer.next().unwrap();
                Some(Bop::Add)
            },
            Token::Sub => {
                self.lexer.next().unwrap();
                Some(Bop::Sub)
            },
            _ => None,
        }
    }

    fn parse_id(&mut self) -> Option<String> {
        match self.lexer.next()? {
            Token::Var(s) => Some(s),
            _ => None,
        }
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.lexer.next()? {
            Token::TypeI32 => Some(Type::I32),
            _ => None,
        }
    }

    fn expect(&mut self, expected: Token) -> Option<Token> {
        let token = self.lexer.next()?;
        if token == expected {
            Some(token)
        } else {
            None
        }
    }

    fn matches(&mut self, expected: Token) -> Option<Token> {
        self.lexer.next_if(|t| *t == expected)
    }
}
