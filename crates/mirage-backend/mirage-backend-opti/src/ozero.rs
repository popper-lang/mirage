use mirage_frontend::object::{function::FunctionValue, label::LabelBodyInstr, statements::{Global, Statement}};

use crate::opti::{
    GlobalOptimizer, OptiLevel, Optimize
};

#[derive(Debug, Clone)]
pub struct OptiZero {
    stmts: Vec<Statement>
}

impl OptiZero {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self {
            stmts
        }
    }
}

impl GlobalOptimizer for OptiZero {
    fn optimize_level() -> OptiLevel {
        OptiLevel::O0
    }

    fn optimize_statement(&mut self, stmt: Statement) -> Statement {
        stmt
    }

    fn optimize_function(&mut self, func: FunctionValue) -> FunctionValue {
        func
    }

    fn optimize_global(&mut self, global: Global) -> Global {
        global
    }

    fn optimize_label_instr(&mut self, instr: LabelBodyInstr) -> LabelBodyInstr {
        instr
    }
}
