use crate::meta::{Flag, Flags};
use crate::stringify::Stringify;
use crate::types::{
    Float32Type, Float64Type, Int16Type, Int32Type, Int64Type, Int8Type, UInt16Type, UInt32Type,
    UInt64Type, UInt8Type,
};
use crate::{ArrayType, MirageTypeEnum, PointerType, StructType};

macro_rules! new_value {
    ($name:ident[$mn:ident] : $t:ty[$c:ty]) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            pub ty: $t,
            pub value: $c,
        }

        impl $name {
            pub fn new(value: $c) -> Self {
                Self {
                    ty: <$t>::new(),
                    value,
                }
            }

            pub fn print_to_string(&self) -> String {
                format!("{} {}", self.ty.print_to_string(), self.value)
            }

            pub fn to_value_enum(&self) -> MirageValueEnum {
                MirageValueEnum::$mn(*self)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum MirageValueEnum {
    Int8(Int8Value),
    Int16(Int16Value),
    Int32(Int32Value),
    Int64(Int64Value),
    UInt8(UInt8Value),
    UInt16(UInt16Value),
    UInt32(UInt32Value),
    UInt64(UInt64Value),
    Float32(Float32Value),
    Float64(Float64Value),
    Array(ArrayValue),
    Pointer(PointerValue),
    Register(RegisterValue),
    Struct(StructValue),
}

impl MirageValueEnum {
    pub fn get_type(&self) -> MirageTypeEnum {
        match self {
            MirageValueEnum::Int8(v) => v.ty.into(),
            MirageValueEnum::Int16(v) => v.ty.into(),
            MirageValueEnum::Int32(v) => v.ty.into(),
            MirageValueEnum::Int64(v) => v.ty.into(),
            MirageValueEnum::UInt8(v) => v.ty.into(),
            MirageValueEnum::UInt16(v) => v.ty.into(),
            MirageValueEnum::UInt32(v) => v.ty.into(),
            MirageValueEnum::UInt64(v) => v.ty.into(),
            MirageValueEnum::Float32(v) => v.ty.into(),
            MirageValueEnum::Float64(v) => v.ty.into(),
            MirageValueEnum::Array(v) => MirageTypeEnum::Array(v.ty.clone()),
            MirageValueEnum::Register(index) => index.ty.clone(),
            MirageValueEnum::Pointer(ty) => MirageTypeEnum::Pointer(ty.ty.clone()),
            MirageValueEnum::Struct(v) => MirageTypeEnum::Struct(v.ty.clone()),
        }
    }

    pub fn expect_const_value(&self) -> Option<Self> {
        match self {
            MirageValueEnum::Register(_) => None,
            MirageValueEnum::Pointer(_) => None,
            _ => Some(self.clone()),
        }
    }

    pub fn expect_register_value(&self) -> Option<RegisterValue> {
        match self {
            MirageValueEnum::Register(e) => Some(e.clone()),
            _ => None,
        }
    }

    pub fn expect_int_value(&self) -> Option<IntValue> {
        match self {
            MirageValueEnum::Int8(v) => Some(IntValue::Int8(*v)),
            MirageValueEnum::Int16(v) => Some(IntValue::Int16(*v)),
            MirageValueEnum::Int32(v) => Some(IntValue::Int32(*v)),
            MirageValueEnum::Int64(v) => Some(IntValue::Int64(*v)),
            MirageValueEnum::Register(v) => {
                if v.ty.is_int() {
                    Some(IntValue::Register(v.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn try_to_rust_string(&self) -> Option<String> {
        if !self.get_type().is_string() {
            return None;
        }
        match self {
            MirageValueEnum::Array(a) => {
                let mut s = Vec::new();
                for c in a.values.clone() {
                    let c = c.expect_int_value().unwrap().expect_int8_value();
                    s.push(c.value as u8);
                }

                Some(String::from_utf8(s).unwrap())
            }
            _ => None,
        }
    }

    pub fn is_const(&self) -> bool {
        !matches!(self, MirageValueEnum::Register(_))
    }

    pub fn print_to_string(&self) -> String {
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
            MirageValueEnum::Array(v) => v.print_to_string(),
            MirageValueEnum::Pointer(v) => v.print_to_string(),
            MirageValueEnum::Register(v) => v.print_to_string(),
            MirageValueEnum::Struct(v) => v.print_to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntValue {
    Int8(Int8Value),
    Int16(Int16Value),
    Int32(Int32Value),
    Int64(Int64Value),
    Register(RegisterValue),
}

impl IntValue {
    pub fn expect_int8_value(&self) -> Int8Value {
        match self {
            IntValue::Int8(e) => e.clone(),
            e => panic!("expect int8 type got {:?}", e),
        }
    }

    pub fn expect_int16_value(&self) -> Int16Value {
        match self {
            IntValue::Int16(e) => e.clone(),
            e => panic!("expect int16 type got {:?}", e),
        }
    }

    pub fn expect_int32_value(&self) -> Int32Value {
        match self {
            IntValue::Int32(e) => e.clone(),
            e => panic!("expect int32 type got {:?}", e),
        }
    }

    pub fn expect_int64_value(&self) -> Int64Value {
        match self {
            IntValue::Int64(e) => e.clone(),
            e => panic!("expect int64 type got {:?}", e),
        }
    }

    pub fn get_mem_size(&self) -> usize {
        match self {
            IntValue::Int8(_) => 1,
            IntValue::Int16(_) => 2,
            IntValue::Int32(_) => 4,
            IntValue::Int64(_) => 8,
            IntValue::Register(_) => 8,
        }
    }

    pub fn get_max_bits(&self) -> usize {
        match self {
            IntValue::Int8(_) => 8,
            IntValue::Int16(_) => 16,
            IntValue::Int32(_) => 32,
            IntValue::Int64(_) => 64,
            IntValue::Register(e) => e.get_type().size().size() * 8,
        }
    }

    pub fn to_mirage_value(&self) -> MirageValueEnum {
        match self {
            IntValue::Int8(v) => MirageValueEnum::Int8(*v),
            IntValue::Int16(v) => MirageValueEnum::Int16(*v),
            IntValue::Int32(v) => MirageValueEnum::Int32(*v),
            IntValue::Int64(v) => MirageValueEnum::Int64(*v),
            IntValue::Register(v) => MirageValueEnum::Register(v.clone()),
        }
    }
}

new_value!(Int8Value[Int8]: Int8Type[i8]);
new_value!(Int16Value[Int16]: Int16Type[i16]);
new_value!(Int32Value[Int32]: Int32Type[i32]);
new_value!(Int64Value[Int64]: Int64Type[i64]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIntValue {
    UInt8(UInt8Value),
    UInt16(UInt16Value),
    UInt32(UInt32Value),
    UInt64(UInt64Value),
}

impl Into<MirageValueEnum> for UIntValue {
    fn into(self) -> MirageValueEnum {
        match self {
            UIntValue::UInt8(v) => MirageValueEnum::UInt8(v),
            UIntValue::UInt16(v) => MirageValueEnum::UInt16(v),
            UIntValue::UInt32(v) => MirageValueEnum::UInt32(v),
            UIntValue::UInt64(v) => MirageValueEnum::UInt64(v),
        }
    }
}

impl Into<UIntValue> for MirageValueEnum {
    fn into(self) -> UIntValue {
        match self {
            MirageValueEnum::UInt8(v) => UIntValue::UInt8(v),
            MirageValueEnum::UInt16(v) => UIntValue::UInt16(v),
            MirageValueEnum::UInt32(v) => UIntValue::UInt32(v),
            MirageValueEnum::UInt64(v) => UIntValue::UInt64(v),
            _ => panic!("Cannot convert {:?} into UIntValue", self),
        }
    }
}

new_value!(UInt8Value[UInt8]: UInt8Type[u8]);
new_value!(UInt16Value[UInt16]: UInt16Type[u16]);
new_value!(UInt32Value[UInt32]: UInt32Type[u32]);
new_value!(UInt64Value[UInt64]: UInt64Type[u64]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatValue {
    Float32(Float32Value),
    Float64(Float64Value),
}

impl Into<MirageValueEnum> for FloatValue {
    fn into(self) -> MirageValueEnum {
        match self {
            FloatValue::Float32(v) => MirageValueEnum::Float32(v),
            FloatValue::Float64(v) => MirageValueEnum::Float64(v),
        }
    }
}

impl Into<FloatValue> for MirageValueEnum {
    fn into(self) -> FloatValue {
        match self {
            MirageValueEnum::Float32(v) => FloatValue::Float32(v),
            MirageValueEnum::Float64(v) => FloatValue::Float64(v),
            _ => panic!("Cannot convert {:?} into FloatValue", self),
        }
    }
}

new_value!(Float32Value[Float32]: Float32Type[f32]);
new_value!(Float64Value[Float64]: Float64Type[f64]);

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayValue {
    pub ty: ArrayType,
    pub values: Vec<MirageValueEnum>,
}

impl ArrayValue {
    pub fn new(ty: ArrayType, values: Vec<MirageValueEnum>) -> Self {
        Self { ty, values }
    }

    pub fn print_to_string(&self) -> String {
        let values = self
            .values
            .iter()
            .map(|x| x.print_to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} [{}]", self.ty.print_to_string(), values)
    }

    pub fn to_mirage_value(&self) -> MirageValueEnum {
        MirageValueEnum::Array(self.clone())
    }
}

impl From<ArrayValue> for MirageValueEnum {
    fn from(value: ArrayValue) -> Self {
        Self::Array(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointerValue {
    pub ty: PointerType,
}

impl PointerValue {
    pub fn new(ty: PointerType) -> Self {
        Self { ty }
    }

    pub fn print_to_string(&self) -> String {
        format!("{}*", self.ty.print_to_string())
    }
}

impl From<PointerValue> for MirageValueEnum {
    fn from(value: PointerValue) -> Self {
        Self::Pointer(value)
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct RegisterValue {
    pub index: usize,
    pub register_type: RegisterType,
    pub ty: MirageTypeEnum,
    pub flags: Flags,
}

impl RegisterValue {
    pub fn new(index: usize, register_type: RegisterType, ty: MirageTypeEnum) -> Self {
        Self {
            index,
            register_type,
            ty,
            flags: Flags::new(Vec::new()),
        }
    }

    pub fn get_type(&self) -> MirageTypeEnum {
        self.ty.clone()
    }

    pub fn print_to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.register_type.print_to_string(),
            self.index,
            self.flags.to_string()
        )
    }

    pub fn to_mirage_value(&self) -> MirageValueEnum {
        MirageValueEnum::Register(self.clone())
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn contains_flag(&self, flag: &Flag) -> bool {
        self.flags.contains(flag)
    }

    pub fn remove_flag(&mut self, flag: &Flag) {
        self.flags = Flags::new(
            self.flags
                .inner
                .clone()
                .into_iter()
                .filter(|x| x != flag)
                .collect(),
        )
    }
}

impl PartialEq for RegisterValue {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.register_type == other.register_type
    }
}

impl From<RegisterValue> for MirageValueEnum {
    fn from(value: RegisterValue) -> Self {
        Self::Register(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegisterType {
    Register,
    Variable,
    Argument,
    Global,
}

impl RegisterType {
    pub fn print_to_string(&self) -> String {
        match self {
            RegisterType::Register => "r",
            RegisterType::Variable => "v",
            RegisterType::Argument => "arg",
            RegisterType::Global => "g",
        }
        .to_string()
    }
}

/// A register user is a register that is used by a user ( Not a register that is used by the compiler )
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterUserType {
    Register,
    Variable,
}

impl Into<RegisterType> for RegisterUserType {
    fn into(self) -> RegisterType {
        match self {
            RegisterUserType::Register => RegisterType::Register,
            RegisterUserType::Variable => RegisterType::Variable,
        }
    }
}

impl TryFrom<&str> for RegisterType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "r" => Ok(RegisterType::Register),
            "v" => Ok(RegisterType::Variable),
            "arg" => Ok(RegisterType::Argument),
            "g" => Ok(RegisterType::Global),
            _ => Err(format!("Cannot convert {} into RegisterType", value)),
        }
    }
}

#[macro_export]
macro_rules! register {
    (@$n:ident r $i:literal) => {
        $crate::RegisterValue::new(
            $i,
            $crate::RegisterType::Register,
            $crate::MirageTypeEnum::from_str(stringify!($n)).unwrap(),
        )
    };
    (@$n:ident v $i:literal) => {
        $crate::RegisterValue::new(
            $i,
            $crate::RegisterType::Variable,
            $crate::MirageTypeEnum::from_str(stringify!($n)).unwrap(),
        )
    };
    (@$n:ident arg $i:literal) => {
        $crate::RegisterValue::new(
            $i,
            $crate::RegisterType::Argument,
            $crate::MirageTypeEnum::from_str(stringify!($n)).unwrap(),
        )
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructValue {
    pub ty: StructType,
    pub values: Vec<MirageValueEnum>,
}

impl StructValue {
    pub fn new(ty: StructType, values: Vec<MirageValueEnum>) -> Self {
        Self { ty, values }
    }
    pub fn print_to_string(&self) -> String {
        let values = self
            .values
            .iter()
            .map(|x| x.print_to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} {{ {} }}", self.ty.print_to_string(), values)
    }
}
