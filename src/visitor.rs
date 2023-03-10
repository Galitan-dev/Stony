use crate::tokens::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    assembly: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut free_registers = Vec::new();
        for i in 0..31 {
            free_registers.push(i);
        }
        free_registers.reverse();
        Compiler {
            free_registers: free_registers,
            used_registers: Vec::new(),
            assembly: Vec::new(),
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::AdditionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("ADD ${right_register} ${left_register} ${result_register}");
                self.assembly.push(line);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
                self.used_registers.push(result_register);
            }
            &Token::SubtractionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("SUB ${right_register} ${left_register} ${result_register}");
                self.assembly.push(line);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
                self.used_registers.push(result_register);
            }
            &Token::MultiplicationOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("MUL ${right_register} ${left_register} ${result_register}");
                self.assembly.push(line);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
                self.used_registers.push(result_register);
            }
            &Token::DivisionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("DIV ${right_register} ${left_register} ${result_register}");
                self.assembly.push(line);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
                self.used_registers.push(result_register);
            }
            &Token::Expression {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for expression in right {
                    self.visit_token(&expression.1);
                    self.visit_token(&expression.0);
                }
            }
            &Token::Factor { ref value } => {
                self.visit_token(value);
            }
            &Token::Integer { value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${next_register} #{value}");
                self.used_registers.push(next_register);
                self.assembly.push(line);
            }
            &Token::Float { value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${next_register} #{value}");
                self.used_registers.push(next_register);
                self.assembly.push(line);
            }
            &Token::Term {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for expression in right {
                    self.visit_token(&expression.1);
                    self.visit_token(&expression.0);
                }
            }
            &Token::Program { ref expressions } => {
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
    use crate::program_parser::program;

    fn test_program(source: &str) -> String {
        let (_, test_program) = program(source).unwrap();
        let mut compiler = Compiler::new();
        compiler.visit_token(&test_program);
        compiler.assembly.join("\n")
    }

    #[test]
    fn test_visit_simple_program() {
        let assembly = test_program("1+2");
        assert_eq!(
            assembly,
            ["LOAD $0 #1", "LOAD $1 #2", "ADD $0 $1 $2"]
                .join("\n")
                .as_str()
        )
    }

    #[test]
    fn test_visit_complex_program() {
        let assembly = test_program("(5 + (2 * 6)) / 4");
        assert_eq!(
            assembly,
            [
                "LOAD $0 #5",
                "LOAD $1 #2",
                "LOAD $2 #6",
                "MUL $1 $2 $3",
                "ADD $0 $3 $1",
                "LOAD $0 #4",
                "DIV $1 $0 $3"
            ]
            .join("\n")
            .as_str()
        );
    }

    #[test]
    fn test_visit_program_with_priorities() {
        let assembly = test_program("2 + 3 * 5 - 6 / 2");
        assert_eq!(
            assembly,
            [
                "LOAD $0 #2",
                "LOAD $1 #3",
                "LOAD $2 #5",
                "MUL $1 $2 $3",
                "ADD $0 $3 $1",
                "LOAD $0 #6",
                "LOAD $3 #2",
                "DIV $0 $3 $2",
                "SUB $1 $2 $0"
            ]
            .join("\n")
            .as_str()
        );
    }
}
