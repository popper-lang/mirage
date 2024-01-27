use crate::util::List;
use crate::MirageTypeEnum;
use crate::stringify::Stringify;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    pub name: String,
    pub ty: List<MirageTypeEnum>
}

impl TypeDef {
    pub fn new(name: String, ty: List<MirageTypeEnum>) -> Self {
        Self {
            name,
            ty
        }
    }
}

impl Stringify for TypeDef {
    fn to_string(&self) -> String {
        format!("type {} = {};", self.name.to_string(), self.ty.to_string())
    }
}
