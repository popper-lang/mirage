use crate::MirageObject;
use crate::MirageTypeEnum;
use crate::MirageValueEnum;

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

impl Stringify for MirageObject {
    fn to_string(&self) -> String {
        format!("{}", self.value.print_to_string())
    }
}
