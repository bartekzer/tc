use crate::inference::Type;
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::ops::Range;
use yansi::Paint;

type Span = Range<usize>;

pub enum Error {
    UnificationFailure { t1: Type, t2: Type },
    InfiniteType { t1: Type, t2: Type },
    UndefinedSymbol { name: String, span: Span },
}

impl Error {
    pub fn report(&self, filename: &str) {
        let source = &std::fs::read_to_string(filename).unwrap();

        let mut report = Report::build(ReportKind::Error, filename, 1);

        match &self {
            Error::UnificationFailure { t1, t2 } => {
                report = report
                    .with_code("unification-failure")
                    .with_message(format!(
                        "Cannot unify `{}` with `{}`.",
                        t1.to_string().cyan().bold(),
                        t2.to_string().cyan().bold()
                    ))
            }
            Error::InfiniteType { t1, t2 } => {
                report = report.with_code("infinite-type").with_message(format!(
                    "Infinite loop detected between `{}` and `{}`.",
                    t1.to_string().cyan().bold(),
                    t2.to_string().cyan().bold()
                ))
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
