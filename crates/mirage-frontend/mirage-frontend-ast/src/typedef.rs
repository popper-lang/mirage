use mirage_frontend_object::MirageTypeEnum;
use crate::ident::Ident;
use crate::list::List;
use crate::Stringify;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    pub name: Ident,
    pub ty: List<MirageTypeEnum>
}

impl TypeDef {
    pub fn new(name: Ident, ty: List<MirageTypeEnum>) -> Self {
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
