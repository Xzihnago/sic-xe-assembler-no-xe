use std::fmt;

#[derive(Debug)]
pub struct InstructionSymbolOpcodeOperand {
    pub line: usize,
    pub addr: usize,
    pub symbol: String,
    pub opcode: String,
    pub operand: String,
}

impl fmt::Display for InstructionSymbolOpcodeOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04X}\t{}\t{}\t{}",
            self.addr, self.symbol, self.opcode, self.operand
        )
    }
}

impl InstructionSymbolOpcodeOperand {
    pub fn new(
        line: usize,
        addr: usize,
        symbol: impl Into<String>,
        opcode: impl Into<String>,
        operand: impl Into<String>,
    ) -> Self {
        Self {
            line,
            addr,
            symbol: symbol.into(),
            opcode: opcode.into(),
            operand: operand.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_symbol_opcode_operand() {
        let instruction = InstructionSymbolOpcodeOperand::new(1, 0, "LOOP", "JSUB", "RDREC");
        assert_eq!(format!("{}", instruction), "0000\tLOOP\tJSUB\tRDREC");
    }
}
