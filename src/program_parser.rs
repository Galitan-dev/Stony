use nom::types::CompleteStr;
use crate::{
    expression_parsers::expression,
    tokens::Token
};

named!(program<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many1!(expression) >>
            (
                Token::Program {
                    expressions: expressions
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;
    use crate::tokens::Token;
    use super::program;

    #[test]
    fn test_parse_program() {
        let test_program = CompleteStr("1 + 2");
        let result = program(test_program);
        assert_eq!(result, Ok((CompleteStr(""), Token::Program { 
            expressions: vec![
                Token::Expression { 
                    left: Box::new(Token::Integer { value: 1 }), 
                    op: Box::new(Token::AdditionOperator), 
                    right: Box::new(Token::Integer { value: 2 }),
                }
           ]
        })));
    }
}