use mirage_frontend::object::{function::FunctionValue, label::{Command, Label, LabelBodyInstr}, statements::{Global, Statement}};




#[derive(Debug, Clone, Copy)]
pub enum OptiLevel {
    /// No optimizations, also the default
    O0,
    /// Optimize for size instead of speed
    O1,
    /// Optimize for speed instead of size
    O2,
    /// Optimize more for speed
    O3,
    /// Like O2 with size optimizations
    Os,
    /// Like O2 with size optimizations
    Oz,
}

impl OptiLevel {
    pub fn as_str(&self) -> &str {
        match self {
            OptiLevel::O0 => "O0",
            OptiLevel::O1 => "O1",
            OptiLevel::O2 => "O2",
            OptiLevel::O3 => "O3",
            OptiLevel::Os => "Os",
            OptiLevel::Oz => "Oz",
        }
    }
}

pub trait Optimize: Sized {
    type Output;
    fn optimize_level() -> OptiLevel;
    fn optimize(&mut self, o: Self::Output) -> Self::Output;
}


pub trait GlobalOptimizer: Sized {
    fn optimize_level() -> OptiLevel;
    fn optimize_statements(&mut self, stmts: Vec<Statement>) -> Vec<Statement> {
        stmts
            .into_iter()
            .map(|x| self.optimize_statement(x))
            .collect()
    }
    fn optimize_statement(&mut self, stmt: Statement) -> Statement;
    fn optimize_function(&mut self, func: FunctionValue) -> FunctionValue;
    fn optimize_global(&mut self, global: Global) -> Global;
    fn optimize_labels(&mut self, labels: Label) -> Label {
        Label::new(
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
    fn optimize_label_instr(&mut self, instr: LabelBodyInstr) -> LabelBodyInstr;

}

impl<T: GlobalOptimizer> Optimize for T {
    type Output = Vec<Statement>;
    fn optimize_level() -> OptiLevel {
        T::optimize_level()
    }
    fn optimize(&mut self, o: Self::Output) -> Self::Output {
        self.optimize_statements(o)
    }
}
