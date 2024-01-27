
use mirage_frontend_builder::Builder;
use mirage_frontend_object::stringify::Stringify;
use mirage_frontend_object::MirageTypeEnum;
use mirage_frontend_object::function::{FunctionType, FunctionValue};
use mirage_frontend_module::Module;

fn main() {
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
    let mut func = fn_type.fn_value("add".to_string());
    let lhs = func.get_nth_arg(0).unwrap();
    let rhs = func.get_nth_arg(1).unwrap();
    let mut basic_block0 = builder.new_basic_block("entry");
    let mut basic_block1 = builder.new_basic_block("hello");

    basic_block0.build_int_add(lhs.expect_int_value().unwrap(), rhs.expect_int_value().unwrap(), true).unwrap();
    basic_block1.build_int_add(lhs.expect_int_value().unwrap(), rhs.expect_int_value().unwrap(), true).unwrap();
    func.add_label(
        basic_block0.build()
    );
    func.add_label(
        basic_block1.build()
    );

    builder.build_function(func.clone());
    let asts = builder.asts;

    asts.iter().for_each(|ast| {
        println!("{}", ast.to_string());
    });


}
