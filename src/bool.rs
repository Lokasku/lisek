use crate::eval::Evaluator;

impl Evaluator {
    pub fn low(&mut self, line: usize, column: usize) {}
    pub fn sup(&mut self, line: usize, column: usize) {}
    pub fn eq(&mut self, line: usize, column: usize) {}
    pub fn uneq(&mut self, line: usize, column: usize) {}
}