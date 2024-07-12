/*
⟨program⟩ ⩴  ⟨expression⟩

⟨expression⟩ ⩴  ⟨variable⟩
              | ⟨abstraction⟩
              | ⟨application⟩
              | ⟨addition⟩
              | ⟨int⟩
              | ⟨unit⟩
              | '(' ⟨expression⟩ ')'

⟨variable⟩ ⩴  ⟨letter⟩

⟨letter⟩ ⩴  'a' | 'b' | 'c' | ... | 'z'

⟨abstraction⟩ ⩴  'λ' ⟨variable⟩ ':' ⟨type⟩ '.' ⟨expression⟩

⟨type⟩ ⩴  ⟨base-type⟩
        | ⟨function-type⟩

⟨base-type⟩ ⩴  'Int'
             | 'Unit'

⟨function-type⟩ ⩴  ⟨type⟩ ' -> ' ⟨type⟩

⟨unit⟩ ⩴  '()'

⟨int⟩ ⩴  ⟨integer⟩

⟨integer⟩ ⩴  ⟨digit⟩+
           | '-' ⟨digit⟩+

⟨addition⟩ ⩴  ⟨digit⟩ '+' ⟨digit⟩

⟨digit⟩ ⩴  '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'

⟨application⟩ ⩴  ⟨expression⟩ ⟨expression⟩
*/

use std::ops::Range;

type Span = Range<usize>;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(Variable),
    Abstraction {
        variable: Variable,
        typ: Type,
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
    Int {
        _n: i32,
        span: Span,
    },
    Unit {
        span: Span,
    },
}

impl Expression {
    pub fn get_span(self) -> Span {
        match self {
            Expression::Variable(Variable { span, .. }) => span,
            Expression::Abstraction { span, .. } => span,
            Expression::Application { span, .. } => span,
            Expression::Addition { span, .. } => span,
            Expression::Int { span, .. } => span,
            Expression::Unit { span } => span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: char,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Type {
    Function(Box<Type>, Box<Type>),
    Unit,
    Int,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Function(param1, ret1) => {
                let param_str = param1.to_string();
                let ret_str = ret1.to_string();
                let param_parenthesized = if let Type::Function(..) = **param1 {
                    format!("({})", param_str)
                } else {
                    param_str
                };
                format!("{} -> {}", param_parenthesized, ret_str)
            }
            Type::Unit => "Unit".to_string(),
            Type::Int => "Int".to_string(),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Function(param1, ret1), Type::Function(param2, ret2)) => {
                param1 == param2 && ret1 == ret2
            }
            (Type::Unit, Type::Unit) => true,
            (Type::Int, Type::Int) => true,
            _ => false,
        }
    }
}
