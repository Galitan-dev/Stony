use nom::{
    branch::alt, combinator::map, error::VerboseError, multi::many0, sequence::tuple, IResult,
};

use crate::{
    factor_parsers::factor,
    operator_parsers::{division_operator, multiplication_operator},
    tokens::Token,
    utils::ws,
};

pub fn term<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(
        tuple((
            factor,
            many0(tuple((
                alt((multiplication_operator, division_operator)),
                factor,
            ))),
        )),
        |(left, right)| Token::Term {
            left: Box::new(left),
            right,
        },
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let result = term("3*4");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term("(3*4)*2");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_really_nested_term() {
        let result = term("(3*4)*2)");
        assert_eq!(result.is_ok(), true);
    }
}
