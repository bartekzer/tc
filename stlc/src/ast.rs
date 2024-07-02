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

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(Variable),
    Abstraction {
        variable: Variable,
        typ: Type,
        expression: Box<Expression>,
    },
    Application {
        callee: Box<Expression>,
        args: Box<Expression>,
    },
    Addition(Box<Expression>, Box<Expression>),
    Int(i32),
    Unit,
}

#[derive(Debug, Clone)]
pub struct Variable(pub char);

#[derive(Debug, Clone)]
pub enum Type {
    Function(Box<Type>, Box<Type>),
    Unit,
    Int,
}
