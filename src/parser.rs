use std::mem;
use std::vec;

use crate::lexer::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinaryExpr>),
    Number(i64),
    Variable(String),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub right: Expr,
    pub op: OperatorType,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assignment(AssignmentStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(Box<Option<Expr>>),
}

#[derive(Debug, Clone)]
pub struct AssignmentStmt {
    pub definition: bool,
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub body: Vec<Stmt>,
    pub else_body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct FunctionStmt {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

pub struct Parser {
    pub program: Program,
    tokens: Vec<Token>,
    idx: usize,
}

// This trait will extract the value from enums that have a value associated with them
// For example, TokenType::Identifier("skibidi".to_string()) will be extracted to Some("skibidi")
pub trait EnumExtractor<T> {
    fn extract(&self) -> Option<&T>;
}

impl EnumExtractor<String> for TokenType {
    fn extract(&self) -> Option<&String> {
        match self {
            TokenType::Identifier(ref value) => Some(value),
            _ => None,
        }
    }
}

impl EnumExtractor<i64> for TokenType {
    fn extract(&self) -> Option<&i64> {
        match self {
            TokenType::Integer(ref value) => Some(value),
            _ => None,
        }
    }
}

impl Parser {
    fn current(&self) -> Token {
        self.tokens.get(self.idx).cloned().expect("Unexpected EOF")
    }
    fn peek(&self) -> Token {
        self.tokens
            .get(self.idx + 1)
            .cloned()
            .expect("Unexpected EOF")
    }
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            program: Program { statements: vec![] },
            tokens,
            idx: 0,
        }
    }

    fn increment(&mut self) {
        self.idx += 1;
    }

    pub fn expect_with_value<T>(&mut self, expected: TokenType) -> Result<T, String>
    where
        TokenType: EnumExtractor<T>, // We are using the EnumExtractor trait to extract the value from the enum, only works for String and i64
        T: Clone,                    // This is needed because we are returning a clone of the value
    {
        match self.current().token_type.extract() {
            Some(inner)
                if mem::discriminant(&self.current().token_type)
                    == mem::discriminant(&expected) =>
            {
                self.increment();
                Ok(inner.clone())
            }
            _ => Err(format!(
                "Expected {:?}, got {:?} at index {}",
                expected,
                self.current().token_type,
                self.current().index
            )),
        }
    }

    pub fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        // The mem::discriminant will disregard the value of the enum and only compare the enum-type
        if mem::discriminant(&self.current().token_type) == mem::discriminant(&expected) {
            self.increment();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?} at index {}",
                expected,
                self.current().token_type,
                self.current().index
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        while self.idx < self.tokens.len() {
            let stmt = self.parse_stmt()?;
            self.program.statements.push(stmt);
        }

