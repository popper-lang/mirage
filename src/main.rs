use mirage::backend::opti::optimize;
use mirage::backend::opti::OptiLevel;
use mirage::frontend::builder::Builder;
use mirage::frontend::module::Module;
use mirage::frontend::object::function::FunctionType;
use mirage::frontend::object::stringify::Stringify;
use mirage::frontend::object::MirageTypeEnum;
use mirage::backend::codegen_llvm::Compiler;

fn main() {
    let module = Module::new("test".to_string());
    let mut builder = Builder::new(module);
    builder.set_target_triple("linux", "x86", "gcc");

    let fn_type = FunctionType::new(
        vec![
            MirageTypeEnum::type_int32().into(),
            MirageTypeEnum::type_int32().into(),
        ],
        MirageTypeEnum::type_int32().into(),
        false
    );

    let mut fn_main = fn_type.fn_value("add".to_string());

    let mut basic_block = builder.new_basic_block("entry");

    let zero = MirageTypeEnum
        ::type_int32()
        .const_value(0)
        .to_value_enum();
    
    let one = MirageTypeEnum
        ::type_int32()
        .const_value(1)
        .to_value_enum();

    let c = basic_block
        .build_const(zero)
        .unwrap();
    
    let d = basic_block
        .build_const(one)
        .unwrap();
    
    let e = basic_block
        .build_int_add(c.expect_int_value().unwrap(), d.expect_int_value().unwrap())
        .unwrap();

    basic_block
        .build_ret(e)
        .unwrap();

    fn_main.add_label(basic_block.build());

    builder.build_function(fn_main);
    let asts  = builder.asts;
    println!("{}", asts.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
    let mut compiler = Compiler::new(asts).unwrap();
    
    compiler.compile();
    
    compiler.dump();
}
