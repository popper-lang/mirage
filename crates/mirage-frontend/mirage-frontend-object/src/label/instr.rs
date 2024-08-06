use crate::stringify::Stringify;
use super::value::Value;
use crate::{IntValue, MirageObject, MirageTypeEnum, MirageValueEnum, RegisterValue};
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
    Store(RegisterValue, Value),
    New(String, List<Value>),
    Get(RegisterValue, usize),
    Const(MirageObject),
    Free(Vec<RegisterValue>),
    Ret(Value),
    Jump(String),
    Jeq(String, Value, Value),
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
    SubInt8(Value, Value),
    SubInt16(Value, Value),
    SubInt32(Value, Value),
    SubInt64(Value, Value),
    SubFloat32(Value, Value),
    SubFloat64(Value, Value),

    Ref(Value),
    Load(MirageTypeEnum, Value)
}

impl Stringify for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Store(mem, val) => format!("store {}, {}", mem.print_to_string(), val.to_string()),
            Command::New(name, args) => format!("new {}, {}", name, args.to_string()),
            Command::Get(mem, index) => format!("get {}, {}", mem.print_to_string(), index),
            Command::Free(mems) => format!("free {}", mems.iter().map(|x| x.print_to_string()).collect::<Vec<String>>().join(", ")),
            Command::Jump(name) => format!("jump {}", name),
            Command::Jeq(name, mem, val) => format!("jeq {}, {}, {}", name, mem.to_string(), val.to_string()),
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
            Command::SubInt8(val1, val2) => format!("sub_i8 {}, {}", val1.to_string(), val2.to_string()),
            Command::SubInt16(val1, val2) => format!("sub_i16 {}, {}", val1.to_string(), val2.to_string()),
            Command::SubInt32(val1, val2) => format!("sub_i32 {}, {}", val1.to_string(), val2.to_string()),
            Command::SubInt64(val1, val2) => format!("sub_i64 {}, {}", val1.to_string(), val2.to_string()),
            Command::SubFloat32(val1, val2) => format!("sub_f32 {}, {}", val1.to_string(), val2.to_string()),
            Command::SubFloat64(val1, val2) => format!("sub_f64 {}, {}", val1.to_string(), val2.to_string()),
            Command::Const(val) => val.to_string(),
            Command::Ret(val) => format!("ret {}", val.to_string()),
            Command::Ref(val) => format!("ref {}", val.to_string()),
            Command::Load(ty, val) => format!("load {}, {}", ty.print_to_string(), val.to_string())

        }
    }
}

pub fn add(lhs: IntValue, rhs: IntValue) -> Command {
    assert_eq!(lhs.get_max_bits(), rhs.get_max_bits());

    match lhs.get_max_bits() {
        8 => Command::AddInt8(
            lhs.to_mirage_value().try_into().unwrap(),
            lhs.to_mirage_value().try_into().unwrap()
        ),
        16 => Command::AddInt16(
            lhs.to_mirage_value().try_into().unwrap(),
            lhs.to_mirage_value().try_into().unwrap()
        ),
        32 => Command::AddInt32(
            lhs.to_mirage_value().try_into().unwrap(),
            lhs.to_mirage_value().try_into().unwrap()
        ),
        64 => Command::AddInt64(
            lhs.to_mirage_value().try_into().unwrap(),
            lhs.to_mirage_value().try_into().unwrap()
        ),
        _ => panic!("Invalid bit size")
    }
}

pub fn incr(val: RegisterValue) -> Command {
    let ty = val.get_type();
    if ty.is_float() {
        match ty.get_max_bits() {
            32 => Command::IncrFloat32(val),
            64 => Command::IncrFloat64(val),
            _ => panic!("Invalid bit size")
        }
    } else {
        match ty.get_max_bits() {
            8 => Command::IncrInt8(val),
            16 => Command::IncrInt16(val),
            32 => Command::IncrInt32(val),
            64 => Command::IncrInt64(val),
            _ => panic!("Invalid bit size")
        }
    }
}

pub fn jump(name: &str) -> Command {
    Command::Jump(name.to_string())
}

pub fn jeq(name: &str, lhs: MirageValueEnum, rhs: MirageValueEnum) -> Command {
    assert_eq!(lhs.get_type(), rhs.get_type());
    let lhs = lhs.try_into().unwrap();
    let rhs = rhs.try_into().unwrap();
    Command::Jeq(name.to_string(), lhs, rhs)
}
