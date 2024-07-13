use crate::ast::*;
use crate::error::Error;
use std::collections::HashMap;

pub struct TypeChecker(HashMap<char, Type>);

impl TypeChecker {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn infer(&mut self, expr: Expression) -> Result<Type, Error> {
        match expr {
            Expression::Variable(Variable { name, span }) => {
                if let Some(typ) = self.0.get(&name) {
                    Ok(typ.clone())
                } else {
                    Err(Error::UndefinedSymbol { name, span })
                }
            }
            Expression::Abstraction {
                variable,
                typ,
                expression,
                ..
            } => {
                self.0.insert(variable.name, typ.clone());
                let ret_type = self.infer(*expression)?;
                Ok(Type::Function(Box::new(typ), Box::new(ret_type)))
            }
            Expression::Application { callee, arg, .. } => {
                let callee_type = self.infer(*callee)?;
                if let Type::Function(head_type, subsequent_type) = callee_type.clone() {
                    let arg_type = self.infer(*arg.clone())?;
                    if *head_type == arg_type {
                        return Ok(*subsequent_type);
                    } else {
                        return Err(Error::TypeMismatch {
                            expected: head_type.to_string(),
                            found: arg_type.to_string(),
                            span: arg.get_span(),
                        });
                    }
                } else {
                    return Err(Error::TypeMismatch {
                        expected: "Function".to_string(),
                        found: callee_type.to_string(),
                        span: arg.get_span(),
                    });
                }
            }
            Expression::Addition { lhs, rhs, .. } => {
                let lhs_type = self.infer(*lhs.clone())?;
                if Type::Int == lhs_type {
                    let rhs_type = self.infer(*rhs.clone())?;
                    if Type::Int == rhs_type {
                        return Ok(Type::Int);
                    } else {
                        return Err(Error::TypeMismatch {
                            expected: Type::Int.to_string(),
                            found: rhs_type.to_string(),
                            span: rhs.get_span(),
                        });
                    }
                } else {
                    return Err(Error::TypeMismatch {
                        expected: Type::Int.to_string(),
                        found: lhs_type.to_string(),
                        span: lhs.get_span(),
                    });
                }
            }
            Expression::Int { .. } => Ok(Type::Int),
            Expression::Unit { .. } => Ok(Type::Unit),
        }
    }
}
