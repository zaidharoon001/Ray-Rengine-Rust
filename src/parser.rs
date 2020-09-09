use std::collections::HashMap;
use crate::token;
use crate::error;
use crate::tokenTypes::TokenTypes::*;
use crate::tokenTypes;
use crate::nodes;
use crate::position;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<token::Token>,
}

impl Parser {
    fn eof_token(&self) -> token::Token {
        token::Token{
            tok_type: Eof,
            tok_value: String::from(""),
            pos_start: position::Position{filename: String::from("Yoi"), ftext: String::from(""), index: 0u64, ln: 1u64, cn: 1u64},
            pos_end: position::Position{filename: String::from("Yoi"), ftext: String::from(""), index: 0u64, ln: 1u64, cn: 1u64}
        }
    }

    fn current_tok(&mut self, no_newline: bool) -> token::Token {
        let current_tok = match self.tokens.pop() {
            Some(a) => a,
            None => return self.eof_token()
        };
        if no_newline && current_tok.tok_type == Newline {
            return self.current_tok(no_newline)
        } else {
            return current_tok
        }
    }

    fn add_tok(&mut self, tok: token::Token) {
        self.tokens.push(tok)
    }

    pub fn parse(&mut self) -> Result<nodes::Node, error::Error> {
        let res = self.rules();
        match res {
            Ok(obj) => {
                let (pos_start, pos_end) = obj.get_pos();
                let current_tok = self.current_tok(true);
                if current_tok.tok_type != Eof {
                    return Err(
                        error::Error{
                            name: String::from("ParseError"),
                            message: String::from(format!("Inappropriate ending")),
                            pos_start,
                            pos_end
                        }
                    )
                } else {
                    return Ok(obj)
                }
            },
            Err(err) => Err(err)
        }
    }

    fn sequence<T, R>(
        &mut self,
        elem: fn(&mut Parser) -> Result<T, error::Error>,
        create_sequence: fn(Vec<T>, position::Position, position::Position) -> R,
        tok: Option<token::Token>,
        should_start: fn(&token::Token) -> bool,
        should_end: fn(&token::Token) -> bool,
        is_sep: fn(&token::Token) -> bool
    ) -> Result<R, error::Error> {
        let mut elems : Vec<T> = Vec::new();
        let tok = match tok {
            Some(t) => t,
            None => self.current_tok(true)
        };
        if !(should_start(&tok)) {
            return Err(
                error::Error{
                    name: String::from("ParseError"),
                    message: String::from("Inappropriate starting"),
                    pos_start: tok.pos_start.copy(),
                    pos_end: tok.pos_end.copy()
                }
            )
        }
        match elem(self) {
            Ok(elem) => elems.push(elem),
            Err(err) => return Err(err)
        };
        let mut sep = self.current_tok(true);
        while is_sep(&sep) {
            match elem(self) {
                Ok(elem) => elems.push(elem),
                Err(err) => return Err(err)
            };
            sep = self.current_tok(true);
        }
        if !(should_end(&sep)) {
            return Err(
                error::Error{
                    name: String::from("ParseError"),
                    message: String::from(format!("Inappropriate ending")),
                    pos_start: tok.pos_start.copy(),
                    pos_end: tok.pos_end.copy()
                }
            )
        }
        return Ok(create_sequence(elems, tok.pos_start, sep.pos_end))
    }

    fn rules(&mut self) -> Result<nodes::Node, error::Error> {
        let mut rules = Vec::new();
        let mut pos_end = self.eof_token().pos_end;
        let mut current_tok = self.current_tok(true);
        while current_tok.tok_type == Identifier {
            let rule = self.rule(Some(current_tok));
            rules.push(
                match rule {
                    Ok(a) => {
                        let (_, pos) = a.get_pos();
                        pos_end = pos;
                        a
                    },
                    Err(err) => return Err(err)
                }
            );
            current_tok = self.current_tok(true);
        };
        let node = nodes::Node::RulesNode{rules, pos_start: pos_end.copy(), pos_end: pos_end};
        return Ok(node);
    }

    fn rule(&mut self, token: Option<token::Token>) -> Result<nodes::Node, error::Error> {
        let tok = match token {
            Some(t) => t,
            None => self.current_tok(true)
        };
        let assign_tok = self.current_tok(true);
        if assign_tok.tok_type != Assign{
            return Err(
                error::Error {
                    name: String::from("ParseError"),
                    message: String::from(format!("Expected ':=', got {}", assign_tok.tok_type)),
                    pos_start: tok.pos_start.copy(),
                    pos_end: tok.pos_end.copy()
                }
            );
        };
        let lhs = match self.operation() {
            Ok(a) => a,
            Err(err) => return Err(err)
        };
        let (_, pos_end) = lhs.get_pos();
        return Ok(nodes::Node::RuleNode{rhs: tok.tok_value, lhs: Box::new(lhs), pos_start: assign_tok.pos_start, pos_end})
    }

