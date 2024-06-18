use llvm_sys::prelude::*;
use llvm_sys::core::*;

#[derive(Debug, Copy, Clone)]
pub struct PassManager {
    pass_manager: LLVMPassManagerRef
}

impl PassManager {
    
    pub fn create() -> Self {
        let pass_manager = unsafe { LLVMCreatePassManager() };
        Self { 
            pass_manager
        }
    }
    
    pub fn new(pass_manager: LLVMPassManagerRef) -> Self {
        Self {
            pass_manager
        }
    }
    
    pub fn as_llvm_ref(&self) -> LLVMPassManagerRef {
        self.pass_manager
    }
}