use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn open_input_file() -> BufReader<File> {
    let file = File::open("input").unwrap();
    return BufReader::new(file)
}

fn drop_differing_chars(id1: &String, id2: &String, id_length: usize) -> String {
    let mut dropped = String::new();
    let mut id1_chars = id1.chars();
    let mut id2_chars = id2.chars();
    
    for _i in 0..id_length {
        let c1 = id1_chars.next().unwrap();
        let c2 = id2_chars.next().unwrap();
        
        // Beware! Unicode scalar chars! not grapheme clusters!
        if c1 == c2 {
            dropped.push(c1);
        }
    }
    dropped
}

fn find_similar_id(box_ids: &mut Vec<String>) -> String {
    let id_length = match box_ids.get(0) {
        Some(s) => s.chars().count(),
        None => 0,
    };
    println!("id_length {}", id_length);
    loop {
        let id1 = match box_ids.pop(){
            Some(s) => s,
            None => String::from("empty")
        };
        // Borrow a slice of the complete vector
        for id2 in &box_ids[..] {
            let s = drop_differing_chars(&id1, id2, id_length);
            if s.chars().count() == id_length - 1 {
                return s;
            }
        }
        if box_ids.is_empty() {
            break;
        }
    }
    String::from("No box ID found")
}

fn main() {
    let buf_reader = open_input_file();
    
    let mut two_letters_in_id = 0;
    let mut three_letters_in_id = 0;
    
    let mut box_ids: Vec<String> = Vec::new();
    
    for (_nr, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
        let mut char_counts = HashMap::new();
        for c in l.chars() {
            if !char_counts.contains_key(&c) {
                char_counts.insert(c,1);
            } else {
                let counter = match char_counts.get(&c) {
                    Some(i) => i+1,
                    None => 0,
                };
                char_counts.insert(c, counter);
            }
        }
        let mut two_letters_found = 0;
        let mut three_letters_found = 0;
        for (_key, value) in char_counts {
            if value == 2 {
               two_letters_found = 1;
            }
            if value == 3 {
                three_letters_found = 1;
            }
        }
        two_letters_in_id = two_letters_in_id + two_letters_found;
        three_letters_in_id = three_letters_in_id + three_letters_found;
        
        box_ids.push(l)
    }
    
    println!("The checksum is {}", two_letters_in_id*three_letters_in_id);
    
    println!("The similar box IDs are: {}", find_similar_id(&mut box_ids));
}
