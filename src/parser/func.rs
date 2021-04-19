use crate::interp::Func;
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn continue_parse_fn(&mut self) -> Func {
        let mut params = Vec::new();

        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::LeftParen, ..})) {
            panic!("expected left parenthesis");
        }

        self.lexer.consume_whitespace(self.is_multiline);
        if matches!(self.lexer.peek(), Some(Token { kind: TokenKind::RightParen, .. })) {
            self.lexer.next();
        } else {
            loop {
                self.lexer.consume_whitespace(self.is_multiline);
                params.push(self.must_identifier_maybe_with_type());
                self.lexer.consume_whitespace(self.is_multiline);

                match self.lexer.next() {
                    Some(Token { kind: TokenKind::Comma, .. }) => (),
                    Some(Token { kind: TokenKind::RightParen, .. }) => break,
                    _ => panic!("expected comma or right parenthesis"),
                }
            }
        }

        self.lexer.consume_whitespace(self.is_multiline);
        let body = self.parse_block();

        Func::User {
            name: None,
            params,
            body: Box::new(body),
            captured_env: None,
        }
    }

    fn must_identifier_maybe_with_type(&mut self) -> FuncParam {
        let mut param = FuncParam {
            name: "".to_owned(),
            has_type_hint: false,
            type_hints: vec!["".to_owned()]
        };

        if let Some(Token { kind: TokenKind::Identifier(name), .. }) = self.lexer.next() {
            param.name = name;
        } else {
            panic!("expected identifier");
        }

        self.lexer.consume_whitespace(self.is_multiline);

        if let Some(Token { kind: TokenKind::Colon, .. }) = self.lexer.peek() {
            self.lexer.next();

            param.has_type_hint = true;

            self.lexer.consume_whitespace(self.is_multiline);

            let mut type_hints = vec![];

            loop {
                self.lexer.consume_whitespace(self.is_multiline);

                if let Some(Token { kind: TokenKind::Identifier(type_hint), .. }) = self.lexer.next() {
                    type_hints.push(type_hint);
                } else {
                    panic!("expected typehint");
                }

                self.lexer.consume_whitespace(self.is_multiline);

                match self.lexer.peek() {
                    Some(Token { kind: TokenKind::Pipe, .. }) => self.lexer.next(),
                    Some(Token { kind: TokenKind::RightParen, .. })
                    | Some(Token { kind: TokenKind::Comma, .. }) => break,
                    _ => panic!("unexpected token, expect pipe, comma or right paren")
                };
            }

            param.type_hints = type_hints.clone();
        }

        param
    }
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub has_type_hint: bool,
    pub type_hints: Vec<String>,
}