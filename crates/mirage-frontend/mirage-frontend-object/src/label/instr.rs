use crate::stringify::Stringify;
use super::value::Value;
use crate::{MirageObject, RegisterValue};
use crate::util::List;

#[derive(Debug, Clone, PartialEq)]
pub enum LabelBodyInstr {
    Assign(RegisterValue, Box<LabelBodyInstr>),
    Call(String, Vec<Value>),
    Command(Command),
}

impl Stringify for LabelBodyInstr {
    fn to_string(&self) -> String {
        match self {
            LabelBodyInstr::Assign(mem, instr) => format!("{} = {}", mem.print_to_string(), instr.to_string()),
            LabelBodyInstr::Call(name, args) => format!("{} {{ {} }}", name, args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            LabelBodyInstr::Command(command) => command.to_string()
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Copy(String),
    New(String, List<Value>),
    Get(RegisterValue, usize),
    Const(MirageObject),
    Free(Vec<RegisterValue>),
    Jump(String),
    Jeq(String, RegisterValue, Value),
    IncrInt8(RegisterValue),
    IncrInt16(RegisterValue),
    IncrInt32(RegisterValue),
    IncrInt64(RegisterValue),
    IncrFloat32(RegisterValue),
    IncrFloat64(RegisterValue),
    AddInt8(Value, Value),
    AddInt16(Value, Value),
    AddInt32(Value, Value),
    AddInt64(Value, Value),
    AddFloat32(Value, Value),
    AddFloat64(Value, Value),
}

impl Stringify for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Copy(name) => format!("copy {}", name.to_string()),
            Command::New(name, args) => format!("new {}, {}", name, args.to_string()),
            Command::Get(mem, index) => format!("get {}, {}", mem.print_to_string(), index),
            Command::Free(mems) => format!("free {}", mems.iter().map(|x| x.print_to_string()).collect::<Vec<String>>().join(", ")),
            Command::Jump(name) => format!("jump {}", name),
            Command::Jeq(name, mem, val) => format!("jeq {}, {}, {}", name, mem.print_to_string(), val.to_string()),
            Command::IncrInt8(mem) => format!("incr_i8 {}", mem.print_to_string()),
            Command::IncrInt16(mem) => format!("incr_i16 {}", mem.print_to_string()),
            Command::IncrInt32(mem) => format!("incr_i32 {}", mem.print_to_string()),
            Command::IncrInt64(mem) => format!("incr_i64 {}", mem.print_to_string()),
            Command::IncrFloat32(mem) => format!("incr_f32 {}", mem.print_to_string()),
            Command::IncrFloat64(mem) => format!("incr_f64 {}", mem.print_to_string()),
            Command::AddInt8(val1, val2) => format!("add_i8 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt16(val1, val2) => format!("add_i16 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt32(val1, val2) => format!("add_i32 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt64(val1, val2) => format!("add_i64 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddFloat32(val1, val2) => format!("add_f32 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddFloat64(val1, val2) => format!("add_f64 {}, {}", val1.to_string(), val2.to_string()),
            Command::Const(val) => val.to_string(),

        }
    }
}
