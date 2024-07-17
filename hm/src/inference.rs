use crate::ast::*;
use crate::error::Error;
use im::HashMap;
use yansi::Paint;

#[derive(Debug, Clone)]
pub enum Type {
    Function(Box<Type>, Box<Type>),
    Variable(u16),
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
            Type::Variable(id) => format!("t{}", id),
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
            (Type::Variable(id1), Type::Variable(id2)) => id1 == id2,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Constraint(Type, Type);

pub struct Inference {
    pub constraints: HashMap<u16, Constraint>,
    pub substitutions: HashMap<u16, Type>,
}

impl Inference {
    pub fn new() -> Self {
        Self {
            constraints: HashMap::new(),
            substitutions: HashMap::new(),
        }
    }
    pub fn debug(&self) {
        println!("Constraints:");
        for (id, constraint) in &self.constraints {
            println!(
                "{}: {} = {}",
                id.bold(),
                constraint.0.to_string().bold().cyan(),
                constraint.1.to_string().bold().cyan()
            );
        }

        println!("\nSubstitutions:");
        for (id, substitution) in &self.substitutions {
            println!("{}: {}", id.bold(), substitution.to_string().bold().cyan());
        }
    }
    fn fresh_type_variable(&mut self) -> Type {
        let tyvar = Type::Variable(self.substitutions.len() as u16);
        self.substitutions
            .insert(self.substitutions.len() as u16, tyvar.clone());
        tyvar
    }
    pub fn infer(&mut self, expr: Expression, env: HashMap<String, Type>) -> Result<Type, Error> {
        match expr {
            Expression::Abstraction {
                variable,
                expression,
                ..
            } => {
                let t1 = self.fresh_type_variable();
                let mut env = env.clone();
                env.insert(variable.name, t1.clone());
                let t2 = self.infer(*expression, env)?;
                Ok(Type::Function(Box::new(t1), Box::new(t2)))
            }
            Expression::Application { callee, arg, .. } => {
                let t1 = self.infer(*callee, env.clone())?;
                let t2 = self.infer(*arg, env)?;
                let ret = self.fresh_type_variable();
                self.constraints.insert(
                    self.constraints.len() as u16,
                    Constraint(t1, Type::Function(Box::new(t2), Box::new(ret.clone()))),
                );
                Ok(ret)
            }
            Expression::Variable(Variable { name, span }) => env
                .get(&name)
                .ok_or(Error::UndefinedSymbol { name, span })
                .cloned(),
            Expression::Addition { lhs, rhs, .. } => {
                let t1 = self.infer(*lhs, env.clone())?;
                let t2 = self.infer(*rhs, env)?;
                self.constraints
                    .insert(self.constraints.len() as u16, Constraint(t1, Type::Int));
                self.constraints
                    .insert(self.constraints.len() as u16, Constraint(t2, Type::Int));
                Ok(Type::Int)
            }
            Expression::Let { bind, body, .. } => {
                let t = self.infer(bind.expression.clone(), env.clone())?;
                let mut env = env.clone();
                env.insert(bind.variable.name, t);
                self.infer(*body, env)
            }
            Expression::Letrec { binds, body, .. } => {
                let mut env = env.clone();
                for bind in binds.iter() {
                    let new_tyvar = self.fresh_type_variable();
                    env.insert(bind.variable.name.clone(), new_tyvar);
                }
                for bind in binds.iter() {
                    let tyvar = env.get(&bind.variable.name).unwrap();
                    let t = self.infer(bind.expression.clone(), env.clone())?;
                    self.constraints.insert(
                        self.constraints.len() as u16,
                        Constraint(tyvar.clone(), t.clone()),
                    );
                }
                self.infer(*body, env)
            }
            Expression::Int { .. } => Ok(Type::Int),
            Expression::Unit { .. } => Ok(Type::Unit),
        }
    }
    pub fn solve_constraints(&mut self) -> Result<(), Error> {
        let constraints = self.constraints.clone();
        self.constraints.resetting();
        for (_, constraint) in constraints {
            self.unify(constraint.0, constraint.1)?;
        }
        Ok(())
    }
    pub fn unify(&mut self, t1: Type, t2: Type) -> Result<(), Error> {
        match (t1, t2) {
            (t1 @ Type::Variable(id), t2) if *self.substitutions.get(&id).unwrap() != t1 => {
                self.unify(self.substitutions.get(&id).unwrap().clone(), t2)
            }
            (t1, t2 @ Type::Variable(id)) if *self.substitutions.get(&id).unwrap() != t2 => {
                self.unify(t1, self.substitutions.get(&id).unwrap().clone())
            }
            (t1 @ Type::Variable(id), t2) => {
                if Self::occurs_in(id, t2.clone()) {
                    return Err(Error::InfiniteType { t1, t2 });
                } else {
                    self.substitutions.insert(id, t2);
                    Ok(())
                }
            }
            (t1, t2 @ Type::Variable(id)) => {
                if Self::occurs_in(id, t1.clone()) {
                    return Err(Error::InfiniteType { t1, t2 });
                } else {
                    self.substitutions.insert(id, t1);
                    Ok(())
                }
            }
            (Type::Function(param1, ret1), Type::Function(param2, ret2)) => {
                self.unify(*param1, *param2)?;
                self.unify(*ret1, *ret2)
            }
            (Type::Int, Type::Int) => Ok(()),
            (Type::Unit, Type::Unit) => Ok(()),
            (t1, t2) => Err(Error::UnificationFailure { t1, t2 }),
        }
    }
    fn occurs_in(index: u16, t: Type) -> bool {
        match t {
            Type::Function(param, ret) => {
                Self::occurs_in(index, *param) || Self::occurs_in(index, *ret)
            }
            Type::Variable(id) => id == index,
            _ => false,
        }
    }
    pub fn substitute(&self, t: Type) -> Type {
        match t {
            t @ Type::Variable(id) if *self.substitutions.get(&id).unwrap() != t => {
                self.substitute(self.substitutions.get(&id).unwrap().clone())
            }
            Type::Function(param, ret) => Type::Function(
                Box::new(self.substitute(*param)),
                Box::new(self.substitute(*ret)),
            ),
            _ => t,
        }
    }
}
