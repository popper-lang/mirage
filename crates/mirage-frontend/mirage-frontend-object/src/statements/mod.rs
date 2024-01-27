mod typedef;
mod module;
mod target;
mod global;
mod external;

pub use typedef::*;
pub use module::*;
pub use target::*;
pub use global::*;
pub use external::*;

use crate::function::FunctionValue;
use crate::stringify::Stringify;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Module(ModuleDecl),
    Target(Target),
    Global(Global),
    Function(FunctionValue),
    External(External),
    Typedef(TypeDef),
}

impl Statement {
    pub fn is_module(&self) -> bool {
        match self {
            Statement::Module(_) => true,
            _ => false
        }
    }

    pub fn is_target(&self) -> bool {
        match self {
            Statement::Target(_) => true,
            _ => false
        }
    }

    pub fn is_global(&self) -> bool {
        match self {
            Statement::Global(_) => true,
            _ => false
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Statement::Function(_) => true,
            _ => false
        }
    }

    pub fn is_external(&self) -> bool {
        match self {
            Statement::External(_) => true,
            _ => false
        }
    }

    pub fn is_typedef(&self) -> bool {
        match self {
            Statement::Typedef(_) => true,
            _ => false
        }
    }

}

impl Stringify for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Global(global) => global.to_string(),
            Statement::Function(function) => function.print_to_string(),
            Statement::External(external) => external.to_string(),
            Statement::Typedef(typedef) => typedef.to_string(),
            Statement::Module(module) => module.to_string(),
            Statement::Target(target) => target.to_string(),
        }
    }
}
