use crate::{expression_parsers::expression, tokens::Token, utils::ws};
use nom::{combinator::map, error::VerboseError, multi::many1, IResult};

pub fn program<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(many1(ws(expression)), |expressions| Token::Program {
        expressions,
    })(i)
}

#[cfg(test)]
mod tests {
    use super::program;

    #[test]
    fn test_parse_program() {
        let test_program = "1+2";
        let result = program(test_program);
        assert_eq!(result.is_ok(), true);
    }
}
