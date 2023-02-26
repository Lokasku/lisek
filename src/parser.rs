#![allow(unused)]

use std::collections::HashMap;
use crate::eval::Evaluator;
use crate::decl::*;

pub enum TType {
    Integer(i32),
    Float(f32),
    String(String),
    SParen(Vec<Token>), // ()
    SBrac(Vec<Token>),  // {}
    Ident(usize),
    Builtin(fn(&mut Evaluator, usize, usize))
}

pub struct Token {
    pub ttype: TType,
    pub line: usize,
    pub column: usize
}

impl Token {
    pub fn new(ttype: TType, line: usize, column: usize) -> Self {
        Self {
            ttype,
            line,
            column
        }
    }
}

pub struct Parser {
    pub input: String,
    pub output: Vec<Token>,
    pub line: usize,
    pub column: usize,
    pub current: usize,
    pub start: usize,

    pub symbols: Vec<String>,
    pub values: Vec<Token>,

    pub builtins: HashMap<String, fn(&mut Evaluator, usize, usize)>
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

    pub fn is_eof(&self) -> bool {
        self.current >= self.input.chars().count() - 1
    }

    pub fn peek(&self, k: usize) -> char {
        self.input
        .chars()
        .nth(self.current + k)
        .expect("Unexpected EOF while peeking.")
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        match self.peek(0) {
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            _ => self.column += 1
        }
        self.input
            .chars()
            .nth(self.current)
            .expect("Unexpected EOF while advancing.")
    }

    pub fn skip_blanks(&mut self) {
        println!(">> skip_blanks()");

        while self.peek(0) == ' ' {
            self.advance();
        }
        self.start = self.current
    }

    pub fn string(&mut self) -> TType {
        println!(">> number()");
        self.advance();
        self.start = self.current;
        while self.peek(0) != '"' {
            self.advance();
        }
        let string = self.input[self.start..self.current].to_string();
        println!("   -> {:?}", string);

        TType::String(string)
    }

    pub fn number(&mut self) -> TType {
        println!(">> number()");
        if self.peek(0) == '-' {
            self.advance();
        }

        while self.peek(0).is_digit(10) || self.peek(0) == '.' {
            self.advance();
        }

        let num = self.input[self.start..self.current].to_string();
        println!("   -> {:?}", num);

        match num.parse::<i32>() {
            Ok(n) => TType::Integer(n),
            Err(_) => TType::Float(num.parse::<f32>().unwrap())
        }
    }

    pub fn stuck(&mut self, delimiter: char) -> Vec<Token> {
        println!(">> stuck()");
        let mut content = vec![];
        while self.peek(0) != ')' {
            match self.unit_parse() {
                Some(x) => content.push(x),
                None => {}
            }
        }
        content
    }

    pub fn unit_parse(&mut self) -> Option<Token> {
        let cc = self.advance();
        println!("> unit_parse(): {}", cc);
        if self.current != 1 {self.start = self.current};

        match cc {
            '\n' | '\r' | ' ' | ')' => None,
            '"' => Some(Token::new(self.string(), self.line, self.column)),
            '(' => Some(Token::new(TType::SParen(self.stuck(')')), self.line, self.column)),
            '{' => Some(Token::new(TType::SBrac(self.stuck('}')), self.line, self.column)),
            x => if x.is_digit(10) || x == '-' && self.peek(1).is_digit(10) {
                Some(Token::new(self.number(), self.line, self.column))
            } else {
                self.identifiers()
            }
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            println!("PARSE()");
            match self.unit_parse() {
                Some(tok) => self.output.push(tok),
                None => {}
            }
        }
    }
}