use std::fs::File;
use std::io::{self, BufRead};

fn find_direction(direction: &str, number: u64, mut password: u64) -> u64 {
    if direction == "R" {
        password = add_direction(number, password);
    } else {
        password = subtract_direction(number, password);
    }

    password
}

fn subtract_direction(number: u64, password: u64) -> u64 {
    if number > 99 {
        subtract_direction( number - 99, password);
    };
    println!("{password}");
    password - number
}

fn add_direction(number: u64, password: u64) -> u64 {
    if number > 99 {
        add_direction(number - 99, password);
    };
    println!("{password}");
    password + number
}

fn find_password<T: BufRead>(reader: T) -> u64 {
    let password: u64 = 50;
    let base: u64 = 0;
    let mut answer: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let direction = &line[0..1];
        let number: u64 = match &line[1..].trim().parse() {
        Ok(num) => *num,
        Err(_) => continue,
        };

        let password = find_direction(&direction, number, password);
        if password == base {
            answer += 1;
        }
    }

    answer
}

fn main() {
    let file = File::open("input").expect("Failed to Read File.");
    let reader = io::BufReader::new(file);

    let password = find_password(reader);
}