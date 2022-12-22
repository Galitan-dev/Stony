use crate::{tokens::Token, utils::ws};
use nom::{branch::alt, bytes::complete::tag, combinator::map, error::VerboseError, IResult};

pub fn operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(alt((
        addition_operator,
        subtraction_operator,
        multiplication_operator,
        division_operator,
    )))(i)
}

pub fn addition_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(tag("+"), |_| Token::AdditionOperator))(i)
}

pub fn subtraction_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(tag("-"), |_| Token::SubtractionOperator))(i)
}

pub fn multiplication_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(tag("*"), |_| Token::MultiplicationOperator))(i)
}

pub fn division_operator<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(tag("/"), |_| Token::DivisionOperator))(i)
}
