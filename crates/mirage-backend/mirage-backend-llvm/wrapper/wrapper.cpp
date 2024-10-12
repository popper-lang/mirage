#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm-c/Core.h>




extern "C" LLVMValueRef LLVMBuildGlobalStringWithModule(LLVMBuilderRef B, LLVMModuleRef M, const char *Str,
                                   const char *Name) {
  return llvm::wrap(llvm::unwrap(B)->CreateGlobalString(Str, Name, 0, llvm::unwrap(M)));
}
