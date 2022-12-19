use nom::{types::CompleteStr, digit};

use crate::tokens::Token;

named!(pub operand<CompleteStr, Token>,
    ws!(
        alt!(
            integer
        )
    )
);

named!(integer<CompleteStr, Token>,
    ws!(
        do_parse!(
            sign: opt!(tag!("-")) >>
            reg_num: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&reg_num.to_string());
                    let converted = tmp.parse::<i64>().unwrap();
                    Token::Integer{ value: converted }
                }
            )
        )
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Token;
    use nom::types::CompleteStr;

    #[test]
    fn test_parse_integer() {
        let test_integers = vec!["0", "-1", "1"];
        for o in test_integers {
            let parsed_o = o.parse::<i64>().unwrap();
            let result = integer(CompleteStr(o));
            assert!(result.is_ok());
            let (_, token) = result.unwrap();
            assert_eq!(token, Token::Integer{value: parsed_o});
        }
    }

}