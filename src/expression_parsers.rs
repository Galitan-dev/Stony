use nom::types::CompleteStr;
use crate::{
    tokens::Token,
    operand_parsers::operand,
    operator_parsers::operator
};

named!(pub expression<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: ws!(operand) >>
            op: ws!(operator) >>
            right: ws!(operand) >>
            (
                Token::Expression { 
                    left: Box::new(left), 
                    op: Box::new(op), 
                    right: Box::new(right), 
                }
            )
        )
    )
);