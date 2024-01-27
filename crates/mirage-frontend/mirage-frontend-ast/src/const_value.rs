use mirage_frontend_object::{MirageTypeEnum, MirageValueEnum};
use crate::Stringify;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstValueInstr {
    pub val: ConstValue
}

impl ConstValueInstr {
    pub fn new(val: ConstValue) -> Self {
        Self {
            val
        }
    }
}

impl Stringify for ConstValueInstr {
    fn to_string(&self) -> String {
        format!("const {}", self.val.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstValue {
    pub ty: MirageTypeEnum,
    pub val: MirageValueEnum
}



impl ConstValue {
    pub fn from(val: MirageValueEnum) -> Self {
        Self {
            ty: val.get_type(),
            val
        }
    }

    pub fn new(ty: MirageTypeEnum, val: MirageValueEnum) -> Self {
        Self {
            ty,
            val
        }
    }
}

impl Stringify for ConstValue {
    fn to_string(&self) -> String {
        format!("{}", self.val.to_string())
    }
}
