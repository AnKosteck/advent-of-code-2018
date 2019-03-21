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

// One unit reaction for the whole line
fn unit_reaction(l: &String) -> bool {
    let mut l_iter = l.chars();
    let mut first = l_iter.next();
    let mut no_reaction_found = true;
    if first != None {
        loop {
            let mut second = l_iter.next();
            if second != None {
                //Example: cC, then if both are equal as uppercase then CC or cc will be selected too
                //Thus compare the non uppercased chars too. If they are different then the situation is cC or Cc, not cc or CC
                //But why does this evaluate for everything?
                if first.unwrap().to_ascii_uppercase() == second.unwrap().to_ascii_uppercase()
                    && first.unwrap() != second.unwrap()
                {
                    first = l_iter.next();
                    println!("Dropping {}{}", first.unwrap(), second.unwrap());
                    println!(
                        "Uppercase {}{}",
                        first.unwrap().to_ascii_uppercase(),
                        second.unwrap().to_ascii_uppercase()
                    );
                    println!(
                        "Compare 1: {}",
                        (first.unwrap().to_ascii_uppercase()
                            == second.unwrap().to_ascii_uppercase())
                    );
                    println!("Compare 2: {}", first.unwrap() != second.unwrap());
                    println!(
                        "Complete compare: {}",
                        (first.unwrap().to_ascii_uppercase()
                            == second.unwrap().to_ascii_uppercase())
                            && first.unwrap() != second.unwrap()
                    );
                    no_reaction_found = false;
                    println!(
                        "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
                    );
                } else {
                    first = second;
                }
            } else {
                // reached the end
                break;
            }
        }
    }
    no_reaction_found
}
fn main() {
    let buf_reader = open_input_file();

    for (nr, line) in buf_reader.lines().enumerate() {
        let mut l = line.unwrap();

        let mut no_reaction_found = false;
        while !no_reaction_found {
            no_reaction_found = unit_reaction(&mut l);
        }

        println!("Line {}: {}", nr, l.replace("*", ""));
    }
}
