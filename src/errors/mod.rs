use crate::Range;
use severity::Severity;

pub mod collector;
pub mod compiler_errors;
pub mod severity;

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
