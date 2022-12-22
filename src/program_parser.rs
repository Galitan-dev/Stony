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
    use crate::tokens::Token;

    #[test]
    fn test_parse_simple_program() {
        let test_program = "1 + 2";
        let result = program(test_program);
        assert_eq!(
            result,
            Ok((
                "",
                Token::Program {
                    expressions: vec![Token::Expression {
                        left: Box::new(Token::Integer { value: 1 }),
                        op: Box::new(Token::AdditionOperator),
                        right: Box::new(Token::Integer { value: 2 }),
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_parse_complex_program() {
        let test_program = "(5 + (2 * 6)) / 4";
        let result = program(test_program);
        assert_eq!(
            result,
            Ok((
                "",
                Token::Program {
                    expressions: vec![Token::Expression {
                        left: Box::new(Token::Expression {
                            left: Box::new(Token::Integer { value: 5 }),
                            op: Box::new(Token::AdditionOperator),
                            right: Box::new(Token::Expression {
                                left: Box::new(Token::Integer { value: 2 }),
                                op: Box::new(Token::MultiplicationOperator),
                                right: Box::new(Token::Integer { value: 6 }),
                            })
                        }),
                        op: Box::new(Token::DivisionOperator),
                        right: Box::new(Token::Integer { value: 4 }),
                    }]
                }
            ))
        );
    }
}
