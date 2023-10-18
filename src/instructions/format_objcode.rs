use super::InstructionType;

pub fn format_objcode(instructions: &Vec<InstructionType>, objcodes: Vec<String>) -> Vec<String> {
    let mut lines = Vec::new();

    let mut pname = String::new();
    let mut paddr = 0;
    let mut temp = String::new();
    for (ins, objcode) in instructions.iter().zip(objcodes) {
        match ins {
            InstructionType::SymbolOpcodeOperand(ins) => match ins.opcode.as_str() {
                "START" => {
                    pname = ins.symbol.to_owned();
                    paddr = ins.addr;
                }
                "RESB" | "RESW" => {
                    if !temp.is_empty() {
                        let len = temp.len() >> 1;
                        lines.push(format!("T{:06X}{:02X}{}", ins.addr - len, len, temp));
                    }
                    temp = objcode;
                }
                _ => {
                    if temp.len() + objcode.len() > 60 {
                        let len = temp.len() >> 1;
                        lines.push(format!("T{:06X}{:02X}{}", ins.addr - len, len, temp));
                        temp = objcode;
                    } else {
                        temp.push_str(&objcode);
                    }
                }
            },

            InstructionType::OpcodeOperand(ins) => match ins.opcode.as_str() {
                "END" => {
                    if !temp.is_empty() {
                        let len = temp.len() >> 1;
                        lines.push(format!("T{:06X}{:02X}{}", ins.addr - len, len, temp));
                    }
                    lines.insert(
                        0,
                        format!("H{:<6}{:06X}{:06X}", pname, paddr, ins.addr - paddr),
                    );
                    lines.push(format!("E{:06X}", paddr));
                }
                _ => {
                    if temp.len() + objcode.len() > 60 {
                        let len = temp.len() >> 1;
                        lines.push(format!("T{:06X}{:02X}{}", ins.addr - len, len, temp));
                        temp = objcode;
                    } else {
                        temp.push_str(&objcode);
                    }
                }
            },

            InstructionType::OpcodeOnly(ins) => {
                if temp.len() + objcode.len() > 60 {
                    let len = temp.len() >> 1;
                    lines.push(format!("T{:06X}{:02X}{}", ins.addr - len, len, temp));
                    temp = objcode;
                } else {
                    temp.push_str(&objcode);
                }
            }
        }
    }

    lines
}
