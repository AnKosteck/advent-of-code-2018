use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    pos_left: u32,
    pos_top: u32,
    start: Coordinate,
    end: Coordinate,
    id: u32,
}

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn _distance_from_origin(&self) -> f32 {
        ((self.x as f32).powi(2) + (self.y as f32).powi(2)).sqrt()
    }
    fn _cmp(&self, other: &Coordinate) -> Ordering {
        if self.x == other.x && self.y == other.y {
            return Ordering::Equal;
        }
        if self.x <= other.x {
            if self.y > other.y {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        Ordering::Greater
    }
}

impl Rectangle {
    fn new(width: u32, height: u32, pos_left: u32, pos_top: u32, id: u32) -> Rectangle {
        let start = Coordinate {
            x: pos_left,
            y: pos_top,
        };
        let end = Coordinate {
            x: pos_left + width,
            y: pos_top + height,
        };
        Rectangle {
            width,
            height,
            pos_left,
            pos_top,
            start,
            end,
            id,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn cmp(&self, other: &Rectangle) -> Ordering {
        if self.start.x < other.start.x {
            return Ordering::Less;
        }
        if self.start.x == other.start.x {
            return self.start.y.cmp(&other.start.y);
        }
        Ordering::Greater
    }

    fn construct_overlapping_area(&self, other: &Rectangle) -> Option<Rectangle> {
        //start.x can only be <= other.start.x

        //startx is always <= other.startx, same for starty
        if !(self.end.x <= other.start.x || self.end.y <= other.start.y) {
            // Since other has to lie inside the self rectangle
            // the starting point for the overlapping area starts right there
            let pos_left = other.pos_left;
            let pos_top = other.pos_top;
            let height: u32;
            let width: u32;
            if self.end.x <= other.end.x {
                width = self.end.x - other.start.x;
            } else {
                width = other.width;
            }
            if self.end.y <= other.end.y {
                height = self.end.y - other.start.y;
            } else {
                height = other.height
            }
            return Some(Rectangle::new(width, height, pos_left, pos_top, 0));
        }
        None
    }

    fn overlaps(&self, other: &Rectangle) -> bool {
        if !(self.end.x <= other.start.x || self.end.y <= other.start.y) {
            return true;
        }
        false
    }
}

fn open_input_file() -> BufReader<File> {
    let file = File::open("input").unwrap();
    return BufReader::new(file);
}

// Find all overlapping areas
fn find_overlapping_areas(rectangles: &Vec<Rectangle>) -> Vec<Rectangle> {
    let mut v: Vec<Rectangle> = Vec::new();
    for i in 0..rectangles.len() {
        if let Some(r1) = rectangles.get(i) {
            for r2 in &rectangles[i + 1..] {
                match r1.construct_overlapping_area(r2) {
                    Some(r) => v.push(r),
                    None => continue,
                }
            }
        }
    }

    v.sort_by(|a, b| a.cmp(&b));

    //merge TODO
    let mut v_merged: Vec<Rectangle> = Vec::new();
    while !v.is_empty() {
        let _r1 = v.pop();
        for i in 0..v.len() {
            if let Some(_r2) = v.get(i) {}
        }
        v_merged.push(Rectangle::new(0, 0, 0, 0, 0));
    }

    v_merged
}

fn convert_str_to_uint(s: &str) -> u32 {
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Conversion error of {}", s);
            0
        }
    }
}

fn print_rectangles(rectangles: &Vec<Rectangle>) {
    for i in 0..rectangles.len() {
        if let Some(r) = rectangles.get(i) {
            println!(
                "#{} @ {},{}: {}x{}",
                r.id, r.pos_left, r.pos_top, r.width, r.height
            );
        }
    }
}

fn main() {
    let buf_reader = open_input_file();

    let mut rectangles: Vec<Rectangle> = Vec::new();

    // For now read all claims
    for (nr, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
        let v: Vec<&str> = l.split('@').collect();
        if let Some(s) = v.get(1) {
            let v: Vec<&str> = s.split(':').collect();
            let mut width: u32 = 0;
            let mut height: u32 = 0;
            let mut pos_left: u32 = 0;
            let mut pos_top: u32 = 0;

            if let Some(s) = v.get(0) {
                let position: Vec<&str> = s.split(",").collect();
                if let Some(s_pos_left) = position.get(0) {
                    pos_left = convert_str_to_uint(s_pos_left);
                }
                if let Some(s_pos_top) = position.get(1) {
                    pos_top = convert_str_to_uint(s_pos_top);
                }
            }
            if let Some(s) = v.get(1) {
                let area: Vec<&str> = s.split('x').collect();
                if let Some(s_width) = area.get(0) {
                    width = convert_str_to_uint(s_width);
                }
                if let Some(s_height) = area.get(1) {
                    height = convert_str_to_uint(s_height);
                }
            }
            rectangles.push(Rectangle::new(
                width,
                height,
                pos_left,
                pos_top,
                (nr + 1) as u32,
            ));
        }
    }

    // In ascending order in start x and furhter in start y
    rectangles.sort_by(|a, b| a.cmp(&b));

    let mut id_not_overlapping: u32 = 0;
    let mut v: Vec<u32> = vec![0; 1000 * 1000];
    for r in &rectangles {
        for h in r.pos_top..(r.pos_top + r.height) {
            for w in r.pos_left..(r.pos_left + r.width) {
                v[(h * 1000 + w) as usize] += 1;
            }
        }
    }

    for r in &rectangles {
        let mut overlap_found = false;

        for h in r.pos_top..(r.pos_top + r.height) {
            for w in r.pos_left..(r.pos_left + r.width) {
                if v[(h * 1000 + w) as usize] > 1 {
                    overlap_found = true;
                }
            }
        }

        if !overlap_found {
            id_not_overlapping = r.id;
            break;
        }
    }

    let mut overlapping_area: u64 = 0;
    for p in v {
        if p > 1 {
            overlapping_area += 1;
        }
    }

    //     print_rectangles(&rectangles);

    println!(
        "The overall overlapping area in square inches: {}",
        overlapping_area
    );
    println!(
        "The ID of the one rectangle without overlapping area: {}",
        id_not_overlapping
    );
}
