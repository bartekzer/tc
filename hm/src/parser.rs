use crate::ast::*;
use chumsky::prelude::*;

pub trait HMParser = Parser<char, Expression, Error = Simple<char>>;

pub fn parser() -> impl HMParser {
    let variable = filter(|l: &char| l.is_ascii_lowercase())
        .map_with_span(|l, span| Variable { name: l, span })
        .padded();

    let expr = recursive(|expr| {
        let atom = choice((
            // Variable
            variable.map(|var| Expression::Variable(var)),
            // Priority
            expr.clone().delimited_by(just('('), just(')')),
        ))
        .padded();

        let bind = variable
            .then_ignore(just('=').padded())
            .then(expr.clone())
            .then_ignore(just(';').padded())
            .map(|(variable, expression)| Bind {
                variable,
                expression,
            })
            .padded();

        let op = choice((
            // Let
            just("let")
                .ignore_then(bind.clone())
                .then_ignore(just("in"))
                .then(expr.clone())
                .map_with_span(|(bind, body), span| Expression::Let {
                    bind: Box::new(bind),
                    body: Box::new(body),
                    span,
                }),
            // Letrec
            just("letrec")
                .ignore_then(bind.clone().repeated())
                .then_ignore(just("in"))
                .then(expr.clone())
                .map_with_span(|(binds, body), span| Expression::Letrec {
                    binds,
                    body: Box::new(body),
                    span,
                }),
            // Abs
            just('λ')
                .or(just('\\'))
                .ignore_then(variable)
                .then_ignore(just('.'))
                .then(expr.clone())
                .map_with_span(|(var, expression), span| Expression::Abstraction {
                    variable: var,
                    expression: Box::new(expression),
                    span,
                }),
            // App
            atom.clone()
                .then(atom.clone().repeated())
                .foldl(|e1, e2| Expression::Application {
                    callee: Box::new(e1.clone()),
                    arg: Box::new(e2.clone()),
                    span: {
                        let first_span = e1.get_span();
                        let second_span = e2.get_span();

                        first_span.start..second_span.end
                    },
                }),
        ))
        .padded();

        op.or(atom)
    })
    .then_ignore(end());

    expr
}
