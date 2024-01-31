use mirage::backend::opti::optimize;
use mirage::backend::opti::OptiLevel;
use mirage::frontend::builder::Builder;
use mirage::frontend::module::Module;
use mirage::frontend::object::function::FunctionType;
use mirage::frontend::object::stringify::Stringify;
use mirage::frontend::object::MirageTypeEnum;

fn main() {
    let mut module = Module::new("test".to_string());
    let mut builder = Builder::new(module);

    let fn_type = FunctionType::new(
        vec![
            MirageTypeEnum::type_int32().into(),
            MirageTypeEnum::type_int32().into(),
        ],
        MirageTypeEnum::type_int32().into(),
    );

    let mut fn_main = fn_type.fn_value("add".to_string());
    let lhs = fn_main.get_nth_arg(0).unwrap().expect_int_value().unwrap();
    let rhs = fn_main
        .get_nth_arg(1)
        .unwrap()
        .expect_register_value()
        .unwrap();
    let mut basic_block = builder.new_basic_block("entry");
    let zero = MirageTypeEnum::type_int32().const_value(0).to_value_enum();
    let c = basic_block.build_const(zero).unwrap();
    basic_block.build_ret(c).unwrap();
    fn_main.add_label(basic_block.build());

    builder.build_function(fn_main);

    let asts = builder.asts;
    let asts = optimize(OptiLevel::O1, asts);

    println!(
        "{}",
        asts.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
