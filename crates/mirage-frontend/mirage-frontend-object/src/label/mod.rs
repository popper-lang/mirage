mod instr;
mod value;
use crate::meta::Flags;
use crate::stringify::Stringify;
pub use instr::*;
pub use value::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: String,
    pub flags: Flags,
    pub body: Vec<LabelBodyInstr>
}

impl Label {
    pub fn new(name: String, flags: Flags, body: Vec<LabelBodyInstr>) -> Self {
        Self {
            name,
            flags,
            body
        }
    }
}

impl Stringify for Label {
    fn to_string(&self) -> String {
        format!("{}{}: {}",
            self.name.to_string(),
            self.flags.to_string(),
            "\n".to_owned() + &self.body
                .iter()
                .map(|x| "\t".to_owned() + &x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
