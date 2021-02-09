use crate::Token::{Token, Operator};
use std::ops::Index;

pub struct Parser {
    pub toks: Vec<Token>
}

impl Parser {
    pub(crate) fn new(toks: Vec<Token>) -> Parser {
        Parser {
            toks
        }
    }

    pub fn parse(&mut self) -> i32 {

        let num_value = |index: usize| {
            self.toks.index(index as usize).value.unwrap()
        };

        //Parsing multiply divide
        for tok in self.toks.clone().iter().enumerate() {
            if tok.1.is_operator {
                if tok.1.operator.expect("Operator error") == Operator::Multiply {
                    let result = self.toks.index(tok.0 - 1 as usize).value.unwrap() * self.toks.index(tok.0 + 1 as usize).value.unwrap();
                    self.toks.drain(tok.0-1..tok.0+2);
                    self.toks.insert(tok.0-1, Token::new(false, None, Some(result)));
                }
                if tok.1.operator.expect("Operator error") == Operator::Divide {
                    let result = self.toks.index(tok.0 - 1 as usize).value.unwrap() / self.toks.index(tok.0 + 1 as usize).value.unwrap();
                    self.toks.drain(tok.0-1..tok.0+2);
                    self.toks.insert(tok.0-1, Token::new(false, None, Some(result)));
                }
            }
        }

        //Parsing add subtract

        let mut value = 0;
        let mut first_pass = true;
        let mut next_operator = Operator::Plus;

        let do_operator = |lhs: i32, rhs: i32, op: Operator| -> i32 {
            match op {
                Operator::Plus => {lhs + rhs},
                Operator::Minus => {lhs - rhs},
                Operator::Divide => {lhs / rhs},
                Operator::Multiply => {lhs * rhs}
            }
        };

        for tok in self.toks.iter() {
            if first_pass {
                first_pass = false;
                value = tok.value.unwrap();
                continue;
            }

            if tok.is_operator {
                next_operator = tok.operator.unwrap();
                continue;
            }
            //numeric case, switch operator and apply to value

            let num = tok.value.unwrap();

            value = do_operator(value, num, next_operator);

        }

        value
    }
}