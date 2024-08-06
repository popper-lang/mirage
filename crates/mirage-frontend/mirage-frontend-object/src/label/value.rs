use crate::{util::List, stringify::Stringify, MirageObject, MirageValueEnum, RegisterValue, MirageTypeEnum};



#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    ConstValue(MirageObject),
    Register(RegisterValue),
    List(List<Value>)
}

impl Value {
    pub fn get_type(&self) -> MirageTypeEnum {
        match self {
            Value::ConstValue(val) => val.get_type(),
            Value::Register(mem) => mem.get_type(),
            Value::List(list) => panic!("Cannot get type of list")
        }
    }
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
                val.value
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
                    MirageObject::from(
                        e
                    )
                ))
            }
        }
    }
}
