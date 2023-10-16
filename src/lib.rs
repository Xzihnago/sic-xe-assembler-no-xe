use std::{
    fs::{self, File},
    io::{BufReader, Write},
};

mod instruction;
pub mod loader;
mod mapping;

use instruction::Instruction;

pub fn parse_sic(source: BufReader<File>) {
    let instructions = Instruction::parse_into_vec(source);

    let symtab: Vec<(String, usize)> = instructions
        .iter()
        .filter(|ins| ins.symbol.is_some())
        .map(|ins| (ins.symbol.clone().unwrap(), ins.loc))
        .collect();

    let obj_codes = instructions
        .iter()
        .map(|ins| ins.obj_code(&symtab))
        .collect::<Vec<_>>();

    // Write loc table
    let mut loc_file = fs::File::create("loc.txt").unwrap();
    for ins in &instructions {
        writeln!(loc_file, "{}", ins).unwrap();
    }

    // Write output table
    let mut output_file = fs::File::create("output.txt").unwrap();
    for (ins, code) in instructions.iter().zip(&obj_codes) {
        writeln!(output_file, "{}\t{}", ins, code).unwrap();
    }

    // Write objectcode code
    let objectcode = Instruction::generate_objcode(instructions, obj_codes);
    let mut objectcode_file = fs::File::create("objectcode.txt").unwrap();
    writeln!(objectcode_file, "{}", objectcode).unwrap();
}
