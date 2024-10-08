use std::mem::forget;
use crate::module::Module;
use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};
use crate::util::to_c_str;
use crate::analysis::FailureAction;
use crate::types::{function_types};

use crate::value::{RawValue, Value, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FunctionValue {
    pub(crate) function_value: RawValue,
    pub(crate) function_type: Option<function_types::FunctionType>,
}

impl FunctionValue {
    /// # Safety
    /// This function is unsafe because it does not check if the LLVMValueRef is a valid function value.
    pub unsafe fn new_llvm_ref(lref: LLVMValueRef, tref: Option<LLVMTypeRef>) -> Self {

        let function_type =
            tref.map(
                function_types::FunctionType::new_with_llvm_ref
            )
            ;

        Self {
            function_value: RawValue::new(lref),
            function_type,
        }
    }
    pub fn new_constant(
        function_type: function_types::FunctionType,
        module: Module,
        name: &str,
    ) -> Self {
        let name = to_c_str(name);
        let function_value = unsafe {
            LLVMAddFunction(
                module.module,
                name.as_ptr(),
                function_type.function_type.as_llvm_ref(),
            )
        };
        Self {
            function_value: RawValue::new(function_value),
            function_type: Some(function_type),
        }
    }

    pub fn get_name(&self) -> &str {
        use std::ffi::CStr;
        use std::str;

        unsafe {
            let ptr = LLVMGetValueName2(self.function_value.as_llvm_ref(), std::ptr::null_mut());
            let cstr = CStr::from_ptr(ptr);
            str::from_utf8_unchecked(cstr.to_bytes())
        }
    }

    pub fn get_nth_param(&self, index: u32) -> Option<ValueEnum> {
        let param = unsafe { LLVMGetParam(self.function_value.as_llvm_ref(), index) };
        if param.is_null() {
            None
        } else {
            Some(param.into())
        }
    }
    
    pub fn count_params(&self) -> u32 {
        unsafe { LLVMCountParams(self.function_value.as_llvm_ref()) }
    }

    pub fn get_all_params(&self) -> Vec<ValueEnum> {
        let length = self.count_params() as usize;
        let mut params = Vec::with_capacity(length);
        let ptr = params.as_mut_ptr();
        forget(params);
        let raw_vec = unsafe { 
            LLVMGetParams(self.function_value.as_llvm_ref(), ptr); 
            Vec::from_raw_parts(ptr, length, length)
        };
        
        raw_vec.into_iter().map(|x| x.into()).collect()
        
    }

    pub fn verify(&self, failure_action: FailureAction) -> bool {
        let result = unsafe {
            llvm_sys::analysis::LLVMVerifyFunction(
                self.function_value.as_llvm_ref(),
                failure_action.into()
            )
        };
        result == 0
    }

    pub fn get_raw_function_type(&self) -> Option<LLVMTypeRef> {
        self.function_type.map(|x| x.function_type.as_llvm_ref())
    }

}

impl Value for FunctionValue {

    fn as_raw(&self) -> RawValue {
        self.function_value
    }
    fn is_null_or_undef(&self) -> bool {
        false
    }

    fn is_const(&self) -> bool {
        false
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_undef(&self) -> bool {
        false
    }
}
