use std::collections::HashMap;

use mirage_frontend::object::{
    function::FunctionValue, label::{
        Command, LabelBodyInstr, Value
    }, meta::Flag, statements::{
        Global,
        Statement
    }, util::List, MirageObject, MirageValueEnum, RegisterValue
};

use crate::opti::{GlobalOptimizer, OptiLevel};



#[derive(Debug, Clone)]
pub struct OptiOne {
    stmts: Vec<Statement>,
    register_const: HashMap<usize, MirageValueEnum>,
    register_def_index: HashMap<usize, usize>,
    global_const: HashMap<String, MirageValueEnum>
}

impl OptiOne {
    pub fn new() -> Self {
        Self {
            stmts: Vec::new(),
            register_const: HashMap::new(),
            register_def_index: HashMap::new(),
            global_const: HashMap::new()
        }
    }

    fn replace_const_opti(&mut self, r: RegisterValue, v: LabelBodyInstr) -> LabelBodyInstr {
        match v {
            LabelBodyInstr::Command(
                Command::Const(
                    obj
                )
            ) => {
                    if let MirageValueEnum::Register(r) = obj.get_value() {
                    if let Some(v) = self.register_const.get(&r.index) {
                        return LabelBodyInstr::Command(
                            Command::Const(
                                MirageObject::from(*v)
                            )
                        )
                    }
                }
            },
            _ => {}
        }

        LabelBodyInstr::Assign(r, Box::new(v))
    }

    fn insert_const_value(&mut self, r: RegisterValue, v: LabelBodyInstr) {
        if let LabelBodyInstr::Command(Command::Const(obj)) = v {
            self.register_const.insert(r.index, obj.get_value());
        }
    }

    fn optimize_mirage_value(&mut self, val: MirageValueEnum) -> MirageValueEnum {
        if let MirageValueEnum::Register(r) = val {
            if let Some(v) = self.register_const.get(&r.index) {
                *v
            } else {
                val
            }
        } else {
            val
        }
    }

    fn optimize_object(&mut self, obj: MirageObject) -> MirageObject {
        MirageObject::from(
            self.optimize_mirage_value(obj.get_value())
        )
    }

    fn optimize_value(&mut self, val: Value) -> Value {
        match val {
            Value::ConstValue(c) => {
                Value::ConstValue(
                    self.optimize_object(c)
                )
            },
            Value::List(l) => {
                Value::List(
                    List::from_vec(
                        l.iter()
                        .map(|x| self.optimize_value(x.clone()))
                        .collect()
                    )
                )

            },
            Value::Register(r) => {
                if let Some(r) = self.register_const.get(&r.index) {
                    Value::ConstValue(
                        MirageObject::from(
                            r.clone()
                        )
                    )
                } else {
                    Value::Register(r)
                }
            }
        }
    }
}

impl GlobalOptimizer for OptiOne {
    fn optimize_level() -> OptiLevel {
        OptiLevel::O1
    }


    fn optimize_function(&mut self, func: FunctionValue) -> FunctionValue {
        let mut func = func.clone();
        if func.len_labels() == 1 {
            func
                .get_nth_label_mut(0)
                .unwrap()
                .flags
                .push(
                    Flag::new("inline".to_string())
                ); // This is a inline function that is mean to be inlined in the assembly
        }

        func
            .get_labels_mut()
            .iter_mut()
            .for_each(|x| {
                x.body = x.body
                    .iter()
                    .map(|x| self.optimize_label_instr(x.clone()))
                    .collect()
            });
        func
    }
    fn optimize_global(&mut self, global: Global) -> Global {
        self.global_const.insert(global.name.clone(), global.value.get_value());
        global
    }

    fn optimize_label_instr(&mut self, instr: LabelBodyInstr) -> LabelBodyInstr {
        match instr {
            LabelBodyInstr::Assign(r, v)  => {
                let v = self.optimize_label_instr(*v);
                self.insert_const_value(r, v.clone());
                let instr = self.replace_const_opti(r, v);
                instr
            },
            LabelBodyInstr::Call(name, args) => {
                LabelBodyInstr::Call(
                    name,
                    args
                        .iter()
                        .map(|x| self.optimize_value(x.clone()))
                        .collect()
                )
            },
            LabelBodyInstr::Command(Command::Const(obj)) => {
                LabelBodyInstr::Command(
                    Command::Const(
                        self.optimize_object(obj)
                    )
                )
            },
            LabelBodyInstr::Command(Command::AddInt8(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddInt8(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },
            LabelBodyInstr::Command(Command::AddInt16(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddInt16(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },

            LabelBodyInstr::Command(Command::AddInt32(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddInt32(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },

            LabelBodyInstr::Command(Command::AddInt64(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddInt64(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },

            LabelBodyInstr::Command(Command::AddFloat32(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddFloat32(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },

            LabelBodyInstr::Command(Command::AddFloat64(l, r )) => {
                LabelBodyInstr::Command(
                    Command::AddFloat64(
                        self.optimize_value(l),
                        self.optimize_value(r)
                    )
                )
            },
            LabelBodyInstr::Command(Command::Copy(c)) => {
                if let Some(g) = self.global_const.get(&c) {
                    LabelBodyInstr::Command(
                        Command::Const(
                            MirageObject::from(
                                g.clone()
                            )
                        )
                    )
                } else {
                    LabelBodyInstr::Command(
                        Command::Copy(c)
                    )
                }
            },
            LabelBodyInstr::Command(Command::Ret(r)) => {
                LabelBodyInstr::Command(
                    Command::Ret(
                        self.optimize_value(r)
                    )
                )
            },
            e => e
        }
    }

    fn optimize_statements(&mut self, stmts: Vec<Statement>) -> Vec<Statement> {
        self.stmts = stmts.clone();
        stmts
            .into_iter()
            .map(|x| self.optimize_statement(x))
            .collect()
    }

    fn optimize_statement(&mut self, stmt: Statement) -> Statement {
        match stmt {
            Statement::Function(func) => Statement::Function(self.optimize_function(func)),
            Statement::Global(global) => Statement::Global(self.optimize_global(global)),
            e => e
        }
    }

    fn optimize_labels(&mut self, labels: mirage_frontend::object::label::Label) -> mirage_frontend::object::label::Label {
        mirage_frontend::object::label::Label::new(
            labels.name,
            labels.flags,
            self.optimize_label_instrs(labels.body)
        )
    }

    fn optimize_label_instrs(&mut self, instrs: Vec<LabelBodyInstr>) -> Vec<LabelBodyInstr> {
        instrs
            .into_iter()
            .map(|x| self.optimize_label_instr(x))
            .collect()
    }

}
