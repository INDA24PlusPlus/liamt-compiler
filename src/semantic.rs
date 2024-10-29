use std::collections::HashMap;

use crate::parser::*;

#[derive(Debug, Clone)]
struct Scope {
    vars: Vec<String>,
    funcs: HashMap<String, usize>, // Name and number of parameters
}

pub struct Semantic {
    program: Program,
}

impl Semantic {
    pub fn new(program: Program) -> Self {
        Self { program }
    }
    pub fn analyze(&self) -> Result<(), String> {
        let mut scope = Scope {
            vars: vec!["_".to_string()],
            funcs: HashMap::new(),
        };
        // Add built-in functions
        scope.funcs.insert("print".to_string(), 1);

        self.analyze_stmts(self.program.statements.clone(), scope)?;
        Ok(())
    }

    // Very ugly i know
    fn func_exists(&self, scope: &Scope, name: String) -> bool {
        scope
            .funcs
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .contains(&name)
    }

    fn ident_exists(&self, scope: &Scope, name: String) -> bool {
        self.func_exists(scope, name.clone()) || scope.vars.contains(&name)
    }

    fn analyze_stmts(&self, stmts: Vec<Stmt>, scope: Scope) -> Result<(), String> {
        let mut scope = scope;
        for stmt in stmts {
            match stmt {
                Stmt::Assignment(ass_stmt) => {
                    if self.func_exists(&scope, ass_stmt.name.clone()) {
                        return Err(format!(
                            "Identifier {} already defined as function",
                            ass_stmt.name
                        ));
                    }

                    if ass_stmt.definition {
                        scope.vars.push(ass_stmt.name);
                    } else if !scope.vars.contains(&ass_stmt.name) {
                        return Err(format!("Variable {} not defined", &ass_stmt.name));
                    }

                    self.analyze_expr(ass_stmt.expr, &scope)?;
                }
                Stmt::Function(func) => {
                    if self.ident_exists(&scope, func.name.clone()) {
                        return Err(format!("Identifier {} already defined", func.name));
                    }
                    scope.funcs.insert(func.name.clone(), func.params.len());

                    let mut new_scope = scope.clone();
                    new_scope.vars.extend(func.params.clone());

                    self.analyze_stmts(func.body, new_scope)?;
                }
                Stmt::If(if_stmt) => {
                    self.analyze_expr(if_stmt.condition, &scope)?;
                    self.analyze_stmts(if_stmt.body, scope.clone())?;
                    self.analyze_stmts(if_stmt.else_body, scope.clone())?;
                }
                Stmt::Return(expr) => {
                    if expr.is_some() {
                        self.analyze_expr(expr.unwrap(), &scope)?;
                    }
                }
                Stmt::While(while_stmt) => {
                    self.analyze_expr(while_stmt.condition, &scope)?;
                    self.analyze_stmts(while_stmt.body, scope.clone())?;
                }
            }
        }
        Ok(())
    }

    fn analyze_expr(&self, expr: Expr, scope: &Scope) -> Result<(), String> {
        match expr {
            Expr::Binary(bin_expr) => {
                self.analyze_expr(bin_expr.left, scope)?;
                self.analyze_expr(bin_expr.right, scope)?;
            }
            Expr::Call(name, params) => {
                if !self.func_exists(scope, name.clone()) {
                    return Err(format!("Function {} not defined", name));
                }
                if !scope.funcs.get(&name).unwrap().eq(&params.len()) {
                    return Err(format!(
                        "Function {} expects {} parameters, got {}",
                        name,
                        scope.funcs.get(&name).unwrap(),
                        params.len()
                    ));
                }

                for arg in params {
                    self.analyze_expr(arg, scope)?;
                }
            }
            Expr::Variable(name) => {
                if !scope.vars.contains(&name) {
                    return Err(format!("Variable {} not defined", name));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
