#[derive(Debug)]
pub struct Position {
    pub filename: String,
    pub ftext: String,
    pub index: u64,
    pub ln: u64,
    pub cn: u64
}

impl Position {
    pub fn advance(& mut self, current_char: char) -> &Position {
        self.index += 1;
        self.cn += 1;
        if current_char == '\n' {
            self.ln += 1;
            self.cn = 0
        }
    return self
    }

    pub fn copy(&self) -> Position {
        return Position{index: self.index, ln: self.ln, cn: self.cn, filename: self.filename.clone(), ftext: self.ftext.clone()}
    }
}
