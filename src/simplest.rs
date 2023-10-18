use std::{collections::HashMap, fmt, fs, io::Write};

fn main() {
    let op_map = HashMap::from([
        ("ADD", "18"),
        ("ADDF", "58"),
        ("ADDR", "90"),
        ("AND", "40"),
        ("CLEAR", "B4"),
        ("COMP", "28"),
        ("COMPF", "88"),
        ("COMPR", "A0"),
        ("DIV", "24"),
        ("DIVF", "64"),
        ("DIVR", "9C"),
        ("FIX", "C4"),
        ("FLOAT", "C0"),
        ("HIO", "F4"),
        ("J", "3C"),
        ("JEQ", "30"),
        ("JGT", "34"),
        ("JLT", "38"),
        ("JSUB", "48"),
        ("LDA", "00"),
        ("LDB", "68"),
        ("LDCH", "50"),
        ("LDF", "70"),
        ("LDL", "08"),
        ("LDS", "6C"),
        ("LDT", "74"),
        ("LDX", "04"),
        ("LPS", "D0"),
        ("MUL", "20"),
        ("MULF", "60"),
        ("MULR", "98"),
        ("NORM", "C8"),
        ("OR", "44"),
        ("RD", "D8"),
        ("RMO", "AC"),
        ("RSUB", "4C"),
        ("SHIFTL", "A4"),
        ("SHIFTR", "A8"),
        ("SIO", "F0"),
        ("SSK", "EC"),
        ("STA", "0C"),
        ("STB", "78"),
        ("STCH", "54"),
        ("STF", "80"),
        ("STI", "D4"),
        ("STL", "14"),
        ("STS", "7C"),
        ("STSW", "E8"),
        ("STT", "84"),
        ("STX", "10"),
        ("SUB", "1C"),
        ("SUBF", "5C"),
        ("SUBR", "94"),
        ("SVC", "B0"),
        ("TD", "E0"),
        ("TIO", "F8"),
        ("TIX", "2C"),
        ("TIXR", "B8"),
        ("WD", "DC"),
    ]);

    let mut instructions = Vec::new();

    let source = fs::read_to_string("input.txt").unwrap();

    // Pass 1
    let mut iter = source.lines().filter(|l| !l.starts_with(".")).map(|l| {
        l.to_uppercase()
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    });

    let start = iter.next().unwrap();
    let mut addr = usize::from_str_radix(&start[2], 16).unwrap();

    instructions.push(Instruction::new(
        addr,
        Some(start[0].to_owned()),
        start[1].to_owned(),
        Some(start[2].to_owned()),
    ));
    for line in iter {
        match &line[..] {
            [symbol, opcode, operand] => {
                instructions.push(Instruction::new(
                    addr,
                    Some(symbol.to_owned()),
                    opcode.to_owned(),
                    Some(operand.to_owned()),
                ));

                match opcode.as_str() {
                    "BYTE" => {
                        match operand.split("'").collect::<Vec<_>>()[..] {
                            ["X", hex, ""] => addr += hex.len() >> 1,
                            ["C", chars, ""] => addr += chars.len(),
                            _ => unreachable!(),
                        };
                    }
                    "WORD" => addr += 3,
                    "RESB" => addr += usize::from_str_radix(&operand, 10).unwrap(),
                    "RESW" => addr += 3 * usize::from_str_radix(&operand, 10).unwrap(),
                    _ => addr += 3,
                }
            }

            [opcode, operand] => {
                instructions.push(Instruction::new(
                    addr,
                    None,
                    opcode.to_owned(),
                    Some(operand.to_owned()),
                ));

                addr += 3;
            }

            [opcode] => {
                instructions.push(Instruction::new(addr, None, opcode.to_owned(), None));

                addr += 3;
            }

            _ => unreachable!(),
        };
    }

    let symtab = instructions
        .iter()
        .filter(|ins| ins.symbol.is_some())
        .map(|ins| (ins.symbol.clone().unwrap(), ins.addr))
        .collect::<Vec<_>>();

    // Write addr table
    let mut addr_file = fs::File::create("loc.txt").unwrap();
    for ins in &instructions {
        writeln!(addr_file, "{}", ins).unwrap();
    }

    // Pass 2
    let mut objcodes = Vec::new();

    for ins in &instructions {
        let objcode = match ins.opcode.as_str() {
            "START" => "".to_owned(),
            "END" => "".to_owned(),
            "BYTE" => match ins.operand.as_ref().unwrap().split("'").collect::<Vec<_>>()[..] {
                ["X", hex, ""] => hex.to_owned(),
                ["C", chars, ""] => chars.chars().map(|c| format!("{:02X}", c as u8)).collect(),
                _ => unreachable!(),
            },
            "WORD" => {
                format!(
                    "{:06X}",
                    usize::from_str_radix(ins.operand.as_ref().unwrap(), 10).unwrap()
                )
            }
            "RESB" => "".to_owned(),
            "RESW" => "".to_owned(),
            _ => {
                let addr = if let Some(operand) = &ins.operand {
                    let mut is_x = false;

                    let operand = match operand.split(",").collect::<Vec<_>>()[..] {
                        [operand, "X"] => {
                            is_x = true;
                            operand
                        }
                        [operand] => operand,
                        _ => unreachable!(),
                    };

                    let (_, addr) = symtab.iter().find(|(sym, _)| sym == operand).unwrap();
                    addr + if is_x { 0x8000 } else { 0 }
                } else {
                    0
                };

                format!("{}{:04X}", op_map.get(ins.opcode.as_str()).unwrap(), addr)
            }
        };

        objcodes.push(objcode);
    }

    // Write output table
    let mut output_file = fs::File::create("output.txt").unwrap();
    for (ins, code) in instructions.iter().zip(&objcodes) {
        writeln!(output_file, "{}\t{}", ins, code).unwrap();
    }

    // Format objcode
    let mut lines = Vec::new();

    let mut pname = String::new();
    let mut paddr = 0;
    let mut temp = String::new();
    for (ins, objcode) in instructions.iter().zip(objcodes) {
        match ins.opcode.as_str() {
            "START" => {
                pname = ins.symbol.as_ref().unwrap().to_owned();
                paddr = ins.addr;
            }
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
        }
    }

    // Write object code
    let mut objcode_file = fs::File::create("objectcode.txt").unwrap();
    for line in &lines {
        writeln!(objcode_file, "{}", line).unwrap();
    }
}

struct Instruction {
    pub addr: usize,
    pub symbol: Option<String>,
    pub opcode: String,
    pub operand: Option<String>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.addr, &self.symbol, &self.opcode, &self.operand) {
            (addr, Some(symbol), opcode, Some(operand)) => {
                write!(f, "{:04X}\t{}\t{}\t{}", addr, symbol, opcode, operand)
            }
            (addr, None, opcode, Some(operand)) => {
                write!(f, "{:04X}\t\t{}\t{}", addr, opcode, operand)
            }
            (addr, None, opcode, None) => write!(f, "{:04X}\t\t{}\t", addr, opcode),
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    pub fn new(
        addr: usize,
        symbol: Option<String>,
        opcode: String,
        operand: Option<String>,
    ) -> Self {
        Self {
            addr,
            symbol,
            opcode,
            operand,
        }
    }
}
