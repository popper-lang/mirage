#[cfg(test)]
mod test;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;

use mirage_frontend_module::Module;
use mirage_frontend_object::function::{FunctionType, FunctionValue};
use mirage_frontend_object::label::{Command, Label, LabelBodyInstr, Value};
use mirage_frontend_object::meta::{Flag, Flags};
use mirage_frontend_object::statements::{External, Global, ModuleDecl, Statement, Target};
use mirage_frontend_object::{IntValue, MirageObject, RegisterType, RegisterValue};
use mirage_frontend_object::{MirageTypeEnum, MirageValueEnum};

#[derive(Debug, Clone, PartialEq)]
pub enum BuilderError {
    NoCurrentBlock,
    GlobalNotFound(String),
    FunctionNotFound(String),
    BlockNotFound(String),
    ExpectConstValue,
    DifferentSize,
    InternalError(String),
    ReturnIsDefined,
}

type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug, Clone)]
pub struct Builder {
    pub module: Module,
    pub asts: Vec<Statement>,
    pub index_g: usize,
}

impl Builder {
    pub fn new(module: Module) -> Self {
        Self {
            module: module.clone(),
            asts: vec![Statement::Module(ModuleDecl::new(module.name))],
            index_g: 0,
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
            .push(Statement::External(External::new(name, func)))
    }

    pub fn build_global(&mut self, obj: MirageObject) -> MirageValueEnum {
        let reg = RegisterValue::new(self.index_g, RegisterType::Global, obj.get_type());
        let global = Global::new(reg.print_to_string(), obj.clone());
        self.module.add_global(global.clone());
        self.asts.push(Statement::Global(Global::new(
            reg.print_to_string(),
            obj.clone(),
        )));
        self.index_g += 1;
        MirageValueEnum::Register(reg)
    }

    pub fn build_function(&mut self, func: FunctionValue) {
        self.module.add_function(func.clone());
        self.asts.push(Statement::Function(func));
    }

    pub fn join_function(&mut self, f: &mut FunctionValue, basic_block: BasicBlock) {
        f.add_label(basic_block.build());
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
                Label::new(name.clone(), Flags::new(vec![]), vec![]),
            ),
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn build_store(&mut self, reg: RegisterValue, value: MirageObject) -> BuilderResult<()> {
        self.inner.build_store(reg, value)
    }

    pub fn build_const(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.inner.build_const(val)
    }

    pub fn build_int_add(
        &mut self,
        lhs: IntValue,
        rhs: IntValue,
    ) -> BuilderResult<MirageValueEnum> {
        self.inner.build_int_add(lhs, rhs)
    }

    pub fn build_int_sub(
        &mut self,
        lhs: IntValue,
        rhs: IntValue,
    ) -> BuilderResult<MirageValueEnum> {
        self.inner.build_int_sub(lhs, rhs)
    }

    pub fn build_call(
        &mut self,
        f_name: String,
        args: Vec<MirageValueEnum>,
    ) -> BuilderResult<MirageValueEnum> {
        self.inner.build_call(f_name, args)
    }

    pub fn build_ret(&mut self, val: MirageValueEnum) -> BuilderResult<()> {
        self.inner.build_ret(val)
    }

    pub fn build_ref(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.inner.build_ref(val)
    }

    pub fn build_load(
        &mut self,
        ty: MirageTypeEnum,
        val: MirageValueEnum,
    ) -> BuilderResult<MirageValueEnum> {
        self.inner.build_load(ty, val)
    }

    pub fn build_getelementptr(
        &mut self,
        real_ty: MirageTypeEnum,
        out: MirageTypeEnum,
        ptr: MirageValueEnum,
        indices: Vec<MirageValueEnum>,
    ) -> BuilderResult<MirageValueEnum> {
        self.inner.build_getelementptr(real_ty, out, ptr, indices)
    }

    pub fn build(&self) -> Label {
        self.inner.block.clone()
    }

    pub fn pop_instr(&mut self) -> Option<LabelBodyInstr> {
        self.inner.pop_instr()
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

    fn build_store(&mut self, reg: RegisterValue, value: MirageObject) -> BuilderResult<()> {
        self.check_return()?;
        self.block.body.push(LabelBodyInstr::Command(Command::Store(
            reg,
            Value::ConstValue(value),
        )));
        Ok(())
    }

    pub fn build_const(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        if let Some(c) = val.expect_const_value() {
            let memory = RegisterValue::new(self.index_r, RegisterType::Register, c.get_type());
            self.index_r += 1;
            let const_value = MirageObject::from(c);
            self.block.body.push(LabelBodyInstr::Assign(
                memory.clone(),
                Box::new(LabelBodyInstr::Command(Command::Const(const_value))),
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
        let memory = RegisterValue::new(self.index_r, RegisterType::Register, ty);
        self.index_r += 1;

        if lhs.get_max_bits() != rhs.get_max_bits() {
            return Err(BuilderError::DifferentSize);
        }

        let command = match lhs.get_max_bits() {
            8 => Command::AddInt8(lhs_value, rhs_value),
            16 => Command::AddInt16(lhs_value, rhs_value),
            32 => Command::AddInt32(lhs_value, rhs_value),
            64 => Command::AddInt64(lhs_value, rhs_value),
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

    fn build_int_sub(&mut self, lhs: IntValue, rhs: IntValue) -> BuilderResult<MirageValueEnum> {
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
        let memory = RegisterValue::new(self.index_r, RegisterType::Register, ty);
        self.index_r += 1;

        if lhs.get_max_bits() != rhs.get_max_bits() {
            return Err(BuilderError::DifferentSize);
        }

        let command = match lhs.get_max_bits() {
            8 => Command::SubInt8(lhs_value, rhs_value),
            16 => Command::SubInt16(lhs_value, rhs_value),
            32 => Command::SubInt32(lhs_value, rhs_value),
            64 => Command::SubInt64(lhs_value, rhs_value),
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

    fn build_call(
        &mut self,
        f_name: String,
        args: Vec<MirageValueEnum>,
    ) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        let args = args
            .iter()
            .cloned()
            /*            .map(|x| {
                            if let MirageValueEnum::Register(e) = x.clone() {
                                if let MirageTypeEnum::Struct(_) = e.get_type() {
                                    let mut e = e;
                                    e.remove_flag(&Flag::not_loadable());
                                    MirageValueEnum::Register(e.clone())
                                } else {
                                    x
                                }
                            } else {
                                x
                            }
                        })
            */
            .map(|x| Value::ConstValue(MirageObject::from(x.clone())))
            .collect::<Vec<Value>>();
        let func = self
            .inner
            .module
            .get_function(&f_name)
            .ok_or(BuilderError::FunctionNotFound(f_name.clone()))?;
        let memory = RegisterValue::new(
            self.index_r,
            RegisterType::Register,
            func.get_type().get_ret().clone(),
        );
        self.index_r += 1;
        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Call(f_name, args.clone())),
        ));
        Ok(memory.to_mirage_value())
    }

    fn build_ret(&mut self, val: MirageValueEnum) -> BuilderResult<()> {
        if self.is_return {
            return Err(BuilderError::ReturnIsDefined);
        }
        self.is_return = true;
        let value = val.try_into().map_err(BuilderError::InternalError)?;
        self.block
            .body
            .push(LabelBodyInstr::Command(Command::Ret(value)));
        Ok(())
    }

    pub fn build_ref(&mut self, val: MirageValueEnum) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        let ty = MirageTypeEnum::type_ptr(val.get_type()).into();
        let memory = RegisterValue::new(self.index_r, RegisterType::Register, ty);
        let val = val.try_into().map_err(BuilderError::InternalError)?;
        self.index_r += 1;
        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Command(Command::Ref(val))),
        ));
        Ok(memory.to_mirage_value())
    }

    fn build_load(
        &mut self,
        ty: MirageTypeEnum,
        val: MirageValueEnum,
    ) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;

        let memory = RegisterValue::new(self.index_r, RegisterType::Register, ty.clone());
        let val = val.try_into().map_err(|x| BuilderError::InternalError(x))?;
        self.index_r += 1;
        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Command(Command::Load(ty, val))),
        ));
        Ok(memory.to_mirage_value())
    }

    fn build_getelementptr(
        &mut self,
        real_ty: MirageTypeEnum,
        out: MirageTypeEnum,
        ptr: MirageValueEnum,
        indices: Vec<MirageValueEnum>,
    ) -> BuilderResult<MirageValueEnum> {
        self.check_return()?;
        let memory = RegisterValue::new(self.index_r, RegisterType::Register, real_ty);
        let ptr = ptr.try_into().map_err(BuilderError::InternalError)?;
        let indices = indices
            .iter()
            .map(|x| x.clone().try_into().unwrap())
            .collect::<Vec<Value>>();
        self.index_r += 1;
        self.block.body.push(LabelBodyInstr::Assign(
            memory.clone(),
            Box::new(LabelBodyInstr::Command(Command::GetElementPtr(
                out,
                ptr,
                indices.clone(),
            ))),
        ));
        Ok(memory.to_mirage_value())
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

    fn pop_instr(&mut self) -> Option<LabelBodyInstr> {
        self.block.body.pop()
    }
}
