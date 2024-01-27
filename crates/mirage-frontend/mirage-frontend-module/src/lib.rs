use mirage_frontend_object::function::FunctionValue;
use mirage_frontend_object::statements::Global;



#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: Vec<FunctionValue>,
    pub globals: Vec<Global>
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            globals: Vec::new()
        }
    }

    pub fn add_function(&mut self, function: FunctionValue) {
        self.functions.push(function);
    }

    pub fn add_global(&mut self, global: Global) {
        self.globals.push(global);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionValue> {
        self.functions.iter().find(|x| x.get_name() == name)
    }

    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut FunctionValue> {
        self.functions.iter_mut().find(|x| x.get_name() == name)
    }

    pub fn get_global(&self, name: &str) -> Option<&Global> {
        self.globals.iter().find(|x| x.name == name)
    }
}
