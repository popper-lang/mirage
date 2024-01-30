

#[derive(Debug, Clone)]
pub struct AsmData {
    label: String,
    data_type: AsmDataType,
    data: AsmDataValue,
}

#[derive(Debug, Clone)]
pub enum AsmDataType {
    Byte,
    Word,
    Dword,
    Qword,
    String,
}

#[derive(Debug, Clone)]
pub enum AsmDataValue {
    Byte(u8),
    String(String),
    Word(u16),
    Dword(u32),
    Qword(u64),
}


#[derive(Debug, Clone, Copy)]
pub enum AsmCommand {
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Jmp,
    Call
}

#[derive(Debug, Clone)]
pub struct Asm {
    asm_op: AsmCommand,
    asm_arg: Vec<AsmArg>,
}

impl Asm {
    pub fn new(asm_op: AsmCommand, asm_arg: Vec<AsmArg>) -> Self {
        Self {
            asm_op,
            asm_arg,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AsmArg {
    Reg(Reg),
    Mem(Reg, i64),
    Imm(i64),
    Label(String),
    None,
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15
}

impl Reg {
    pub fn all() -> Vec<Reg> {
        vec![
            Reg::R0,
            Reg::R1,
            Reg::R2,
            Reg::R3,
            Reg::R4,
            Reg::R5,
            Reg::R6,
            Reg::R7,
            Reg::R8,
            Reg::R9,
            Reg::R10,
            Reg::R11,
            Reg::R12,
            Reg::R13,
            Reg::R14,
            Reg::R15
        ]
    }
}

#[derive(Debug, Clone)]
pub struct AsmLabel {
    label: String,
    asm: Vec<Asm>,
}

impl AsmLabel {
    pub fn new(label: String) -> Self {
        Self {
            label,
            asm: Vec::new(),
        }
    }

    pub fn builder(&self) -> AsmLabelBuilder {
        AsmLabelBuilder::new(&self.label)
    }

    pub fn add_asm(&mut self, asm: Asm) {
        self.asm.push(asm);
    }
}

#[derive(Debug, Clone)]
pub struct AsmProgram {
    labels: Vec<AsmLabel>,
    data: Vec<AsmData>,
}

impl AsmProgram {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: AsmLabel) {
        self.labels.push(label);
    }

    pub fn add_data(&mut self, data: AsmData) {
        self.data.push(data);
    }
}

#[derive(Debug, Clone)]
pub struct AsmProgramBuilder {
    program: AsmProgram,
}

impl AsmProgramBuilder {
    pub fn new() -> Self {
        Self {
            program: AsmProgram::new()
        }
    }

    pub fn build_label(&mut self, label: AsmLabel) {
        self.program.add_label(label)
    }

    pub fn build(self) -> AsmProgram {
        self.program
    }
}

#[derive(Debug, Clone)]
pub struct AsmLabelBuilder {
    label: AsmLabel
}

impl AsmLabelBuilder {
    pub fn from(l: AsmLabel) -> Self {
        Self {
            label: l
        }

    }
    pub fn new(name: &str) -> Self {
        Self {
            label: AsmLabel::new(name.to_string())
        }
    }

    pub fn build_mov(&mut self, arg1: AsmArg, arg2: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Mov, vec![arg1, arg2])
        );
    }

    pub fn build_add(&mut self, arg1: AsmArg, arg2: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Add, vec![arg1, arg2])
        );
    }

    pub fn build_sub(&mut self, arg1: AsmArg, arg2: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Sub, vec![arg1, arg2])
        );
    }

    pub fn build_mul(&mut self, arg1: AsmArg, arg2: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Mul, vec![arg1, arg2])
        );
    }

    pub fn build_div(&mut self, arg1: AsmArg, arg2: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Div, vec![arg1, arg2])
        );
    }

    pub fn build_jmp(&mut self, arg1: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Jmp, vec![arg1])
        );
    }

    pub fn build_call(&mut self, arg1: AsmArg) {
        self.label.add_asm(
            Asm::new(AsmCommand::Call, vec![arg1])
        );
    }

    pub fn build(self) -> AsmLabel {
        self.label.clone()
    }
}
