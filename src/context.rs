use std::collections::HashMap;
use crate::nodes;

pub struct Lazy {
    pub fun: Box<nodes::Node>,
}

pub struct Context {
    pub symbols:  HashMap<String, Lazy>
}

impl Context {
    pub fn get(&mut self, name: String) -> Option<&Lazy> {
        return self.symbols.get(&name)
    }

    pub fn set(&mut self, name: String, thunk: Lazy) {
        self.symbols.insert(name, thunk);
    }

}
