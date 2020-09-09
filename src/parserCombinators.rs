use crate::position;
use crate::error;
use crate::interpreter;
use crate::tokenTypes;
use crate::nodes;
use crate::context;
use crate::tokenTypes::TokenTypes::*;

pub type Value = (String, String);

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

pub fn literal_parse(expected: String, string: Value, pos_start: position::Position, pos_end: position::Position) -> Result<Value, error::Error> {
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

pub fn copy_value(value: &Result<Value, error::Error>) -> Result<Value, error::Error> {
    match value {
        Ok(t) => {
            let (a, b) = t;
            Ok((a.clone(), b.clone()))
        },
        Err(err) => Err(err.copy())
    }
}

pub fn handle_binary_op(
        left: Box<nodes::Node>,
        op: tokenTypes::TokenTypes,
        right: Box<nodes::Node>,
        ctx: &mut context::Context,
        input: Result<Value, error::Error>
    ) -> Result<Value, error::Error> {
    let left = interpreter::interpret(left, ctx, copy_value(&input));
    match op {
        Pipe => {
            match left {
                Ok(a) => return Ok(a),
                Err(_) => return interpreter::interpret(right, ctx, input)
            };
        },
        _ => panic!("This should not have happend, I expected {} but got {}", Pipe, op)
    }
}

pub fn handle_postfix_op(postfix: Box<nodes::Node>, op: tokenTypes::TokenTypes, context: &mut context::Context, input: Result<Value, error::Error>) -> Result<Value, error::Error> {
    let post_node = (&postfix).copy();
    let postfix_res = interpreter::interpret(Box::new(post_node), context, copy_value(&input));
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
                let interpreted = interpreter::interpret(Box::new(postfix.copy()), context, copy_value(&output));
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
