use crate::function::FunctionType;
use crate::stringify::Stringify;


/// A extern declaration.
/// Syntax: extern <name> : <type>;
#[derive(Debug, Clone, PartialEq)]
pub struct External {
    pub name: String,
    pub ty: FunctionType
}

impl External {
    pub fn new(name: String, ty: FunctionType) -> Self {
        Self {
            name,
            ty
        }
    }
}

impl Stringify for External {
    fn to_string(&self) -> String {
        format!("extern {} : {};", self.name, self.ty.print_to_string())
    }
}
