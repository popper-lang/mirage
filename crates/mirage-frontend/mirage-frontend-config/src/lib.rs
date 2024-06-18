

#[derive(Default)]
pub enum OptimizationLevel {
    None,
    Less,

    #[default]
    Default,

    Medium,
    Big
}

#[derive(Default)]
pub enum Bits {
    B8,
    B16,
    
    #[default]
    B32,
    
    B64,
}

pub struct Config {
    pub optimization_level: OptimizationLevel,
    pub bits: Bits,
    
}

impl Default for Config {
    fn default() -> Self {
        Self {
            optimization_level: Default::default(),
            bits: Bits::B32
        }
    }
}