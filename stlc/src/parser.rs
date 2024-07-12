use crate::ast::*;
use chumsky::prelude::*;

pub trait STLCParser = Parser<char, Expression, Error = Simple<char>>;

pub fn parser() -> impl STLCParser {
    let typ = recursive(|typ| {
        let base_type = choice((
            just("Unit").to(Type::Unit),
            just("Int").to(Type::Int),
            typ.clone().delimited_by(just('('), just(')')),
        ))
        .padded();

        let function_type = base_type
            .clone()
            .then_ignore(just("->").padded())
            .then(typ)
            .map(|(lhs, rhs)| Type::Function(Box::new(lhs), Box::new(rhs)));

        function_type.or(base_type)
    });

    let variable = filter(|l: &char| l.is_ascii_lowercase())
        .map_with_span(|l, span| Variable { name: l, span })
        .padded();

    let expr = recursive(|expr| {
        let atom = choice((
            // Unit
            just("()").map_with_span(|_, span| Expression::Unit { span }),
            // Int
            text::int(10).map_with_span(|n: String, span| Expression::Int {
                _n: n.parse::<i32>().unwrap(),
                span,
            }),
            // Variable
            variable.map(|var| Expression::Variable(var)),
            // Priority
            expr.clone().delimited_by(just('('), just(')')),
        ))
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
            // Abs
            just('λ')
                .or(just('\\'))
                .ignore_then(variable)
                .then_ignore(just(':'))
                .then(typ)
                .then_ignore(just('.'))
                .then(expr.clone())
                .map_with_span(|((var, r#type), expr), span| Expression::Abstraction {
                    variable: var,
                    typ: r#type,
                    expression: Box::new(expr),
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
