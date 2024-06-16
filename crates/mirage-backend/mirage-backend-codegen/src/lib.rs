#![allow(dead_code)]
mod register;
mod environement;

use environement::Environement;
use mirage_backend_asm::builder::{AsmLabel, AsmLabelBuilder, AsmProgram, AsmProgramBuilder, Reg};
use mirage_frontend::{
    module::Module,
    object::{
        function::FunctionValue, label::{Command, Label}, statements::Statement
    }
};
use register::RegisterAllocator;

#[derive(Debug, Clone)]
pub enum CodeGenError {
    InvalidStatement
}

type CodeGenResult<T> = Result<T, CodeGenError>;

#[derive(Debug, Clone)]
pub struct CodeGen {
    stmts: Vec<Statement>,
    code: AsmProgram,
    builder: AsmProgramBuilder,
    reg_alloc: RegisterAllocator,
    module: Module,
    env: Environement,
    lid: usize
}

impl CodeGen {
    pub fn new(stmts: Vec<Statement>, module: Module) -> Self {
        let mut reg_alloc = RegisterAllocator::all();
        reg_alloc.make_reserved(Reg::R1); // R1 is reserved for the stack pointer
        reg_alloc.make_reserved(Reg::R2); // R2 is reserved for the frame pointer
        reg_alloc.make_reserved(Reg::R3); // R3 is reserved for the return address

        Self {
            stmts,
            code: AsmProgram::new(),
            builder: AsmProgramBuilder::new(),
            reg_alloc,
            module,
            env: Environement::new(),
            lid: 0,
        }
    }

    pub fn compile(&mut self) -> Self {
        for stmt in self.stmts.clone().iter() {
            self.compile_stmt(stmt);
        }
        self.clone()
    }

    pub fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt.clone() {
            Statement::Function(f) => {
                self.compile_function(f);
            },
            _ => {}
        }
    }

    pub fn compile_function(&mut self, func: FunctionValue) {
        let label = AsmLabel::new(func.get_name().clone());
        let builder = label.builder();
    }

    pub fn compile_label(&mut self, label: Label) -> AsmLabel {
        let asmlabel = AsmLabel::new(label.name);
        let builder = asmlabel.builder();
        todo!()
    }
    pub fn compile_command(&mut self, builder: &mut AsmLabelBuilder, command: Command) {
        todo!()
    }

    pub fn compile_value() {
        todo!()
    }
}
