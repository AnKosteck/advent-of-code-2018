use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn convert_str_to_uint(s: &str) -> u32 {
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Conversion error of {}", s);
            0
        },
    }
}

fn open_input_file() -> BufReader<File> {
    let file = File::open("input").unwrap();
    return BufReader::new(file)
}

// One 
fn unit_reaction(l: &String) -> Option<String>{
let mut l_iter = l.chars();
let mut s: String = String::with_capacity(l.capacity);
let mut first = l_iter.next();
if first != None {
                loop {
                    let mut second = l_iter.next();
                    if second != None {
                        // Do the comparison
                        // Wenn first und second selber char sind und dennoch unterschiedlich => erhoehe i+1 und ein next mehr
                        // else: first = second
                        if first.unwrap().to_ascii_lowercase() == second.unwrap().to_ascii_lowercase() && first.unwrap() != second.unwrap() {
                            first = l_iter.next();
                            no_reaction_found = false;
                        } else {
                            s.push(first.unwrap());
                            first = second;
                        }
                    } else {    // reached the end
                        s.push(first.unwrap());
                        break;
                    }
                }
                return s;
                }
                None
                }
fn main() {
    let buf_reader = open_input_file();

    for (nr, line) in buf_reader.lines().enumerate() {
        let mut l = line.unwrap();
        let mut s: String = String::with_capacity(l.capacity());
        
        let mut l_iter = l.chars();
        //First iteration
        let mut first = l_iter.next();
        
        if first != None {
            let mut no_reaction_found = false;
            while !no_reaction_found {
                no_reaction_found = true;

                if !no_reaction_found {
                    let mut l = String::with_capacity(s.capacity());
                    l.push_str(&s.as_str());
                    let mut l_iter = l.chars();
                    s = String::with_capacity(l.capacity());
                }
            }
        } else {
            println!("Empty string! {}",&l);
        }
        
        println!("Line {}: {}",nr,s);
    }
}
