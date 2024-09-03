use crate::tokens::Token;
use std::{iter::Peekable, str::CharIndices};

pub struct Lexer<'src> {
    input: &'src str,
    /// The input string. We use [`Peekable`] here because of when we lex longer lexemes, peekable
    /// helps us not overshoot the end of certain multi-character lexemes (e.g. ...).
    chars: Peekable<CharIndices<'src>>,
    /// Where we are in the input string, should always round to the beginning of a UTF-8
    /// codepoint.
    position: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        // TODO: Error here? Panic?
        if input.is_empty() {
            panic!("cannot lex empty input");
        }

        let chars = input.char_indices().peekable();

        Self {
            input,
            chars,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some((idx, chr)) = self.chars.peek() {
            println!("lexing at {idx}");
            let token = match chr {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '0'..='9' => return self.lex_number(),
                _ => None,
            };

            // Pass over the current token to ready the lexer for the next token
            self.position += chr.len_utf8();
            self.chars.next();

            return token;
        }

        None
    }

    pub fn lex_number(&mut self) -> Option<Token> {
        let mut chrs = vec![];

        while let Some((_, chr)) = self.chars.peek() {
            // If the upcoming character is a numeric, we can just keep rolling.
            if !chr.is_digit(10) {
                break;
            }

            chrs.push(chr.to_string());
            self.chars.next();
        }

        if chrs.len() == 0 {
            panic!("Somehow, a zero-width lex_number() occurred!");
        }

        self.position += chrs.len();

        return Some(Token::Number(chrs.join("").parse::<usize>().unwrap()));
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_input() {
        let input = "++--";
        let tokens: Vec<Token> = Lexer::new(&input).collect();

        let expected = vec![Token::Plus, Token::Plus, Token::Minus, Token::Minus];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_numeric_inputs() {
        let input = "10+20-30";
        let tokens: Vec<Token> = Lexer::new(&input).collect();

        let expected = vec![
            Token::Number(10),
            Token::Plus,
            Token::Number(20),
            Token::Minus,
            Token::Number(30),
        ];

        assert_eq!(tokens, expected);
    }
}
