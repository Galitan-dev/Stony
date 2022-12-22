use nom::{
    branch::alt, combinator::map, error::VerboseError, multi::many0, sequence::tuple, IResult,
};

use crate::{
    operator_parsers::{addition_operator, subtraction_operator},
    term_parsers::term,
    tokens::Token,
};

pub fn expression<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(
        tuple((
            term,
            many0(tuple((
                alt((addition_operator, subtraction_operator)),
                term,
            ))),
        )),
        |(left, right)| Token::Expression {
            left: Box::new(left),
            right,
        },
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let result = expression("3*4");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_expression() {
        let result = expression("(3*4)+1");
        assert_eq!(result.is_ok(), true);
    }
}
