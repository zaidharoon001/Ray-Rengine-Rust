use std::collections::HashMap;
use crate::tokenTypes;
use crate::token;
use crate::position;

#[derive(Debug)]
pub enum Node {
    StrNode{string: String, pos_start: position::Position, pos_end: position::Position},
    BinOpNode{left: Box<Node>, op: tokenTypes::TokenTypes, right: Box<Node>, pos_start: position::Position, pos_end: position::Position},
    RulesNode{rules: Vec<Node>, pos_start: position::Position, pos_end: position::Position},
    RuleNode{rhs: String, lhs: Box<Node>, pos_start: position::Position, pos_end: position::Position},
    PostFixNode{postfix: Box<Node>, op: tokenTypes::TokenTypes, pos_start: position::Position, pos_end: position::Position},
    RuleAccessNode{identifier: String, pos_start: position::Position, pos_end: position::Position},
    RuleChainNode{chain: Vec<Node>, pos_start: position::Position, pos_end: position::Position}
}

impl Node {
    pub fn get_pos(&self) -> (position::Position, position::Position) {
        match self {
            Node::StrNode{string: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::RulesNode{rules: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::RuleNode{rhs: _, lhs: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::BinOpNode{left: _, op: _, right: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::PostFixNode{postfix: _, op: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::RuleAccessNode{identifier: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::RuleChainNode{chain: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy())
        }
    }

    pub fn copy(&self) -> Node {
        match self {
            Node::StrNode{string, pos_start, pos_end} => Node::StrNode{string: string.clone(), pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::RulesNode{rules, pos_start, pos_end} => Node::RulesNode{rules: rules.into_iter().map(|x| x.copy()).collect(), pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::RuleNode{rhs, lhs, pos_start, pos_end} => Node::RuleNode{rhs: rhs.clone(), lhs: Box::new(lhs.copy()), pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::BinOpNode{left, op, right, pos_start, pos_end} =>
                Node::BinOpNode{left: Box::new(left.copy()), op: *op, right: Box::new(right.copy()), pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::PostFixNode{postfix, op, pos_start, pos_end} => Node::PostFixNode{postfix: Box::new(postfix.copy()), op: *op, pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::RuleAccessNode{identifier, pos_start, pos_end} => Node::RuleAccessNode{identifier: identifier.clone(), pos_start: pos_start.copy(), pos_end: pos_end.copy()},
            Node::RuleChainNode{chain, pos_start, pos_end} => Node::RuleChainNode{chain: chain.into_iter().map(|x| x.copy()).collect(), pos_start: pos_start.copy(), pos_end: pos_end.copy()}
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::StrNode{string, pos_start: _, pos_end: _} => write!(f, "{}", string),
            Node::RulesNode{rules, pos_start: _, pos_end: _} => {
                write!(f, "{}{}{}", "[", rules.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(", "), "]")
            },
            Node::BinOpNode{left, op, right, pos_start: _, pos_end: _} => write!(f, "({} {} {})", left, op, right),
            Node::RuleNode{rhs, lhs, pos_start: _, pos_end: _} => write!(f, "{} := {}", rhs, lhs),
            Node::PostFixNode{postfix, op, pos_start: _, pos_end: _} => write!(f, "({} {})", postfix, op),
            Node::RuleAccessNode{identifier, pos_start: _, pos_end: _} => write!(f, "{}", identifier),
            Node::RuleChainNode{chain, pos_start: _, pos_end: _} => {
                write!(f, "{}{}{}", "(", chain.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(" "), ")")
            }
        }
    }
}
