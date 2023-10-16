use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

use crate::mapping::OP_MAP;

#[derive(Debug)]
pub struct Instruction {
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

    pub fn obj_code(&self, symtab: &Vec<(String, usize)>) -> String {
        match self.opcode.as_str() {
            "START" => "".to_owned(),
            "END" => "".to_owned(),

            "BYTE" => {
                if let Some(operand) = &self.operand {
                    match operand.split("'").collect::<Vec<_>>()[..] {
                        ["X", hex, _] => hex.to_owned(),
                        ["C", chars, _] => {
                            chars.chars().map(|c| format!("{:02X}", c as u8)).collect()
                        }
                        _ => {
                            eprintln!("Error: Invalid operand \"{}\"", operand);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: Missing operand for \"BYTE\" instruction");
                    process::exit(1);
                }
            }

            "WORD" => {
                if let Some(operand) = &self.operand {
                    format!("{:06X}", usize::from_str_radix(operand, 10).unwrap())
                } else {
                    eprintln!("Error: Missing operand for \"WORD\" instruction");
                    process::exit(1);
                }
            }

            "RESB" => "".to_owned(),
            "RESW" => "".to_owned(),

            _ => {
                if let Some(opcode) = OP_MAP.get(self.opcode.as_str()) {
                    let mut is_x = false;

                    let loc = if let Some(operand) = &self.operand {
                        let operand = match operand.split(",").collect::<Vec<_>>()[..] {
                            [operand, "X"] => {
                                is_x = true;
                                operand
                            }

                            [operand] => operand,

                            _ => {
                                eprintln!("Error: Invalid operand \"{}\"", operand);
                                process::exit(1);
                            }
                        };

                        if let Some((_, loc)) = symtab.iter().find(|(sym, _)| sym == operand) {
                            *loc + if is_x { 0x8000 } else { 0 }
                        } else {
                            eprintln!("Error: Invalid symbol \"{}\"", operand);
                            process::exit(1);
                        }
                    } else {
                        0
                    };

                    format!("{}{:04X}", opcode, loc)
                } else {
                    eprintln!("Error: Invalid opcode \"{}\"", self.opcode);
                    process::exit(1);
                }
            }
        }
    }

    pub fn parse_into_vec(source: BufReader<File>) -> Vec<Self> {
        let mut source = source.lines().map(|l| {
            l.unwrap()
                .to_uppercase()
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        });

        let mut instructions = Vec::new();

        let mut loc = if let [symbol, opcode, operand] = &source.next().unwrap()[..] {
            if opcode == "START" {
                let loc = usize::from_str_radix(&operand, 16).unwrap();
                instructions.push(Instruction::new(
                    loc,
                    Some(symbol.to_owned()),
                    opcode.to_owned(),
                    Some(operand.to_owned()),
                ));

                loc
            } else {
                eprintln!("Error: Missing instruction \"START\" on first line");
                process::exit(1);
            }
        } else {
            eprintln!("Error: Missing instruction \"START\" on first line");
            process::exit(1);
        };

        for line in source {
            // Check for instruction overflow
            if loc > std::i16::MAX as usize {
                eprintln!("Error: Instruction overflow");
                process::exit(1);
            }

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
                                _ => {
                                    eprintln!("Error: Invalid operand \"{}\"", operand);
                                    process::exit(1);
                                }
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

                [] => continue,

                _ => unreachable!(),
            };
        }

        instructions
    }

    pub fn generate_objcode(mut instructions: Vec<Self>, mut obj_codes: Vec<String>) -> String {
        let end = (instructions.pop().unwrap(), obj_codes.pop().unwrap());

        let mut iter = instructions.iter().zip(obj_codes);
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

        [line_start, line_texts.join("\n"), line_end].join("\n")
    }
}
