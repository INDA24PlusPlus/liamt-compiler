use core::num;

use crate::{parser::*, OperatorType};

pub struct CodeGenerator {
    program: Program,
    code: String,
}

impl CodeGenerator {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            code: String::new(),
        }
    }

    pub fn generate(&mut self) -> Result<String, String> {
        self.code += "#include <stdio.h>\n";
        self.code += "#define print(num) printf(\"%d\\n\", (num))\n";
        self.code += "int _ = 0;\n";
        self.code += "int main() {\n";
        self.generate_stmts(self.program.statements.clone())?;
        self.code += "}";

        Ok(self.code.clone())
    }

    fn generate_stmts(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Assignment(ass_stmt) => {
                    if ass_stmt.definition {
                        self.code += "int "
                    }
                    self.code += &format!(
                        "{} = {};\n",
                        ass_stmt.name,
                        self.generate_expr(ass_stmt.expr)?
                    );
                }
                Stmt::Function(func_stmt) => {
                    self.code += &format!("int {}(", func_stmt.name);
                    for (i, param) in func_stmt.params.iter().enumerate() {
                        self.code += &format!("int {}", param);
                        if i < func_stmt.params.len() - 1 {
                            self.code += ", ";
                        }
                    }
                    self.code += ") {\n";

                    self.generate_stmts(func_stmt.body)?;

                    self.code += "}\n";
                }
                Stmt::If(if_stmt) => {
                    self.code += &format!("if ({}) {{\n", self.generate_expr(if_stmt.condition)?);
                    self.generate_stmts(if_stmt.body)?;
                    self.code += "}";
                    if !if_stmt.else_body.is_empty() {
                        self.code += " else {\n";
                        self.generate_stmts(if_stmt.else_body)?;
                        self.code += "}";
                    }
                    self.code += "\n";
                }
                Stmt::Return(expr) => {
                    if expr.is_none() {
                        self.code += "return 0;\n";
                    } else {
                        self.code += &format!("return {};\n", self.generate_expr(expr.unwrap())?);
                    }
                }
                Stmt::While(while_stmt) => {
                    self.code +=
                        &format!("while ({}) {{\n", self.generate_expr(while_stmt.condition)?);
                    self.generate_stmts(while_stmt.body)?;
                    self.code += "}\n";
                }
            }
        }

        Ok(())
    }

    fn generate_expr(&self, expr: Expr) -> Result<String, String> {
        match expr {
            Expr::Binary(bin_expr) => {
                let left = self.generate_expr(bin_expr.left)?;
                let right = self.generate_expr(bin_expr.right)?;

                let op = match bin_expr.op {
                    OperatorType::Addition => "+",
                    OperatorType::Subtraction => "-",
                    OperatorType::Multiplication => "*",
                    OperatorType::Division => "/",
                    OperatorType::Equals => "==",
                    OperatorType::NotEquals => "!=",
                };

                Ok(format!("{} {} {}", left, op, right))
            }
            Expr::Call(name, params) => {
                let mut code = format!("{}(", name);
                for (i, param) in params.iter().enumerate() {
                    code += &self.generate_expr(param.clone())?;
                    if i < params.len() - 1 {
                        code += ", ";
                    }
                }
                code += ")";
                Ok(code)
            }
            Expr::Number(num) => Ok(num.to_string()),
            Expr::Variable(name) => Ok(name),
        }
    }
}
