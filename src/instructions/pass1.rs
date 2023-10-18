use super::InstructionOpcodeOnly;
use super::InstructionOpcodeOperand;
use super::InstructionSymbolOpcodeOperand;
use super::InstructionType;
use super::SymbolMapping;

pub fn pass1(
    source: impl Iterator<Item = String>,
) -> anyhow::Result<(Vec<InstructionType>, Vec<SymbolMapping>)> {
    let mut source = source.map(|l| l.trim().to_uppercase());

    let mut instructions = Vec::new();
    let mut symtab = Vec::new();

    // Line 1
    let mut addr = if let [symbol, "START", operand] = source
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()[..]
    {
        let addr = usize::from_str_radix(&operand, 16)?;

        instructions.push(InstructionType::SymbolOpcodeOperand(
            InstructionSymbolOpcodeOperand::new(1, addr, symbol, "START", operand),
        ));

        addr
    } else {
        return Err(anyhow::anyhow!("Missing START instruction"));
    };

    // Line 2+
    for (line_num, line) in source.enumerate() {
        if line.starts_with('.') {
            continue;
        }

        if addr > 0x7fff {
            return Err(anyhow::anyhow!(
                "error: instruction overflow\n{} | {}",
                line_num,
                line
            ));
        }

        match line.split_whitespace().collect::<Vec<_>>()[..] {
            [symbol, opcode, operand] => {
                instructions.push(InstructionType::SymbolOpcodeOperand(
                    InstructionSymbolOpcodeOperand::new(line_num, addr, symbol, opcode, operand),
                ));

                symtab.push(SymbolMapping::new(symbol, addr));

                match opcode {
                    "BYTE" => {
                        match operand.split("'").collect::<Vec<_>>()[..] {
                            ["X", hex, ""] => addr += hex.len() >> 1,
                            ["C", chars, ""] => addr += chars.len(),
                            _ => {
                                return Err(anyhow::anyhow!(
                                    "error: invalid operand `{}`\n{} | {}",
                                    operand,
                                    line_num,
                                    line
                                ))
                            }
                        };
                    }
                    "RESB" => addr += usize::from_str_radix(&operand, 10)?,
                    "RESW" => addr += 3 * usize::from_str_radix(&operand, 10)?,
                    _ => addr += 3,
                }
            }

            [opcode, operand] => {
                instructions.push(InstructionType::OpcodeOperand(
                    InstructionOpcodeOperand::new(line_num, addr, opcode, operand),
                ));

                addr += 3;
            }

            [opcode] => {
                instructions.push(InstructionType::OpcodeOnly(InstructionOpcodeOnly::new(
                    line_num, addr, opcode,
                )));

                addr += 3;
            }

            _ => {
                return Err(anyhow::anyhow!(
                    "error: invalid instruction\n{} | {}",
                    line_num,
                    line
                ))
            }
        };
    }

    Ok((instructions, symtab))
}
