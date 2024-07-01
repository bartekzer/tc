use crate::ast::*;
use chumsky::prelude::*;

pub trait STLCParser = Parser<char, Expression, Error = Simple<char>>;

pub fn parser() -> impl STLCParser {
    let typ = recursive(|typ| {
        let parenthesized_type = typ.clone().delimited_by(just('('), just(')')).padded();

        let base_type = choice((
            just("Unit").to(Type::Unit),
            just("Int").to(Type::Int),
            parenthesized_type,
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
        let unit = just("()").to(Expression::Unit).padded();

        let int = text::int(10)
            .map(|n: String| Expression::Int(n.parse::<i32>().unwrap()))
            .padded();

        let variable_expr = variable.map(|var| Expression::Variable(var));

        let abstraction = just('λ')
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
            });

        let application = expr
            .clone()
            .then(expr.clone())
            .map(|(e1, e2)| Expression::Application {
                function: Box::new(e1),
                argument: Box::new(e2),
            });

        let parenthesized_expr = expr.clone().delimited_by(just('('), just(')')).padded();

        let base_expr = choice((
            int,
            unit,
            parenthesized_expr,
            variable_expr,
            abstraction,
            application,
        ));

        let sum = base_expr
            .clone()
            .then_ignore(just('+').padded())
            .then(expr)
            .map(|(lhs, rhs)| Expression::Addition(Box::new(lhs), Box::new(rhs)))
            .padded();

        sum.or(base_expr)
    })
    .then_ignore(end());

    expr
}
