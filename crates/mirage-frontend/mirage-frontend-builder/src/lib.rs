#[cfg(test)]
mod test;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;

use mirage_frontend_object::label::{Command, Label, LabelBodyInstr, Value};
use mirage_frontend_object::meta::Flags;
use mirage_frontend_object::statements::{External, Global, ModuleDecl, Statement, Target};
use mirage_frontend_module::Module;
use mirage_frontend_object::function::{FunctionType, FunctionValue};
use mirage_frontend_object::MirageValueEnum;
use mirage_frontend_object::{IntValue, MirageObject, RegisterType, RegisterValue};

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderError {
    NoCurrentBlock,
    GlobalNotFound(String),
    FunctionNotFound(String),
    BlockNotFound(String),
    ExpectConstValue,
    DifferentSize,
    InternalError(String),
    ReturnIsDefined
}

type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug, Clone)]
pub struct Builder {
    pub module: Module,
    pub asts: Vec<Statement>,
}

impl Builder {
    pub fn new(module: Module) -> Self {
        Self {
            module: module.clone(),
            asts: vec![Statement::Module(ModuleDecl::new(module.name))],
        }
    }

    pub fn add(&mut self, ast: Statement) {
        self.asts.push(ast);
    }

    pub fn new_basic_block(&mut self, name: &str) -> BasicBlock {
        BasicBlock::new(name.to_string(), self.clone())
    }

    pub fn set_target_triple(&mut self, os: &str, arch: &str, compiler: &str) {

        if self.asts.len() > 1 && self.asts[1].is_target() {
            self.asts.remove(1);
        }
        self.asts
            .insert(1, Statement::Target(Target::from(os, arch, compiler)));
    }

    pub fn set_target(&mut self, target: &str) {
        if self.asts.len() > 1 && self.asts[1].is_target() {
            self.asts.remove(1);
        }
        self.asts
            .insert(1, Statement::Target(Target::parse(target)));
    }

    pub fn build_extern(&mut self, name: String, func: FunctionType) {
        self.module.add_function(func.fn_value(name.clone()));
        self.asts

            .push(Statement::External(
                External::new(name, func),
            ))
    }

    pub fn build_global(&mut self, name: String, obj: MirageObject) {
        let global = Global::new(name.clone(), obj.clone());
        self.module.add_global(global.clone());
        self.asts
            .push(Statement::Global(Global::new(name, obj)));
    }

    pub fn build_function(&mut self, func: FunctionValue) {
        self.module.add_function(func.clone());
        self.asts.push(Statement::Function(
            func
        ));
    }
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    inner: BasicBlockBuilder,
    name: String,
}

impl BasicBlock {
    pub fn new(name: String, builder: Builder) -> Self {
        Self {
            inner: BasicBlockBuilder::new(
                builder,
                Label::new(name.clone().into(), Flags::new(vec![]), vec![]),
            ),
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn build_copy(&mut self, global_name: &str) -> BuilderResult<MirageValueEnum> {
        self.inner.build_copy(global_name)
    }

    pub fn build_const(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.inner.build_const(val)
    }

    pub fn build_int_add(&mut self, lhs: IntValue, rhs: IntValue) -> BuilderResult<MirageValueEnum> {
        self.inner.build_int_add(lhs, rhs)
    }

    pub fn build_ret(&mut self, val: MirageValueEnum) -> BuilderResult<()> {
        self.inner.build_ret(val)
    }


    pub fn build(&self) -> Label {
        self.inner.block.clone()
    }
}

#[derive(Debug, Clone)]
struct BasicBlockBuilder {
    inner: Box<Builder>,
    index_v: usize,
    index_r: usize,
    block: Label,
    is_return: bool,
}

impl BasicBlockBuilder {
    fn new(inner: Builder, block: Label) -> Self {
        Self {
            inner: Box::new(inner),
            index_v: 0,
            index_r: 0,
            is_return: false,
            block,
        }
    }

    fn build_copy(&mut self, global_name: &str) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        if self.inner.module.get_global(global_name).is_none() {
            return Err(BuilderError::GlobalNotFound(global_name.to_string()));
        }
        let global = self.inner.module.get_global(global_name).unwrap();
        let ty = global.value.get_type();

        let memory = RegisterValue::new(self.index_v, RegisterType::Variable, ty);
        self.index_v += 1;

        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Command(Command::Copy(global_name.into()))),
        ));

        Ok(memory.to_mirage_value())
    }

    pub fn build_const(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        if let Some(c) = val.expect_const_value() {

            let memory = RegisterValue::new(self.index_r, RegisterType::Register, c.get_type());
            self.index_r += 1;
            let const_value = MirageObject::from(c);
            self.block.body.push(LabelBodyInstr::Assign(
                memory.clone(),
                Box::new(LabelBodyInstr::Command(Command::Const(
                    const_value
                ))),
            ));
            Ok(memory.to_mirage_value())
        } else {
            Err(BuilderError::ExpectConstValue)
        }
    }

    fn build_int_add(&mut self, lhs: IntValue, rhs: IntValue) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        let ty = lhs.to_mirage_value().get_type();

        let lhs_value = lhs
            .to_mirage_value()
            .try_into()
            .map_err(|e| BuilderError::InternalError(e))?;

        let rhs_value = rhs
            .to_mirage_value()
            .try_into()
            .map_err(|e| BuilderError::InternalError(e))?;
        let memory= RegisterValue::new(self.index_r, RegisterType::Register, ty);
        self.index_r += 1;

        if lhs.get_max_bits() != rhs.get_max_bits() {
            return Err(BuilderError::DifferentSize);
        }

        let command = match lhs.get_max_bits() {
            8 => {
                Command::AddInt8(
                    lhs_value,
                    rhs_value,
                )
            },
            16 => {
                Command::AddInt16(
                    lhs_value,
                    rhs_value,
                )
            },
            32 => {
                Command::AddInt32(
                    lhs_value,
                    rhs_value,
                )
            },
            64 => {
                Command::AddInt64(
                    lhs_value,
                    rhs_value,
                )
            },
            _ => {
                return Err(BuilderError::DifferentSize);
            }
        };

        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Command(command)),
        ));
        Ok(memory.to_mirage_value())
    }

    pub fn build_ret(&mut self, val: MirageValueEnum) -> BuilderResult<()> {
        if self.is_return {
            return Err(BuilderError::ReturnIsDefined);
        }
        self.is_return = true;
        let value = val.try_into().map_err(|e| BuilderError::InternalError(e))?;
        self.block.body.push(LabelBodyInstr::Command(
            Command::Ret(value)
        ));
        Ok(())
    }

    fn check_return(&mut self) -> BuilderResult<()> {
        if self.is_return {
            Err(BuilderError::ReturnIsDefined)
        } else {
            Ok(())
        }
    }
    
    fn dump(&self) {
        println!("{:?}", self.block);
    }
    
}
