use std::fmt;

use super::InstructionOpcodeOnly;
use super::InstructionOpcodeOperand;
use super::InstructionSymbolOpcodeOperand;

#[derive(Debug)]
pub enum InstructionType {
    SymbolOpcodeOperand(InstructionSymbolOpcodeOperand),
    OpcodeOperand(InstructionOpcodeOperand),
    OpcodeOnly(InstructionOpcodeOnly),
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionType::SymbolOpcodeOperand(ins) => write!(f, "{}", ins),
            InstructionType::OpcodeOperand(ins) => write!(f, "{}", ins),
            InstructionType::OpcodeOnly(ins) => write!(f, "{}", ins),
        }
    }
}
