mod parser;
mod decl;
mod eval;
mod builtins;
mod arithmetic;
mod bool;

use parser::Parser;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {panic!("You must provide filecode.")}

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons.");
    let mut parser = Parser::new(content);
    parser.parse();
    dbg!("{:#?}", parser.output);
}