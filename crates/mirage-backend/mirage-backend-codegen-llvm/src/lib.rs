mod string;

use mirage_backend_llvm::builder::{Builder, MathOpType};
use mirage_backend_llvm::context::Context;
use mirage_backend_llvm::execution_engine::ExecutionEngine;
use mirage_backend_llvm::module::Module;
use mirage_backend_llvm::target::{
    CodeGenFileType, CodeModel, OptimizationLevel, RelocMode, Target,
};
use mirage_backend_llvm::types::struct_type::StructType;
use mirage_backend_llvm::types::{Type, TypeBuilder, TypeEnum};
use mirage_backend_llvm::value::function_value::FunctionValue as LLVMFunctionValue;
use mirage_backend_llvm::value::ValueEnum;
use mirage_backend_output::{CompilerOutput, ExecutionEngineOutput, ObjectOutput};
use mirage_frontend::object::function::FunctionValue;
use mirage_frontend::object::label::{Command, LabelBodyInstr, Value};
use mirage_frontend::object::meta::Flag;
use mirage_frontend::object::statements::{External, Statement, TypeDef};
use mirage_frontend::object::{
    MirageObject, MirageTypeEnum, MirageValueEnum, RegisterType, RegisterValue,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Arc;

/// A compiler error
/// # Variants
/// * `InvalidStatement` - Invalid statement
/// * `ModuleDeclMissing` - Module declaration missing
/// * `TargetMissing` - Target missing
#[derive(Debug)]
pub enum CompilerError {
    InvalidStatement,
    ModuleDeclMissing,
    TargetMissing,
}

type CompilerResult<T> = Result<T, CompilerError>;

/// The LLVM Compiler struct
#[derive(Debug, Clone)]
pub struct Compiler {
    context: Context,
    module: Module,
    builder: Builder,
    stmts: Vec<Statement>,
    env: HashMap<RegisterValue, ValueEnum>,
    fn_env: HashMap<String, LLVMFunctionValue>,
    struct_env: HashMap<String, StructType>,
    index_g: usize,
    no_store: bool,
    debug: bool,
}

impl Compiler {
    /// Convert Mirage type to LLVM type
    /// # Arguments
    /// * `ty` - Mirage type
    /// # Returns
    /// * LLVM type
    pub fn mirage_ty_to_llvm_ty(&self, ty: MirageTypeEnum) -> TypeEnum {
        match ty {
            MirageTypeEnum::Int8(_) | MirageTypeEnum::UInt8(_) => {
                self.context.i8_type().to_type_enum()
            }
            MirageTypeEnum::Int16(_) | MirageTypeEnum::UInt16(_) => {
                self.context.i16_type().to_type_enum()
            }
            MirageTypeEnum::Int32(_) | MirageTypeEnum::UInt32(_) => {
                self.context.i32_type().to_type_enum()
            }
            MirageTypeEnum::Int64(_) | MirageTypeEnum::UInt64(_) => {
                self.context.i64_type().to_type_enum()
            }
            MirageTypeEnum::Float32(_) => self.context.float_type().to_type_enum(),
            MirageTypeEnum::Float64(_) => self.context.float_type().to_type_enum(),
            MirageTypeEnum::Array(t) => {
                let ta: MirageTypeEnum = t.clone().into();
                if ta.is_string() {
                    return self.context.i8_type().ptr().to_type_enum();
                }
                let element_ty = self.mirage_ty_to_llvm_ty(t.element_ty());
                let length = t.length();
                element_ty.array(length as u64).to_type_enum()
            }
            MirageTypeEnum::Pointer(t) => {
                let ty = self.mirage_ty_to_llvm_ty(*t.element_ty.clone());
                ty.ptr().to_type_enum()
            }
            MirageTypeEnum::Struct(s) => {
                let struct_elt = s
                    .fields
                    .iter()
                    .map(|x| self.mirage_ty_to_llvm_ty(x.clone()))
                    .collect::<Vec<_>>();
                let struct_ty = self.context.struct_type(&struct_elt, false);
                struct_ty.to_type_enum()
            }
        }
    }

    /// Create a new compiler
    pub fn new(stmts: Vec<Statement>, debug: bool) -> CompilerResult<Self> {
        let context = Context::create();
        if stmts.len() == 0 {
            return Err(CompilerError::ModuleDeclMissing);
        }

        let module_decl = stmts[0].clone();
        let module = match module_decl {
            Statement::Module(m) => context.new_module(&m.name),
            _ => return Err(CompilerError::ModuleDeclMissing),
        };

        let builder = context.new_builder(module);

        Ok(Self {
            context,
            module,
            builder,
            stmts,
            env: HashMap::new(),
            struct_env: HashMap::new(),
            fn_env: HashMap::new(),
            index_g: 0,
            no_store: false,
            debug,
        })
    }

    /// Compile the module
    pub fn compile(&mut self) {
        for stmt in self.stmts.clone().iter() {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt.clone() {
            Statement::Function(f) => {
                self.compile_function(f);
            }
            Statement::External(e) => {
                self.compile_external(e);
            }
            Statement::Typedef(t) => {
                self.compile_typedef(t);
            }
            Statement::Global(global) => {
                let obj = global.value.clone();
                if !obj.get_type().is_string() {
                    panic!("Global value who arent string doesn't work right now")
                }
                let s = global.value.get_value().try_to_rust_string().unwrap();
                let s = string::to_string_with_special_char(&s);
                let s = self.builder.build_global_string(&global.name, &s);
                let reg = RegisterValue::new(self.index_g, RegisterType::Global, obj.get_type());
                self.index_g += 1;
                self.env.insert(reg, s);
            }
            _ => {}
        }
    }

    fn compile_typedef(&mut self, t: TypeDef) {
        let members =
            t.ty.into_vec()
                .iter()
                .map(|x| self.mirage_ty_to_llvm_ty(x.clone()))
                .collect::<Vec<_>>();
        let struct_ty = self.context.named_struct_type(&t.name);
        struct_ty.set_body(&members, false);
        self.struct_env.insert(t.name, struct_ty);
    }

    fn compile_external(&mut self, external: External) {
        let fn_ty = external.ty;
        let args: Vec<_> = fn_ty
            .get_args()
            .iter()
            .map(|ty| self.mirage_ty_to_llvm_ty(ty.clone()))
            .collect();
        let ret = self.mirage_ty_to_llvm_ty(fn_ty.get_ret().clone());
        let fn_ty = ret.func(args, fn_ty.is_var_arg());
        let f = self.module.add_function(&external.name, fn_ty);
        self.fn_env.insert(external.name, f);
    }

    fn compile_function(&mut self, func: FunctionValue) {
        let args_ty: Vec<_> = func
            .get_type()
            .get_args()
            .iter()
            .map(|ty| self.mirage_ty_to_llvm_ty(ty.clone()))
            .collect();

        let ret_ty = self.mirage_ty_to_llvm_ty(func.get_type().get_ret().clone());

        let fn_ty = ret_ty.func(args_ty, func.get_type().is_var_arg());
        let fn_value = self.module.add_function(func.get_name(), fn_ty);
        self.fn_env.insert(func.get_name().clone(), fn_value);
        for label in func.get_labels() {
            let bb = self.context.append_basic_block(&label.name, fn_value);
            self.builder.position_at_end(bb);
            for stmt in &label.body {
                self.compile_instr(bb, stmt);
            }
        }
    }

    fn compile_instr(
        &mut self,
        bb: mirage_backend_llvm::basic_block::BasicBlock,
        instr: &LabelBodyInstr,
    ) -> Option<ValueEnum> {
        match instr {
            LabelBodyInstr::Command(c) => self.compile_command(c.clone()),
            LabelBodyInstr::Assign(r, val) => {
                let val = self.compile_instr(bb, val).unwrap();
                if self.no_store {
                    self.no_store = false;
                    self.env.insert(r.clone(), val);
                    return Some(val);
                }
                let ty = self.mirage_ty_to_llvm_ty(r.get_type());
                let ptr = self.builder.build_alloca(ty, "");
                self.builder.build_store(val, ptr);

                self.env.insert(r.clone(), ptr.to_value_enum());
                None
            }
            LabelBodyInstr::Call(f, args) => {
                let fn_value = self.fn_env.get(f).unwrap().clone();
                let args: Vec<_> = args.iter().map(|arg| self.compile_value(arg)).collect();
                self.builder.build_call(fn_value, &args, "")
            }
            _ => None,
        }
    }

    fn compile_command(&mut self, cmd: Command) -> Option<ValueEnum> {
        match cmd {
            Command::New(s, args) => {
                let struct_ty = self.struct_env.get(&s).unwrap().clone();
                let ptr = self.builder.build_alloca(struct_ty.to_type_enum(), "");

                for (i, arg) in args.iter().enumerate() {
                    let val = self.compile_value(arg);
                    let zero = self.context.i32_type().int(0, false);
                    let i = self.context.i32_type().int(i as u64, false);
                    let gep = self
                        .builder
                        .build_get_element_ptr(struct_ty.to_type_enum(), ptr, &[zero, i], "")
                        .into_ptr_value();
                    self.builder.build_store(val, gep);
                }

                Some(ptr.to_value_enum())
            }

            Command::Store(r, v) => {
                let r = self.compile_register_value(r);
                let v = self.compile_value(&v);
                self.builder.build_store(v, r.into_ptr_value());
                None
            }
            Command::Const(v) => Some(self.compile_object(v)),
            Command::AddInt8(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            }
            Command::AddInt16(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            }
            Command::AddInt32(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            }
            Command::AddInt64(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            }
            Command::AddFloat32(v1, v2) => {
                let v1 = self.compile_value(&v1).into_float_value();
                let v2 = self.compile_value(&v2).into_float_value();
                Some(self.builder.build_float_add(v1, v2, ""))
            }
            Command::AddFloat64(v1, v2) => {
                let v1 = self.compile_value(&v1).into_float_value();
                let v2 = self.compile_value(&v2).into_float_value();
                Some(self.builder.build_float_add(v1, v2, ""))
            }

            Command::SubInt8(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_sub(v1, v2, MathOpType::None, ""))
            }
            Command::SubInt16(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_sub(v1, v2, MathOpType::None, ""))
            }
            Command::SubInt32(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_sub(v1, v2, MathOpType::None, ""))
            }
            Command::SubInt64(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_sub(v1, v2, MathOpType::None, ""))
            }

            Command::IncrInt8(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(
                    v,
                    self.context.i8_type().int(1, false),
                    MathOpType::None,
                    "",
                ))
            }
            Command::IncrInt16(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(
                    v,
                    self.context.i16_type().int(1, false),
                    MathOpType::None,
                    "",
                ))
            }
            Command::IncrInt32(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(
                    v,
                    self.context.i32_type().int(1, false),
                    MathOpType::None,
                    "",
                ))
            }
            Command::IncrInt64(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(
                    v,
                    self.context.i64_type().int(1, false),
                    MathOpType::None,
                    "",
                ))
            }

            Command::IncrFloat32(v) => {
                let v = self.compile_register_value(v).into_float_value();
                Some(
                    self.builder
                        .build_float_add(v, self.context.float_type().float(1.0), ""),
                )
            }
            Command::IncrFloat64(v) => {
                let v = self.compile_register_value(v).into_float_value();
                Some(
                    self.builder
                        .build_float_add(v, self.context.float_type().float(1.0), ""),
                )
            }
            Command::Ret(v) => {
                let v = self.compile_value(&v);
                self.builder.build_ret(Some(v));
                None
            }
            Command::Ref(v) => {
                let value = self.compile_value(&v);
                let ty = v.get_type();
                let ty = self.mirage_ty_to_llvm_ty(ty);
                let ptr = self.builder.build_alloca(ty, "");

                self.builder.build_store(value, ptr);
                Some(ptr.to_value_enum())
            }
            Command::Load(ty, v) => {
                let value = self.compile_value(&v);
                let ty = self.mirage_ty_to_llvm_ty(ty);

                Some(self.builder.build_load(ty, value.into_ptr_value(), ""))
            }
            Command::GetElementPtr(t, v, l) => {
                let ty = self.mirage_ty_to_llvm_ty(t);
                let ptr = self.compile_value(&v);
                let indices = l
                    .iter()
                    .map(|x| self.compile_value(x).into_int_value())
                    .collect::<Vec<_>>();
                self.no_store = true;
                Some(
                    self.builder
                        .build_get_element_ptr(ty, ptr.into_ptr_value(), &indices, ""),
                )
            }
            _ => todo!(),
        }
    }

    fn compile_value(&mut self, val: &Value) -> ValueEnum {
        match val {
            Value::ConstValue(c) => self.compile_object(c.clone()),
            Value::Register(r) => self.compile_register_value(r.clone()),
            _ => todo!(),
        }
    }
    fn compile_object(&mut self, obj: MirageObject) -> ValueEnum {
        match obj.get_value() {
            MirageValueEnum::Register(r) => self.compile_register_value(r),
            MirageValueEnum::Int8(v) => self
                .context
                .i8_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::Int16(v) => self
                .context
                .i16_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::Int32(v) => self
                .context
                .i32_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::Int64(v) => self
                .context
                .i64_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::UInt8(v) => self
                .context
                .i8_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::UInt16(v) => self
                .context
                .i16_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::UInt32(v) => self
                .context
                .i32_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::UInt64(v) => self
                .context
                .i64_type()
                .int(v.value as u64, false)
                .to_value_enum(),
            MirageValueEnum::Float32(v) => self
                .context
                .float_type()
                .float(v.value as f64)
                .to_value_enum(),
            MirageValueEnum::Float64(v) => self.context.float_type().float(v.value).to_value_enum(),
            MirageValueEnum::Array(a) => {
                let elts = a
                    .values
                    .iter()
                    .map(|x| self.compile_object(MirageObject::new(x.clone(), a.ty.clone().into())))
                    .collect::<Vec<_>>();

                let ty = self.mirage_ty_to_llvm_ty(a.ty.clone().into());
                ty.into_array_type().const_array(&elts).to_value_enum()
            }
            MirageValueEnum::Pointer(_) => panic!("A pointer is not a constant value"),
            MirageValueEnum::Struct(s) => {
                let elts = s
                    .values
                    .iter()
                    .map(|x| self.compile_object(MirageObject::from(x.clone())))
                    .collect::<Vec<_>>();
                let ty = self.mirage_ty_to_llvm_ty(s.ty.into());
                let alloc = self.builder.build_alloca(ty, "");
                for (i, elt) in elts.iter().enumerate() {
                    let zero = self.context.i32_type().int(0, false);
                    let i = self.context.i32_type().int(i as u64, false);
                    let gep = self
                        .builder
                        .build_get_element_ptr(ty, alloc, &[zero, i], "")
                        .into_ptr_value();
                    self.builder.build_store(elt.clone(), gep);
                }

                alloc.to_value_enum()
            }
        }
    }

    fn compile_register_value(&mut self, val: RegisterValue) -> ValueEnum {
        let ty = self.mirage_ty_to_llvm_ty(val.get_type());

        let ptr = self
            .env
            .get(self.env.keys().find(|x| x == &&val).unwrap())
            .unwrap()
            .into_ptr_value();
        if val.ty.is_string() || val.contains_flag(&Flag::not_loadable()) {
            return ptr.to_value_enum();
        }
        self.builder.build_load(ty, ptr, "")
    }

    pub fn dump(&self) {
        self.module.dump();
    }

    pub fn print_to_string(&self) -> String {
        self.module.print_to_string()
    }
}

impl CompilerOutput for Compiler {
    fn object<'a>(&mut self) -> ObjectOutput<'a> {
        todo!()
    }

    fn execution_engine(&mut self) -> impl ExecutionEngineOutput {
        self.clone()
    }
}

impl ExecutionEngineOutput for Compiler {
    fn get_function<T: Copy + Sized>(&mut self, name: &str) -> T {
        let execution_engine = ExecutionEngine::new_with_module(&self.module);

        execution_engine.get_function(name)
    }
}

