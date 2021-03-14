#![feature(if_let_guard)]

use std::env::args;
use std::fs::read_to_string;
mod scan;

fn main() {
    println!();

    let filename = args().last().unwrap();
    let source_text = read_to_string(filename).unwrap();
    println!("Source text: {:?}", source_text);
    println!();
    
    let tokens = scan::scan(source_text);
    println!("Tokens: {:?}", tokens);
    println!();
    println!("Source text:\n{}", scan::reassemble(tokens.unwrap_or(vec!())))
}
