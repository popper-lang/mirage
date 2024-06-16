use std::env::var;
use std::env::set_var;

static LLVM_VERSION: u64 = 180;
fn main() {
    if let Ok(llvm_config) = var("MIRAGE_LLVM_PREFIX") {
        set_var(format!("LLVM_SYS_{}_PREFIX", LLVM_VERSION), llvm_config);
    }

}