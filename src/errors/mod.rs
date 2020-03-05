use crate::Range;

pub type ErrorCollector = dyn Fn(Error) -> ();

pub struct Error {
    pub range: Range,
    pub message: String,
}
