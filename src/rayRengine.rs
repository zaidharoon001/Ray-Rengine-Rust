use crate::position;
use crate::lexer;
use crate::parser;
use crate::interpreter;

pub fn match_string(regex:String, input: String) -> bool {
    let position = position::Position{filename: String::from("Yoi"), ftext: regex, index: 0u64, ln: 1u64, cn: 1u64};
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
    let input = (String::from(""), input);
    let (_, left) = match interpreter::run_interpreter(ast, input) {
            Ok(a) => a,
            Err(err) => panic!("{}", err)
        };
    return left == "".to_string()
}
