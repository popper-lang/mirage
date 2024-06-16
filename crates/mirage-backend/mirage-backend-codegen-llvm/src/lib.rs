use std::collections::HashMap;
use mirage_backend_llvm::builder::{Builder, MathOpType};
use mirage_backend_llvm::context::Context;
use mirage_backend_llvm::module::Module;
use mirage_backend_llvm::types::{Type, TypeBuilder, TypeEnum};
use mirage_backend_llvm::types::struct_type::StructType;
use mirage_backend_llvm::value::ValueEnum;
use mirage_frontend::object::function::FunctionValue;
use mirage_frontend::object::label::{Command, LabelBodyInstr, Value};
use mirage_frontend::object::{MirageObject, MirageTypeEnum, MirageValueEnum, RegisterValue};
use mirage_frontend::object::statements::{External, Statement, TypeDef};
use mirage_frontend::object::stringify::Stringify;

#[derive(Debug)]
pub enum CompilerError {
    InvalidStatement,
    ModuleDeclMissing,
    TargetMissing
}

type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone)]
pub struct Compiler {
    context: Context,
    module: Module,
    builder: Builder,
    stmts: Vec<Statement>,
    env: HashMap<RegisterValue, ValueEnum>,
    struct_env: HashMap<String, StructType>
}

impl Compiler {

    pub fn mirage_ty_to_llvm_ty(&self, ty: MirageTypeEnum) -> TypeEnum {
        match ty {
            MirageTypeEnum::Int8(_) | MirageTypeEnum::UInt8(_) => self.context.i8_type().to_type_enum(),
            MirageTypeEnum::Int16(_) | MirageTypeEnum::UInt16(_) => self.context.i16_type().to_type_enum(),
            MirageTypeEnum::Int32(_) | MirageTypeEnum::UInt32(_) => self.context.i32_type().to_type_enum(),
            MirageTypeEnum::Int64(_) | MirageTypeEnum::UInt64(_) => self.context.i64_type().to_type_enum(),
            MirageTypeEnum::Float32(_) => self.context.float_type().to_type_enum(),
            MirageTypeEnum::Float64(_) => self.context.float_type().to_type_enum(),
        }
    }


    pub fn new(stmts: Vec<Statement>) -> CompilerResult<Self> {
        let context = Context::create();
        if stmts.len() == 0 {
            return Err(CompilerError::ModuleDeclMissing);
        }

        if stmts.len() == 1 {
            return Err(CompilerError::TargetMissing);
        }

        let module_decl = stmts[0].clone();
        let module = match module_decl {
            Statement::Module(m) => context.new_module(&m.name),
            _ => return Err(CompilerError::ModuleDeclMissing),
        };

        let target_triple = stmts[1].clone();
        match target_triple {
            Statement::Target(t) => module.set_target_triple(&t.0.to_str()),
            _ => return Err(CompilerError::TargetMissing),
        }

        let builder = context.new_builder();



        Ok(Self {
            context,
            module,
            builder,
            stmts,
            env: HashMap::new(),
            struct_env: HashMap::new()
        })
    }

    pub fn compile(&mut self) {
        for stmt in self.stmts.clone().iter() {
            self.compile_stmt(stmt);
        }
    }

