#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Number(usize),
}
