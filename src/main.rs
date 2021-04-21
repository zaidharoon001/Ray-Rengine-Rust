mod position;
mod token;
mod lexer;
mod tokenTypes;
mod parser;
mod nodes;
mod error;
mod interpreter;
mod rayRengine;
mod context;
mod parserCombinators;
use tokenTypes::TokenTypes::*;

fn main() {
    let regex =
        String::from(
            "
            nums := \"1234567890\"
            whitespace := '\t' | ' ' | '\n'
            alphabets := \"abcdefghijklmnopqrstuvwxyz\"
            expr := term (whitespace* ('+'|'-') whitespace* term)*
            factor := '(' whitespace* expr whitespace* ')' | nums+
            term := factor (whitespace* ('*'|'/') whitespace* factor)*
            main := expr
            "
        );
        let input = String::from("1+28/1-(12*(2-13)+81)");
        println!("{}", rayRengine::string_left(regex, input));
}
