/*
⟨program⟩ ⩴ ⟨expression⟩

⟨expression⟩ ⩴ ⟨variable⟩
              | ⟨abstraction⟩
              | ⟨application⟩
              | ⟨addition⟩
              | ⟨let⟩
              | ⟨letrec⟩
              | ⟨int⟩
              | ⟨unit⟩
              | '(' ⟨expression⟩ ')'

⟨variable⟩ ⩴ ⟨letter⟩

⟨letter⟩ ⩴ 'a' | 'b' | 'c' | ... | 'z'

⟨bind⟩ ⩴ ⟨variable⟩ '=' ⟨expression⟩

⟨let⟩ ⩴ 'let' ⟨bind⟩ 'in' ⟨expression⟩

⟨letrec⟩ ⩴ 'letrec' ⟨bind⟩ ';' ... 'in' ⟨expression⟩

⟨unit⟩ ⩴  '()'

⟨int⟩ ⩴  ⟨integer⟩

⟨integer⟩ ⩴  ⟨digit⟩+
           | '-' ⟨digit⟩+

⟨addition⟩ ⩴  ⟨digit⟩ '+' ⟨digit⟩

⟨digit⟩ ⩴  '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'

⟨abstraction⟩ ⩴ 'λ' ⟨variable⟩ '.' ⟨expression⟩

⟨application⟩ ⩴ ⟨expression⟩ ⟨expression⟩
*/

use std::ops::Range;

type Span = Range<usize>;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(Variable),
    Abstraction {
        variable: Variable,
        expression: Box<Expression>,
        span: Span,
    },
    Application {
        callee: Box<Expression>,
        arg: Box<Expression>,
        span: Span,
    },
    Addition {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        span: Span,
    },
    Let {
        bind: Box<Bind>,
        body: Box<Expression>,
        span: Span,
    },
    Letrec {
        binds: Vec<Bind>,
        body: Box<Expression>,
        span: Span,
    },
    Int {
        span: Span,
    },
    Unit {
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub variable: Variable,
    pub expression: Expression,
}

impl Expression {
    pub fn get_span(self) -> Span {
        match self {
            Expression::Variable(Variable { span, .. }) => span,
            Expression::Abstraction { span, .. } => span,
            Expression::Application { span, .. } => span,
            Expression::Let { span, .. } => span,
            Expression::Letrec { span, .. } => span,
            Expression::Addition { span, .. } => span,
            Expression::Int { span, .. } => span,
            Expression::Unit { span } => span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub span: Span,
}
