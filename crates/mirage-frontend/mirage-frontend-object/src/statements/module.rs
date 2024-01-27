use crate::stringify::Stringify;


/// A module declaration.
/// Syntax: module <name>;
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub name: String
}


impl ModuleDecl {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

}

impl Stringify for ModuleDecl {
    fn to_string(&self) -> String {
        format!("module {};", self.name)
    }
}
