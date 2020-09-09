use std::collections::HashMap;
use crate::nodes;
use crate::position;
use crate::error;
use crate::tokenTypes;
use crate::tokenTypes::TokenTypes::*;
use crate::context;
use crate::parserCombinators;

pub fn interpret(node: Box<nodes::Node>, context: &mut context::Context, input: Result<parserCombinators::Value, error::Error>) -> Result<parserCombinators::Value, error::Error> {
    let node = *node;
    match node {
        nodes::Node::RuleChainNode{chain, pos_start: _, pos_end: _} => {
            let mut input = input;
            for rule in chain {
                let second = parserCombinators::snd(
                    match input {
                        Ok(a) => a,
                        Err(err) => return Err(err)
                    }
                );
                input = interpret(Box::new(rule.copy()), context, Ok((String::from(""), second)))
            };
            match input {
                Ok(a) => Ok(a),
                Err(err) => return Err(err)
            }
        },
        nodes::Node::StrNode{string, pos_start, pos_end} => parserCombinators::literal_parse(
            string.to_string(),
            match input {
                Ok(a) => a,
                Err(err) => return Err(err)
            },
            pos_start.copy(),
            pos_end.copy()
        ),
        nodes::Node::RuleAccessNode{identifier, pos_start, pos_end} => {
            let lazy_value = match context.get(identifier.to_string()) {
                Some(a) => a,
                None => return Err (
                    error::Error{
                        name: "NoDefinitionError".to_string(),
                        message: format!("No definition for {} found", identifier).to_string(),
                        pos_start: pos_start.copy(),
                        pos_end: pos_end.copy()
                    }
                )
            };
            interpret(Box::new(lazy_value.fun.copy()), context, input)
        },
        nodes::Node::BinOpNode{left, op, right, pos_start: _, pos_end: _} => parserCombinators::handle_binary_op(left, op, right, context, input),
        nodes::Node::PostFixNode{postfix, op, pos_start: _, pos_end: _} => parserCombinators::handle_postfix_op(postfix, op, context, input),
        _ => {
            panic!("@::")
        }
    }
}

fn add_def_pass(node: Box<nodes::Node>, context: &mut context::Context) {
    match *node {
        nodes::Node::RulesNode{rules, pos_start: _, pos_end: _} => {
            for rule in rules {
                let (key, value) = match rule {
                    nodes::Node::RuleNode{lhs, rhs, pos_start: _, pos_end: _} => (rhs.clone(), lhs),
                    _ => panic!("Not sure about that one")
                };
                let thunk = context::Lazy{fun: value};
                context.set(key, thunk)
            }
        },
        _ => panic!("No, this should be a RulesNode, yo fcked up")
    }
}

pub fn run_interpreter(ast: nodes::Node, input: parserCombinators::Value) -> Result<parserCombinators::Value, error::Error> {
    let mut context = context::Context{symbols: HashMap::new()};
    add_def_pass(Box::new(ast.copy()), &mut context);
    let main = match context.get("main".to_string()) {
        Some(a) => a,
        None => {
            let (pos_start, pos_end) = ast.get_pos();
            return Err (
                error::Error{
                    name: "NoDefinitionError".to_string(),
                    message: "No definition for main found".to_string(),
                    pos_start,
                    pos_end
                }
            )
        }
    };
    return interpret(Box::new(main.fun.copy()), &mut context, Ok(input))
}
