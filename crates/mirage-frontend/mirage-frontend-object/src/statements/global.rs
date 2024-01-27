
use crate::{MirageObject, stringify::Stringify};


/// A global variable.
/// Syntax: global <name> = <value>
#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub name: String,
    pub value: MirageObject
}

impl Global {
    pub fn new(name: String, value: MirageObject) -> Self {
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
