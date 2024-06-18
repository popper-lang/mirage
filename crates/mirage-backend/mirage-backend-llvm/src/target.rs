use std::ffi::{CStr, CString};
use std::io::Write;
use llvm_sys::prelude::LLVMPassManagerRef;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use crate::module::Module;
use crate::pass_manager::PassManager;
use crate::util::to_c_str;

#[derive(Debug, Copy, Clone)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

impl From<LLVMCodeGenOptLevel> for OptimizationLevel {
    fn from(level: LLVMCodeGenOptLevel) -> Self {
        match level {
            LLVMCodeGenOptLevel::LLVMCodeGenLevelNone => OptimizationLevel::None,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelLess => OptimizationLevel::Less,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault => OptimizationLevel::Default,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive => OptimizationLevel::Aggressive,
        }
    }
}

impl Into<LLVMCodeGenOptLevel> for OptimizationLevel {
    fn into(self) -> LLVMCodeGenOptLevel {
        match self {
            OptimizationLevel::None => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
            OptimizationLevel::Less => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
            OptimizationLevel::Default => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            OptimizationLevel::Aggressive => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RelocMode {
    Default,
    Static,
    PIC,
    DynamicNoPic,
    ROPI,
    RWPI,
    ROPI_RWPI
}

impl From<LLVMRelocMode> for RelocMode {
    fn from(value: LLVMRelocMode) -> Self {
        match value {
            LLVMRelocMode::LLVMRelocDefault => RelocMode::Default,
            LLVMRelocMode::LLVMRelocStatic => RelocMode::Static,
            LLVMRelocMode::LLVMRelocPIC => RelocMode::PIC,
            LLVMRelocMode::LLVMRelocDynamicNoPic => RelocMode::DynamicNoPic,
            LLVMRelocMode::LLVMRelocROPI => RelocMode::ROPI,
            LLVMRelocMode::LLVMRelocRWPI => RelocMode::RWPI,
            LLVMRelocMode::LLVMRelocROPI_RWPI => RelocMode::ROPI_RWPI
        }
    }
}

impl Into<LLVMRelocMode> for RelocMode {
    fn into(self) -> LLVMRelocMode {
        match self { 
            RelocMode::Default => LLVMRelocMode::LLVMRelocDefault,
            RelocMode::Static => LLVMRelocMode::LLVMRelocStatic,
            RelocMode::PIC => LLVMRelocMode::LLVMRelocPIC,
            RelocMode::DynamicNoPic => LLVMRelocMode::LLVMRelocDynamicNoPic,
            RelocMode::ROPI => LLVMRelocMode::LLVMRelocROPI,
            RelocMode::RWPI => LLVMRelocMode::LLVMRelocRWPI,
            RelocMode::ROPI_RWPI => LLVMRelocMode::LLVMRelocROPI_RWPI
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CodeModel {
    Default,
    JITDefault,
    Tiny,
    Small,
    Kernel,
    Medium,
    Large
}

impl From<LLVMCodeModel> for CodeModel {
    fn from(value: LLVMCodeModel) -> Self {
        match value {
            LLVMCodeModel::LLVMCodeModelDefault => CodeModel::Default,
            LLVMCodeModel::LLVMCodeModelJITDefault => CodeModel::JITDefault,
            LLVMCodeModel::LLVMCodeModelTiny => CodeModel::Tiny,
            LLVMCodeModel::LLVMCodeModelSmall => CodeModel::Small,
            LLVMCodeModel::LLVMCodeModelKernel => CodeModel::Kernel,
            LLVMCodeModel::LLVMCodeModelMedium => CodeModel::Medium,
            LLVMCodeModel::LLVMCodeModelLarge => CodeModel::Large
        }
    }
}

impl Into<LLVMCodeModel> for CodeModel {
    fn into(self) -> LLVMCodeModel {
        match self {
            CodeModel::Default => LLVMCodeModel::LLVMCodeModelDefault,
            CodeModel::JITDefault => LLVMCodeModel::LLVMCodeModelJITDefault,
            CodeModel::Tiny => LLVMCodeModel::LLVMCodeModelTiny,
            CodeModel::Small => LLVMCodeModel::LLVMCodeModelSmall,
            CodeModel::Kernel => LLVMCodeModel::LLVMCodeModelKernel,
            CodeModel::Medium => LLVMCodeModel::LLVMCodeModelMedium,
            CodeModel::Large => LLVMCodeModel::LLVMCodeModelLarge
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CodeGenFileType {
    AssemblyFile,
    ObjectFile,
}

impl From<LLVMCodeGenFileType> for CodeGenFileType {
    fn from(value: LLVMCodeGenFileType) -> Self {
        match value {
            LLVMCodeGenFileType::LLVMAssemblyFile => CodeGenFileType::AssemblyFile,
            LLVMCodeGenFileType::LLVMObjectFile => CodeGenFileType::ObjectFile,
        }
    }
}

impl Into<LLVMCodeGenFileType> for CodeGenFileType {
    fn into(self) -> LLVMCodeGenFileType {
        match self {
            CodeGenFileType::AssemblyFile => LLVMCodeGenFileType::LLVMAssemblyFile,
            CodeGenFileType::ObjectFile => LLVMCodeGenFileType::LLVMObjectFile,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Target {
    pub target: LLVMTargetRef,
}

impl Target {
    pub fn init() {
        unsafe {
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmPrinters();
            LLVM_InitializeAllAsmParsers();
        }
    }

    pub fn get_default_target_triple() -> String {
        unsafe {
            let s = LLVMGetDefaultTargetTriple();
            CStr::from_ptr(s as *const _).to_str().unwrap().to_string()
        }
    }
    
    pub fn create_from_default_target_triple() -> Self {
        unsafe {
            let triple = Self::get_default_target_triple();
            let triple = to_c_str(&triple);
            let mut target = std::mem::MaybeUninit::uninit();
            let mut err_msg = std::mem::MaybeUninit::uninit();
            let res = LLVMGetTargetFromTriple(triple.as_ptr(), target.as_mut_ptr(), err_msg.as_mut_ptr());
            
            if res != 1 {
                let err_msg = err_msg.assume_init();
                let err_msg = CStr::from_ptr(err_msg as *const _).to_str().unwrap();
                
                panic!("A error happnened: {}", err_msg);
            }
            
            let target = target.assume_init();
            Target::new(target)
        }
    }
    pub fn new(target: LLVMTargetRef) -> Self {
        Self {
            target,
        }
    }

    pub fn get_target_name(&self) -> String {
        let name = unsafe { LLVMGetTargetName(self.target) };
        let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
        name.to_string()
    }

    pub fn create_target_machine(&self,
                                 triple: &str,
                                 cpu: &str,
                                 features: &str,
                                 opt_level: OptimizationLevel,
                                 reloc_mode: RelocMode,
                                 code_model: CodeModel) -> TargetMachine {

        let triple =  CString::new(triple).unwrap();
        let cpu = CString::new(cpu).unwrap();
        let features = CString::new(features).unwrap();
        let target_machine = unsafe {
            LLVMCreateTargetMachine(
                self.target,
                triple.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                opt_level.into(),
                reloc_mode.into(),
                code_model.into()
            )
        };
        TargetMachine::new(target_machine)
    }

}
#[derive(Debug, Copy, Clone)]
pub struct TargetMachine {
    target_machine: LLVMTargetMachineRef,
}

impl TargetMachine {
    pub fn new(target_machine: LLVMTargetMachineRef) -> Self {
        Self {
            target_machine,
        }
    }

    pub fn get_target_machine_ref(&self) -> LLVMTargetMachineRef {
        self.target_machine
    }

    pub fn get_target(&self) -> Target {
        let target = unsafe { LLVMGetTargetMachineTarget(self.target_machine) };
        Target::new(target)
    }


    pub fn get_target_triple(&self) -> String {
        let triple = unsafe { LLVMGetTargetMachineTriple(self.target_machine) };
        let triple = unsafe { CStr::from_ptr(triple).to_str().unwrap() };
        triple.to_string()
    }

    pub fn get_target_cpu(&self) -> String {
        let cpu = unsafe { LLVMGetTargetMachineCPU(self.target_machine) };
        let cpu = unsafe { CStr::from_ptr(cpu).to_str().unwrap() };
        cpu.to_string()
    }

    pub fn get_target_features(&self) -> String {
        let features = unsafe { LLVMGetTargetMachineFeatureString(self.target_machine) };
        let features = unsafe { CStr::from_ptr(features).to_str().unwrap() };
        features.to_string()
    }

    pub fn create_data_layout(&self) -> TargetData {
        let data_layout = unsafe { LLVMCreateTargetDataLayout(self.target_machine) };
        TargetData::new(data_layout)
    }

    pub fn emit_file(&self, module: Module, dest: String, file_type: CodeGenFileType) {
        let mut dest = dest.clone();
        let c_str = dest.as_mut_ptr();
        unsafe {
            let mut err = std::mem::MaybeUninit::uninit();
            let res = LLVMTargetMachineEmitToFile(self.target_machine, module.module, c_str as *mut _, file_type.into(), err.as_mut_ptr());
            if res == 0 {
                let err = err.assume_init();
                let s = CStr::from_ptr(err as *const _).to_str().unwrap();
                panic!("Failed to emit file: {}", s);
            }
        };

    }
}

#[derive(Debug, Copy, Clone)]
pub struct TargetData {
    target_data: LLVMTargetDataRef,
}

impl TargetData {
    pub fn new(target_data: LLVMTargetDataRef) -> Self {
        Self {
            target_data,
        }
    }
}