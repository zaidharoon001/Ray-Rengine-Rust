mod position;
mod token;
mod lexer;
mod tokenTypes;
mod parser;
mod nodes;
mod error;
mod interpreter;
use tokenTypes::TokenTypes::*;

fn main() {
    let input =
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
    let position = position::Position{filename: String::from("Yoi"), ftext: input, index: 0u64, ln: 1u64, cn: 1u64};
    let mut lexer = lexer::Lexer{current_index: 0usize, chars: position.ftext.as_bytes().to_vec(), position: position};
    let mut toks = match lexer.lex() {
        Ok(a) => a,
        Err(e) => panic!(format!("{}", e))
    };
    toks.reverse();
    let mut parser = parser::Parser{tokens: toks};
    let ast = match parser.parse() {
        Ok(n) => n,
        Err(e) => {
            panic!("{}", e)
        }
    };
    let input = (String::from(""), String::from("1+28/1-(12*(2-13)+81)"));
    let (res, left) = match interpreter::run_interpreter(ast, input) {
            Ok(a) => a,
            Err(err) => panic!("{}", err)
        };
    println!("matched: \"{}\", left: \"{}\"", res, left);
}
