use std::fmt;

#[derive(Debug)]
pub struct InstructionOpcodeOnly {
    pub line: usize,
    pub addr: usize,
    pub opcode: String,
}

impl fmt::Display for InstructionOpcodeOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}\t\t{}\t", self.addr, self.opcode)
    }
}

impl InstructionOpcodeOnly {
    pub fn new(line: usize, addr: usize, opcode: impl Into<String>) -> Self {
        Self {
            line,
            addr,
            opcode: opcode.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_opcode_only() {
        let instruction = InstructionOpcodeOnly::new(1, 0, "RSUB");
        assert_eq!(format!("{}", instruction), "0000\t\tRSUB\t");
    }
}
