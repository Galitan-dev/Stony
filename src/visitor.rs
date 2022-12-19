use crate::tokens::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

pub struct Compiler{
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    assembly: Vec<String>
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut free_registers = Vec::new();
        for i in 0..31 {
            free_registers.push(i);
        }
        free_registers.reverse();
        Compiler{
            free_registers: free_registers,
            used_registers: Vec::new(),
            assembly: Vec::new()
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::AdditionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let line = format!("ADD ${left_register} ${right_register} ${result_register}");
                self.assembly.push(line);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::SubtractionOperator => {},
            &Token::MultiplicationOperator => {},
            &Token::DivisionOperator => {},
            &Token::Integer{ value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${next_register} #{value}");
                self.used_registers.push(next_register);
                self.assembly.push(line);
            },
            &Token::Expression{ ref left, ref op, ref right } => {
                self.visit_token(left);
                self.visit_token(right);
                self.visit_token(op);
            },
            &Token::Program{ ref expressions } => {
                for expression in expressions {
                    self.visit_token(expression);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::CompleteStr;
    use crate::program_parser::program;

    fn generate_test_program() -> Token {
        let source = CompleteStr("1+2");
        let (_, tree) = program(source).unwrap();
        tree
    }

    #[test]
    fn test_visit_addition_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program();
        compiler.visit_token(&test_program);
        let assembly = compiler.assembly.join("\n");
        assert_eq!(assembly, [
            "LOAD $0 #1",
            "LOAD $1 #2",
            "ADD $0 $1 $2"
        ].join("\n").as_str())
    }

}