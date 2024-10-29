mod codegen;
mod lexer;
mod parser;
mod semantic;

use codegen::*;
use lexer::*;
use parser::*;
use semantic::*;

use clap::{Parser as ClapParser, Subcommand};
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transpile the specified file to C code
    Transpile {
        #[arg(short, long)]
        verbose: bool,
        file: PathBuf,
    },
    /// Compile the specified file
    Compile {
        #[arg(short, long)]
        verbose: bool,
        file: PathBuf,
        out_file: PathBuf,
    },
    /// Run the specified file
    Run {
        #[arg(short, long)]
        verbose: bool,
        file: PathBuf,
    },
    /// Generate the AST for the specified file
    Ast {
        #[arg(short, long)]
        verbose: bool,
        file: PathBuf,
    },
}

fn read_file(file: &PathBuf) -> String {
    let code = std::fs::read_to_string(file);

    if code.is_err() {
        panic!("Error reading file: {:?}", code.err().unwrap());
    }

    code.unwrap()
}

fn lex(code: String, v: bool) -> Vec<Token> {
    let tokens = Lexer::new(code).lex();

    if tokens.is_err() {
        panic!("Lexing error: {:?}", tokens.err().unwrap());
    }

    if v {
        println!("Tokens:\n{:?}", tokens.clone().unwrap());
    }

    tokens.unwrap()
}

fn parse(tokens: Vec<Token>, v: bool) -> Program {
    let prog = Parser::new(tokens).parse();

    if prog.is_err() {
        panic!("Parsing error: {:?}", prog.err().unwrap());
    }

    if v {
        println!("AST:\n{:#?}", prog.clone().unwrap());
    }

    prog.unwrap()
}

fn analyze(prog: Program) {
    let sem = Semantic::new(prog).analyze();

    if sem.is_err() {
        panic!("Semantic analysis error: {:?}", sem.err().unwrap());
    }
}

fn transpile(prog: Program, v: bool) -> String {
    let code = CodeGenerator::new(prog).generate();

    if code.is_err() {
        panic!("Code generation error: {:?}", code.err().unwrap());
    }

    if v {
        println!("C code:\n{}", code.clone().unwrap());
    }

    code.unwrap()
}

fn compile(c_code: String, out_file: PathBuf) {
    let mut file = File::create(".skibidi.c").unwrap();
    file.write_all(c_code.as_bytes()).unwrap();

    Command::new("gcc")
        .arg(".skibidi.c")
        .arg("-o")
        .arg(out_file)
        .output()
        .expect("Failed to compile with gcc, make sure gcc is installed");

    remove_file(".skibidi.c").unwrap();
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Transpile { verbose, file } => {
            let code = read_file(file);
            let tokens = lex(code, *verbose);
            let prog = parse(tokens, *verbose);
            analyze(prog.clone());
            transpile(prog, true);
        }
        Commands::Compile {
            verbose,
            file,
            out_file,
        } => {
            let code = read_file(file);
            let tokens = lex(code, *verbose);
            let prog = parse(tokens, *verbose);
            analyze(prog.clone());
            let c_code = transpile(prog, *verbose);
            compile(c_code, out_file.clone());
        }
        Commands::Run { verbose, file } => {
            let code = read_file(file);
            let tokens = lex(code, *verbose);
            let prog = parse(tokens, *verbose);
            analyze(prog.clone());
            let c_code = transpile(prog, *verbose);
            compile(c_code, ".skibidi.temp".into());
            let output = Command::new("./.skibidi.temp").output().unwrap();
            println!("{}", String::from_utf8_lossy(&output.stdout));
            remove_file(".skibidi.temp").unwrap();
        }
        Commands::Ast { verbose, file } => {
            let code = read_file(file);
            let tokens = lex(code, *verbose);
            parse(tokens, true);
        }
    }
}
