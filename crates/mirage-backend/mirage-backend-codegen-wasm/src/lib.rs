use std::cell::RefCell;
use std::rc::Rc;
use mirage_frontend::object::function::FunctionValue;
use mirage_frontend::object::label::Label;
use mirage_frontend::object::MirageTypeEnum;
use mirage_frontend::object::statements::Statement;


#[derive(Debug, Clone)]
pub struct Compiler {
    stmts: Vec<Statement>,
    module: Rc<RefCell<walrus::Module>>,
    fn_builder: Option<Rc<walrus::FunctionBuilder>>,
}


impl Compiler {
    pub fn new(stmts: Vec<Statement>) -> Self {
        let config = walrus::ModuleConfig::new();
        let module = walrus::Module::with_config(config);
        Self {
            stmts,
            module: Rc::new(
                RefCell::new(module)
            ),
            fn_builder: None,
        }
    }
    
    pub fn mirage_ty_to_wasm_ty(&self, ty: MirageTypeEnum) -> walrus::ValType {
        match ty {
            MirageTypeEnum::Int8(..) | MirageTypeEnum::UInt8(_) |
            MirageTypeEnum::Int16(..) | MirageTypeEnum::UInt16(_) |
            MirageTypeEnum::Int32(..) | MirageTypeEnum::UInt32(_) => walrus::ValType::I32,
            MirageTypeEnum::Int64(..) | MirageTypeEnum::UInt64(_) => walrus::ValType::I64,
            MirageTypeEnum::Float32(..) => walrus::ValType::F32,
            MirageTypeEnum::Float64(..) => walrus::ValType::F64,
            MirageTypeEnum::Array(..) => todo!(),
            MirageTypeEnum::Pointer(..) => todo!(),
        }
        
    }
    
    pub fn compile(&mut self) {
        for stmt in self.stmts.clone() {
            match stmt {
                Statement::Function(ref f) => {
                    self.compile_function(f);
                }
                _ => {}
            }
        }
    }
    
    pub fn compile_function(&mut self, f: &FunctionValue) {
        let fn_ty = f.get_type();
        let ret_ty = self.mirage_ty_to_wasm_ty(fn_ty.get_ret().clone());
        let args_ty = fn_ty
            .get_args()
            .iter()
            .map(|x| self.mirage_ty_to_wasm_ty(x.clone()))
            .collect::<Vec<_>>();
        
        let fn_builder = walrus::FunctionBuilder::new(&mut self.module.borrow_mut().types, &args_ty, &[ret_ty]);
        self.fn_builder = Some(Rc::new(fn_builder));
        
        for label in f.get_labels() {
            self.compile_label(label.clone());
        }
        
    }
    
    pub fn compile_label(&mut self, label: Label) {
        todo!()
    }


}