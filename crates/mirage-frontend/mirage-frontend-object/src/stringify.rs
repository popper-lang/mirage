use crate::MirageObject;
use crate::MirageTypeEnum;
use crate::MirageValueEnum;

pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for MirageTypeEnum {
    fn to_string(&self) -> String {
        self.print_to_string()
    }
}

impl Stringify for MirageValueEnum {
    fn to_string(&self) -> String {
        self.print_to_string()
    }
}

impl Stringify for MirageObject {
    fn to_string(&self) -> String {
        format!("{}", self.value.print_to_string())
    }
}
