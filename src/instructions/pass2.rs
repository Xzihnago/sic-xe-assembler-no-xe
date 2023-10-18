use super::InstructionType;
use super::SymbolMapping;
use super::OPCODE_MAP;

pub fn pass2(
    instructions: &Vec<InstructionType>,
    symtab: &Vec<SymbolMapping>,
) -> anyhow::Result<Vec<String>> {
    let mut objcodes = Vec::new();

    for ins_type in instructions {
        let objcode = match ins_type {
            InstructionType::SymbolOpcodeOperand(ins) => match ins.opcode.as_str() {
                "START" => "".to_owned(),
                "BYTE" => match ins.operand.split("'").collect::<Vec<_>>()[..] {
                    ["X", hex, ""] => hex.to_owned(),
                    ["C", chars, ""] => chars.chars().map(|c| format!("{:02X}", c as u8)).collect(),
                    _ => {
                        return Err(anyhow::anyhow!(
                            "error: invalid operand `{}`\n{} | {}",
                            ins.operand,
                            ins.line,
                            ins.opcode
                        ))
                    }
                },
                "WORD" => {
                    format!("{:06X}", usize::from_str_radix(&ins.operand, 10)?)
                }
                "RESB" => "".to_owned(),
                "RESW" => "".to_owned(),
                _ => {
                    if let Some(opcode) = OPCODE_MAP.get(ins.opcode.as_str()) {
                        let mut is_x = false;

                        let operand = match ins.operand.split(",").collect::<Vec<_>>()[..] {
                            [operand, "X"] => {
                                is_x = true;
                                operand
                            }
                            [operand] => operand,
                            _ => unreachable!(),
                        };

                        if let Some(sym) = symtab.iter().find(|sym| sym.symbol == operand) {
                            format!(
                                "{:02X}{:04X}",
                                opcode,
                                sym.addr + if is_x { 0x8000 } else { 0 }
                            )
                        } else {
                            return Err(anyhow::anyhow!(
                                "error: invalid operand `{}`\n{} | {}",
                                ins.operand,
                                ins.line,
                                ins.opcode
                            ));
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "error: invalid opcode\n{} | {}",
                            ins.line,
                            ins.opcode
                        ));
                    }
                }
            },

            InstructionType::OpcodeOperand(ins) => match ins.opcode.as_str() {
                "END" => "".to_owned(),
                _ => {
                    if let Some(opcode) = OPCODE_MAP.get(ins.opcode.as_str()) {
                        let mut is_x = false;

                        let operand = match ins.operand.split(",").collect::<Vec<_>>()[..] {
                            [operand, "X"] => {
                                is_x = true;
                                operand
                            }
                            [operand] => operand,
                            _ => unreachable!(),
                        };

                        if let Some(sym) = symtab.iter().find(|sym| sym.symbol == operand) {
                            format!(
                                "{:02X}{:04X}",
                                opcode,
                                sym.addr + if is_x { 0x8000 } else { 0 }
                            )
                        } else {
                            return Err(anyhow::anyhow!(
                                "error: invalid operand `{}`\n{} | {}",
                                ins.operand,
                                ins.line,
                                ins.opcode
                            ));
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "error: invalid opcode\n{} | {}",
                            ins.line,
                            ins.opcode
                        ));
                    }
                }
            },

            InstructionType::OpcodeOnly(ins) => {
                if let Some(opcode) = OPCODE_MAP.get(ins.opcode.as_str()) {
                    format!("{:02X}0000", opcode)
                } else {
                    return Err(anyhow::anyhow!(
                        "error: invalid opcode\n{} | {}",
                        ins.line,
                        ins.opcode
                    ));
                }
            }
        };

        objcodes.push(objcode);
    }

    Ok(objcodes)
}
