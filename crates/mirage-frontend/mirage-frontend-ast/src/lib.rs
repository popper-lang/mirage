use mirage_frontend_object::{MirageTypeEnum, MirageValueEnum};

pub mod target;
pub mod module;
pub mod ident;
pub mod external;
pub mod typedef;
pub mod list;
pub mod const_value;
pub mod global;
pub mod function;
pub mod flag;
pub mod labels;

pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for MirageTypeEnum {
    fn to_string(&self) -> String {
        match self {
            MirageTypeEnum::Int8(_) => String::from("@int8"),
            MirageTypeEnum::Int16(_) => String::from("@int16"),
            MirageTypeEnum::Int32(_) => String::from("@int32"),
            MirageTypeEnum::Int64(_) => String::from("@int64"),
            MirageTypeEnum::UInt8(_) => String::from("@uint8"),
            MirageTypeEnum::UInt16(_) => String::from("@uint16"),
            MirageTypeEnum::UInt32(_) => String::from("@uint32"),
            MirageTypeEnum::UInt64(_) => String::from("@uint64"),
            MirageTypeEnum::Float32(_) => String::from("@float32"),
            MirageTypeEnum::Float64(_) => String::from("@float64"),
        }
    }
}

impl Stringify for MirageValueEnum {
    fn to_string(&self) -> String {
        match self {
            MirageValueEnum::Int8(v) => v.print_to_string(),
            MirageValueEnum::Int16(v) => v.print_to_string(),
            MirageValueEnum::Int32(v) => v.print_to_string(),
            MirageValueEnum::Int64(v) => v.print_to_string(),
            MirageValueEnum::UInt8(v) => v.print_to_string(),
            MirageValueEnum::UInt16(v) => v.print_to_string(),
            MirageValueEnum::UInt32(v) => v.print_to_string(),
            MirageValueEnum::UInt64(v) => v.print_to_string(),
            MirageValueEnum::Float32(v) => v.print_to_string(),
            MirageValueEnum::Float64(v) => v.print_to_string(),
            MirageValueEnum::Register(index) => index.print_to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Module(module::Module),
    Target(target::Target),
    Global(global::Global),
    Function(function::Function),
    External(external::External),
    Typedef(typedef::TypeDef),
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
            Statement::Function(function) => function.to_string(),
            Statement::External(external) => external.to_string(),
            Statement::Typedef(typedef) => typedef.to_string(),
            Statement::Module(module) => module.to_string(),
            Statement::Target(target) => target.to_string(),
        }
    }
}
