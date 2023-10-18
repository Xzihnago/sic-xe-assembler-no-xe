use std::fmt;

#[derive(Debug)]
pub struct InstructionOpcodeOperand {
    pub line: usize,
    pub addr: usize,
    pub opcode: String,
    pub operand: String,
}

impl fmt::Display for InstructionOpcodeOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}\t\t{}\t{}", self.addr, self.opcode, self.operand)
    }
}

impl InstructionOpcodeOperand {
    pub fn new(
        line: usize,
        addr: usize,
        opcode: impl Into<String>,
        operand: impl Into<String>,
    ) -> Self {
        Self {
            line,
            addr,
            opcode: opcode.into(),
            operand: operand.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_opcode_operand() {
        let instruction = InstructionOpcodeOperand::new(1, 0, "LDA", "#4096");
        assert_eq!(format!("{}", instruction), "0000\t\tLDA\t#4096");
    }
}
