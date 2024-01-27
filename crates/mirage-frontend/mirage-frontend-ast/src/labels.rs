use mirage_frontend_object::{MirageValueEnum, RegisterValue};
use crate::const_value::{
    ConstValue,
    ConstValueInstr
};
use crate::flag::Flags;
use crate::ident::Ident;
use crate::list::List;
use crate::Stringify;

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub name: Ident,
    pub flags: Flags,
    pub body: Vec<LabelBodyInstr>
}

impl Label {
    pub fn new(name: Ident, flags: Flags, body: Vec<LabelBodyInstr>) -> Self {
        Self {
            name,
            flags,
            body
        }
    }
}

impl Stringify for Label {
    fn to_string(&self) -> String {
        format!("{}{}: {}", self.name.to_string(), self.flags.to_string(), "\n".to_owned() + &self.body.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LabelBodyInstr {
    Assign(Value, Box<LabelBodyInstr>),
    Call(Ident, Vec<Value>),
    Command(Command),
}

impl Stringify for LabelBodyInstr {
    fn to_string(&self) -> String {
        match self {
            LabelBodyInstr::Assign(mem, instr) => format!("{} = {}", mem.to_string(), instr.to_string()),
            LabelBodyInstr::Call(name, args) => format!("{} {{ {} }}", name.to_string(), args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            LabelBodyInstr::Command(command) => command.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    ConstValue(ConstValueInstr),
    Register(RegisterValue),
    List(List<Value>)
}

impl Stringify for Value {
    fn to_string(&self) -> String {
        match self {
            Value::ConstValue(val) => val.to_string(),
            Value::Register(mem) => mem.print_to_string(),
            Value::List(list) => list.to_string()
        }
    }
}

impl TryInto<MirageValueEnum> for Value {
    type Error = String;

    fn try_into(self) -> Result<MirageValueEnum, Self::Error> {
        match self {
            Value::ConstValue(val) => Ok(
                val.val.val
            ),
            Value::Register(mem) => Ok(MirageValueEnum::Register(mem)),
            Value::List(_list) => Err("Cannot convert list to MirageValueEnum".to_string()),
        }
    }

}

impl TryInto<Value> for MirageValueEnum {
    type Error = String;
    fn try_into(self) -> Result<Value, String> {
        match self {
            MirageValueEnum::Register(mem) => Ok(Value::Register(mem)),
            e => {
                Ok(Value::ConstValue(
                    ConstValueInstr::new(
                        ConstValue::from(e)
                    )
                ))
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Copy(Ident),
    New(Ident, List<Value>),
    Get(RegisterValue, usize),
    Const(ConstValueInstr),
    Free(Vec<RegisterValue>),
    Jump(Ident),
    Jeq(Ident, RegisterValue, Value),
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
            Command::New(name, args) => format!("new {}, {}", name.to_string(), args.to_string()),
            Command::Get(mem, index) => format!("get {}, {}", mem.print_to_string(), index),
            Command::Free(mems) => format!("free {}", mems.iter().map(|x| x.print_to_string()).collect::<Vec<String>>().join(", ")),
            Command::Jump(name) => format!("jump {}", name.to_string()),
            Command::Jeq(name, mem, val) => format!("jeq {}, {}, {}", name.to_string(), mem.print_to_string(), val.to_string()),
            Command::IncrInt8(mem) => format!("incr.i8 {}", mem.print_to_string()),
            Command::IncrInt16(mem) => format!("incr.i16 {}", mem.print_to_string()),
            Command::IncrInt32(mem) => format!("incr.i32 {}", mem.print_to_string()),
            Command::IncrInt64(mem) => format!("incr.i64 {}", mem.print_to_string()),
            Command::IncrFloat32(mem) => format!("incr.f32 {}", mem.print_to_string()),
            Command::IncrFloat64(mem) => format!("incr.f64 {}", mem.print_to_string()),
            Command::AddInt8(val1, val2) => format!("add.i8 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt16(val1, val2) => format!("add.i16 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt32(val1, val2) => format!("add.i32 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddInt64(val1, val2) => format!("add.i64 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddFloat32(val1, val2) => format!("add.f32 {}, {}", val1.to_string(), val2.to_string()),
            Command::AddFloat64(val1, val2) => format!("add.f64 {}, {}", val1.to_string(), val2.to_string()),
            Command::Const(val) => val.to_string(),

        }
    }
}
