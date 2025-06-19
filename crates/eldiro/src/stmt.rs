use crate::{binding_def::BindingDef, env::Env, expr::Expr, func_def::FuncDef, val::Val};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    BindingDef(BindingDef),
    FuncDef(FuncDef),
    Expr(Expr),
}

impl Stmt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| FuncDef::new(s).map(|(s, func_def)| (s, Self::FuncDef(func_def))))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            }
            Self::FuncDef(func_def) => {
                func_def.eval(env)?;
                Ok(Val::Unit)
            }
            Self::Expr(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    // snip

    use crate::{
        binding_def::BindingDef,
        env::Env,
        expr::{Expr, Number, Op, binding_usage::BindingUsage, block::Block},
        func_def::FuncDef,
        stmt::Stmt,
        val::Val,
    };

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Stmt::BindingDef(BindingDef {
                name: "whatever".to_string(),
                val: Expr::Number(Number(-10)),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(
            Stmt::Expr(Expr::Number(Number(5))).eval(&mut Env::default()),
            Ok(Val::Number(5)),
        );
    }

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        );
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Stmt::FuncDef(FuncDef {
                name: "always_return_one".to_string(),
                params: Vec::new(),
                body: Box::new(Stmt::Expr(Expr::Number(Number(1)))),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }
}
