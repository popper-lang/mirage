use std::ffi::c_char;
use llvm_sys::prelude::*;

extern "C" {
    pub fn LLVMBuildGlobalStringWithModule(
        builder: LLVMBuilderRef,
        module: LLVMModuleRef,
        s: *const c_char,
        name: *const c_char,
    ) -> LLVMValueRef;
}