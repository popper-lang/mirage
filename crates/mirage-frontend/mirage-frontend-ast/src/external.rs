use mirage_frontend_object::MirageTypeEnum;
use mirage_frontend_object::function::{FunctionType, FunctionValue};
use crate::ident::Ident;
use crate::Stringify;


/// A extern declaration.
/// Syntax: extern <name> : <type>;
#[derive(Debug, Clone, PartialEq)]
pub struct External {
    pub name: Ident,
    pub ty: ExternalType
}

impl External {
    pub fn new(name: Ident, ty: ExternalType) -> Self {
        Self {
            name,
            ty
        }
    }
}

impl Stringify for External {
    fn to_string(&self) -> String {
        format!("extern {} : {};", self.name.to_string(), self.ty.to_string())
    }
}

impl Into<FunctionValue> for External {
    fn into(self) -> FunctionValue {
        let (args, ret) =
            match self.ty {
                ExternalType::Function(func) => {
                    (func.args, func.ret)
                },
                ExternalType::Variable(var) => {
                    panic!("Cannot convert variable to function")
                }
            }
        ;

        let args: Vec<MirageTypeEnum> = args.0.iter().map(|x| {
            match x {
                ExternalArgumentsType::Simple(ty) => ty.clone(),
                ExternalArgumentsType::Infinite => panic!("Infinite arguments are not supported yet")
            }
        }).collect();



        FunctionValue::new(
            self.name.into(),
            FunctionType::new(
                args, ret
            )

        )
    }
}

impl Into<External> for FunctionValue {
    fn into(self) -> External {
        External::new(
            self.get_name().clone().into(),
            ExternalType::Function(
                ExternalFunctionType::new(
                    ExternalArguments::new(
                        self.get_type().get_args().iter().map(|x| {
                            ExternalArgumentsType::Simple(x.clone())
                        }).collect()
                    ),
                    self
                        .get_type()
                        .get_ret()
                        .clone()
                )
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalType {
    Function(ExternalFunctionType),
    Variable(ExternalVariableType)
}

impl Stringify for ExternalType {
    fn to_string(&self) -> String {
        match self {
            ExternalType::Function(func) => func.to_string(),
            ExternalType::Variable(var) => var.to_string()
        }
    }
}


/// A extern function declaration.
/// Syntax: args[types...] ret[type]
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFunctionType {
    pub args: ExternalArguments,
    pub ret: MirageTypeEnum
}



impl ExternalFunctionType {
    pub fn new(args: ExternalArguments, ret: MirageTypeEnum) -> Self {
        Self {
            args,
            ret
        }
    }
}


impl Stringify for ExternalFunctionType {
    fn to_string(&self) -> String {
        format!("args {} ret[{}]", self.args.to_string(), self.ret.to_string())
    }
}


/// A extern variable declaration.
/// Syntax: <type>
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalVariableType {
    pub ty: MirageTypeEnum
}

impl ExternalVariableType {
    pub fn new(ty: MirageTypeEnum) -> Self {
        Self {
            ty
        }
    }
}

impl Stringify for ExternalVariableType {
    fn to_string(&self) -> String {
        self.ty.to_string()
    }
}


/// A extern function arguments declaration.
/// Syntax: [types...]
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalArguments(Vec<ExternalArgumentsType>);


impl ExternalArguments {
    pub fn new(args: Vec<ExternalArgumentsType>) -> Self {
        Self(args)
    }
}

impl Stringify for ExternalArguments {
    fn to_string(&self) -> String {
        let mut args = String::from("[");
        for (i, arg) in self.0.iter().enumerate() {
            args.push_str(arg.to_string().as_str());
            if i != self.0.len() - 1 {
                args.push_str(", ");
            }
        }

        args.push(']');
        args
    }
}


/// A extern function arguments type declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalArgumentsType {
    Simple(MirageTypeEnum), // Simple type
    Infinite                // Infinite type (...)
}

impl Stringify for ExternalArgumentsType {
    fn to_string(&self) -> String {
        match self {
            Self::Simple(ty) => ty.to_string(),
            Self::Infinite => "...".to_string()
        }
    }
}
