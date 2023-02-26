#![allow(unused)]

use std::fmt;
use std::collections::HashMap;
use crate::eval::Evaluator;

#[derive(Debug)]
enum TType {
    Integer(i32),
    Float(f32),
    String(String),
    SParen(Vec<Token>), // ()
    SBrac(Vec<Token>),  // {}
    Ident(usize),
    Builtin(usize)
}

#[derive(Debug)]
struct Token {
    ttype: TType,
    line: usize,
    column: usize
}

impl Token {
    fn new(ttype: TType, line: usize, column: usize) -> Self {
        Self {
            ttype,
            line,
            column
        }
    }
}

pub struct Parser {
    input: String,
    output: Vec<Token>,
    line: usize,
    column: usize,
    current: usize,
    start: usize,

    symbols: Vec<String>,
    values: Vec<Token>,

    builtins: HashMap<String, fn(&mut Evaluator, usize, usize)>
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut parser = Self {
            input,
            output: vec![],
            line: 1,
            column: 1,
            current: 0,
            start: 0,
            symbols: vec![],
            values: vec![],
            builtins: HashMap::new()
        };

        parser.add_builtin("+", Evaluator::add);
        parser.add_builtin("-", Evaluator::sub);
        parser.add_builtin("*", Evaluator::mul);
        parser.add_builtin("/", Evaluator::div);
        parser.add_builtin("%", Evaluator::mdl);
        parser.add_builtin("^", Evaluator::mdl);

        parser.add_builtin("<", Evaluator::low);
        parser.add_builtin(">", Evaluator::sup);
        parser.add_builtin("=", Evaluator::eq);
        parser.add_builtin("!=", Evaluator::uneq);

        parser.add_builtin("format", Evaluator::cond);
        parser.add_builtin("conc", Evaluator::fun);
        parser.add_builtin("cond", Evaluator::cond);
        parser.add_builtin("fun", Evaluator::fun);
        parser.add_builtin("until", Evaluator::until) ;
        parser
    }

    fn add_builtin(&mut self, name: &str, func: fn(&mut Evaluator, usize, usize)) {
        self.builtins.insert(name.to_string(), func);
    }
}