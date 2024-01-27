use crate::ident::Ident;
use crate::Stringify;


/// A module declaration.
/// Syntax: module <name>;
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Ident
}


impl Module {
    pub fn new(name: Ident) -> Self {
        Self {
            name
        }
    }

}

impl Stringify for Module {
    fn to_string(&self) -> String {
        format!("module {};", self.name.to_string())
    }
}
