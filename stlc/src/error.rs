use crate::ast::*;
use ariadne::{Label, Report, ReportKind, Source};
use std::ops::Range;

enum ErrorKind {
    TypeMismatch {
        expected: String,
        found: Range<usize>,
        type_found: String,
    },
}

pub struct Error {
    kind: ErrorKind,
    pub span: Range<usize>,
}

impl Error {
    pub fn new(kind: ErrorKind, span: Range<usize>) -> Error {
        Error { kind, span }
    }

    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();

        let mut line = 1;
        for c in source
            .chars()
            .skip(self.span.start)
            .take(self.span.end - self.span.start)
        {
            if c == '\n' {
                line += 1;
            }
        }

        let mut report = Report::build(ReportKind::Error, filename, line);

        match &self.kind {
            ErrorKind::TypeMismatch {
                expected,
                found,
                type_found,
            } => {
                report = report
                    .with_code("type-mismatch")
                    // .with_message("Expected type {}, but found {}.", expected, )
                    .with_label(
                        Label::new((filename, self.span.start..self.span.end)).with_message(
                            format!("Found type {}, expected {}.", type_found, expected),
                        ),
                    )
            }
        }
        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap()
    }
}
