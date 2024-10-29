mod codegen;
mod lexer;
mod parser;
mod semantic;

use codegen::*;
use lexer::*;
use parser::*;
use semantic::*;

fn main() {
    let code = r#"
        looksmaxxing lol = 123|

        skibidi fib(n) >>
            sus n == 0 >>
                sigma 0|
            <<
            sus n == 1 >>
                sigma 1|
            << 
            sussy >>
                sigma fib(n fanumtax 1) rizz fib(n fanumtax 2)|
            << 
        <<

        looksmaxxing i = 0|
        edge (i != 10) >>
            looksmaxxing a = fib(i)|
            print(a)|

            i = i rizz 1|
        <<
    "#
    .to_string();

    println!("Lexing: {:?}", code);

    let tokens = Lexer::new(code).lex();

    if tokens.is_err() {
        println!("Lexing error: {:?}", tokens.err().unwrap());
        return;
    }

    let prog = Parser::new(tokens.unwrap()).parse();

    if prog.is_err() {
        println!("Parsing error: {:?}", prog.err().unwrap());
        return;
    }

    println!("{:#?}", prog.clone().unwrap());

    let sem = Semantic::new(prog.clone().unwrap()).analyze();

    if sem.is_err() {
        println!("Semantic analysis error: {:?}", sem.err().unwrap());
        return;
    }

    println!("Semantic analysis successful");

    let code = CodeGenerator::new(prog.unwrap()).generate();

    if code.is_err() {
        println!("Code generation error: {:?}", code.err().unwrap());
        return;
    }

    println!("{}", code.unwrap());
}
