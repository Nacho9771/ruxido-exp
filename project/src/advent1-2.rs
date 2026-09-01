use std::fs::File;
use std::io::{self, BufRead};

fn find_direction(direction: &str, number: u64, password: u64) -> (u64, u64) {
    let rotations = number / 100;
    let remainder = number % 100;

    if direction == "R" {
        let new_password = (password + number) % 100;
        let extra = if password + remainder >= 100 {
            1
        } else {
            0
        };
        (new_password, rotations + extra)
    } else {
        let new_password = (password + 100 - remainder) % 100;
        let extra = if password != 0 && remainder >= password {
            1
        } else {
            0
        };

        (new_password, rotations + extra)
    }
}

fn find_password<T: BufRead>(reader: T) -> u64 {
    let mut password: u64 = 50;
    let mut answer: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let direction = &line[0..1];
        let number: u64 = match line[1..].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let (new_password, zero_crossings) =
            find_direction(direction, number, password);

        password = new_password;
        answer += zero_crossings;
    }
    println!("{answer}");
    answer
}

fn main() {
    let file = File::open("input").expect("Failed to Read File.");
    let reader = io::BufReader::new(file);

    find_password(reader);
}