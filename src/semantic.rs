use crate::lexer::*;
use crate::parser::*;

pub struct Semantic {
    program: Program,
}

impl Semantic {
    pub fn new(program: Program) -> Self {
        Self { program }
    }
    pub fn analyze(&self) -> Result<(), String> {
        let builtins = vec!["print".to_string()];
        self.analyze_stmts(self.program.statements.clone(), builtins)?;
        Ok(())
    }

    fn analyze_stmts(&self, stmt: Vec<Stmt>, vars: Vec<String>) -> Result<(), String> {
        let mut vars = vars;
        for stmt in stmt {
            println!("{:?}", vars);
            match stmt {
                Stmt::Assignment(name, expr) => {
                    vars.push(name);

                    self.analyze_expr(expr, &vars)?;
                }
                Stmt::Function(func) => {
                    if vars.contains(&func.name) {
                        return Err(format!("Function {} already defined", func.name));
                    }
                    vars.push(func.name);

                    let mut new_vars = vars.clone();
                    new_vars.extend(func.params);

                    self.analyze_stmts(func.body, new_vars)?;
                }
                Stmt::If(if_stmt) => {
                    self.analyze_expr(if_stmt.condition, &vars)?;
                    self.analyze_stmts(if_stmt.body, vars.clone())?;
                    self.analyze_stmts(if_stmt.else_body, vars.clone())?;
                }
                Stmt::Return(expr) => {
                    if expr.is_some() {
                        self.analyze_expr(expr.unwrap(), &vars)?;
                    }
                }
                Stmt::While(while_stmt) => {
                    self.analyze_expr(while_stmt.condition, &vars)?;
                    self.analyze_stmts(while_stmt.body, vars.clone())?;
                }
            }
        }
        Ok(())
    }

    fn analyze_expr(&self, expr: Expr, vars: &Vec<String>) -> Result<(), String> {
        match expr {
            Expr::Binary(bin_expr) => {
                self.analyze_expr(bin_expr.left, vars)?;
                self.analyze_expr(bin_expr.right, vars)?;
            }
            Expr::Call(name, params) => {
                if !vars.contains(&name) {
                    return Err(format!("Function {} not defined", name));
                }
                for arg in params {
                    self.analyze_expr(arg, vars)?;
                }
            }
            Expr::Variable(name) => {
                if !vars.contains(&name) {
                    return Err(format!("Variable {} not defined", name));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
