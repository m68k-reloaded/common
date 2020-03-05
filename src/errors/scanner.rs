use super::{super::Range, Error, Severity, Source};

impl Error {
    pub fn no_match(range: Range, current: char, next: char) -> Error {
        Error {
            code: "no_match",
            severity: Severity::Error,
            source: Source::Scanner,
            range,
            message: format!("No token matches for '{}', '{}'.", current, next),
        }
    }
}
