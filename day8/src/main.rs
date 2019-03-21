use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn convert_str_to_uint(s: &str) -> u32 {
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Conversion error of {}", s);
            0
        }
    }
}

fn open_input_file() -> BufReader<File> {
    let file = File::open("input").unwrap();
    return BufReader::new(file);
}

fn main() {
    let buf_reader = open_input_file();

    for (nr, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
    }
}
