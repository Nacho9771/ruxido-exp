use std::fs::File;
use std::io::{self, BufRead};

fn find_direction(direction: &str, number: u64, password: u64) -> u64 {
    if direction == "R" {
        (password + 100 - (number % 100)) % 100
    } else {
        (password + number) % 100
    }
}

fn find_password<T: BufRead>(reader: T) -> u64 {
    let mut password: u64 = 50;
    let base: u64 = 0;
    let mut answer: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let direction = &line[0..1];
        let number: u64 = match &line[1..].trim().parse() {
            Ok(num) => *num,
            Err(_) => continue,
        };

        password = find_direction(direction, number, password);
        if password == base {
            answer += 1;
        }
    }

    println!("{answer}");
    answer
}

fn main() {
    let file = File::open("input").expect("Failed to Read File.");
    let reader = io::BufReader::new(file);

    find_password(reader);
}