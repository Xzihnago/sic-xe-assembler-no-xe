use std::{fs::File, io::BufReader, process};

pub fn read_asm_file(path: &str) -> BufReader<File> {
    match File::open(path) {
        Ok(source) => BufReader::new(source),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(error.raw_os_error().unwrap_or_default());
        }
    }
}