    pub fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt.clone() {
            Statement::Function(f) => {
                self.compile_function(f);
            },
            Statement::External(e) => {
                self.compile_external(e);
            },
            Statement::Typedef(t) => {
                self.compile_typedef(t);
            },
            _ => {}
        }
    }
    
    pub fn compile_typedef(&mut self, t: TypeDef) {
        let members = t.ty.into_vec().iter().map(|x| self.mirage_ty_to_llvm_ty(x.clone())).collect::<Vec<_>>();
        let struct_ty = self.context.named_struct_type(&t.name);
        struct_ty.set_body(&members, false);
        self.struct_env.insert(t.name, struct_ty);
    }
    
    pub fn compile_external(&mut self, external: External) {
        let fn_ty = external.ty;
        let args: Vec<_> = fn_ty.get_args().iter().map(|ty| self.mirage_ty_to_llvm_ty(ty.clone())).collect();
        let ret = self.mirage_ty_to_llvm_ty(fn_ty.get_ret().clone());
        let fn_ty = ret.func(args, false);
        self.module.add_function(&external.name, fn_ty);
    }
        
        

    pub fn compile_function(&mut self, func: FunctionValue) {
        let args_ty: Vec<_> = func
            .get_type()
            .get_args()
            .iter()
            .map(|ty|
                self.mirage_ty_to_llvm_ty(ty.clone())
            )
            .collect();

        let ret_ty = self.mirage_ty_to_llvm_ty(func.get_type().get_ret().clone());

        let fn_ty = ret_ty.func(args_ty, false);
        let fn_value = self.module.add_function(func.get_name(), fn_ty);
        for label in func.get_labels() {
            let bb = self.context.append_basic_block(&label.name, fn_value);
            self.builder.position_at_end(bb);
            for stmt in &label.body {
                self.compile_instr(bb, stmt);
            }
        }
    }

    pub fn compile_instr(&mut self, bb: mirage_backend_llvm::basic_block::BasicBlock, instr: &LabelBodyInstr) -> Option<ValueEnum> {
        match instr {
            LabelBodyInstr::Command(c) => {
                self.compile_command(c.clone())
            },
            LabelBodyInstr::Assign(r, val) => {
                let val = self.compile_instr(bb, val).unwrap();
                let ty = self.mirage_ty_to_llvm_ty(r.get_type());
                let ptr = self.builder.build_alloca(ty, "");
                self.builder.build_store(val, ptr);

                self.env.insert(r.clone(), ptr.to_value_enum());
                None
            },
            LabelBodyInstr::Call(f, args) => {
                let fn_value = self.module.get_function(f).unwrap();
                let args: Vec<_> = args.iter().map(|arg| self.compile_value(arg)).collect();
                self.builder.build_call(fn_value, &args, &format!("{}.call.", f))
            },
            _ => None
        }
    }

    pub fn compile_command(&mut self, cmd: Command) -> Option<ValueEnum> {
        match cmd {
            Command::New(s, args) => {
                let struct_ty = self.struct_env.get(&s).unwrap().clone();
                let ptr = self.builder.build_alloca(struct_ty.to_type_enum(), "");

                for (i, arg) in args.iter().enumerate() {
                    let val = self.compile_value(arg);
                    let zero = self.context.i32_type().int(0, false);
                    let i = self.context.i32_type().int(i as u64, false);
                    let gep = self.builder.build_get_element_ptr(struct_ty.to_type_enum(), ptr, &[zero, i], "").into_ptr_value();
                    self.builder.build_store(val, gep);
                }

                Some(ptr.to_value_enum())
            },
            Command::Const(v) => {
                Some(self.compile_object(v))
            },
            Command::AddInt8(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            },
            Command::AddInt16(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            },
            Command::AddInt32(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            },
            Command::AddInt64(v1, v2) => {
                let v1 = self.compile_value(&v1).into_int_value();
                let v2 = self.compile_value(&v2).into_int_value();
                Some(self.builder.build_int_add(v1, v2, MathOpType::None, ""))
            },
            Command::AddFloat32(v1, v2) => {
                let v1 = self.compile_value(&v1).into_float_value();
                let v2 = self.compile_value(&v2).into_float_value();
                Some(self.builder.build_float_add(v1, v2,  ""))
            },
            Command::AddFloat64(v1, v2) => {
                let v1 = self.compile_value(&v1).into_float_value();
                let v2 = self.compile_value(&v2).into_float_value();
                Some(self.builder.build_float_add(v1, v2,  ""))
            },

            Command::IncrInt8(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(v, self.context.i8_type().int(1, false), MathOpType::None, ""))
            },
            Command::IncrInt16(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(v, self.context.i16_type().int(1, false), MathOpType::None, ""))
            },
            Command::IncrInt32(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(v, self.context.i32_type().int(1, false), MathOpType::None, ""))
            },
            Command::IncrInt64(v) => {
                let v = self.compile_register_value(v).into_int_value();
                Some(self.builder.build_int_add(v, self.context.i64_type().int(1, false), MathOpType::None, ""))
            },

            Command::IncrFloat32(v) => {
                let v = self.compile_register_value(v).into_float_value();
                Some(self.builder.build_float_add(v, self.context.float_type().float(1.0), ""))
            },
            Command::IncrFloat64(v) => {
                let v = self.compile_register_value(v).into_float_value();
                Some(self.builder.build_float_add(v, self.context.float_type().float(1.0), ""))
            },
            Command::Ret(v) => {
                let v = self.compile_value(&v);
                self.builder.build_ret(Some(v));
                None
            },
            _ => todo!()
        }
    }

    pub fn compile_value(&mut self, val: &Value) -> ValueEnum {
        match val {
            Value::ConstValue(c) => self.compile_object(c.clone()),
            Value::Register(r) => self.compile_register_value(r.clone()),
            _ => todo!()
        }
    }
    pub fn compile_object(&mut self, obj: MirageObject) -> ValueEnum {
        let ty = self.mirage_ty_to_llvm_ty(obj.get_type());
        match obj.get_value() {
            MirageValueEnum::Register(r) => {
                self.compile_register_value(r)
            },
            MirageValueEnum::Int8(v) => self.context.i8_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::Int16(v) => self.context.i16_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::Int32(v) => self.context.i32_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::Int64(v) => self.context.i64_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::UInt8(v) => self.context.i8_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::UInt16(v) => self.context.i16_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::UInt32(v) => self.context.i32_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::UInt64(v) => self.context.i64_type().int(v.value as u64, false).to_value_enum(),
            MirageValueEnum::Float32(v) => self.context.float_type().float(v.value as f64).to_value_enum(),
            MirageValueEnum::Float64(v) => self.context.float_type().float(v.value as f64).to_value_enum(),
        }
    }

    pub fn compile_register_value(&mut self, val: RegisterValue) -> ValueEnum {
        let ty = self.mirage_ty_to_llvm_ty(val.get_type());
        let ptr = self.env.get(&val).unwrap().into_ptr_value();
        self.builder.build_load(ty, ptr, "")
    }
    
    pub fn dump(&self) {
        self.module.dump();
    }
    

}