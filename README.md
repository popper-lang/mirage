# mirage
A Compiler Tool Written in Rust for The popper langage

# Example

in Popper code
```
external {
    func printf(a: string, ...): int
}

struct Foo {
    a: int,
    b: int
}

func add(a: int, b: int): int {
    return a + b;
}

func main() {
    let foo = Foo { a = 11, b = 12 };
    printf("11 + 22 = %d\n", add(foo.a, foo.b));
    for i in 3:9 {
        printf("i = %d\n", i);
    }
}
```
is compiled to the Mirage Code

```

module helloworld;
target x86_64-all-macos;

extern printf: args[@string, ...], ret[@void]

type Foo = { @int32, @int32 }

global .str0 = @string "11 + 22 = %d\n\0"
global .str1 = @string "i = %d\n\0"

add(@int32, @int32) @int32 {
block0 #start:
    ret = add_int32 arg0, arg1
}

main() @void #main {
block0 #start:                              ; the flag start show that the function start by block0
    v0 = copy .str0                         ; the copy operator copy a value into a variable
    v1 = copy .str1
    v2 = new Foo, {@int32 11, @int32 12}    ; v0 is a immutable const variable thats mean that the variable can countains big size data but cant be changed
    r0 = get v2, 0                          ; get the value at the 0 index of the struct Foo
    r1 = get v2, 1
    r2 = add { r0, r1 }
    printf { v0, r2 }
    free v0, v2                             ; we don't use anymore the variable v0, v2 so we delete
    r0 = const @int64 3
    jump for0
for0:
    jeq block1, r0, 9
    printf { v1, r0 }
    incr_int64 r0
    jump for0
block1:
    free r0, r1
}

```

# How to use it
## A Basic Function
```rust
use mirage::ir::builder::{BasicBlockBuilder, Builder};
use mirage::ir::module::Module;
use mirage::ir::object::{MirageTypeEnum, FunctionType}

fn main() {
    let module = Module::new("sum"); // Create a new module named sum
    let mut builder = Builder::new(module); // Create a new builder with the module sum

    builder.set_default_target(); // Set the target to the target's machine you're running

    let fn_type = FunctionType::new(
                        vec![
                            MirageTypeEnum::type_int32().into(),
                            MirageTypeEnum::type_int32().into()
                        ],
                        MirageTypeEnum::type_int32()
                    );

    let mut fn_value = fn_type.fn_value("sum"); // create the function `sum` with the type `(int32, int32) -> int32`

    let lhs = fn_value.get_nth_args(0); // get the first argument
    let rhs = fn_value.get_nth_args(1); // get the second argument

    let mut basic_block_builder = fn_value
                        .append_basic_block("entry")
                        .builder(); // add a basic block to the function `sum` and create a basic block builder

    let res = basic_block_builder.build_int_add(lhs, rhs) // emulate the add operation and store the result into res variable
    basic_block_builder.build_ret(res); // return the `res` variable

    builder.add_function(fn_value); // add the `sum` function to the builder
    builder.print_to_stdout();     // print the ir generated into stdout
}
```

that will get this ir:
```
module sum;
target macos-x86_64-gcc;

sum(@int32, @int32) @int32 {
entry:
    r0 = add_int32 arg0, arg1
    ret = r0
}
```
# Compiled Function
```rust
use mirage::compiler::{
    Compiler, Config
};
/* SNIP */

fn main() {
/* SNIP */
    let compiler = Compiler::new(Config::default());
    let output = compiler
        .compile(
            builder.build()
        );  // compile the ir generated to a Output
}
```

## Different option with `Output`
### Object
```rust
output
    .object()
    .write_to("sum.o")
```
### Assembler
```rust
output
    .assembler()
    .write_to("sum.s");
```
### Execution Engine
Execution Engine are a way to interact with the generated code
```rust
let sum: extern "C" fn(i32, i32) -> i32 = output
    .execution_engine()
    .get_function("sum");
!("{}", sum(1, 2)) // 3
```
