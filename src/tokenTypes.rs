#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum TokenTypes {
    Str,
    Comma,
    LParen,
    RParen,
    Assign,
    Identifier,
    Newline,
    Asterisk,
    QuestionMark,
    Pipe,
    Plus,
    Eof
}

impl std::fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
