use crate::{operand_parsers::operand, operator_parsers::operator, tokens::Token, utils::ws};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::VerboseError,
    sequence::{delimited, tuple},
    IResult,
};

pub fn expression<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(
        tuple((ws(inner_expression), ws(operator), ws(inner_expression))),
        |(left, op, right)| Token::Expression {
            left: Box::new(left),
            op: Box::new(op),
            right: Box::new(right),
        },
    )(i)
}

pub fn inner_expression<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    alt((operand, delimited(tag("("), expression, tag(")"))))(i)
}
