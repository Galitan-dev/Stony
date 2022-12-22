use crate::tokens::Token;
use nom::{branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, IResult};

pub fn operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    alt((
        addition_operator,
        subtraction_operator,
        multiplication_operator,
        division_operator,
    ))(i)
}

fn addition_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(tag("+"), |_| Token::AdditionOperator)(i)
}

fn subtraction_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(tag("-"), |_| Token::SubtractionOperator)(i)
}

fn multiplication_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(tag("*"), |_| Token::MultiplicationOperator)(i)
}

fn division_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(tag("/"), |_| Token::DivisionOperator)(i)
}
