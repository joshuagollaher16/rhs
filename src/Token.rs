use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Operator::Plus => f.write_str("+"),
            Operator::Minus => f.write_str("-"),
            Operator::Multiply => f.write_str("*"),
            Operator::Divide => f.write_str("/"),
            _ => Err(std::fmt::Error::default())
        }
    }
}

#[derive(Copy, Clone)]
pub struct Token {
    pub is_operator: bool,
    pub operator: Option<Operator>,
    pub value: Option<i32>
}

impl Token {
    pub fn new(is_operator: bool, operator: Option<Operator>, value: Option<i32>) -> Token {
        Token {
            is_operator,
            operator,
            value
        }
    }
}