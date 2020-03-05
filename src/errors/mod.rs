use crate::Range;
pub use collector::Collector;
pub use severity::Severity;

mod collector;
pub mod compiler;
mod severity;

pub struct Error<'e> {
    pub code: &'e str,
    pub severity: Severity,
    pub source: Source,
    pub range: Range,
    pub message: &'e str,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Source {
    Scanner,
    Parser,
    Compiler,
}
