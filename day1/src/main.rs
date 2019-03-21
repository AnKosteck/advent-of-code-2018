use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn loop_changes(changes: &Vec<i32>, starting_freq: i32) -> i32 {
    let mut current_freq = starting_freq;
    let mut seen_frequencies = HashSet::new();
    loop {
        for c in changes {
            let resulting_freq = current_freq + c;
            if seen_frequencies.contains(&resulting_freq) {
                return resulting_freq;
            } else {
                seen_frequencies.insert(resulting_freq);
            }
            current_freq = resulting_freq;
        }
    }
}

fn main() {
    // Open file and read into string, example taken from
    // https://doc.rust-lang.org/std/fs/struct.File.html
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    //     let mut contents = String::new();
    //     buf_reader.read_to_string(&mut contents)?;
    //     assert_eq!(contents, "Hello, world!");
    let mut freq: i32 = 0;
    //     let mut v: Vec<i32> = Vec::new();
    let mut changes: Vec<i32> = Vec::new();
    let mut periodic_freq = 0;
    for (nr, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
        let change: i32 = match l.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        changes.push(change);
        //         freq += change;
        //         println!("Line nr {}, content: {}, resulting freq {}", nr, l, freq);
    }

    println!("first periodic freq {}", loop_changes(&changes, 0));

    //     println!("first periodic freq {}", periodic_freq);
    //     println!("v {:#?}", v);
    //     Ok(())
}
