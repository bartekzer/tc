use ariadne::{Color, Label, Report, ReportKind, Source};
use std::ops::Range;
use yansi::Paint;

pub enum Error {
    TypeMismatch {
        expected: String,
        found: String,
        span: Range<usize>,
    },
    UndefinedSymbol {
        name: char,
        span: Range<usize>,
    },
}

impl Error {
    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();

        let mut report = Report::build(ReportKind::Error, filename, 1);

        match &self {
            Error::TypeMismatch {
                expected,
                found,
                span,
            } => {
                report = report
                    .with_code("type-mismatch")
                    .with_message(format!(
                        "Expected type `{}`, but found `{}`.",
                        expected.cyan().bold(),
                        found.cyan().bold()
                    ))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message(format!("Found type `{}`.", found.cyan().bold()))
                            .with_color(Color::Magenta),
                    )
            }
            Error::UndefinedSymbol { name, span } => {
                report = report
                    .with_code("undefined-symbol")
                    .with_message(format!("Undefined symbol `{}`.", name.cyan().bold()))
                    .with_label(
                        Label::new((filename, span.start..span.end))
                            .with_message(format!("`{}` is not defined.", name.cyan().bold()))
                            .with_color(Color::Magenta),
                    )
            }
        }
        report
            .finish()
            .print((filename, Source::from(source)))
            .unwrap()
    }
}
