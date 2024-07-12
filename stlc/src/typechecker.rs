use crate::ast::*;
use crate::error::{Error, ErrorKind};
use std::collections::HashMap;
use std::ops::Range;

pub struct TypeChecker(HashMap<char, Type>);

impl TypeChecker {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn check(&mut self, expr: Expression, context_span: Range<usize>) -> Result<Type, Error> {
        match expr {
            Expression::Variable(Variable { name, span }) => {
                if let Some(typ) = self.0.get(&name) {
                    Ok(typ.clone())
                } else {
                    Err(Error {
                        kind: ErrorKind::UndefinedSymbol { name, span },
                        span: context_span,
                    })
                }
            }
            Expression::Abstraction {
                variable,
                typ,
                expression,
                span,
            } => {
                self.0.insert(variable.name, typ.clone()); // /!\
                let ret_type = self.check(*expression, span)?;
                Ok(Type::Function(Box::new(typ), Box::new(ret_type)))
            }
            Expression::Application { callee, arg, span } => {
                let callee_type = self.check(*callee, span.clone())?;
                if let Type::Function(head_type, tail_type) = callee_type.clone() {
                    let arg_type = self.check(*arg.clone(), span.clone())?;
                    if *head_type == arg_type {
                        return Ok(*tail_type)
                    } else {
                        return Err(Error {
                            kind: ErrorKind::TypeMismatch {
                                expected: head_type.to_string(),
                                found_type: arg_type.to_string(),
                                found_span: arg.get_span()
                            },
                            span
                        })
                    }
                } else {
                    return Err(Error {
                        kind: ErrorKind::TypeMismatch {
                            expected: "Function".to_string(),
                            found_type: callee_type.to_string(),
                            found_span: arg.get_span()
                        },
                        span
                    })
                }
            }
            Expression::Addition { lhs, rhs, span } => {
                let lhs_type = self.check(*lhs.clone(), span.clone())?;
                if Type::Int == lhs_type {
                    let rhs_type = self.check(*rhs.clone(), span.clone())?;
                    if Type::Int == rhs_type {
                        return Ok(Type::Int)
                    } else {
                        return Err(Error {
                            kind: ErrorKind::TypeMismatch {
                                expected: Type::Int.to_string(),
                                found_type: lhs_type.to_string(),
                                found_span: rhs.get_span()
                            },
                            span
                        })
                    }
                } else {
                    return Err(Error {
                        kind: ErrorKind::TypeMismatch {
                            expected: Type::Int.to_string(),
                            found_type: lhs_type.to_string(),
                            found_span: lhs.get_span()
                        },
                        span
                    })
                }
            }
            Expression::Int { .. } => Ok(Type::Int),
            Expression::Unit { .. } => Ok(Type::Unit)
        }
    }
}
