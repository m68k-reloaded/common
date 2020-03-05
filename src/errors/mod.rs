use crate::Range;
mod collector;
pub use collector::ErrorCollector;

pub struct Error {
    pub range: Range,
    pub message: String,
}
