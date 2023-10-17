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
    let mut iter = source.lines().map(|l| {
        l.to_uppercase()
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    });

    let start = iter.next().unwrap();
    let mut loc = usize::from_str_radix(&start[2], 16).unwrap();

    instructions.push(Instruction::new(
        loc,
        Some(start[0].to_owned()),
        start[1].to_owned(),
        Some(start[2].to_owned()),
    ));
    for line in iter {
        match &line[..] {
            [symbol, opcode, operand] => {
                instructions.push(Instruction::new(
                    loc,
                    Some(symbol.to_owned()),
                    opcode.to_owned(),
                    Some(operand.to_owned()),
                ));

                match opcode.as_str() {
                    "BYTE" => {
                        match operand.split("'").collect::<Vec<_>>()[..] {
                            ["X", hex, _] => loc += hex.len() >> 1,
                            ["C", chars, _] => loc += chars.len(),
                            _ => unreachable!(),
                        };
                    }
                    "WORD" => loc += 3,
                    "RESB" => loc += usize::from_str_radix(&operand, 10).unwrap(),
                    "RESW" => loc += 3 * usize::from_str_radix(&operand, 10).unwrap(),
                    _ => loc += 3,
                }
            }

            [opcode, operand] => {
                instructions.push(Instruction::new(
                    loc,
                    None,
                    opcode.to_owned(),
                    Some(operand.to_owned()),
                ));

                loc += 3;
            }

            [opcode] => {
                instructions.push(Instruction::new(loc, None, opcode.to_owned(), None));

                loc += 3;
            }

            _ => unreachable!(),
        };
    }

    let symtab = instructions
        .iter()
        .filter(|ins| ins.symbol.is_some())
        .map(|ins| (ins.symbol.clone().unwrap(), ins.loc))
        .collect::<Vec<_>>();

    // Pass 2
    let mut objcodes = Vec::new();

    for ins in &instructions {
        let objcode = match ins.opcode.as_str() {
            "START" => "".to_owned(),
            "END" => "".to_owned(),
            "BYTE" => match ins.operand.as_ref().unwrap().split("'").collect::<Vec<_>>()[..] {
                ["X", hex, _] => hex.to_owned(),
                ["C", chars, _] => chars.chars().map(|c| format!("{:02X}", c as u8)).collect(),
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

    // Write loc table
    let mut loc_file = fs::File::create("loc.txt").unwrap();
    for ins in &instructions {
        writeln!(loc_file, "{}", ins).unwrap();
    }

    // Write output table
    let mut output_file = fs::File::create("output.txt").unwrap();
    for (ins, code) in instructions.iter().zip(&objcodes) {
        writeln!(output_file, "{}\t{}", ins, code).unwrap();
    }

    // Write object code
    let mut objcode_file = fs::File::create("objectcode.txt").unwrap();
    let end = (instructions.pop().unwrap(), objcodes.pop().unwrap());

    let mut iter = instructions.iter().zip(objcodes);
    let start = iter.next().unwrap();

    let line_start = format!(
        "H{:<6}{:06X}{:06X}",
        start.0.symbol.as_ref().unwrap(),
        start.0.loc,
        end.0.loc - start.0.loc
    );

    let mut line_texts = Vec::new();
    while let Some((ins, code)) = iter.next() {
        let loc = ins.loc;

        let mut text = code;
        while text.len() <= 54 {
            if let Some((ins, code)) = iter.next() {
                if ins.opcode == "RESW" || ins.opcode == "RESB" {
                    break;
                } else {
                    text.push_str(&code);
                }
            } else {
                break;
            }
        }

        if text.len() == 0 {
            continue;
        }

        line_texts.push(format!("T{:06X}{:02X}{}", loc, text.len() >> 1, text));
    }

    let line_end = format!("E{:06X}", start.0.loc);

    writeln!(
        objcode_file,
        "{}\n{}\n{}",
        line_start,
        line_texts.join("\n"),
        line_end
    )
    .unwrap();
}

struct Instruction {
    pub loc: usize,
    pub symbol: Option<String>,
    pub opcode: String,
    pub operand: Option<String>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.loc, &self.symbol, &self.opcode, &self.operand) {
            (loc, Some(symbol), opcode, Some(operand)) => {
                write!(f, "{:04X}\t{}\t{}\t{}", loc, symbol, opcode, operand)
            }
            (loc, None, opcode, Some(operand)) => {
                write!(f, "{:04X}\t\t{}\t{}", loc, opcode, operand)
            }
            (loc, None, opcode, None) => write!(f, "{:04X}\t\t{}\t", loc, opcode),
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    pub fn new(
        loc: usize,
        symbol: Option<String>,
        opcode: String,
        operand: Option<String>,
    ) -> Self {
        Self {
            loc,
            symbol,
            opcode,
            operand,
        }
    }
}
