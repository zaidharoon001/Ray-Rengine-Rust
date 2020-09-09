use crate::position;
use crate::token;
use crate::error;
use crate::tokenTypes::TokenTypes::*;
use crate::tokenTypes;

#[derive(Debug)]
pub struct Lexer {
    pub current_index: usize,
    pub chars: Vec<u8>,
    pub position: position::Position
}

impl Lexer {
    fn get_byte(&self) -> Option<&u8> {
        return self.chars.get(self.current_index)
    }

    fn get_char(&self) -> char {
        let byte = if self.get_byte() == None { '\0' } else { self.chars[self.current_index] as char };
        return byte as char
    }

    fn advance(&mut self) {
        self.current_index += 1usize;
        self.position.advance(self.get_char());
    }

    fn is_num(&self) -> bool {
        let current_char = self.get_char();
        return current_char != '\0' && current_char.to_string().parse::<i64>().is_ok() || current_char == '.';
    }

    fn is_space(&self) -> bool {
        return self.get_char() == ' ' || self.get_char() == '\t' || self.get_char() == '\r';
    }

    fn is_char(&self, chararater: char) -> bool {
        return self.get_char() == chararater;
    }

    fn make_token(&mut self, tok: tokenTypes::TokenTypes) -> token::Token {
        let pos_start = self.position.copy();
        self.advance();
        return token::Token{tok_type: tok, tok_value: String::from(""), pos_start: pos_start, pos_end: self.position.copy()};
    }

    fn make_string(&mut self, predicate: fn(this: &mut Lexer) -> bool) -> token::Token {
        let pos_start = self.position.copy();
        let mut chars = String::from("");
        self.advance();
        let mut current_char = self.get_char();
        while !(self.is_char('\0') || predicate(self)) {
            chars.push_str(&format!("{}", &current_char));
            self.advance();
            current_char = self.get_char();
        }
        self.advance();
        return token::Token{tok_type: Str, tok_value: chars, pos_start: pos_start, pos_end: self.position.copy()};
    }

    fn is_ident(&self) -> bool {
        let alphabets = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', '_',
            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        ];
        return alphabets.contains(&self.get_char())
    }

    fn make_ident(&mut self) -> token::Token {
        let pos_start = self.position.copy();
        let mut chars = String::from(format!("{}", self.get_char()));
        self.advance();
        let mut current_char = self.get_char();
        while self.is_ident() || self.is_num() {
            chars.push_str(&format!("{}", &current_char));
            self.advance();
            current_char = self.get_char();
        }
        return token::Token{tok_type: Identifier, tok_value: chars, pos_start: pos_start, pos_end: self.position.copy()};
    }

    fn errored_tok(&mut self) -> Result<Vec<token::Token>, error::Error> {
        let pos_start = self.position.copy();
        self.advance();
        return Err(error::Error{name: String::from("IllegalCharError"), message: format!("Illegal Chararater '{}'", self.get_char()), pos_start: pos_start, pos_end: self.position.copy()})
    }

    fn two_char_tok(&mut self, tokens: &mut Vec<token::Token>, c: char) -> Result<(), error::Error> {
        self.advance();
        if !(self.is_char(c)){
            return
                match self.errored_tok() {
                    Ok(_) => Ok(()),
                    Err(a) => Err(a)
                }
        };
        tokens.push(self.make_token(Assign));
        return Ok(())
    }

    fn make_orstring(&mut self) -> Vec<token::Token> {
        let pos_start = self.position.copy();
        let mut toks = Vec::new();
        toks.push(token::Token{tok_type: RParen, tok_value: "".to_string(), pos_start: pos_start.copy(), pos_end: self.position.copy()});
        self.advance();
        let mut current_char = self.get_char();
        while !(self.is_char('\0') || self.is_char('"')) {
            toks.push(token::Token{tok_type: Str, tok_value: format!("{}", current_char), pos_start: pos_start.copy(), pos_end: self.position.copy()});
            toks.push(token::Token{tok_type: Pipe, tok_value: "".to_string(), pos_start: pos_start.copy(), pos_end: self.position.copy()});
            self.advance();
            current_char = self.get_char();
        }
        self.advance();
        toks.pop();
        toks.push(token::Token{tok_type: LParen, tok_value: "".to_string(), pos_start: pos_start.copy(), pos_end: self.position.copy()});
        return toks;
    }

    pub fn lex(&mut self) -> Result<Vec<token::Token>, error::Error> {
        let mut tokens: Vec<token::Token> = Vec::new();
        while self.get_char() != '\0' {
            let current_char = self.get_char();
            match current_char {
                _ if self.is_space() => self.advance(),
                _ if self.is_ident() => tokens.push(self.make_ident()),
                _ if self.is_char('\n') => tokens.push(self.make_token(Newline)),
                _ if self.is_char('\'') => tokens.push(self.make_string(|this: &mut Lexer| this.get_char() == '\'')),
                _ if self.is_char('"') => {
                    for tok in self.make_orstring() {
                        tokens.push(tok)
                    };
                },
                _ if self.is_char('(') => tokens.push(self.make_token(RParen)),
                _ if self.is_char(')') => tokens.push(self.make_token(LParen)),
                _ if self.is_char(',') => tokens.push(self.make_token(Comma)),
                _ if self.is_char('|') => tokens.push(self.make_token(Pipe)),
                _ if self.is_char('*') => tokens.push(self.make_token(Asterisk)),
                _ if self.is_char('+') => tokens.push(self.make_token(Plus)),
                _ if self.is_char('?') => tokens.push(self.make_token(QuestionMark)),
                _ if self.is_char(':') =>
                    match self.two_char_tok(&mut tokens, '=') {
                        Ok(_) => (),
                        Err(a) => return Err(a)
                    }
                _ => {
                    return self.errored_tok();
                }
            }
        }
        tokens.push(self.make_token(Eof));
        return Ok(tokens)
    }
}
