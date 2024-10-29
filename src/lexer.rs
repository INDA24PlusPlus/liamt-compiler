#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordType {
    If,
    Else,
    While,
    Function,
    VarDef,
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum OperatorType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equals,
    NotEquals,
}

#[derive(Debug, Clone, PartialEq)]

pub enum TokenType {
    Identifier(String),
    Integer(i64),
    Keyword(KeywordType),
    Operator(OperatorType),
    LeftParan,
    RightParan,
    LeftBrack,
    RightBrack,
    Assignment,
    Return,
    Comma,
    Pipe, // Semicolon type shit
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub index: usize,
}

pub struct Lexer {
    idx: usize,
    code: String,
}

impl Lexer {
    fn check_identifier(&self, string: String) -> TokenType {
        match string.as_str() {
            "looksmaxxing" => TokenType::Keyword(KeywordType::VarDef),
            "skibidi" => TokenType::Keyword(KeywordType::Function),
            "edge" => TokenType::Keyword(KeywordType::While),
            "sus" => TokenType::Keyword(KeywordType::If),
            "sussy" => TokenType::Keyword(KeywordType::Else),
            "rizz" => TokenType::Operator(OperatorType::Addition),
            "fanumtax" => TokenType::Operator(OperatorType::Subtraction),
            "gyatt" => TokenType::Operator(OperatorType::Multiplication),
            "mog" => TokenType::Operator(OperatorType::Division),
            "sigma" => TokenType::Return,

            _ => TokenType::Identifier(string),
        }
    }
    fn peek(&self) -> Option<char> {
        self.code.chars().nth(self.idx + 1)
    }
    fn current(&self) -> Option<char> {
        self.code.chars().nth(self.idx)
    }

    pub fn new(code: String) -> Self {
        Lexer { code, idx: 0 }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.idx < self.code.len() {
            let start_idx = self.idx;
            let tok_typ = match self.current().unwrap() {
                '|' => {
                    self.idx += 1;
                    Some(TokenType::Pipe)
                }
                'a'..='z' => {
                    while self.current().is_some_and(|c| c.is_ascii_alphabetic()) {
                        self.idx += 1;
                    }

                    Some(self.check_identifier(self.code[start_idx..self.idx].to_string()))
                }
                '0'..='9' => {
                    let mut num = 0;
                    while self.current().is_some_and(|c| c.is_ascii_digit()) {
                        num = num * 10 + self.current().unwrap().to_digit(10).unwrap() as i64;
                        self.idx += 1;
                    }
                    Some(TokenType::Integer(num))
                }
                ' ' => {
                    self.idx += 1;
                    continue;
                }
                '\n' => {
                    self.idx += 1;
                    continue;
                }
                '\r' => {
                    self.idx += 1;
                    continue;
                }
                '=' => {
                    self.idx += 1;
                    if self.current() == Some('=') {
                        self.idx += 1;
                        Some(TokenType::Operator(OperatorType::Equals))
                    } else {
                        Some(TokenType::Assignment)
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.idx += 2;
                        Some(TokenType::Operator(OperatorType::NotEquals))
                    } else {
                        None
                    }
                }
                '(' => {
                    self.idx += 1;
                    Some(TokenType::LeftParan)
                }
                ')' => {
                    self.idx += 1;
                    Some(TokenType::RightParan)
                }
                '>' => {
                    if self.peek() == Some('>') {
                        self.idx += 2;
                        Some(TokenType::LeftBrack)
                    } else {
                        None
                    }
                }
                '<' => {
                    if self.peek() == Some('<') {
                        self.idx += 2;
                        Some(TokenType::RightBrack)
                    } else {
                        None
                    }
                }
                ',' => {
                    self.idx += 1;
                    Some(TokenType::Comma)
                }
                _ => None,
            };

            if let Some(tok_typ) = tok_typ {
                tokens.push(Token {
                    token_type: tok_typ,
                    index: start_idx,
                });
                continue;
            }

            return Err(format!(
                "Something wrong here with this token: {:?} at index: {}",
                self.code.clone()[start_idx..].chars().next().unwrap(),
                start_idx
            ));
        }

        Ok(tokens)
    }
}
