use mirage_frontend::object::statements::Statement;

mod opti;
mod ozero;
mod oone;

use opti::GlobalOptimizer;
pub use opti::OptiLevel;
use log::warn;

pub fn optimize(level: OptiLevel, stmts: Vec<Statement>) -> Vec<Statement> {
    match level {
        OptiLevel::O0 => stmts,
        OptiLevel::O1 => oone::OptiOne::new(stmts.clone()).optimize_statements(stmts),
        _ => {
            warn!("Optimization level {} is not supported yet, falling back to O0", level.as_str());
            stmts
        }
    }
}
