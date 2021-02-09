mod Lexer;
mod Token;
mod Parser;

use std::io::stdin;

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Please enter valid input.");

    let lexer = Lexer::Lexer::new(&input);

    let tokens = lexer.lex();
    let mut parser = Parser::Parser::new(tokens);
    let value = parser.parse();

    println!("Result: {}", value);

}
