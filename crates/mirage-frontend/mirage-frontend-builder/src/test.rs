use mirage_frontend_object::{statements::{Arch, Compiler, Os}, MirageTypeEnum};

use super::*;

#[test]
fn test_module_name() {
    let builder = Builder::new(
        Module::new("name".to_string())
    );

    assert_eq!(
        builder.asts[0],
        Statement::Module(
            ModuleDecl::new("name".to_string())
        )
    );
}

#[test]
fn test_target_triple() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );

    builder.set_target_triple("linux", "x86", "gcc");

    assert_eq!(
        builder.asts[1],
        Statement::Target(
            Target::new(
                Os::Linux, Arch::X86, Compiler::Gcc
            )
        )
    );
}

#[test]
fn test_target() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );

    builder.set_target("linux-x86-gcc");

    assert_eq!(
        builder.asts[1],
        Statement::Target(
            Target::new(
                Os::Linux, Arch::X86, Compiler::Gcc
            )
        )
    );
}

#[test]
fn test_extern() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );
    let fn_type = FunctionType::new(
        vec![
            MirageTypeEnum::type_int32().into(),
            MirageTypeEnum::type_int32().into()
        ],
        MirageTypeEnum::type_int32().into(),
    );

    builder.build_extern("add".to_string(), fn_type.clone());

    assert_eq!(
        builder.asts[1],
        Statement::External(
            External::new(
                "add".to_string(),
                fn_type
            )
        )
    );
}

#[test]
fn test_global() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );
    let value = MirageTypeEnum
        ::type_int32()
        .const_value(12)
        .to_value_enum();
    let obj = MirageObject::from(value.clone());

    builder.build_global("global".to_string(), obj.clone());

    assert_eq!(
        builder.asts[1],
        Statement::Global(
            Global::new(
                "global".to_string(),
                obj
            )
        )
    );
}

#[test]
fn test_function() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );
    let fn_type = FunctionType::new(
        vec![
            MirageTypeEnum::type_int32().into(),
            MirageTypeEnum::type_int32().into()
        ],
        MirageTypeEnum::type_int32().into(),
    );
    let func = fn_type.fn_value("add".to_string());

    builder.build_function(func.clone());

    assert_eq!(
        builder.asts[1],
        Statement::Function(
            func
        )
    );
}

#[test]
fn test_add() {
    let mut builder = Builder::new(
        Module::new("name".to_string())
    );
    let fn_type = FunctionType::new(
        vec![
            MirageTypeEnum::type_int32().into(),
            MirageTypeEnum::type_int32().into()
        ],
        MirageTypeEnum::type_int32().into(),
    );
    let func = fn_type.fn_value("add".to_string());
    let lhs = func.get_nth_arg(0).unwrap();
    let rhs = func.get_nth_arg(1).unwrap();
    let mut basic_block = builder.new_basic_block("entry");
    let res = basic_block.build_int_add(lhs.expect_int_value().unwrap(), rhs.expect_int_value().unwrap()).unwrap();

    builder.append_basic_block(func, basic_block).unwrap();
    assert_eq!(
        res,

    )


}
