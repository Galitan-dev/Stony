use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    error::VerboseError,
    sequence::tuple,
    IResult,
};

use crate::tokens::Token;

pub fn operand<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    integer(i)
}

fn integer<'a>(i: &'a str) -> IResult<&'a str, Token, VerboseError<&'a str>> {
    map(
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
    )(i)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Token;

    #[test]
    fn test_parse_integer() {
        let test_integers = vec!["0", "-1", "1"];
        for o in test_integers {
            let parsed_o = o.parse::<i64>().unwrap();
            let result = integer(o);
            assert!(result.is_ok());
            let (_, token) = result.unwrap();
            assert_eq!(token, Token::Integer { value: parsed_o });
        }
    }
}
