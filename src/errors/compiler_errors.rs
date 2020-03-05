use super::{super::Range, Error, Severity, Source};

impl Error<'_> {
    pub fn unspecified_size<'a>(range: Range) -> Error<'a> {
        Error {
            code: "unspecified_size",
            severity: Severity::Error,
            source: Source::Compiler,
            range: range,
            message: "A size attribute isn't present and could not be inferred.",
        }
    }
}
