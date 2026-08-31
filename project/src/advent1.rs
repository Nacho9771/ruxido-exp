use std::fs::File;
use std::io::{self, BufRead};


fn find_password<T: BufRead>(reader: T) -> u64 {
    let mut password: u64 = 50;
    let base: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
    }

    password
}

fn main() {
    let file = File::open("input").expect("Failed to Read File.");
    let reader = io::BufReader::new(file);

    let password = find_password(reader);
}