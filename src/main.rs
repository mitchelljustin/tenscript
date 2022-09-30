#![feature(let_else)]

use std::error::Error;
use std::fs;

mod scanner;


fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("example.clj")?;
    let tokens = scanner::scan(&source)?;
    let just_tokens: Vec<_> = tokens.iter().map(|t| t.tok.clone()).collect();
    println!("{just_tokens:?}");
    Ok(())
}
