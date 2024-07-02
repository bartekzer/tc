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
        .map(|l| Variable(l))
        .padded();

    let expr = recursive(|expr| {
        let atom = choice((
            // Unit
            just("()").to(Expression::Unit),
            // Int
            text::int(10).map(|n: String| Expression::Int(n.parse::<i32>().unwrap())),
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
                .map(|(lhs, rhs)| Expression::Addition(Box::new(lhs), Box::new(rhs))),
            // Abs
            just('λ')
                .or(just('\\'))
                .ignore_then(variable)
                .then_ignore(just(':'))
                .then(typ)
                .then_ignore(just('.'))
                .then(expr.clone())
                .map(|((var, r#type), expr)| Expression::Abstraction {
                    variable: var,
                    typ: r#type,
                    expression: Box::new(expr),
                }),
            // App
            atom.clone()
                .then(atom.clone().repeated())
                .foldl(|e1, e2| Expression::Application {
                    callee: Box::new(e1),
                    args: Box::new(e2),
                }),
        ))
        .padded();

        op.or(atom)
    })
    .then_ignore(end());

    expr
}
