use crate::const_value::ConstValue;
use crate::ident::Ident;
use crate::Stringify;


/// A global variable.
/// Syntax: global <name> = <value>
#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub name: Ident,
    pub value: ConstValue
}

impl Global {
    pub fn new(name: Ident, value: ConstValue) -> Self {
        Self {
            name,
            value
        }
    }
}

impl Stringify for Global {
    fn to_string(&self) -> String {
        format!("global {} = {}", self.name.to_string(), self.value.to_string())
    }
}
