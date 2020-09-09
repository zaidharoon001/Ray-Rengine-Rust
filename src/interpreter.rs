use std::collections::HashMap;
use crate::nodes;
use crate::position;
use crate::error;
use crate::tokenTypes;
use crate::tokenTypes::TokenTypes::*;

struct Lazy {
    fun: Box<nodes::Node>,
}

impl Lazy {
    fn evaluate(&self, context: &mut Context, input: Result<Value, error::Error>) -> Result<Value, error::Error> {
        let value = interpret(Box::new(self.fun.copy()), context, input);
        return value
    }
}

struct Context {
    symbols:  HashMap<String, Lazy>
}

impl Context {
    fn get(&mut self, name: String) -> Option<&Lazy> {
        return self.symbols.get(&name)
    }

    fn set(&mut self, name: String, thunk: Lazy) {
        self.symbols.insert(name, thunk);
    }

}

type Value = (String, String);

fn add_def_pass(node: Box<nodes::Node>, context: &mut Context) {
    match *node {
        nodes::Node::RulesNode{rules, pos_start: _, pos_end: _} => {
            for rule in rules {
                let (key, value) = match rule {
                    nodes::Node::RuleNode{lhs, rhs, pos_start: _, pos_end: _} => (rhs.clone(), lhs),
                    _ => panic!("Not sure about that one")
                };
                let thunk = Lazy{fun: value};
                context.set(key, thunk)
            }
        },
        _ => panic!("No, this should be a RulesNode, yo fcked up")
    }
}

pub fn fst<A, B>(tuple: (A, B)) -> A {
    match tuple {
        (a, _) => a
    }
}

pub fn snd<A, B>(tuple: (A, B)) -> B {
    match tuple {
        (_, b) => b
    }
}

fn literal_parse(expected: String, string: Value, pos_start: position::Position, pos_end: position::Position) -> Result<Value, error::Error> {
    let mut match_str = String::from("");
    let mut i = 0;
    let mut second = snd(string);
    if expected.len() > second.len() {
        return Err(
            error::Error{
                name: String::from("InputError"),
                message: String::from(format!("cannot match {} with {}", second, expected)),
                pos_start,
                pos_end
            }
        )
    };
    while i<expected.len(){
        let character = second.remove(0);
        match_str = format!("{}{}", match_str, character);
        i += 1;
    };
    if match_str == expected {
        Ok((match_str, second))
    } else {
        Err(
            error::Error{
                name: String::from("InputError"),
                message: String::from(format!("expected {}, found {}", expected, match_str)),
                pos_start,
                pos_end
            }
        )
    }
}

fn copy_value(value: &Result<Value, error::Error>) -> Result<Value, error::Error> {
    match value {
        Ok(t) => {
            let (a, b) = t;
            Ok((a.clone(), b.clone()))
        },
        Err(err) => Err(err.copy())
    }
}

fn handle_binary_op(
        left: Box<nodes::Node>,
        op: tokenTypes::TokenTypes,
        right: Box<nodes::Node>,
        ctx: &mut Context,
        input: Result<Value, error::Error>
    ) -> Result<Value, error::Error> {
    let left = interpret(left, ctx, copy_value(&input));
    match op {
        Pipe => {
            match left {
                Ok(a) => return Ok(a),
                Err(_) => return interpret(right, ctx, input)
            };
        },
        _ => panic!("This should not have happend, I expected {} but got {}", Pipe, op)
    }
}

fn handle_postfix_op(postfix: Box<nodes::Node>, op: tokenTypes::TokenTypes, context: &mut Context, input: Result<Value, error::Error>) -> Result<Value, error::Error> {
    let post_node = (&postfix).copy();
    let postfix_res = interpret(Box::new(post_node), context, copy_value(&input));
    match op {
        QuestionMark => {
            match postfix_res {
                Ok(a) => Ok(a),
                Err(_) => return input
            }
        },
        Asterisk | Plus => {
            let mut output = postfix_res;
            match output {
                Ok(_) => (),
                Err(err) => return if op == Asterisk {input} else {Err(err)}
            };
            loop {
                let interpreted = interpret(Box::new(postfix.copy()), context, copy_value(&output));
                match interpreted {
                    Ok(a) => {
                        output = Ok(a);
                    },
                    Err(_) => return output
                }
            }
        },
        _ => panic!("Can't understand postfix operator, {}", op)
    }
}

fn interpret(node: Box<nodes::Node>, context: &mut Context, input: Result<Value, error::Error>) -> Result<Value, error::Error> {
    let node = *node;
    match node {
        nodes::Node::RuleChainNode{chain, pos_start: _, pos_end: _} => {
            let mut input = input;
            for rule in chain {
                let second = snd(
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
        nodes::Node::StrNode{string, pos_start, pos_end} => literal_parse(
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
        nodes::Node::BinOpNode{left, op, right, pos_start: _, pos_end: _} => handle_binary_op(left, op, right, context, input),
        nodes::Node::PostFixNode{postfix, op, pos_start: _, pos_end: _} => handle_postfix_op(postfix, op, context, input),
        _ => {
            panic!("@::")
        }
    }
}

pub fn run_interpreter(ast: nodes::Node, input: Value) -> Result<Value, error::Error> {
    let mut context = Context{symbols: HashMap::new()};
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
