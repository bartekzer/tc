use crate::ast::*;
use chumsky::prelude::*;

pub trait HMParser = Parser<char, Expression, Error = Simple<char>>;

pub fn parser() -> impl HMParser {
    let variable = filter(|l: &char| l.is_ascii_lowercase())
        .repeated()
        .at_least(1)
        .map_with_span(|x, span| Variable {
            name: x.iter().collect::<String>(),
            span,
        })
        .padded();

    let expr = recursive(|expr| {
        let atom = choice((
            // Unit
            just("()").map_with_span(|_, span| Expression::Unit { span }),
            // Int
            text::int(10).map_with_span(|_, span| Expression::Int { span }),
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
            // Sum
            atom.clone()
                .then_ignore(just('+').padded())
                .then(expr.clone())
                .map_with_span(|(lhs, rhs), span| Expression::Addition {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span,
                }),
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
                .map_with_span(|(variable, expression), span| Expression::Abstraction {
                    variable,
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
