use std::cmp::Ordering;

use crate::lexer::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Power,
    Negative,
    Sentinel,
}

impl Operator {
    pub fn cmp_val(&self) -> usize {
        match self {
            Operator::Power => 6,
            Operator::Negative => 4,
            Operator::Multiply => 5,
            Operator::Divide => 5,
            Operator::Add => 3,
            Operator::Subtract => 3,
            Operator::Sentinel => 1,
        }
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s = self.cmp_val();
        let ot = other.cmp_val();
        if s > ot {
            return Ordering::Greater;
        }
        if s < ot {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

impl TryFrom<Token> for Operator {
    type Error = &'static str;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Multiply => Ok(Operator::Multiply),
            Token::Minus => Ok(Operator::Subtract),
            Token::Div => Ok(Operator::Divide),
            Token::Pow => Ok(Operator::Power),
            _ => Err("Can only convert operators"),
        }
    }
}

impl TryFrom<&Token> for Operator {
    type Error = &'static str;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(Operator::Add),
            Token::Multiply => Ok(Operator::Multiply),
            Token::Minus => Ok(Operator::Subtract),
            Token::Div => Ok(Operator::Divide),
            Token::Pow => Ok(Operator::Power),
            _ => Err("Can only convert operators"),
        }
    }
}
