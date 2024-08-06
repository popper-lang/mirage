#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/IRBuilder.h>
#include "llvm-c/Core.h"
#include "llvm/IR/Attributes.h"
#include "llvm/IR/BasicBlock.h"
#include "llvm/IR/ConstantRange.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DebugInfoMetadata.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/DiagnosticInfo.h"
#include "llvm/IR/DiagnosticPrinter.h"
#include "llvm/IR/GlobalAlias.h"
#include "llvm/IR/GlobalVariable.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/InlineAsm.h"
#include "llvm/IR/IntrinsicInst.h"
#include "llvm/IR/LLVMContext.h"



extern "C" LLVMValueRef LLVMBuildGlobalStringWithModule(LLVMBuilderRef B, LLVMModuleRef M, const char *Str,
                                   const char *Name) {
  return llvm::wrap(llvm::unwrap(B)->CreateGlobalString(Str, Name, 0, llvm::unwrap(M)));
}

