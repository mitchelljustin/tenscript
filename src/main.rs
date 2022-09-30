#![feature(let_else)]

use std::error::Error;
use std::fs;

mod scanner;
mod sexp;
mod error;
mod interpreter;


fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("example.clj")?;
    let tokens = scanner::scan(&source)?;
    // let just_tokens: Vec<_> = tokens.iter().map(|t| t.tok.clone()).collect();
    // println!("{just_tokens:?}");
    let sexp = sexp::parse_tokens(tokens)?;
    println!("{sexp}");
    let fabric = interpreter::interpret_sexp(&sexp)?;
    println!("{fabric:#?}");
    Ok(())
}
