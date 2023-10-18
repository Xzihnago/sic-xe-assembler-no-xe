use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

pub mod instructions;
pub mod loader;

pub fn parse_sic(source: BufReader<File>) {
    let pass1 = instructions::pass1(source.lines().map(|l| l.unwrap()));

    match pass1 {
        Ok((instructions, symtab)) => {
            // Write loc table
            let mut loc_file = fs::File::create("loc.txt").unwrap();
            for ins in &instructions {
                writeln!(loc_file, "{}", ins).unwrap();
            }

            let objcodes = instructions::pass2(&instructions, &symtab);

            match objcodes {
                Ok(objcodes) => {
                    // Write output table
                    let mut output_file = fs::File::create("output.txt").unwrap();
                    for (ins, code) in instructions.iter().zip(&objcodes) {
                        writeln!(output_file, "{}\t{}", ins, code).unwrap();
                    }

                    let objectcode = instructions::format_objcode(&instructions, objcodes);

                    // Write objectcode code
                    let mut objectcode_file = fs::File::create("objectcode.txt").unwrap();
                    for line in &objectcode {
                        writeln!(objectcode_file, "{}", line).unwrap();
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }

        Err(error) => {
            println!("{}", error);
        }
    };
}
