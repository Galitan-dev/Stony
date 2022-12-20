use crate::{operand_parsers::operand, operator_parsers::operator, tokens::Token};
use nom::types::CompleteStr;

named!(parenthised_expresion<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("(") >>
            expr: expression >>
            tag!(")") >>
            (
                expr
            )
        )
    )
);

named!(pub expression<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: ws!(alt!(operand | parenthised_expresion)) >>
            op: ws!(operator) >>
            right: ws!(alt!(operand | parenthised_expresion)) >>
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
