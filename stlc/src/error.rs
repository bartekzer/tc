use crate::ast::*;
use ariadne::{Label, Report, ReportKind, Source};
use std::ops::Range;

pub enum ErrorKind {
    TypeMismatch {
        expected: String,
        found_type: String,
        found_span: Range<usize>,
    },
    UndefinedSymbol {
        name: char,
        span: Range<usize>,
    },
}

pub struct Error {
    pub kind: ErrorKind,
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
                found_type,
                found_span,
            } => {
                report =
                    report
                        .with_code("type-mismatch")
                        // .with_message("Expected type {}, but found {}.", expected, )
                        .with_label(Label::new((filename, found_span.start..found_span.end)).with_message(
                            format!("Found type {}, expected {}.", found_type, expected),
                        ))
            }

            ErrorKind::UndefinedSymbol { name, span } => {
                report = report
                    .with_code("undefined-symbol")
                    // .with_message("Expected type {}, but found {}.", expected, )
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message(format!("`{}` is not defined.", name)),
                    )
            }
        }
        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap()
    }
}
