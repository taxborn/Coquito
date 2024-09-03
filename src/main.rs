use lexer::Lexer;
use tokens::Token;

mod lexer;
mod tokens;

fn main() {
    let input = "10+20";
    let tokens: Vec<Token> = Lexer::new(&input).collect();
}
