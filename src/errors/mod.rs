use crate::Range;
use severity::Severity;

pub mod collector;
pub mod compiler_errors;
pub mod severity;

pub struct Error<'a> {
    pub code: &'a str,
    pub severity: Severity,
    pub source: Source,
    pub range: Range,
    pub message: &'a str,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Source {
    Scanner,
    Parser,
    Compiler,
}
