use crate::parser::{Token, TType};

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Self { Self {} }

    pub fn unit_format(&mut self, tokens: Vec<Token>, start_to: usize) -> (Token, usize)  {
        let token = tokens[start_to].clone();
        println!(">> UNIT_FORMAT() : {:?}", token);
        match token {
            Token {ttype: TType::Builtin(func), line, column} => {
                let arity = match func {
                    0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 11 | 13 => 2,
                    10 => 1,
                    12 => 3,
                    _ => 0 // will never happen
                }; 
                println!("  -> GET BUILTIN: Builtin({func}) with {arity} args.\n");
                let mut res: Vec<Token> = vec![token];
                let mut position = start_to + 1;
                for _ in 0..arity {
                    let r = self.unit_format(tokens.clone(), position);
                    res.push(r.0);
                    println!("  ===> BUILTIN CONTENT: {:?}\n", res);
                    position += 1; 
                }
                let expr = TType::Expr(res);
                (Token::new(expr, line, column), position as usize)
            }
            Token {ttype: TType::SParen(v), line, column} => {
                println!("  -> ENTER IN SParen\n");
                let mut result: Vec<Token> = vec![];
                self.formatter(v, &mut result, 0, 0);
                println!("  ===> GET in SPAREN: {:?}\n", result);
                (Token::new(TType::SParen(result.clone()), line, column), 0)
            }
            Token {ttype: TType::SBrac(v), line, column} => {
                println!("  -> ENTER IN SBrac\n");
                let mut result: Vec<Token> = vec![];
                self.formatter(v, &mut result, 0, 0);
                println!("  ===> GET in SBRAC: {:?}\n", result);
                (Token::new(TType::SBrac(result), line, column), 0)
            }
            t => (t, 0)
        }

    }

    pub fn formatter(&mut self, tokens: Vec<Token>, result: &mut Vec<Token>, start_to: usize, mut pos: usize) {
        let r = self.unit_format(tokens.clone(), start_to);
        match r {
            (Token {ttype: TType::Expr(v), line, column}, n) => {
                result.push(Token::new(TType::Expr(v), line, column));
                pos += n + 1;
                if pos >= tokens.len() - 1 {}
                else {
                    self.formatter(tokens, result, pos + 1, pos);
                }
            }
            _ => panic!("Every instruction must start with a Builtin, or an assignment.")
        }
    }
}