        Ok(self.program.clone())
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current().token_type {
            TokenType::Identifier(i) => {
                // Check if its a function call
                if self.peek().token_type == TokenType::LeftParan {
                    let expr = self.parse_expr()?;
                    self.expect(TokenType::Pipe)?;
                    Ok(Stmt::Assignment(AssignmentStmt {
                        definition: false,
                        name: "_".to_string(), // Assign to a temporary variable
                        expr,
                    }))
                } else {
                    let ident = i;
                    self.increment();
                    self.expect(TokenType::Assignment)?;
                    let expr = self.parse_expr()?;
                    self.expect(TokenType::Pipe)?;
                    Ok(Stmt::Assignment(AssignmentStmt {
                        definition: false,
                        name: ident,
                        expr,
                    }))
                }
            }
            TokenType::Return => {
                self.increment();
                if self.current().token_type == TokenType::Pipe {
                    self.increment();
                    Ok(Stmt::Return(Box::new(None)))
                } else {
                    let expr = self.parse_expr()?;
                    self.expect(TokenType::Pipe)?;
                    Ok(Stmt::Return(Box::new(Some(expr))))
                }
            }
            TokenType::Keyword(k) => match k {
                KeywordType::VarDef => {
                    self.increment();
                    let name =
                        self.expect_with_value::<String>(TokenType::Identifier("".to_string()))?;
                    self.expect(TokenType::Assignment)?;
                    let expr = self.parse_expr()?;
                    self.expect(TokenType::Pipe)?;
                    Ok(Stmt::Assignment(AssignmentStmt {
                        definition: true,
                        name,
                        expr,
                    }))
                }
                KeywordType::If => {
                    self.increment();
                    let condition: Expr = self.parse_expr()?;
                    self.expect(TokenType::LeftBrack)?;

                    let mut body = vec![];
                    while self.current().token_type != TokenType::RightBrack {
                        let stmt = self.parse_stmt()?;
                        body.push(stmt);
                    }
                    self.expect(TokenType::RightBrack)?;

                    let mut else_body = vec![];
                    if self.current().token_type == TokenType::Keyword(KeywordType::Else) {
                        self.increment();
                        self.expect(TokenType::LeftBrack)?;

                        while self.current().token_type != TokenType::RightBrack {
                            let stmt = self.parse_stmt()?;
                            else_body.push(stmt);
                        }
                        self.expect(TokenType::RightBrack)?;
                    }

                    Ok(Stmt::If(IfStmt {
                        condition,
                        body,
                        else_body,
                    }))
                }
                KeywordType::While => {
                    self.increment();
                    let condition = self.parse_expr()?;
                    self.expect(TokenType::LeftBrack)?;

                    let mut body = vec![];
                    while self.current().token_type != TokenType::RightBrack {
                        let stmt = self.parse_stmt()?;
                        body.push(stmt);
                    }
                    self.expect(TokenType::RightBrack)?;

                    Ok(Stmt::While(WhileStmt { condition, body }))
                }
                KeywordType::Function => {
                    self.increment();
                    let name =
                        self.expect_with_value::<String>(TokenType::Identifier("".to_string()))?;

                    self.expect(TokenType::LeftParan)?;
                    let mut params = vec![];
                    while self.current().token_type != TokenType::RightParan {
                        if self.current().token_type == TokenType::Comma {
                            self.increment();
                        }
                        let param_name = self
                            .expect_with_value::<String>(TokenType::Identifier("".to_string()))?;
                        params.push(param_name);
                    }
                    self.expect(TokenType::RightParan)?;

                    self.expect(TokenType::LeftBrack)?;
                    let mut body = vec![];
                    while self.current().token_type != TokenType::RightBrack {
                        let stmt = self.parse_stmt()?;
                        body.push(stmt);
                    }
                    self.expect(TokenType::RightBrack)?;

                    Ok(Stmt::Function(FunctionStmt { name, params, body }))
                }
                _ => Err(format!(
                    "Unexpected keyword {:?} at index {}",
                    k,
                    self.current().index
                )),
            },
            _ => Err(format!(
                "Unexpected token {:?} at index {}",
                self.current().token_type,
                self.current().index
            )),
        }
    }

    //Precidence:
    // 1. Parentheses
    // 2. Equals and Not Equals
    // 3. Multiplication and Division
    // 4. Addition and Subtraction
    fn parse_expr(&mut self) -> Result<Expr, String> {
        // Start with the lowest precidence :)
        self.parse_add_sub_expr()
    }

    fn parse_add_sub_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div_expr()?;

        while let Some(op) = self.parse_operator() {
            match op {
                OperatorType::Addition | OperatorType::Subtraction => {
                    self.increment();
                    let right = self.parse_mul_div_expr()?;
                    left = Expr::Binary(Box::new(BinaryExpr { left, right, op }));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_mul_div_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equals_expr()?;

        while let Some(op) = self.parse_operator() {
            match op {
                OperatorType::Multiplication | OperatorType::Division => {
                    self.increment();
                    let right = self.parse_equals_expr()?;
                    left = Expr::Binary(Box::new(BinaryExpr { left, right, op }));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_equals_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary_expr()?;

        while let Some(op) = self.parse_operator() {
            match op {
                OperatorType::Equals | OperatorType::NotEquals => {
                    self.increment();
                    let right = self.parse_primary_expr()?;
                    left = Expr::Binary(Box::new(BinaryExpr { left, right, op }));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.current().token_type {
            TokenType::Integer(value) => {
                self.increment();
                Ok(Expr::Number(value))
            }
            TokenType::Identifier(ref name) => {
                self.increment();
                // Check if this is a function call
                if self.current().token_type == TokenType::LeftParan {
                    self.increment();
                    let mut params = vec![];
                    while self.current().token_type != TokenType::RightParan {
                        params.push(self.parse_expr()?);
                        if self.current().token_type == TokenType::Comma {
                            self.increment();
                        }
                    }
                    self.expect(TokenType::RightParan)?;
                    Ok(Expr::Call(name.clone(), params))
                } else {
                    Ok(Expr::Variable(name.clone()))
                }
            }
            TokenType::LeftParan => {
                self.increment();
                let expr = self.parse_expr()?;
                self.expect(TokenType::RightParan)?;
                Ok(expr)
            }
            _ => Err(format!(
                "Unexpected token {:?} in expression at index {}",
                self.current().token_type,
                self.current().index
            )),
        }
    }

    fn parse_operator(&mut self) -> Option<OperatorType> {
        match self.current().token_type {
            TokenType::Operator(op) => Some(op),
            _ => None,
        }
    }
}
