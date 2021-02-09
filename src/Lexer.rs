use crate::Token::{Token, Operator};

pub struct Lexer {
    str: String
}

impl Lexer {
    pub fn new(str: &String) -> Lexer {
        Lexer {
            str: str.clone()
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        let is_alpha = |x: String | {
            match x.trim().parse::<i32>() {
                Ok(val) => true,
                Err(e) => false
            }
        };
        let is_operator = |x: String | {
            x == String::from("+") ||
                   x == String::from("-") ||
                   x == String::from("/") ||
                   x == String::from("*")
        };

        let mut was_just_numeric = false;
        let mut first_pass = true;
        let mut current_number_str = String::new();

        for character in self.str.chars().enumerate() {

            if is_alpha(character.1.to_string()) {
                current_number_str.push(character.1);
                was_just_numeric = true;
            }

            if is_operator(character.1.to_string()) {
                if was_just_numeric {
                    //save numeric token

                    let number = current_number_str.parse::<i32>().expect("Error parsing number");
                    tokens.push(Token::new(false, None, Some(number)));
                    current_number_str.clear();

                    //save operator token

                    let operator = match character.1 {
                        '+' => Operator::Plus,
                        '-' => Operator::Minus,
                        '/' => Operator::Divide,
                        '*' => Operator::Multiply,
                        _ => panic!("Invalid operator")
                    };

                    tokens.push(Token::new(true, Some(operator), None));
                }
            }

            if character.0 == self.str.len() - 1 && was_just_numeric {
                let number = current_number_str.parse::<i32>().expect("Error parsing number");
                tokens.push(Token::new(false, None, Some(number)));
            }

        }

        tokens
    }
}