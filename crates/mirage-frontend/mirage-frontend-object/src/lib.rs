mod values;
mod types;
pub mod label;
pub mod size;
pub mod function;
pub mod util;
pub mod stringify;
pub mod statements;
pub mod meta;
pub use values::*;
pub use types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MirageObject {
    pub(crate) value: MirageValueEnum,
    pub(crate) ty: MirageTypeEnum,
}

impl MirageObject {
    pub fn from(value: MirageValueEnum) -> Self {
        Self {
            value,
            ty: value.get_type()
        }
    }
    pub fn new(value: MirageValueEnum, ty: MirageTypeEnum) -> Self {
        Self {
            value,
            ty
        }
    }

    pub fn get_type(&self) -> MirageTypeEnum {
        self.ty
    }

    pub fn get_value(&self) -> MirageValueEnum {
        self.value.clone()
    }
}
