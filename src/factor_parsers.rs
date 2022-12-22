use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    error::VerboseError,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{expression_parsers::expression, tokens::Token, utils::ws};

pub fn factor<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(
        alt((integer, float64, delimited(tag("("), expression, tag(")")))),
        |f| Token::Factor { value: Box::new(f) },
    ))(i)
}

fn float64<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(
        tuple((opt(tag("-")), digit1, preceded(tag("."), digit1))),
        |(sign, left_nums, right_nums): (Option<&str>, &str, &str)| {
            let mut tmp = String::from("");
            if sign.is_some() {
                tmp.push_str("-");
            }
            tmp.push_str(&left_nums.to_string());
            tmp.push_str(".");
            tmp.push_str(&right_nums.to_string());
            let converted = tmp.parse::<f64>().unwrap();
            Token::Float { value: converted }
        },
    ))(i)
}

fn integer<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    ws(map(
        tuple((opt(tag("-")), digit1)),
        |(sign, reg_num): (Option<&str>, &str)| {
            let mut tmp = String::from("");
            if sign.is_some() {
                tmp.push_str("-");
            }
            tmp.push_str(&reg_num.to_string());
            let converted = tmp.parse::<i64>().unwrap();
            Token::Integer { value: converted }
        },
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factor() {
        let test_program = "(1+2)";
        let result = factor(test_program);
        assert_eq!(result.is_ok(), true);
        let (_, tree) = result.unwrap();
        println!("{:#?}", tree);
    }

    #[test]
    fn test_parse_floats() {
        let test_floats = vec!["100.4", "1.02", "-1.02"];
        for o in test_floats {
            let parsed_o = o.parse::<f64>().unwrap();
            let result = float64(o);
            assert_eq!(
                result,
                Ok((
                    "",
                    Token::Float {
                        value: *Box::new(parsed_o)
                    }
                ))
            );
        }
    }

    #[test]
    fn test_parse_integer() {
        let test_integers = vec!["0", "-1", "1"];
        for o in test_integers {
            let parsed_o = o.parse::<i64>().unwrap();
            let result = integer(o);
            assert_eq!(
                result,
                Ok((
                    "",
                    Token::Integer {
                        value: *Box::new(parsed_o)
                    }
                ))
            );
        }
    }
}
