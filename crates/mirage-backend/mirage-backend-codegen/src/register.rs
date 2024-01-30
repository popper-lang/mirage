use mirage_backend_asm::builder::Reg;


#[derive(Debug, Clone)]
pub struct RegisyerState {
    is_used: bool,
    is_dirty: bool,
    is_live: bool,
    is_reserved: bool,
    reg: Reg,
}

#[derive(Debug, Clone)]
pub struct RegisterAllocator {
    regs: Vec<RegisyerState>,
}

impl RegisterAllocator {
    pub fn all() -> Self {
        Self {
            regs: Reg
                ::all()
                .iter()
                .map(|reg| RegisyerState {
                    is_used: false,
                    is_dirty: false,
                    is_live: false,
                    is_reserved: false,
                    reg: *reg,
                }).collect(),
        }
    }

    pub fn make_dirty(&mut self, reg: Reg) {
        self.regs[reg as usize].is_dirty = true;
    }

    pub fn make_live(&mut self, reg: Reg) {
        self.regs[reg as usize].is_live = true;
    }

    pub fn make_dead(&mut self, reg: Reg) {
        self.regs[reg as usize].is_live = false;
    }

    pub fn make_reserved(&mut self, reg: Reg) {
        self.regs[reg as usize].is_reserved = true;
    }

    pub fn get(&mut self) -> Option<Reg> {
        for reg in self.regs.iter_mut() {
            if !reg.is_used && !reg.is_reserved {
                reg.is_used = true;
                return Some(reg.reg);
            }
        }
        None
    }

    pub fn is_used(&self, reg: Reg) -> bool {
        self.regs[reg as usize].is_used
    }
}
