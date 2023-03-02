use crate::parser::{Token, TType};
use crate::eval::Evaluator;

struct Formatter {
    input: Vec<Token>,
    output: Vec<Token>
}

impl Formatter {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![]
        }
    }

    pub fn unit_format(&mut self, tokens: Vec<Token>, start_to: usize) -> (Token, usize)  {
        let token = tokens[start_to];
        match token {
            Token {ttype: TType::Builtin(func), line, column} => {
                let arity = match func as fn(&mut Evaluator, usize, usize) {
                   Evaluator::add | Evaluator::sub | Evaluator::mul | Evaluator::div
                   | Evaluator::mdl | Evaluator::exp | Evaluator::low | Evaluator::sup
                   | Evaluator::eq | Evaluator::uneq | Evaluator::conc | Evaluator::until => 2,
                   Evaluator::format => 1,
                   Evaluator::cond => 3
                };
                let mut res: Vec<Token> = vec![token];
                let mut pos = &start_to + 1;
                for i in 0..arity {
                    let r = self.unit_format(tokens, pos);
                    res.push(r.0);
                    pos += 1; 
                }
                let expr = TType::Expr(res);
                (Token::new(expr, line, column), pos as usize)
            }
        }
    }
}
