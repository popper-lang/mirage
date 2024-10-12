use std::env::set_var;
use std::env::var;
use std::fs;
use std::path::Path;
use std::process::Command;

static LLVM_VERSION: u64 = 180;
fn main() {
    let llvm_prefix = if let Ok(llvm_config) = var("MIRAGE_LLVM_PREFIX") {
        llvm_config
    } else {
        String::from_utf8(
            Command::new("llvm-config")
                .arg("--prefix")
                .output()
                .unwrap()
                .stdout
                .to_vec(),
        )
        .unwrap()
        .trim()
        .to_string()
    };
    // set_var(
    //     format!("LLVM_SYS_{LLVM_VERSION}_PREFIX"),
    //     llvm_prefix.clone(),
    // );

    let path = Path::new(&llvm_prefix);
    let include_path = if let Ok(include_path) = var("MIRAGE_LLVM_INCLUDE_PATH") {
        include_path
    } else {
        path.join("include/").to_str().unwrap().to_string()
    };

    cc::Build::new()
        .include(include_path)
        .cpp(true)
        .flag("-std=c++20")
        .file("wrapper/wrapper.cpp")
        .compile("wrapper");
    println!("cargo:rerun-if-changed=build.rs");
}
