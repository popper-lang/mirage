use mirage_frontend_object::function::{FunctionType, FunctionValue};
use mirage_frontend_object::MirageTypeEnum;
use crate::flag::Flags;
use crate::ident::Ident;
use crate::labels::Label;
use crate::Stringify;


#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: FunctionBody
}

impl Function {
    pub fn new(signature: FunctionSignature, body: FunctionBody) -> Self {
        Self {
            signature,
            body
        }
    }
}

impl Stringify for Function {
    fn to_string(&self) -> String {
        format!("{} {}", self.signature.to_string(), self.body.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: Ident,
    pub params: Vec<MirageTypeEnum>,
    pub ret: MirageTypeEnum,
    pub flags: Flags
}

impl FunctionSignature {
    pub fn new(name: Ident, params: Vec<MirageTypeEnum>, ret: MirageTypeEnum, flags: Flags) -> Self {
        Self {
            name,
            params,
            ret,
            flags
        }
    }
}

impl Into<FunctionSignature> for FunctionValue {
    fn into(self) -> FunctionSignature {
        FunctionSignature::new(
            Ident::new(
                self.get_name().clone()),
                self.get_type().get_args().clone(),
                self.get_type().get_ret().clone(), Flags::new(vec![]))
    }
}


impl Stringify for FunctionSignature {
    fn to_string(&self) -> String {
        format!("{}({}) {} {}", self.name.to_string(), self.params.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "), self.ret.to_string(), self.flags.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub inner: Vec<Label>
}

impl FunctionBody {
    pub fn new(inner: Vec<Label>) -> Self {
        Self {
            inner
        }
    }

    pub fn push(&mut self, label: Label) {
        self.inner.push(label);
    }
}

impl Stringify for FunctionBody {
    fn to_string(&self) -> String {
        let mut s = String::from("{\n");
        for instr in &self.inner {
            s.push_str(&instr.to_string());
            s.push('\n');
        }
        s.push('}');
        s
    }
}
