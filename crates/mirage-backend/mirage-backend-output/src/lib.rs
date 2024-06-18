
pub mod object {
    pub use object::*;
}

use std::fmt::Debug;
use std::sync::Arc;
use object::*;


pub trait CompilerOutput {

    fn object<'a>(&mut self) -> ObjectOutput<'a>;
    fn execution_engine(&mut self) -> impl ExecutionEngineOutput;

}

pub trait ExecutionEngineOutput {
    fn get_function<T: Copy + Sized>(&mut self, name: &str) -> T;
}

pub struct ObjectOutput<'a> {
    file: Arc<File<'a>>,
}

impl<'a> ObjectOutput<'a> {
    pub fn new(file: Arc<File<'a>>) -> Self {
        Self {
            file,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for section in self.file.sections() {
            let section_name = section.name().unwrap();
            let section_data = section.data().unwrap();
            bytes.extend_from_slice(section_name.as_bytes());
            bytes.extend_from_slice(section_data);
        }

        bytes
    }

    pub fn write_to(&self, file: &str) -> std::io::Result<()> {
        std::fs::write(file, self.to_bytes())
    }

    pub fn get_sections(&self) -> Vec<Section> {
        self.file.sections().collect()
    }
    
}