    fn operation(&mut self) -> Result<nodes::Node, error::Error> {
        return self.bin_op(
            |this: &mut Parser| this.lhs(),
            vec![Pipe],
            |this: &mut Parser| this.lhs()
        )
    }

    fn lhs(&mut self) -> Result<nodes::Node, error::Error> {
        let mut ops = Vec::new();
        let op = match self.atom(None) {
            Ok(a) => a,
            Err(err) => return Err(err)
        };
        ops.push(op);
        while (!self.is_rhs()) && (!self.is_eof()) && (!self.is_tok(Pipe)) && (!self.is_tok(LParen)) {
            let op = match self.atom(None) {
                Ok(a) => a,
                Err(err) => return Err(err)
            };
            ops.push(op)
        };
        return Ok(nodes::Node::RuleChainNode{chain: ops, pos_start: self.eof_token().pos_end, pos_end: self.eof_token().pos_end})
    }

    fn is_tok(&mut self, tok_type: tokenTypes::TokenTypes) -> bool {
        let tok = self.current_tok(true);
        let res = tok.tok_type == tok_type;
        self.add_tok(tok);
        return res;
    }

    fn is_eof(&mut self) -> bool {
        let eof = self.current_tok(true);
        let res = eof.tok_type == Eof;
        if !res {
            self.add_tok(eof);
        }
        return res;
    }

    fn is_rhs(&mut self) -> bool {
        let id = self.current_tok(true);
        let assign = self.current_tok(true);
        let res = id.tok_type == Identifier && assign.tok_type == Assign;
        self.add_tok(assign);
        self.add_tok(id);
        return res;
    }

    fn bin_op(
        &mut self,
        func_a: fn(&mut Parser) -> Result<nodes::Node, error::Error>,
        ops: Vec<tokenTypes::TokenTypes>,
        func_b: fn(&mut Parser) -> Result<nodes::Node, error::Error>
    ) -> Result<nodes::Node, error::Error> {
        let mut left = match func_a(self) {
            Ok(a) => a,
            Err(err) => return Err(err)
        };
        let mut op_tok = self.current_tok(true);
        while ops.contains(&op_tok.tok_type){
            let right = match func_b(self) {
                Ok(a) => a,
                Err(err) => return Err(err)
            };
            let (pos_start, pos_end) =  right.get_pos();
            left = nodes::Node::BinOpNode{left: Box::new(left), op: op_tok.tok_type, right: Box::new(right), pos_start: pos_start, pos_end: pos_end};
            op_tok = self.current_tok(true);
        }
        self.add_tok(op_tok);
        return Ok(left);
    }

    fn atom(&mut self, token: Option<token::Token>) -> Result<nodes::Node, error::Error> {
        let prefix = match self.prefix(token) {
            Ok(a) => a,
            Err(err) => return Err(err)
        };
        let postfix = self.current_tok(false);
        let condition = ![Asterisk, QuestionMark, Plus].contains(&postfix.tok_type);
        if condition {
            self.add_tok(postfix);
            return Ok(prefix)
        } else {
            let pos_end = postfix.pos_end.copy();
            let (pos_start, _) = prefix.get_pos();
            return Ok(nodes::Node::PostFixNode{postfix: Box::new(prefix), op: postfix.tok_type, pos_start: pos_start, pos_end: pos_end})
        }
    }

    fn prefix(&mut self, token: Option<token::Token>) -> Result<nodes::Node, error::Error> {
        let tok = match token {
            Some(t) => t,
            None => self.current_tok(true)
        };
        let value = match tok {
            _ if tok.tok_type == Str => {
                let value = nodes::Node::StrNode{string: tok.tok_value, pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.tok_type == Identifier => {
                let value = nodes::Node::RuleAccessNode{identifier: tok.tok_value, pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.tok_type == RParen => {
                let expr = match self.operation() {
                    Ok(a) => a,
                    Err(err) => return Err(err)
                };
                let current_tok = self.current_tok(true);
                if current_tok.tok_type != LParen {
                    let (pos_start, _) = expr.get_pos();
                    return Err(
                        error::Error{
                            name: "ParseError".to_string(),
                            message: format!("Expected ')', found {}", current_tok.tok_type),
                            pos_start,
                            pos_end: current_tok.pos_end
                        }
                    )
                }
                return Ok(expr)
            },
            _ => {
                return Err(
                    error::Error {
                        name: String::from("ParseError"),
                        message: String::from("Expected number, true, object, or false"),
                        pos_start: tok.pos_start.copy(),
                        pos_end: tok.pos_end.copy()
                    }
                )
            }
        };
        return value
    }
}
